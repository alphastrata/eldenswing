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

    // get an item crop and validate it's the wave sword.
    let weapon_crop = GameWindow::crop_rh_weapon(PathBuf::from("starting_souls.png"))?;

    // DEBUG for pathing>< DO NOT REMOVE
    // println!("weapon_crop is {:?}", weapon_crop.as_path().display());
    // println!("wave_sword is {:?}", WAVE_SWORD);
    let dssim = cv_utils::dssim_compare(weapon_crop, PathBuf::from(WAVE_SWORD))?;
    if dssim > 0.03 {
        println!("weapon is not equipped");
        println!("{}", dssim);
        panic!("WAVE_SWORD not equipped");
    } else {
        println!("{}", dssim);
        println!("weapon is equipped");
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
        let _ = cleanup_tmp_png();
        let delta = mogrun.souls_this_run - mogrun.souls_last_run;

        if mogrun.souls_this_run < 1 {
            panic!("A death has occured");
        }

        std::thread::sleep(Duration::from_millis(4500));
        if delta > best && delta < 99999 {
            best = delta;
        }

        if delta < worst {
            worst = delta;
        }

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

        mogrun.souls_last_run = mogrun.souls_this_run;
        mogrun.yield_total += mogrun.souls_this_run;
        mogrun.souls_this_run = 0;
        mogrun.run_count_total_thusfar += 1;
        let _ = data_utils::write_to_csv(mogrun, best.try_into()?, worst.try_into()?, history);
    }

    println!("see ya tarnished!");
    println!("END_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    println!("--------------------------------------------------------------");
    Ok(())
}

// TODO: Move these to data_utils
fn cleanup_tmp_png() -> Result<()> {
    // remove all png files in dir
    let path = PathBuf::from("./");
    let files = std::fs::read_dir(path)?;
    for file in files {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name.to_str().expect("unable to stringify file_name");
        if file_name.ends_with(".png") {
            let output = format!(
                "./screenshots/{}_soulcounter_crop.png",
                Utc::now().timestamp()
            );
            std::fs::rename(file.path(), output)?;
            std::fs::remove_file(file.path())?;
        }
    }
    Ok(())
}
