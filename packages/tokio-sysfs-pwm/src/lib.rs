use std::{
    io::{self, Write},
    path::PathBuf,
};

use tokio::{
    fs::File,
    io::{AsyncReadExt, AsyncWriteExt},
};

#[derive(Debug)]
pub struct Chip {
    _file: File,
    _export_file: File,
    unexport_file: std::fs::File,
    num: u32,
}

impl Chip {
    /// Opens the pwmchip file and exports it, it is unexported when the chip is dropped
    pub async fn new(chip_num: u32) -> io::Result<Self> {
        asyncify(move || {
            let path = PathBuf::from(format!("/sys/class/pwm/pwmchip{chip_num}"));
            let file = std::fs::OpenOptions::new()
                .read(true)
                .open(format!("/sys/class/pwm/pwmchip{chip_num}"))?;

            let mut export_file = std::fs::OpenOptions::new()
                .write(true)
                .open(path.join("export"))?;
            let unexport_file = std::fs::OpenOptions::new()
                .write(true)
                .open(path.join("unexport"))?;

            export_file.write_all(b"0")?;

            Ok(Self {
                _file: File::from_std(file),
                _export_file: File::from_std(export_file),
                unexport_file,
                num: chip_num,
            })
        })
        .await
    }

    pub async fn pin(&self, pin_num: u32) -> io::Result<Pin<'_>> {
        Ok(Pin {
            chip: self,
            num: pin_num,
        })
    }

    pub fn num(&self) -> u32 {
        self.num
    }
}

impl Drop for Chip {
    fn drop(&mut self) {
        // Attempt to write to unexport file, if it fails, oh well 🤷
        self.unexport_file.write_all(b"0").ok();
    }
}

#[derive(Debug)]
pub struct Pin<'a> {
    /// Ensure pins never outlive the `Chip` they belong to
    chip: &'a Chip,
    num: u32,
}

#[derive(Debug, Clone)]
pub enum Polarity {
    Normal,
    Inverse,
}

impl Polarity {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Normal => "normal",
            Self::Inverse => "inversed",
        }
    }
}

impl<'a> Pin<'a> {
    async fn open_file(&self, name: &str) -> io::Result<File> {
        tokio::fs::OpenOptions::new()
                .read(true)
                .write(true)
                .open(format!(
            "/sys/class/pwm/pwmchip{}/pwm{}/{}",
            self.chip.num, self.num, name
        ))
        .await
    }

    pub async fn enabled(&mut self) -> io::Result<bool> {
        let mut string = String::with_capacity(1);
        self.open_file("enable")
            .await?
            .read_to_string(&mut string)
            .await?;

        match string.parse::<u32>() {
            Ok(num) => match num {
                1 => Ok(true),
                0 => Ok(false),
                _ => unreachable!(
                    "Enable file for chip #{}, pin #{}, contained something other than `1` or `0`",
                    self.chip.num, self.num
                ),
            },
            Err(err) => Err(parse_err(err)),
        }
    }

    pub async fn set_enabled(&mut self, enabled: bool) -> io::Result<()> {
        let contents = if enabled { b"1" } else { b"0" };
        self.open_file("enable").await?.write_all(contents).await
    }

    pub async fn enable(&mut self) -> io::Result<()> {
        self.open_file("enable").await?.write_all(b"1").await
    }

    pub async fn disable(&mut self) -> io::Result<()> {
        self.open_file("enable").await?.write_all(b"0").await
    }

    /// Toggles enabled and returns new state
    pub async fn toggle_enabled(&mut self) -> io::Result<bool> {
        if self.enabled().await? {
            self.disable().await?;
            Ok(false)
        } else {
            self.enable().await?;
            Ok(true)
        }
    }

    pub async fn period_ns(&mut self) -> io::Result<u32> {
        let mut string = String::with_capacity(6);
        self.open_file("period")
            .await?
            .read_to_string(&mut string)
            .await?;

        match string.parse::<u32>() {
            Ok(period) => Ok(period),
            Err(err) => Err(parse_err(err)),
        }
    }

    pub async fn set_period_ns(&mut self, period_ns: u32) -> io::Result<()> {
        self.open_file("period")
            .await?
            .write_all(period_ns.to_string().as_bytes())
            .await
    }

    pub async fn duty_cycle_ns(&mut self) -> io::Result<u32> {
        let mut string = String::with_capacity(6);
        self.open_file("duty_cycle")
            .await?
            .read_to_string(&mut string)
            .await?;

        match string.parse::<u32>() {
            Ok(period) => Ok(period),
            Err(err) => Err(parse_err(err)),
        }
    }

    pub async fn set_duty_cycle_ns(&mut self, duty_cycle_ns: u32) -> io::Result<()> {
        self.open_file("duty_cycle")
            .await?
            .write_all(duty_cycle_ns.to_string().as_bytes())
            .await
    }

    /// Sets the duty cycle as a percentage of the current period
    pub async fn set_duty_cycle_percentage(&mut self, percentage: f32) -> io::Result<()> {
        let period = self.period_ns().await?;
        self.set_duty_cycle_ns(((period as f32) * (percentage / 100.0)).round() as u32)
            .await
    }

    pub async fn polarity(&mut self) -> io::Result<Polarity> {
        let mut string = String::with_capacity(8);
        self.open_file("polarity")
            .await?
            .read_to_string(&mut string)
            .await?;

        match &*string {
            "normal" => Ok(Polarity::Normal),
            "inversed" => Ok(Polarity::Inverse),
            _ => Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Found value not `normal` or `inversed` in polarity for chip #{}, pin #{}",
                    self.chip.num, self.num
                ),
            )),
        }
    }

    pub async fn set_polarity(&mut self, polarity: Polarity) -> io::Result<()> {
        self.open_file("polarity")
            .await?
            .write_all(polarity.as_str().as_bytes())
            .await
    }
}

fn parse_err(err: impl std::error::Error + Send + Sync + 'static) -> io::Error {
    io::Error::new(io::ErrorKind::Other, err)
}

/// Spawn a task on the blocking thread pool, mostly used for pwm access
pub(crate) async fn asyncify<F, T>(f: F) -> std::io::Result<T>
where
    F: FnOnce() -> std::io::Result<T> + Send + 'static,
    T: Send + 'static,
{
    match tokio::task::spawn_blocking(f).await {
        Ok(res) => res,
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "background task failed",
        )),
    }
}
