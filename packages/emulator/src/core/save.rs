use std::{
    ffi::c_void,
    io::{self, BufWriter, Read},
};

use tokio::io::AsyncWriteExt;

use crate::{
    backend::{park_main, unpark_main},
    convert, ARGS,
};

use super::{CORE, CURRENT_FRAME};

/// Saves to save dir with provided slot or `auto` if none is provided
///
/// Calls save_inner while making sure main thread is parked before and unparked after
pub async fn save(slot: Option<usize>) -> io::Result<()> {
    tracing::info!("saving...");

    park_main().await;
    let save_data = tokio::task::spawn_blocking(move || {
        let buf_size = unsafe { (CORE.get().unwrap().retro_serialize_size)() };
        tracing::debug!("Save size: {buf_size}");
        let mut buf = vec![0u8; buf_size];

        // SAFETY: main thread is parked and cannot call retro_run during this time, so it is safe
        let serialize_res = unsafe {
            (CORE.get().unwrap().retro_serialize)(buf.as_mut_ptr() as *mut c_void, buf_size)
        };

        // Allow main thread to continue execution once serialize is complete
        unpark_main();

        if !serialize_res {
            tracing::error!("retro_serialize failed twice, error out.");
            Err(io::Error::new(
                io::ErrorKind::Other,
                "retro_serialize failed.",
            ))
        } else {
            Ok(buf)
        }
    })
    .await
    .unwrap()?;

    // Now buf contains save state, write it to a the correct dir
    let args = ARGS.get().unwrap();
    let save_dir = args.save_dir();

    let slot = slot.map(|slot| slot.to_string()).unwrap_or("auto".into());
    let save_path = format!("{save_dir}/{}-{slot}.sav", args.game_name());
    tokio::fs::remove_file(&save_path).await.ok();
    let mut save_file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&save_path)
        .await
        .unwrap();

    let img_path = format!("{save_path}.png");
    let img_file = tokio::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(img_path)
        .await
        .unwrap();

    let current_frame = CURRENT_FRAME.load_full().unwrap();
    let rgba_frame = convert::xrgb8888_to_rgba888(&current_frame);
    let mut encoder =
        png::Encoder::new(BufWriter::new(img_file.into_std().await), 640_u32, 480_u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    // Write contents to files
    save_file.write_all(&save_data).await?;
    save_file.sync_data().await?;
    tokio::task::spawn_blocking(move || {
        let mut writer = encoder.write_header().unwrap();
        writer.write_image_data(&rgba_frame).unwrap();
    })
    .await
    .unwrap();

    Ok(())
}

/// Doesn't need to be async because it is okay if this blocks
pub fn load(slot: Option<usize>) -> io::Result<()> {
    let args = ARGS.get().unwrap();
    let save_dir = args.save_dir();

    let slot = slot.map(|slot| slot.to_string()).unwrap_or("auto".into());
    let save_path = format!("{save_dir}/{}-{slot}.sav", args.game_name());
    let mut save_file = std::fs::OpenOptions::new().read(true).open(&save_path)?;

    let buf_size = unsafe { (CORE.get().unwrap().retro_serialize_size)() };
    let mut save_buf = Vec::with_capacity(buf_size);
    let bytes_read = save_file.read_to_end(&mut save_buf)?;
    tracing::debug!("Read: {bytes_read} vs Size: {buf_size}");

    let success = unsafe {
        (CORE.get().unwrap().retro_unserialize)(save_buf.as_ptr() as *const c_void, save_buf.len())
    };

    match success {
        true => Ok(()),
        false => Err(io::Error::new(
            io::ErrorKind::Other,
            "Core failed to load state.",
        )),
    }
}
