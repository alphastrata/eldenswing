mod controller;
mod cv_utils;
mod data_utils;
use anyhow::Result;
use chrono::prelude::*;
use cv_utils::GameWindow;
use std::path::PathBuf;
// use winput::message_loop::{self, EventReceiver};
use data_utils::PlayerHistory;
use enigo::*;
use std::time::Duration;

const WAVE_SWORD: &str = r"C:\Users\jer\Documents\GitHub\eldenswing\assets\wave_sword.png";

const COMPASS_TIK: i32 = 381;
const REFRESH_RATE: u64 = 20; // game should be more like 16ms, this means we're slower
                              // ingame constants if required...
                              // let one_second = Duration::from_millis(1000);
                              // let one_frame = one_second / 60;

// +=====+======+ MAIN +=====+======+
fn main() -> Result<()> {
    println!("Hello tarnished...");

    // keyboard and event reader stuff
    let mut enigo = Enigo::new();

    // construct hepler structs to make gameplay easier to control
    let player = controller::PlayerController::new();
    // let gamemenu = controller::GameMenus::new();

    // these are for mostly for data collection
    let mut mogrun = controller::MogRun::new();

    // start at Mog
    mogrun.time_app_spartup_utc = Utc::now();
    std::thread::sleep(Duration::from_secs(5)); // needs to be long enough for initial read..

    // get our initial ingame screengrab to read soulcount etc..
    let _ = GameWindow::screengrab("starting_souls".into(), "png".into(), "".into())?;
    let _ = GameWindow::crop_souls_counter(PathBuf::from("starting_souls.png"))?;

    // *****SAFTEY FIRST******
    // Check player always has weapon equipped.
    let _ = GameWindow::check_rh_weapon()?;
    // Check we haven't died...
    if mogrun.souls_this_run < 1 {
        panic!("A death has occured");
    }
    std::thread::sleep(Duration::from_secs(5)); // needs to be long enough for initial read..

    mogrun.teleport(&mut enigo, &player);

    // allow the user some alt-tab time, also this prevents the first run from going too short.
    std::thread::sleep(Duration::from_secs(4));

    // read it whilst waiting for teleport in...
    mogrun.starting_souls =
        cv_utils::external_tesseract_call("current_souls_cropped.png".into(), "eng".into())?;
    println!("{:#?} starting_souls", mogrun.starting_souls);
    let _ = GameWindow::crop_souls_counter(PathBuf::from(r"starting_souls.png"))?;

    // preset these
    let mut best = 0;
    let mut worst = 999999;

    // ----------------- MAIN LOOP ------------------
    // How many runs do you wanna do?
    mogrun.run_count_total_absolute = 101;

    for n in 1..mogrun.run_count_total_absolute {
        mogrun.current_run_number = n as usize;

        // this is being recreated here because I cannot work out how to solve a lifetime issue with the Copy thing...
        let history: PlayerHistory = PlayerHistory::new_from(108, 64, 90, 0.0, 0.0, 0);

        // Check we haven't died...
        if mogrun.souls_this_run < 1 {
            panic!("A death has occured");
        }

        // ================== MOGRUN ==================
        enigo.key_down(Key::Space);
        mogrun.run_count_total_thusfar += 1;
        mogrun.run(&mut enigo, &player, history);
        enigo.key_up(Key::Space);

        mogrun.current_run_end_utc = Utc::now();

        let _ = GameWindow::crop_souls_counter(PathBuf::from(r"starting_souls.png"))?;
        mogrun.souls_this_run = cv_utils::external_tesseract_call(
            "current_souls_cropped.png".to_string(),
            "eng".to_string(),
        )?;
        let _ = data_utils::cleanup_tmp_png();
        let delta = mogrun.souls_this_run - mogrun.souls_last_run;

        std::thread::sleep(Duration::from_millis(4500));
        if delta > best && delta < 99999 {
            best = delta;
        }

        if delta < worst {
            worst = delta;
        }
        let _ = data_utils::write_to_csv(mogrun, best.try_into()?, worst.try_into()?, history);

        // -------------------- UI -----------------------
        println!("--------------------------------------------------------------");
        println!("Starting Souls: {:^12}", &mogrun.starting_souls);
        println!(
            "Souls from bot: {:^12}",
            &mogrun.souls_this_run - &mogrun.starting_souls
        );
        println!("Souls vs last : {:^12}", &delta);
        println!("Run# :{}/{}", &n, &mogrun.run_count_total_absolute);
        println!("Best run : {:^6}", &best);
        println!("Worst run: {:^6}", &worst);
        println!("--------------------------------------------------------------");

        mogrun.yield_total += mogrun.souls_this_run - mogrun.starting_souls;
        mogrun.souls_last_run = mogrun.souls_this_run;
        mogrun.run_count_total_thusfar += 1;
    }

    println!("see ya tarnished!");
    println!("END_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    println!("--------------------------------------------------------------");
    Ok(())
}
