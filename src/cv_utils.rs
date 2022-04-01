use anyhow::{anyhow, Result};
use opencv::prelude;
use std::Path::{Path, PathBuf};


pub struct Confidence{
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
    pub fn read_souls_counter(img: PathBuf) ->Result<usize>{
        todo!()
    }
    pub fn target_delta(img1: PathBuf, img2: PathBuf) -> Result<Confidence>{
        todo!()
    }
    pub fn fullscreengrab(savepath: PathBuf){
        todo!()
    }
    pub fn screengrab()-> PathBuf{
        todo!()
    }
    pub fn save_png(savepath: PathBuf) -> bool{
        todo!()
    }
    pub fn compress_png(img: PathBuf)-> bool{
        todo!()
    }
    pub fn resize_png(img: PathBuf)-> bool{
        todo!()
    }
    pub fn greyscale(img: PathBuf)-> bool{
        todo!()
    }
    pub fn rename(img: PathBuf)-> bool{
        todo!()
    }
    pub fn purge(dir: PathBuf)-> bool{
        todo!()
    }
    pub fn remove(img: PathBuf)-> bool{
            todo!()
        }
}
