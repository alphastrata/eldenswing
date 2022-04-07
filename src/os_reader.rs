use crate::controller::{GameMenus, PlayerController};
use anyhow::{anyhow, Result};
use chrono::prelude::*;
use enigo::Enigo;
use enigo::*;
use std::fs::File;
use std::io::ErrorKind::WouldBlock;
use std::process::Command;
use std::thread;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};
use winput::message_loop::{self, EventReceiver};
use winput::{Action, Vk};

pub fn check_elden_ring_is_running(enigo: &mut Enigo, gamemenu: &GameMenus) -> Result<bool> {
    // Checks for something like this : 19064 eldenring.exe
    let s = System::new_all();
    // for (pid, process) in s.processes() {
    //     println!("{} {}", pid, process.name());
    // }
    for (_, process) in s.processes() {
        if process.name().contains("eldenring.exe") {
            println!("Elden Ring is running");
            return Ok(true);
        }
    }
    println!("Elden Ring is not running");
    launch_elden_ring(enigo, &gamemenu);
    Ok(false)
}

fn launch_elden_ring(enigo: &mut Enigo, game: &GameMenus) {
    println!("Launching eldenring.exe");
    let output = Command::new(r"E:\SteamLibrary\steamapps\common\ELDEN RING\Game\eldenring.exe")
        .output()
        .expect("failed to run eldenring.exe");
    game.enter_game_from_main_menu(enigo)
}

pub fn read_inputs_from_os(receiver: &EventReceiver, verbose: bool) -> Vk {
    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if verbose {
                    println!("{:?}: {:^80}", vk, Utc::now().format("%H:%M:%S"));
                }
                return vk;
            }
            _ => (),
        }
    }
}

// use scrap::Display
// pub fn check_monitors() {
//     let displays = Display::all().expect("Unable to index displays");

//     for (i, display) in displays.iter().enumerate() {
//         println!(
//             "Display {} [{}x{}]",
//             i + 1,
//             display.width(),
//             display.height()
//         );
//     }
// }
