use anyhow::{anyhow, Result};
// use leptess::*;
// use tesseract::{Result as TessResult, Tesseract};

use image::imageops::FilterType;
use image::ImageFormat;
use opencv::core::Vector;
use opencv::prelude;
use opencv::types;
use opencv::{
    core::{self},
    imgcodecs,
    prelude::*,
};
use std::process::Command;

use std::fs;
use std::path::{Path, PathBuf};

// To indicate the confidence in a value returned by Tesseract.
pub struct Confidence {
    value: f64,
}

// Helper struct to more easily interact with the concept of the Game's screen
pub struct GameWindow {}

impl GameWindow {
    pub fn new() -> GameWindow {
        GameWindow {}
    }
    // Run's an external syscall to ../screenCapture.exe
    // screenCapture- captures the screen or the active window and saves it to a file
    // Usage:
    // screenCapture  filename.format [WindowTitle]
    // filename - the file where the screen capture will be saved
    // format - Bmp,Emf,Exif,Gif,Icon,Jpeg,Png,Tiff and are supported - default is bmp
    // WindowTitle - instead of capturing the whole screen will capture the only a window with the given title if there's such
    pub fn screenshot(filename: String, format: String, window_title: String) -> Result<()> {
        Command::new("screenCapture.exe")
            .arg(format!("screenshots/{}", filename))
            .arg(window_title)
            .spawn()
            .expect("ls command failed to start");
        Ok(())
    }
    pub fn read_souls_counter(img: PathBuf) -> Result<usize> {
        // fn strip_non_digits(s: &str) -> String {
        //     let t = s.chars().filter(|c| c.is_ascii_digit()).collect::<String>();
        //     t
        // }
        // fn read_text(p: String) {
        //     let mut lt = leptess::LepTess::new(None, "eng").unwrap();
        //     lt.set_image(&p[..]);
        //     let mut text = lt.get_utf8_text().unwrap();
        //     println!("{}", strip_non_digits(&text));
        // }
        todo!()
    }
    pub fn target_delta(img1: PathBuf, img2: PathBuf) -> Result<Confidence> {
        todo!()
    }
    pub fn fullscreengrab(savepath: PathBuf) {
        todo!()
    }
    pub fn screengrab() -> PathBuf {
        todo!()
    }
    // Use opencv to write images
    pub fn save_png(m: &core::Mat, p: &str) -> Result<()> {
        let _ = imgcodecs::imwrite(p, &m, &types::VectorOfi32::new()).unwrap();
        Ok(())
        // todo!();
    }
    pub fn compress_png(img: PathBuf) -> bool {
        todo!()
    }
    pub fn resize_png(&mut self, p: PathBuf, width: i32, height: i32) -> Result<()> {
        // let img = image::open(p)?;
        // let resized = img.resize(width as u32, height as u32, FilterType::Lanczos3);
        // let p = self.p.to_str().unwrap().replace(".png", ".jpg");
        // // let _ = resized.save(&p);
        // let _ = resized.save_with_format(&p, ImageFormat::Jpeg)?;
        // let p: PathBuf = PathBuf::from(p);
        // self.path = fs::canonicalize(p)?;
        Ok(())
    }
    pub fn greyscale(img: PathBuf) -> bool {
        //fn greyscale(&self) -> Result<DynamicImage, ImageError> {
        //         Ok(image::open(self.get_path().await)?.grayscale());
        //}
        //
        //fn cv_greyscale(&self) -> Result<()> {
        //         let mat = imgcodecs::imread(self.path_as_str(), imgcodecs::IMREAD_COLOR)?;
        //         let img = mat.get_umat(ACCESS_READ, UMatUsageFlags::USAGE_DEFAULT)?;
        //         let mut gray = UMat::new(UMatUsageFlags::USAGE_DEFAULT);
        //         imgproc::cvt_color(&img, &mut gray, imgproc::COLOR_BGR2GRAY, 0)?;
        //         Ok(());
        //     }
        todo!()
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

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn read_ten_million() {
//         let tess = Tesseract::new();
//         let imgpath = Path::new("assets/10mill.png");
//         let tess_img = tess.set_image(impath);
//         let res = tess.get_text().unwrap();
//         assert_eq!(res, "10,802,577");
//         // should read 10802577 from assets/10mill.png
//         // soulscounter is at 2200 to 2500 on the x
//         // 1330-1410 on the y
//     }
// }
