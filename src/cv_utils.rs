use anyhow::Result;
use chrono::prelude::*;
use image::GrayImage;
use image::Rgb;
// use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::map::map_colors;
use imageproc::rect::Rect;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

const WAVE_SWORD: &str = r"C:\Users\jer\Documents\GitHub\eldenswing\assets\wave_sword.png";

pub struct GameWindow {}
pub struct RoiBox {
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

impl GameWindow {
    pub fn _new() -> GameWindow {
        GameWindow {}
    }
    pub fn crop_souls_counter(filename: PathBuf) -> Result<PathBuf> {
        let roi_box: RoiBox = RoiBox {
            //NOTE: this if calibrated to a 1440p display
            x: 2280,
            y: 1362,
            w: 202,
            h: 38,
        };
        let output_filename: PathBuf = PathBuf::from("current_souls_cropped.png");
        let cropped_img = GameWindow::crop_from_screengrab(
            filename,
            (roi_box.x, roi_box.y, roi_box.w, roi_box.h),
            output_filename,
        );
        cropped_img
    }
    pub fn crop_rh_weapon(filename: PathBuf) -> Result<PathBuf> {
        let roi_box: RoiBox = RoiBox {
            //NOTE: this if calibrated to a 1440p display
            x: 325,
            y: 1150,
            w: 101,
            h: 123,
        };
        let output_filename: PathBuf = PathBuf::from(format!(
            "screenshots/current_weapon_{}.png",
            Utc::now().timestamp()
        ));
        let cropped_img = GameWindow::crop_from_screengrab(
            filename,
            (roi_box.x, roi_box.y, roi_box.w, roi_box.h),
            output_filename,
        );
        Ok(PathBuf::from(fs::canonicalize(cropped_img?.as_path())?))
    }
    // Used to crop the souls counter from screengrab
    // NOTE: could be used for other things...
    fn crop_from_screengrab(
        // img: dyn GenericImageView,
        p: PathBuf,
        roi_box: (u32, u32, u32, u32),
        output_filename: PathBuf,
    ) -> Result<PathBuf> {
        let mut img = image::open(&p)?;
        let cropped = img.crop(roi_box.0, roi_box.1, roi_box.2, roi_box.3);
        cropped.save(&output_filename)?;
        Ok(output_filename)
        // NOTE: return a path or the actual img... can the actual img be passed (in memeory) to tesseract..?
    }
    // Run's an external syscall to ../screenCapture.exe
    // screenCapture- captures the screen or the active window and saves it to a file
    // Usage:
    // screenCapture  filename.format [WindowTitle]
    // filename - the file where the screen capture will be saved
    // format - Bmp,Emf,Exif,Gif,Icon,Jpeg,Png,Tiff and are supported - default is bmp
    // WindowTitle - instead of capturing the whole screen will capture the only a window with the given title if there's such
    pub fn screengrab(filename: String, format: String, _window_title: String) -> Result<()> {
        let output =
            Command::new("C:\\Users\\jer\\Documents\\Github\\eldenswing\\screenCapture.exe")
                // TODO: fix above to be like below...
                // Command::new("screenCapture.exe")
                .arg(format!("{}.{}", filename, format))
                // .arg("ELDEN RING™")
                .output()
                .expect("ls command failed to start");

        // append output to log.txt
        let mut file = fs::File::create("screengrab_log.txt")?;
        file.write_all(output.stdout.as_slice())?;

        Ok(())
    }

    pub fn check_rh_weapon() -> Result<bool> {
        // get an item crop and validate it's the wave sword.
        let weapon_crop = GameWindow::crop_rh_weapon(PathBuf::from("starting_souls.png"))?;

        // DEBUG for pathing>< DO NOT REMOVE
        // println!("weapon_crop is {:?}", weapon_crop.as_path().display());
        // println!("wave_sword is {:?}", WAVE_SWORD);
        let dssim = dssim_compare(weapon_crop, PathBuf::from(WAVE_SWORD))?;
        if dssim > 0.03 {
            println!("weapon is not equipped");
            println!("{}", dssim);
            panic!("WAVE_SWORD not equipped");
        } else {
            println!("{}", dssim);
            println!("weapon is equipped");
        }
        Ok(true)
    }
    // Make a box to cover the souls counter as a % of screen resolution (x and y)
    // or.. other region of interest
    fn _make_roi_box(x: u32, y: u32) -> (i32, i32, i32, i32) {
        // let x = (x as f64 * 0.89).round() as i32;
        let x = (x as f64 * 0.98).round() as i32;
        let y = (y as f64 * 0.94).round() as i32;
        let w = (x as f64 * 0.0789).round() as i32;
        let h = (y as f64 * 0.0264).round() as i32;

        println!("BOX:\n{:#?}", (x, y, w, h));

        (x, y, w, h)
    }
}

/// NOTE: seems to require ABSOLUTE PATHS
pub fn dssim_compare(img1: PathBuf, img2: PathBuf) -> Result<dssim::Val> {
    // let img2 = PathBuf::from(r"C:\Users\jer\Documents\GitHub\eldenswing\assets\wave_sword.png");
    // let img1 = PathBuf::from(r"C:\Users\jer\Documents\GitHub\eldenswing\assets\weapon_crop_1.png");

    let attr = dssim::Dssim::new();
    let orig = dssim::load_image(&attr, &img1)?;
    let comp = dssim::load_image(&attr, &img2)?;
    let (diff, _) = attr.compare(&orig, &comp); //NOTE: You're throwing the error away :(

    Ok(diff)
}

// uses the precompiled tesseract-ocr for windows to detect, write to a res.txt file which this will read and return
pub fn external_tesseract_call(filename: String, lang: String) -> Result<usize> {
    // make the call
    let _output = Command::new("tesseract.exe")
        .arg(filename)
        .arg("res")
        .arg("-l")
        .arg("eng")
        .output()
        .expect("Failed to run tesseract -- perhaps it's not installed?\nperhaps it is but the path is wrong.");

    // read the res.txt file's contents into a string and return it
    let contents: String = fs::read_to_string("res.txt")?;
    let default = 0;
    if contents.trim().len() > 0 {
        let contents = contents.trim().parse().unwrap_or(default); //NOTE this will stop the crashing but.. it's not ideal.
        return Ok(contents);
    } else {
        Ok(external_tesseract_call("res.txt".into(), lang)?)
    }
}
