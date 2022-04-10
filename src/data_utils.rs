use chrono::prelude::*;
use std::path::{Path, PathBuf};

// Data specifically pretaining to the RUN, i.e what inputs did we feed the player.
#[derive(Debug, Clone)]
pub struct PlayerHistory {
    pub walk1: usize, // value dictating the ammount of time/frames that the player walks from at spawn
    pub turn_angle: usize, // value dictating the degrees a player turns NOTE: needs to eventually become something the Compass can discern
    pub walk2: usize, // value dictating the ammount of time/frames that the player walks the second time
    pub wave_wait: f64, // frames or secs?
    pub grace_wait: f64, // frames or secs?
    pub player_lvl: u32, // unsure whether to capture this, maybe useful to make a runs for target level feature
}
impl PlayerHistory {
    pub fn new() -> PlayerHistory {
        PlayerHistory {
            walk1: 0,
            turn_angle: 0,
            walk2: 0,
            wave_wait: 0.0,
            grace_wait: 0.0,
            player_lvl: 0,
        }
    }
    pub fn new_from(
        walk1: usize,
        walk2: usize,
        turn_angle: usize,
        wave_wait: f64,
        grace_wait: f64,
        player_lvl: u32,
    ) -> PlayerHistory {
        PlayerHistory {
            walk1,
            turn_angle,
            walk2,
            wave_wait,
            grace_wait,
            player_lvl,
        }
    }
}

// representing all data we wish to capture from the game and interact with/present
#[derive(Debug, Clone)]
pub struct Data {
    pub session_start: DateTime<Utc>, // will actually be a timestamp
    pub soulscount: u32,              // get this from Tesseract with OCR
    pub timestamp: DateTime<Utc>, // representing the UPDATE time this data was last updated will actually be a Utc::Datetime
    pub run_number: usize,        // count this even if infinite
    pub playerhistory: PlayerHistory, // See comments above struct decleration
    pub session_end: u32,         // will actually be a timestamp
    pub prev_run_yeild: u32,      // soul yield previous run
}
impl Data {
    pub fn new(history: PlayerHistory) -> Data {
        Data {
            session_start: Utc::now(),
            soulscount: 0,
            timestamp: Utc::now(),
            run_number: 0,
            playerhistory: history,
            session_end: 0,
            prev_run_yeild: 0,
        }
    }
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
