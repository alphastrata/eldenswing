mod controller;
mod cv_utils;
mod data_utils;
mod mohgwyn;
mod os_reader;
mod ui;

use anyhow::Result;
use chrono::prelude::*;
use controller::MogRun;
// use csv::*;
// use cv_utils::GameWindow;
use data_utils::PlayerHistory;
use enigo::*;
use os_reader::read_inputs_from_os;
// use serde::Serialize;
// use std::fs::OpenOptions;
// use std::path::PathBuf;
// use std::time::Duration;

// use winput::message_loop::{self, EventReceiver};
// const COMPASS_TIK: i32 = 381;
// const REFRESH_RATE: u64 = 20; // game should be more like 16ms, this means we're slower
// ingame constants if required...
// let one_second = Duration::from_millis(1000);
// let one_frame = one_second / 60;

// +=====+======+ MAIN +=====+======+
fn main() -> Result<()> {
    println!("Hello tarnished!");
    println!("START_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));

    // check game is running and, if it isn't relaunch it

    os_reader::check_monitors(); // TODO: not useful?

    // keyboard and event reader stuff
    let receiver = winput::message_loop::start().expect("unable to read OS events...");
    let mut enigo = Enigo::new();

    // it may look as though the data collection has unnessecary duplication, but this is to potentially allow for extensibility later on (for non Mog runs for example)
    let history: PlayerHistory = PlayerHistory::new_from(98, 87, 90, 0.0, 0.0, 0);
    let mut data = data_utils::Data::new(history);

    // construct hepler structs to make gameplay easier to control
    let player = controller::PlayerController::new();
    let gamemenu = controller::GameMenus::new();
    let mut mogrun = MogRun::new();

    let _ = os_reader::check_elden_ring_is_running(&mut enigo, &gamemenu)?;

    let mut q_count = 0;
    loop {
        let vk = read_inputs_from_os(&receiver, true);
        if vk == winput::Vk::J {
            q_count += 1;
            println!("Q count is {:?}", q_count);
        } else {
            println!("KEY: {:?}", vk);
        }
        if q_count == 3 {
            println!("Speed quitting from game");
            gamemenu.quit_from_game(&mut enigo);
            println!("Completed at: {:?}", Utc::now().date());

            break;
        }
        if vk == winput::Vk::O {
            // mog 100
            mogrun.run_count_total_absolute = 100;
            println!("Mogrun called for 100 iterations");
            mohgwyn::run(&mut enigo, &player, &mut data, &mut mogrun);
        }
        if vk == winput::Vk::M {
            // Close App
            println!("graceful quit!");
            break;
        }
        if vk == winput::Vk::I {
            // single mog
            // let mut mogrun = MogRun::new();
            mogrun.run_count_total_absolute = 1;
            println!("Mogrun called for 1 iteration");
            mohgwyn::run(&mut enigo, &player, &mut data, &mut mogrun);
        }
        if vk == winput::Vk::X {
            println!("panic!");
            panic!()
        }
        // add option to launch/relaunch game
        // add option to increase/decrease the value of w1, w2 and the turn?
        // add option to manually screengrab
    }
    println!("see ya tarnished!");
    println!("END_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    println!("--------------------------------------------------------------");
    Ok(())
}
