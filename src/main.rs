mod controller;
mod os_reader;

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use controller::{CompassDegree, GameMenus, MogRun, PlayerController, LR};
use enigo::*;
use gamepad::*;
use gilrs::{Button, Event, Gilrs};
use std::time::Duration;
use winput::Vk;

use winput::message_loop::{self, EventReceiver};
const COMPASS_TIK: i32 = 381;
const REFRESH_RATE: u64 = 20; // game should be more like 16ms, this means we're slower

//
// +=====+======+ MAIN +=====+======+
fn main() -> Result<()> {
    println!("Hello, world!");
    os_reader::check_monitors();

    let receiver = message_loop::start().expect("unable to read OS events...");
    let vk = os_reader::read_inputs_from_os(&receiver, true);
    let mut enigo = Enigo::new();

    let one_second = Duration::from_millis(1000);
    let one_frame = one_second / 60;

    let gamemenu = controller::GameMenus::new();
    let player = controller::PlayerController::new();
    let mogrun = controller::MogRun::new();

    let mut q_count = 0;
    // let mut count = 0;

    // start at Mog
    std::thread::sleep(Duration::from_secs(5));
    println!("GO");
    mogrun.teleport(&mut enigo, &player);
    std::thread::sleep(Duration::from_secs(7));

    // while count < 570 {
    //     println!("Loop {}", count);
    //     mogrun.run(&mut enigo, &player);
    //     std::thread::sleep(Duration::from_millis(5100));
    //     count += 1;
    // }

    loop {
        // limit the loop rate to 60fps
        std::thread::sleep(Duration::from_millis(REFRESH_RATE));

        // Match on keyboard events
        match os_reader::read_inputs_from_os(&receiver, true) {
            Vk::J => {
                //Ingame quit
                if q_count >= 3 {
                    println!("{:?}\tQuit called.", Utc::now());
                    gamemenu.quit_from_game(&mut enigo);
                } else {
                    q_count += 1;
                    println!("Q count is {:?}", q_count);
                }
            }
            Vk::O => mogrun.speedrun(&mut enigo, &player),
            Vk::I => mogrun.run(&mut enigo, &player),
            Vk::M => mogrun.teleport(&mut enigo, &player),
            //Emergency panic on k
            Vk::K => break,

            //Screengrab
            Vk::L => os_reader::take_screenshot(&one_frame.clone()).unwrap(),
            _ => (),
        }
    }

    println!("see ya tarnished!");
    Ok(())
}
