use crate::cv_utils::*; // Note: this feature is working in the OCR dir, but not building on windows.. investigate
use std::fs::File;
use std::path::{Path, PathBuf};

use prettytable::format;
use prettytable::{Cell, Row, Table};

// Data specifically pretaining to the RUN, i.e what inputs did we feed the player.
struct PlayerHistory {
    walk1: u32, // value dictating the ammount of time/frames that the player walks from at spawn
    turn_angle: u32, // value dictating the degrees a player turns
    walk2: u32, // value dictating the ammount of time/frames that the player walks the second time
    wave_wait: f64, // frames or secs?
    grace_wait: f64, // frames or secs?
    _player_lvl: u32, // unsure whether to capture this, maybe useful to make a runs for target level feature
}

// representing all data we wish to capture from the game and interact with/present
struct Data {
    session_start: u32, // will actually be a timestamp
    soulscount: u32,    // get this from Tesseract with OCR
    timestamp: String,  // will actually be a Utc::Datetime
    run_number: u32,    // count this even if infinite
    playerhistory: PlayerHistory,
    session_end: u32,    // will actually be a timestamp
    prev_run_yeild: u32, // soul yield previous run
}
impl Data {
    // save all data to disk
    fn write_run_data() {
        // let out = File::create("output_csv.txt")?; // apparently possible with prettytable
        // table.to_csv(out)?;
        todo!();
    }
    // Helpers to get avg yield on runs
    // pass verbose as true to have them print to stdout
    fn running_avg_by_run(verbose: bool, history: PlayerHistory, data: Data) -> u32 {
        todo!()
    }
    fn running_avg_by_h(verbose: bool) -> u32 {
        todo!()
    }
    fn running_avg_by_m(verbose: bool) -> u32 {
        todo!()
    }
    fn running_avg_by_s(verbose: bool) -> u32 {
        todo!()
    }
    pub fn data_to_stdout() {
        // lots of precanned convenience formats are available in consts....
        // table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        // table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
        // // Create the table
        // let mut table = Table::new();

        // // Add a row per time
        // table.add_row(row!["ABC", "DEFG", "HIJKLMN"]);
        // table.add_row(row!["foobar", "bar", "foo"]);
        // // A more complicated way to add a row:
        // table.add_row(Row::new(vec![
        //     Cell::new("foobar2"),
        //     Cell::new("bar2"),
        //     Cell::new("foo2"),
        // ]));

        // Print the table to stdout
        // table.printstd();
        todo!();
    }
    // cleanup temporary files
    // fn cleanup_tmp() -> Result<bool, std::io::Error> {
    //     for entry in std::fs::read_dir("tmp")?.into_iter() {
    //         let path = entry?.path();
    //         if path.extension().expect("Unable to view file extension.") == "png" {
    //             std::fs::remove_file(path)?;
    //         }
    //     }
    //     for entry in std::fs::read_dir("completed")?.into_iter() {
    //         let path = entry?.path();
    //         if !path.to_str().unwrap().contains("fulldisc") {
    //             std::fs::remove_file(path)?;
    //         }
    //     }
    //     Ok(true)
    // }
    fn create_tmp_dir() -> Result<(), std::io::Error> {
        // let tmpdir_contents = std::fs::read_dir(Path::new("tmp"))?;
        // Ok();
        todo!()
    }
}

pub struct SoulsCounter {}
impl SoulsCounter {
    fn new() -> SoulsCounter {
        SoulsCounter {}
    }
    // uses the cv_utils.rs which uses OCR via tesseract to read the soul counter from an ingame screengrab
    pub fn read_counter(img: PathBuf) -> u32 {
        // crate::cv_utils::read_souls_counter();
        todo!()
    }
}
