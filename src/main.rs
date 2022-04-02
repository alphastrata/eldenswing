mod controller;
mod cv_utils;
mod data_utils;
mod os_reader;

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use controller::{CompassDegree, GameMenus, MogRun, PlayerController, LR};
use enigo::*;
use gamepad::*;
use gilrs::{Button, Event, Gilrs};
use std::thread::JoinHandle;
use std::time::Duration;
use winput::Vk;

use winput::message_loop::{self, EventReceiver};
const COMPASS_TIK: i32 = 381;
const REFRESH_RATE: u64 = 20; // game should be more like 16ms, this means we're slower
const RUNS: usize = 2;

//
// +=====+======+ MAIN +=====+======+
fn main() -> Result<()> {
    println!("Hello, world!");
    os_reader::check_monitors();

    // keyboard and event reader stuff
    let receiver = message_loop::start().expect("unable to read OS events...");
    let mut enigo = Enigo::new();

    // ingame constants
    let one_second = Duration::from_millis(1000);
    let one_frame = one_second / 60;

    // construct hepler structs to make gameplay easier to control
    let gamemenu = controller::GameMenus::new();
    let player = controller::PlayerController::new();
    let mogrun = controller::MogRun::new();

    let mut q_count = 0;

    // start at Mog
    println!("App running.");
    println!("START_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    std::thread::sleep(Duration::from_secs(5));
    mogrun.teleport(&mut enigo, &player);
    std::thread::sleep(Duration::from_secs(5));

    // let mut handles: Vec<JoinHandle<()>> = Vec::new();

    let mut count = 0;
    let total_time = Utc::now();
    let w1 = 98;
    let w2 = 80;

    loop {
        let runstart = Utc::now();
        println!("RUN: {} {:^40}", count, runstart.format("%H:%M:%S"));

        // the actual run
        enigo.key_down(Key::Space);
        mogrun.run(&mut enigo, &player, w1, w2);
        enigo.key_up(Key::Space);
        std::thread::sleep(Duration::from_millis(4900));
        count += 1;
        let runfinish = Utc::now();

        // timers for user feedback etc
        println!("END: {} {:^40}", count, runfinish.format("%H:%M:%S"));
        println!(
            "\tSPLIT: {} \t *in seconds.",
            (runfinish - runstart).num_seconds()
        );
        println!(
            "RUNTIME: {} \t*in minutes.",
            Utc::now().signed_duration_since(total_time).num_minutes()
        );
    }

    // loop {
    //     // limit the loop rate to 60fps
    //     std::thread::sleep(Duration::from_millis(REFRESH_RATE / 22));

    //     // Match on keyboard events
    //     match os_reader::read_inputs_from_os(&receiver, true) {
    //         Vk::J => {
    //             //Ingame quit
    //             if q_count >= 3 {
    //                 println!("QUIT_CALL{:^40?}.", Utc::now().format("%H:%M:%S"));
    //                 gamemenu.quit_from_game(&mut enigo);
    //             } else {
    //                 q_count += 1;
    //                 println!("Q_COUNT: {:?}", q_count);
    //             }
    //         }
    //         Vk::O => mogrun.speedrun(&mut enigo, &player, RUNS),
    //         Vk::I => mogrun.run(&mut enigo, &player, 190, 420),
    //         Vk::M => mogrun.teleport(&mut enigo, &player),
    //         //Emergency panic on k
    //         Vk::K => panic!(),

    //         //Screengrab
    //         Vk::L => os_reader::take_screenshot(&one_frame.clone()).unwrap(),
    //         _ => (),
    //     }
    // }

    println!("see ya tarnished!");
    println!("END_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    Ok(())
}
