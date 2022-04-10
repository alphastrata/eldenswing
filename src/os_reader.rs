use crate::controller::{GameMenus, MogRun, PlayerController};
use crate::cv_utils::RoiBox;
use crate::data_utils::Data;
use crate::mohgwyn;
use anyhow::Result;
use chrono::prelude::*;
use enigo::Enigo;
use scrap::Display;
use std::path::PathBuf;
use std::process::Command;
use sysinfo::{ProcessExt, System, SystemExt};
use winput::message_loop::{self, EventReceiver};
use winput::{message_loop::Event, Action, Vk};

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

fn check_eac_has_launched(screengrab: PathBuf) -> bool {
    // NOTE: for a 1440p display

    let eac_anti_cheat_window_ft: RoiBox = RoiBox {
        x: 900,
        y: 845,
        w: 215,
        h: 65,
    };
    // take screengrab save it as eac_check.png

    // crop it

    // compare it
    //
    true
}

pub fn read_inputs_from_os(
    receiver: &EventReceiver,
    gamemenu: &GameMenus,
    enigo: &mut Enigo,
    player: &PlayerController,
    data: &mut Data,
    mogrun: &mut MogRun,
) -> bool {
    let mut j_count = 0;
    loop {
        match receiver.next_event() {
            message_loop::Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if vk == winput::Vk::J {
                    j_count += 1;
                    println!("Q count is {:?}", j_count);
                }

                if j_count == 3 {
                    println!("Speed quitting from game");
                    gamemenu.quit_from_game(enigo);
                    println!("Completed at: {:?}", Utc::now().date());

                    return false;
                }
                if vk == winput::Vk::O {
                    // mog 100
                    mogrun.run_count_total_absolute = 100;
                    println!("Mogrun called for 100 iterations");
                    let _ = mohgwyn::run(enigo, player, data, mogrun);
                }
                if vk == winput::Vk::M {
                    // Close App
                    println!("graceful quit!");
                    return false;
                }
                if vk == winput::Vk::I {
                    // single mog
                    // let mut mogrun = MogRun::new();
                    mogrun.run_count_total_absolute = 1;
                    println!("Mogrun called for 1 iteration");
                    let _ = mohgwyn::run(enigo, player, data, mogrun);
                }
                if vk == winput::Vk::X {
                    println!("panic!");
                    panic!()
                }
                // add option to launch/relaunch game
                // add option to increase/decrease the value of w1, w2 and the turn?
                // add option to manually screengrab
                else {
                    println!("{:?} was pressed!", vk);
                }
            }
            _ => (),
        }
    }
}

pub fn check_monitors() {
    let displays = Display::all().expect("Unable to index displays");

    for (i, display) in displays.iter().enumerate() {
        println!(
            "Display {} [{}x{}]",
            i + 1,
            display.width(),
            display.height()
        );
    }
}
