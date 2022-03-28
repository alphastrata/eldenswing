mod ingame;
mod os_reader;

use anyhow::{anyhow, Result};
use chrono::prelude::*;
use enigo::*;
use std::time::Duration;

use winput::message_loop::{self, EventReceiver};
const COMPASS_TIK: i32 = 381;
const REFRESH_RATE: u64 = 20; // game should be more like 16ms, this means we're slower

#[derive(Debug)]
enum CompassDegree {
    ninety,
    fourtyfive,
    oneeighty,
    twozeventy,
    threesixty,
}

enum LR {
    Left,
    Right,
}

fn turn(d: CompassDegree, lr: LR) {
    // The compas is broken down into 16 segments, on a 2560x1440 screen that's 382px per segment
    // NOTE: camera will be reset within this call
    let mut enigo = Enigo::new();

    let mut rotation = match { d } {
        CompassDegree::fourtyfive => COMPASS_TIK * 2,
        CompassDegree::ninety => COMPASS_TIK * 4,
        CompassDegree::oneeighty => COMPASS_TIK * 8,
        CompassDegree::twozeventy => COMPASS_TIK * 12,
        CompassDegree::threesixty => COMPASS_TIK * 16,
    };

    let mut rotation = match { lr } {
        LR::Left => -1 * rotation,
        LR::Right => rotation,
    };

    println!("Mouse at centre");
    std::thread::sleep(Duration::from_secs(2));
    enigo.mouse_move_relative(rotation, 0);
    println!("Turning 90s{:?}", rotation);
    ingame::walk_fwd(2);
}
//
// +=====+======+ MAIN +=====+======+
fn main() -> Result<()> {
    println!("Hello, world!");

    let receiver = message_loop::start().expect("unable to read OS events...");

    os_reader::check_monitors();

    let mut q_count = 0;

    loop {
        std::thread::sleep(Duration::from_millis(REFRESH_RATE));
        q_count = os_reader::check_quit_call(&receiver, true, &mut q_count)?;
        let screenshot = os_reader::check_screenshot(&receiver)?;
        if os_reader::file_exists(&screenshot) {
            println!("{:?}\tScreenshot found.", Utc::now());
        }
    }

    #[allow(unreachable_code)]
    Ok(())
}
