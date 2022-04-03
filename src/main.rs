mod controller;
mod cv_utils;
mod data_utils;
// mod os_reader;
mod ui;

#[macro_use]
extern crate prettytable;
use prettytable::Table;
use ui::{setup_table, update_table};

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use controller::{CompassDegree, GameMenus, MogRun, PlayerController, LR};
use cv_utils::{Confidence, GameWindow};
use data_utils::Data;
use data_utils::PlayerHistory;
use enigo::*;
use std::path::PathBuf;
use std::time::Duration;

// use winput::message_loop::{self, EventReceiver};
const COMPASS_TIK: i32 = 381;
const REFRESH_RATE: u64 = 20; // game should be more like 16ms, this means we're slower

//
// +=====+======+ MAIN +=====+======+
fn main() -> Result<()> {
    // os_reader::check_monitors(); // TODO: not useful?

    // keyboard and event reader stuff
    // let receiver = message_loop::start().expect("unable to read OS events...");
    let mut enigo = Enigo::new();

    // ingame constants
    // let one_second = Duration::from_millis(1000);
    // let one_frame = one_second / 60;

    // it may look as though the data collection has unnessecary duplication, but this is to potentially allow for extensibility later on (for non Mog runs for example)
    let history: PlayerHistory = PlayerHistory::new_from(98, 87, 90, 0.0, 0.0, 0);
    let mut data = data_utils::Data::new(history);

    // construct hepler structs to make gameplay easier to control
    let player = controller::PlayerController::new();
    // let gamemenu = controller::GameMenus::new();

    // these are for mostly for data collection
    let mut mogrun = controller::MogRun::new();

    // NOTE: this is out for the mo because I cannot work out the Copy impl thing...
    // data.playerhistory = &history;
    // we set the ammount to walk/turn etc as history because it will become so, and will be logged for data science later..

    // start at Mog
    println!("App running.");
    mogrun.start_time = Utc::now();
    data.session_start = mogrun.start_time;

    println!(
        "START_TIME: {:^40}",
        data.session_start.format("%H:%M:%S %D%m%Y")
    );
    // get our initial ingame screengrab
    let _ = GameWindow::screengrab("starting_souls".into(), "png".into(), "".into())?;

    // crop it
    let _ = GameWindow::crop_souls_counter(PathBuf::from("starting_souls.png"))?;

    std::thread::sleep(Duration::from_secs(5));
    // allow the user some alt-tab time
    mogrun.teleport(&mut enigo, &player);
    // allow them a moment to cancel/move their char etc before control of their keyboard is snatched
    std::thread::sleep(Duration::from_secs(5));

    // read it
    mogrun.starting_souls =
        GameWindow::external_tesseract_call("current_souls_cropped.png".into(), "eng".into())?;
    println!("{:#?} starting_souls", mogrun.starting_souls);

    let mut table = ui::setup_table(mogrun);
    // ----------------- MAIN LOOP ------------------
    // How many runs do you wanna do?
    mogrun.num_runs = 8;
    for n in 0..mogrun.num_runs {
        cleanup_tmp_png();
        //NOTE: replace this with table
        //NOTE: replace this with table

        // this is being recreated here because I cannot work out how to solve a lifetime issue with the Copy thing...
        let history: PlayerHistory = PlayerHistory::new_from(77, 40, 90, 0.0, 0.0, 0);
        // let history = *data.playerhistory.clone();
        // the actual run
        enigo.key_down(Key::Space);
        mogrun.run(&mut enigo, &player, history);
        enigo.key_up(Key::Space);
        std::thread::sleep(Duration::from_millis(4900));

        mogrun.runs_completed += 1;
        data.run_number = n as usize;
        mogrun.current_run_endtime = Utc::now();
        data.timestamp = mogrun.current_run_endtime;

        let _ = GameWindow::screengrab("starting_souls".into(), "png".into(), "".into())?;
        let _ = GameWindow::crop_souls_counter(PathBuf::from(r"starting_souls.png"))?;
        mogrun.newest_reading = GameWindow::external_tesseract_call(
            "current_souls_cropped.png".to_string(),
            "eng".to_string(),
        )?;
        mogrun.souls_earned = mogrun.newest_reading - &mogrun.starting_souls;

        update_table(mogrun, &mut table);
        // clear the screen on std out
        table.printstd();
        mogrun.prev_run = mogrun.newest_reading;
        mogrun.yield_total += mogrun.newest_reading;
    }

    println!("see ya tarnished!");
    println!("END_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    println!("--------------------------------------------------------------");
    println!("{:#?}", mogrun);
    println!("--------------------------------------------------------------");
    Ok(())
}

fn cleanup_tmp_png() {
    // remove all png files in dir
    let path = PathBuf::from("./");
    let files = std::fs::read_dir(path).unwrap();
    for file in files {
        let file = file.unwrap();
        let file_name = file.file_name();
        let file_name = file_name.to_str().unwrap();
        if file_name.ends_with(".png") {
            // println!("REMOVING: {}", file_name);
            std::fs::remove_file(file.path()).unwrap();
        }
    }
}
