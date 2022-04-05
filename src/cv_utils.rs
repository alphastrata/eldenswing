use anyhow::Result;
use chrono::prelude::*;
use image::DynamicImage;
use image::GrayImage;
use image::RgbImage;
use image::{open, GenericImage, Luma, Rgb};
use imageproc::definitions::Image;
use imageproc::definitions::Score;
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::map::map_colors;
use imageproc::rect::Rect;
use imageproc::template_matching::{match_template, MatchTemplateMethod};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

pub struct Confidence {
    value: f64,
}
pub fn run_match_template(input_img: PathBuf) -> Result<RgbImage> {
    // Match the template and convert to u8 depth to display
    let template_w = 101;
    let template_x = 0;
    let template_y = 0;
    let template_h = 123;

    // let input_img: PathBuf = PathBuf::from("./assets/template.png");

    let image = open(input_img.clone())?.to_luma8();

    let template = open("wave_sword.png")?.to_luma8();

    let method = MatchTemplateMethod::SumOfSquaredErrors;
    // let method = MatchTemplateMethod::CrossCorrelation;
    // let method = MatchTemplateMethod::CrossCorrelation;

    let result = match_template(&image, &template, method);
    let result_scaled = convert_to_gray_image(&result);

    // Pad the result to the same size as the input image, to make them easier to compare
    let mut result_padded = GrayImage::new(image.width(), image.height());
    result_padded
        .copy_from(&result_scaled, template_w / 2, template_h / 2)
        .unwrap();

    // Show location the template was extracted from
    let roi = Rect::at(template_x as i32, template_y as i32).of_size(template_w, template_h);

    Ok(draw_green_rect(&result_padded, roi))
}
fn convert_to_gray_image(image: &Image<Luma<f32>>) -> GrayImage {
    let mut lo = f32::INFINITY;
    let mut hi = f32::NEG_INFINITY;

    for p in image.iter() {
        lo = if *p < lo { *p } else { lo };
        hi = if *p > hi { *p } else { hi };
    }

    let range = hi - lo;
    let scale = |x| (255.0 * (x - lo) / range) as u8;
    map_colors(image, |p| Luma([scale(p[0])]))
}
// Helper struct to more easily interact with the concept of the Game's screen
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
            "assets/current_weapon_{}.png",
            Utc::now().timestamp()
        ));
        let cropped_img = GameWindow::crop_from_screengrab(
            filename,
            (roi_box.x, roi_box.y, roi_box.w, roi_box.h),
            output_filename,
        );
        cropped_img
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
    // uses the precompiled tesseract-ocr for windows to detect, write to a res.txt file which this will read and return
    pub fn external_tesseract_call(filename: String, lang: String) -> Result<usize> {
        // make the call
        let _output = Command::new("tesseract.exe")
            .arg(filename)
            .arg("res")
            .arg("-l")
            .arg("eng")
            .output()
            .expect("ls command failed to start");

        // read the res.txt file's contents into a string and return it
        let contents: String = fs::read_to_string("res.txt")?;
        if contents.len() > 0 {
            let contents = contents.trim().parse()?;
            // write output to log.txt
            return Ok(contents);
        } else {
            Ok(GameWindow::external_tesseract_call("res.txt".into(), lang)?)
        }
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

pub fn draw_green_rect(image: &GrayImage, rect: Rect) -> RgbImage {
    let mut color_image = map_colors(image, |p| Rgb([p[0], p[0], p[0]]));
    draw_hollow_rect_mut(&mut color_image, rect, Rgb([0, 255, 0]));
    color_image
}
