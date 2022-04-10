use crate::controller::{GameMenus, MogRun, PlayerController};
use crate::cv_utils::{dssim_compare, GameWindow, RoiBox};
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
    let _output = Command::new(r"E:\SteamLibrary\steamapps\common\ELDEN RING\Game\eldenring.exe") //TODO: Replace with const
        .output()
        .expect("failed to run eldenring.exe");
    let _ = game.enter_game_from_main_menu(enigo);
}

pub fn check_eac_has_launched() -> Result<bool> {
    // NOTE: for a 1440p display

    let eac_anti_cheat_window_ft: RoiBox = RoiBox {
        x: 900,
        y: 845,
        w: 215,
        h: 65,
    };
    // take screengrab save it as eac_check.png
    let _ = GameWindow::screengrab("eac_check".into(), "png".into(), "".into())?;
    // crop it
    GameWindow::crop_from_screengrab(
        PathBuf::from("eac_check.png"),
        (
            eac_anti_cheat_window_ft.x,
            eac_anti_cheat_window_ft.y,
            eac_anti_cheat_window_ft.w,
            eac_anti_cheat_window_ft.h,
        ),
        PathBuf::from("eac_check_cropped.png"),
    )?;

    let dssim = dssim_compare(
        PathBuf::from("eac_check_cropped.png"),
        PathBuf::from("assets/eac_check.png"), //TODO: replace with const
    )?;

    if dssim > 0.03 {
        println!("EAC hasn't opened...");
        println!("{}", dssim);
    } else {
        println!("EAC is open...");
        println!("{}", dssim);
    }
    Ok(true)
}

pub fn check_main_menu_multiplayer_dialouge() -> Result<bool> {
    // NOTE: for a 1440p display

    let roi: RoiBox = RoiBox {
        x: 1110,
        y: 955,
        w: 355,
        h: 65,
    };
    // take screengrab save it as eac_check.png
    let _ = GameWindow::screengrab("multiplayer_dialouge_check".into(), "png".into(), "".into())?;
    // crop it
    GameWindow::crop_from_screengrab(
        PathBuf::from("multiplayer_dialouge_check.png"),
        (roi.x, roi.y, roi.w, roi.h),
        PathBuf::from("multiplayer_dialouge_check_cropped.png"),
    )?;

    let dssim = dssim_compare(
        PathBuf::from("multiplayer_dialouge_check_cropped.png"),
        PathBuf::from("assets/multiplayer_dialouge_check.png"), //TODO: replace with const
    )?;

    if dssim > 0.03 {
        println!("dialouge hasn't opened...");
        println!("{}", dssim);
    } else {
        println!("dialouge is open...");
        println!("{}", dssim);
    }
    Ok(true)
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
