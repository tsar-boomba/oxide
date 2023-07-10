use std::{
    io::{self, Write},
    marker::PhantomData,
};

use tokio::io::AsyncWriteExt;

use crate::asyncify;

#[derive(Debug)]
pub struct Pin<Dir: Direction> {
    num: u32,
    _state: PhantomData<Dir>,
}

pub trait Direction {
    fn dir() -> &'static str;
}

#[derive(Debug)]
pub struct Input;

impl Direction for Input {
    fn dir() -> &'static str {
        "in"
    }
}

#[derive(Debug)]
pub struct Output;

impl Direction for Output {
    fn dir() -> &'static str {
        "out"
    }
}

impl<Dir: Direction> Pin<Dir> {
    pub async fn new_output(num: u32) -> io::Result<Self> {
        asyncify(move || {
            std::fs::OpenOptions::new()
                .write(true)
                .open("/sys/class/gpio/export")?
                .write_all(num.to_string().as_bytes())?;
            std::fs::OpenOptions::new()
                .write(true)
                .open(format!("/sys/class/gpio/gpio{num}/direction"))?
                .write_all(Output::dir().as_bytes())?;
            Ok(())
        })
        .await?;

        Ok(Self {
            num,
            _state: PhantomData,
        })
    }

    pub async fn new_input(num: u32) -> io::Result<Pin<Input>> {
        asyncify(move || {
            std::fs::OpenOptions::new()
                .write(true)
                .open("/sys/class/gpio/export")?
                .write_all(num.to_string().as_bytes())?;
            std::fs::OpenOptions::new()
                .write(true)
                .open(format!("/sys/class/gpio/gpio{num}/direction"))?
                .write_all(Input::dir().as_bytes())?;
            Ok(())
        })
        .await?;

        Ok(Pin {
            num,
            _state: PhantomData,
        })
    }
}

impl Pin<Output> {
    pub async fn set_value(&self, value: bool) -> io::Result<()> {
        tokio::fs::OpenOptions::new()
            .write(true)
            .open(format!("/sys/class/gpio/gpio{}/value", self.num))
            .await?
            .write_all(if value { b"1" } else { b"0" })
            .await
    }
}

impl<Dir: Direction> Drop for Pin<Dir> {
    fn drop(&mut self) {
        std::fs::OpenOptions::new()
            .write(true)
            .open("/sys/class/gpio/unexport")
            .ok()
            .map(|mut file| file.write_all(self.num.to_string().as_bytes()).ok());
    }
}
