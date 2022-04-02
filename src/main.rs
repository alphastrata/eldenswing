mod controller;
mod cv_utils;
mod data_utils;
mod os_reader;

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use controller::{CompassDegree, GameMenus, MogRun, PlayerController, LR};
use cv_utils::{Confidence, GameWindow};
use data_utils::Data;
use data_utils::PlayerHistory;
use enigo::*;
use std::path::PathBuf;
// use gamepad::*;
// use gilrs::{Button, Event, Gilrs};
// use std::thread::JoinHandle;
use std::time::Duration;
// use winput::Vk;

use winput::message_loop::{self, EventReceiver};
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

    let _ = GameWindow::crop_souls_counter(PathBuf::from(
        "10mill.png",
        // "C:\\Users\\jer\\Documents\\GitHub\\eldenswing\\assets\\10mill.png",
    ));

    // allow the user some alt-tab time
    std::thread::sleep(Duration::from_secs(5));
    mogrun.teleport(&mut enigo, &player);
    // TODO:
    // update mogrun.starting_souls

    // allow them a moment to cancel/move their char etc before control of their keyboard is snatched
    std::thread::sleep(Duration::from_secs(5));

    // ----------------- MAIN LOOP ------------------
    // How many runs do you wanna do?
    mogrun.num_runs = 10;
    for n in 0..mogrun.num_runs {
        //NOTE: replace this with table
        println!(
            "RUN: {} {:^70}", //TODO: how do i pin this left...
            mogrun.runs_completed,
            mogrun.start_time.format("%H:%M:%S")
        );

        // this is being recreated here because I cannot work out how to solve a lifetime issue with the Copy thing...
        let history: PlayerHistory = PlayerHistory::new_from(98, 87, 90, 0.0, 0.0, 0);
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
        // TODO:
        // update data.soulscount // this is grand total the char has on them
        // update mogrun.souls_earned
    }

    println!("see ya tarnished!");
    println!("END_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    Ok(())
}
