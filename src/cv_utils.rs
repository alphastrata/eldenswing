use anyhow::{anyhow, Result};
use opencv::prelude;
use std::Path::{Path, PathBuf};
use tesseract::{Result as TessResult, Tesseract};

pub struct Confidence {
    value: f64,
}
pub struct GameWindow {}

impl GameWindow {
    pub fn new() {
        GameWindow {}
    }
    pub fn screenshot() {
        todo!()
    }
    pub fn read_souls_counter(img: PathBuf) -> Result<usize> {
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
    pub fn save_png(savepath: PathBuf) -> bool {
        todo!()
    }
    pub fn compress_png(img: PathBuf) -> bool {
        todo!()
    }
    pub fn resize_png(img: PathBuf) -> bool {
        todo!()
    }
    pub fn greyscale(img: PathBuf) -> bool {
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

#[cfg(test)]
mod tests {
    #[test]
    fn read_ten_million() {
        let tess = Tesseract::new();
        let imgpath = Path::new("assets/10mill.png");
        let tess_img = tess.set_image(impath);
        let res = tess.get_text().unwrap();
        assert_eq!(res, "10,802,577");
        // should read 10802577 from assets/10mill.png
        // soulscounter is at 2200 to 2500 on the x
        // 1330-1410 on the y
    }
}
