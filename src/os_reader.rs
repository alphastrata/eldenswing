use crate::controller::{GameMenus, MogRun, PlayerController};
use crate::cv_utils::{dssim_compare, GameWindow, RoiBox};
use crate::data_utils::{Data, PlayerHistory};
use crate::mohgwyn;
use anyhow::Result;
use chrono::prelude::*;
use colored::*;
use enigo::Enigo;
use scrap::Display;
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use sysinfo::{ProcessExt, System, SystemExt};
use winput::message_loop::EventReceiver;
use winput::{message_loop::Event, Action, Vk};

const REFRESH_RATE: u64 = 16;

// Elden Ring app running or not helpers
pub fn check_elden_ring_is_running(enigo: &mut Enigo, gamemenu: &GameMenus) -> Result<bool> {
    // Checks for something like this : 19064 eldenring.exe
    let s = System::new_all();
    // for (pid, process) in s.processes() {
    //     println!("{} {}", pid, process.name());
    // }
    for (_, process) in s.processes() {
        if process.name().contains("eldenring.exe") {
            println!("{}", "Elden Ring is running".green());
            return Ok(true);
        }
    }
    println!("Elden Ring is not running");
    launch_elden_ring(enigo, &gamemenu);
    Ok(false)
}
fn launch_elden_ring(enigo: &mut Enigo, game: &GameMenus) {
    println!("{}", "Launching eldenring.exe".green());
    let _output = Command::new(r"E:\SteamLibrary\steamapps\common\ELDEN RING\Game\eldenring.exe") //TODO: Replace with const
        .output()
        .expect("failed to run eldenring.exe");
    std::thread::sleep(Duration::from_secs(2));
    let _ = game.enter_game_from_main_menu(enigo);
}

// MAIN MENU CHECK HELPERS
pub fn check_eac_has_launched() -> Result<bool> {
    // NOTE: for a 1440p display
    let eac_anti_cheat_window_ft: RoiBox = RoiBox {
        x: 900,
        y: 845,
        w: 215,
        h: 65,
    };
    // make sure that EAC has had a chance to start..
    std::thread::sleep(Duration::from_secs(2));
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
        println!("{}", dssim.to_string().cyan());
    } else {
        println!("{}", "EAC is open...".green());
        println!("{}", dssim.to_string().cyan());
    }
    Ok(true)
}
pub fn check_main_menu_multiplayer_dialouge() -> Result<bool> {
    std::thread::sleep(Duration::from_millis(REFRESH_RATE * 8));
    // take screengrab save it as eac_check.png
    let _ = GameWindow::screengrab("multiplayer_dialouge_check".into(), "png".into(), "".into())?;

    let dssim = dssim_compare(
        PathBuf::from("multiplayer_dialouge_check.png"),
        PathBuf::from("assets/mm1.png"), //TODO: replace with const
    )?;

    if dssim > 0.03 {
        println!("INFORMATION window has not opened... trying again...");
        println!("{}", dssim.to_string().cyan());
        let _ = std::fs::remove_file("multiplayer_dialouge_check.png");
        let _ = check_main_menu_multiplayer_dialouge()?;
    } else {
        println!("Closing this POS shitty window... ");
        println!("{}", dssim.to_string().cyan());
    }
    Ok(true)
}
pub fn check_main_menu_continue() -> Result<bool> {
    std::thread::sleep(Duration::from_secs(2));
    let _ = GameWindow::screengrab("main_menu_press_any_button".into(), "png".into(), "".into())?;

    let dssim = dssim_compare(
        PathBuf::from("main_menu_press_any_button.png"),
        PathBuf::from("assets/mm2.png"), //TODO: replace with const
    )?;

    if dssim > 0.03 {
        println!("press any button MENU has not opened... trying again...");
        println!("{}", dssim);
        let _ = std::fs::remove_file("main_menu_continue.png");
        let _ = check_main_menu_continue()?;
    } else {
        println!("{}", "MENU open!".green());
        println!("{}", dssim.to_string().cyan());
    }
    Ok(true)
}
pub fn check_main_menu_options() -> Result<bool> {
    std::thread::sleep(Duration::from_millis(REFRESH_RATE * 8));
    let _ = GameWindow::screengrab("main_menu_options".into(), "png".into(), "".into())?;

    let dssim = dssim_compare(
        PathBuf::from("main_menu_options.png"),
        PathBuf::from("assets/mm3.png"), //TODO: replace with const
    )?;

    if dssim > 0.03 {
        println!("CONTINUE is not pressable... trying again...");
        println!("{}", dssim);
        let _ = std::fs::remove_file("main_menu_options.png");
        let _ = check_main_menu_options()?;
    } else {
        println!("{}", "Pressable!".green());
        println!("{}", dssim.to_string().cyan());
    }
    Ok(true)
}

// INPUT and UI capture stuff:
pub fn read_inputs_from_os(
    receiver: &EventReceiver,
    gamemenu: &GameMenus,
    enigo: &mut Enigo,
    player: &PlayerController,
    data: &mut Data,
    mogrun: &mut MogRun,
    history: &mut PlayerHistory,
) -> Result<bool> {
    let mut j_count = 0;
    loop {
        match receiver.next_event() {
            Event::Keyboard {
                vk,
                action: Action::Press,
                ..
            } => {
                if vk == Vk::J {
                    j_count += 1;
                    println!("Q count is {:?}", j_count);
                }
                if j_count == 3 {
                    println!("Speed quitting from game");
                    gamemenu.quit_from_game(enigo);
                    println!("Completed at: {:?}", Utc::now().date().to_string().blue());

                    return Ok(false);
                }
                if vk == Vk::O {
                    // mog 100
                    mogrun.run_count_total_absolute = 100;
                    println!("Mogrun called for 100 iterations");
                    let _ = mohgwyn::run(enigo, player, data, mogrun, history);
                }
                if vk == Vk::M {
                    // Close App
                    println!("graceful quit!");
                    return Ok(false);
                }
                if vk == Vk::I {
                    mogrun.run_count_total_absolute = 1;
                    println!("Mogrun called for 1 iteration");
                    let _ = mohgwyn::run(enigo, player, data, mogrun, history);
                }
                if vk == Vk::X {
                    println!("panic!");
                    panic!()
                }
                // add option to launch/relaunch game
                if vk == Vk::F1 {
                    if !check_elden_ring_is_running(enigo, gamemenu)? {
                        println!("Relaunching game...");
                        launch_elden_ring(enigo, gamemenu);
                    }
                }
                // add option to increase/decrease the value of w1, w2 and the turn?
                if vk == Vk::F2 {
                    println!(
                        "Increasing walk two by 1.\n{}:{}",
                        "it is now".cyan(),
                        history.walk1.to_string().cyan().bold()
                    );
                    history.walk1 += 1;
                }
                if vk == Vk::F3 {
                    println!(
                        "decreasing walk one by 1.\n{}:{}",
                        "it is now".cyan(),
                        history.walk1.to_string().yellow().bold()
                    );
                    history.walk1 -= 1;
                }
                if vk == Vk::F4 {
                    println!(
                        "Increasing walk two by 1.\n{}:{}",
                        "it is now".cyan(),
                        history.walk2.to_string().cyan().bold()
                    );
                    history.walk2 += 1;
                }
                if vk == Vk::F5 {
                    println!(
                        "decreasing walk two by 1.\n{}:{}",
                        "it is now".cyan(),
                        history.walk2.to_string().yellow().bold()
                    );
                    history.walk2 -= 1;
                }
                if vk == Vk::F12 {
                    println!("Screengrabbing...");
                    let timestamp = Utc::now().timestamp();
                    let _ = GameWindow::screengrab(
                        format!("{}_screengrab", timestamp).into(),
                        "png".into(),
                        "".into(),
                    )?;
                } else {
                    println!("{:?} was pressed!", vk);
                }
            }

            _ => (),
        }
        // clear console
        print!("\x1B[2J\x1B[1;1H");
    }
}

// NOTE use this later to remove all the fixed values by px that're powering the crops
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
