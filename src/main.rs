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
    mogrun.time_app_spartup_utc = Utc::now();
    data.session_start = mogrun.time_app_spartup_utc;
    std::thread::sleep(Duration::from_secs(5)); // needs to be long enough for initial read..

    // get our initial ingame screengrab
    let _ = GameWindow::screengrab("starting_souls".into(), "png".into(), "".into())?;

    // crop it
    let _ = GameWindow::crop_souls_counter(PathBuf::from("starting_souls.png"))?;

    // allow the user some alt-tab time
    mogrun.teleport(&mut enigo, &player);
    // allow them a moment to cancel/move their char etc before control of their keyboard is snatched
    std::thread::sleep(Duration::from_secs(3));

    // read it
    mogrun.starting_souls =
        GameWindow::external_tesseract_call("current_souls_cropped.png".into(), "eng".into())?;
    println!("{:#?} starting_souls", mogrun.starting_souls);
    let _ = GameWindow::crop_souls_counter(PathBuf::from(r"starting_souls.png"))?;
    // let mut souls_total_all_runs = vec![1];

    // let mut table = ui::setup_table(mogrun);
    let mut best = 0;
    let mut worst = 999999;

    // ----------------- MAIN LOOP ------------------
    // How many runs do you wanna do?
    mogrun.run_count_total_absolute = 101;
    for n in 1..mogrun.run_count_total_absolute {
        data.run_number = n as usize;
        mogrun.current_run_number = n as usize;

        // this is being recreated here because I cannot work out how to solve a lifetime issue with the Copy thing...
        let history: PlayerHistory = PlayerHistory::new_from(77, 40, 90, 0.0, 0.0, 0);
        // let history = *data.playerhistory.clone();

        // the actual run
        enigo.key_down(Key::Space);
        mogrun.run_count_total_thusfar += 1;
        mogrun.run(&mut enigo, &player, history);
        enigo.key_up(Key::Space);

        mogrun.current_run_end_utc = Utc::now();

        let _ = GameWindow::crop_souls_counter(PathBuf::from(r"starting_souls.png"))?;
        mogrun.souls_this_run = (GameWindow::external_tesseract_call(
            "current_souls_cropped.png".to_string(),
            "eng".to_string(),
        )?) as i64;

        let _ = cleanup_tmp_png(n);
        let delta = mogrun.souls_this_run - mogrun.souls_last_run;

        std::thread::sleep(Duration::from_millis(4500));
        if delta > best && delta < 99999 {
            best = delta;
        }

        if delta < worst {
            worst = delta;
        }

        println!("--------------------------------------------------------------");
        println!("Starting Souls: {:^12}", mogrun.starting_souls);
        println!(
            "Souls from bot: {:^12}",
            mogrun.souls_this_run - mogrun.starting_souls as i64
        );
        println!("Souls vs last : {:^12}", delta);
        println!("Run# :{}/{}", n, mogrun.run_count_total_absolute);
        println!("Best run : {:^6}", best);
        println!("Worst run: {:^6}", worst);
        println!("--------------------------------------------------------------");
        println!("{:#?}", mogrun);

        mogrun.souls_last_run = mogrun.souls_this_run;
        mogrun.souls_this_run = 0;
    }

    println!("see ya tarnished!");
    println!("END_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    println!("--------------------------------------------------------------");
    println!("{:#?}", mogrun);
    println!("--------------------------------------------------------------");
    Ok(())
}

fn cleanup_tmp_png(run_number: usize) -> Result<()> {
    // remove all png files in dir
    let path = PathBuf::from("./");
    let files = std::fs::read_dir(path)?;
    for file in files {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name.to_str().expect("unable to stringify file_name");
        if file_name.ends_with(".png") {
            // let timestamp = Utc::now().format("%H%M%S%D%m%Y");
            let output = format!("./screenshots/{}_{}", run_number, file_name);
            println!("{}", output);
            std::fs::rename(file.path(), output)?;
            std::fs::remove_file(file.path())?;
        }
    }
    Ok(())
}
