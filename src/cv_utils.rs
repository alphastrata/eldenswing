use anyhow::Result;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

// To indicate the confdence in a value returned by Tesseract.
pub struct _Confidence {
    value: f64,
}

// Helper struct to more easily interact with the concept of the Game's screen
pub struct GameWindow {}

impl GameWindow {
    pub fn _new() -> GameWindow {
        GameWindow {}
    }
    pub fn crop_souls_counter(filename: PathBuf) -> Result<PathBuf> {
        let x: u32 = 2280;
        let y: u32 = 1362;
        let width: u32 = 202;
        let height: u32 = 38;
        let cropped_img = GameWindow::crop_from_screengrab(filename, (x, y, width, height));
        cropped_img
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

    // Used to crop the souls counter from screengrab
    // NOTE: could be used for other things...
    fn crop_from_screengrab(
        // img: dyn GenericImageView,
        p: PathBuf,
        roi_box: (u32, u32, u32, u32),
    ) -> Result<PathBuf> {
        let mut img = image::open("starting_souls.png")?; // it will actually ALWAYS take in starting_souls.png
        let cropped = img.crop(roi_box.0, roi_box.1, roi_box.2, roi_box.3);

        let current_souls_cropped = PathBuf::from("current_souls_cropped.png");
        cropped.save(current_souls_cropped.as_path()).unwrap();
        Ok(current_souls_cropped)
        // NOTE: return a path or the actual img... can the actual img be passed (in memeory) to tesseract..?
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
