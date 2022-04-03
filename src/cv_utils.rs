// use opencv::core::Vector;
// use opencv::prelude;
// use opencv::types;
// use opencv::{
//     core::{self},
//     imgcodecs,
//     prelude::*,
// };
// use tesseract::{Result as TessResult, Tesseract};

use anyhow::Result;
// use chrono::prelude::Utc;
// use image::imageops::FilterType;
// use image::io::Reader;
// use image::ImageFormat;
// use image::{DynamicImage, GenericImage, GenericImageView};
use leptess::LepTess;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;

// To indicate the confidence in a value returned by Tesseract.
pub struct Confidence {
    value: f64,
}
pub enum Os {
    Win,
    Mac,
    Linux,
}

// Helper struct to more easily interact with the concept of the Game's screen
pub struct GameWindow {}

impl GameWindow {
    pub fn new() -> GameWindow {
        GameWindow {}
    }
    pub fn crop_souls_counter(filename: PathBuf) -> Result<PathBuf> {
        let x = 2275;
        let y = 1362;
        let width = 218;
        let height = 39;
        let cropped_img = GameWindow::crop_from_screengrab(filename, x, y, width, height);
        cropped_img
    }
    // Run's an external syscall to ../screenCapture.exe
    // screenCapture- captures the screen or the active window and saves it to a file
    // Usage:
    // screenCapture  filename.format [WindowTitle]
    // filename - the file where the screen capture will be saved
    // format - Bmp,Emf,Exif,Gif,Icon,Jpeg,Png,Tiff and are supported - default is bmp
    // WindowTitle - instead of capturing the whole screen will capture the only a window with the given title if there's such
    pub fn screengrab(
        os: Os,
        filename: String,
        format: String,
        window_title: String, //NOTE: currently unused...
    ) -> Result<()> {
        match os {
            Os::Win => {
                Command::new("screenCapture.exe")
                    .arg(format!("screenshots/{}", filename))
                    .arg(window_title)
                    .spawn()
                    .expect("ls command failed to start");
                Ok(())
            }
            // NOTE: fill this in for whatever macs use...
            Os::Mac => {
                Command::new("screenCapture.exe")
                    .arg(format!("{}", filename))
                    .arg(window_title)
                    .spawn()
                    .expect("ls command failed to start");
                Ok(())
            }
            Os::Linux => {
                Command::new("gnome-screenshot")
                    .arg("-f")
                    // .arg(stringify!(
                    //     "~Documents/rust/eldenswing/{}.{}",
                    //     filename,
                    //     format
                    // ))
                    .arg("souls_counter.png")
                    // command.arg("-w"); // maybe -w will work if the game is the active window..
                    .spawn()
                    .expect("gnome-sceenshot failed");
                Ok(())
            }
        }
    }
    // // read the souls counter with Leptess (wrapping the C API of Tesseract)
    pub fn read_souls_counter(p: PathBuf) -> Result<u64> {
        let mut lt = LepTess::new(None, "eng")?;
        lt.set_image("/home/jer/Documents/rust/eldenswing/souls_counter_cropped.png");
        let mut text = lt.get_utf8_text()?;
        let t = text
            .chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<String>();
        println!("DEBUG read_from_souls() :{}", t);
        let t = t.parse::<u64>().unwrap();
        println!("DEBUG read_from_souls() :{}", t);

        Ok(t)
    }

    fn get_file_as_byte_vec(filename: String) -> Vec<u8> {
        let mut f = File::open(&filename).expect("no file found");
        let metadata = fs::metadata(&filename).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        buffer
    }
    fn read_from_memory(img: &[u8]) {
        let mut lt = leptess::LepTess::new(None, "eng").unwrap();
        lt.set_image_from_mem(img);
        println!("{}", lt.get_utf8_text().unwrap());
    }
    // Used to crop the souls counter from screengrab
    // NOTE: could be used for other things...
    fn crop_from_screengrab(
        // img: dyn GenericImageView,
        p: PathBuf,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        // ) -> Result<DynamicImage> {
    ) -> Result<PathBuf> {
        let mut img = image::open("/home/jer/Documents/rust/eldenswing/souls_counter.png").unwrap();
        // let cropped = img.crop_imm(x, y, width, height); // NOTE: This is going to be the new one from .24
        let cropped = img.crop(x, y, width, height);
        cropped
            .save("/home/jer/Documents/rust/eldenswing/souls_counter_cropped.png")
            .unwrap();
        Ok(PathBuf::from(
            "/home/jer/Documents/rust/eldenswing/souls_counter_cropped.png",
        ))
        // NOTE: return a path or the actual img... can the actual img be passed (in memeory) to tesseract..?
    }

    pub fn rename(img: PathBuf) -> bool {
        todo!()
    }
    pub fn purge(dir: PathBuf) -> bool {
        todo!()
    }
    pub fn remove(img: PathBuf) -> bool {
        todo!()
    }
}
