mod controller;
mod cv_utils;
mod data_utils;
mod os_reader;

use anyhow::Result;
use chrono::prelude::*;
use colored::*;
use cv_utils::GameWindow;
use std::collections::HashMap;
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
    let gamemenu = controller::GameMenus::new();

    // these are for mostly for data collection
    let mut mogrun = controller::MogRun::new();

    if !os_reader::check_elden_ring_is_running(&mut enigo, &gamemenu)? {
        panic!("Elden Ring is not running");
    }

    // start at Mog
    mogrun.time_app_spartup_utc = Utc::now();
    std::thread::sleep(Duration::from_secs(2)); // needs to be long enough for initial read..

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
    std::thread::sleep(Duration::from_secs(3)); // needs to be long enough for initial read..

    mogrun.teleport(&mut enigo, &player);

    // allow the user some alt-tab time, also this prevents the first run from going too short.
    std::thread::sleep(Duration::from_secs(2));

    // read it whilst waiting for teleport in...
    mogrun.starting_souls =
        cv_utils::external_tesseract_call("current_souls_cropped.png".into(), "eng".into())?;
    println!("{:#?} starting_souls", mogrun.starting_souls);
    let _ = GameWindow::crop_souls_counter(PathBuf::from(r"starting_souls.png"))?;

    // ----------------- MAIN LOOP ------------------
    // How many runs do you wanna do?
    mogrun.run_count_total_absolute = 101;

    let mut walk1 = 118;
    let mut walk2 = 68;
    let mut wave_wait = 6789; //ms
    let mut runs: Vec<usize> = Vec::new();

    for n in 1..mogrun.run_count_total_absolute {
        let stopwatch = std::time::Instant::now();
        if !os_reader::check_elden_ring_is_running(&mut enigo, &gamemenu)? {
            panic!("Elden Ring is not running");
        }
        mogrun.current_run_number = n as usize;

        // this is being recreated here because I cannot work out how to solve a lifetime issue with the Copy thing...

        // These values were good when NOT streaming..
        let history: PlayerHistory = PlayerHistory::new_from(walk1, walk2, 90, wave_wait, 0, 0);

        // Values to use when streaming...
        // let history: PlayerHistory = PlayerHistory::new_from(walk1, walk2, 90, wave_wait, 0, 0);

        // Check we haven't died...
        if mogrun.souls_this_run < 1 {
            gamemenu.quit_from_game(&mut enigo);
            panic!("A death has occured");
        }

        // ================== MOGRUN ==================
        enigo.key_down(Key::Space);
        mogrun.run_count_total_thusfar += 1;
        mogrun.run(&mut enigo, &player, history);
        enigo.key_up(Key::Space);

        // ------------------DATA----------------------
        mogrun.current_run_end_utc = Utc::now();

        let _ = GameWindow::crop_souls_counter(PathBuf::from(r"starting_souls.png"))?;
        mogrun.souls_this_run = cv_utils::external_tesseract_call(
            "current_souls_cropped.png".to_string(),
            "eng".to_string(),
        )?;
        let _ = data_utils::cleanup_tmp_png();
        mogrun.souls_delta = mogrun.souls_this_run - mogrun.souls_last_run;

        //TODO: how much smaller can this sleep get...
        std::thread::sleep(Duration::from_millis(4300));
        if mogrun.souls_delta > mogrun.souls_best_thusfar && mogrun.souls_delta < 99999 {
            mogrun.souls_best_thusfar = mogrun.souls_delta;
        }

        if mogrun.souls_delta < mogrun.souls_worst_thusfar {
            mogrun.souls_worst_thusfar = mogrun.souls_delta;
        }

        mogrun.yield_total += mogrun.souls_delta;
        mogrun.souls_last_run = mogrun.souls_this_run;
        mogrun.run_count_total_thusfar += 1;

        // -------------------- UI -----------------------
        println!("--------------------------------------------------------------");
        println!("Starting Souls: {:^12}", &mogrun.starting_souls);
        println!(
            "Souls from bot: {:^12}",
            &mogrun.souls_this_run - &mogrun.starting_souls
        );
        println!("Souls vs last : {:^12}", &mogrun.souls_delta);
        println!("Run# :{}/{}", &n, &mogrun.run_count_total_absolute);
        println!("Best run : {:^6}", &mogrun.souls_best_thusfar);
        println!("Worst run: {:^6}", &mogrun.souls_worst_thusfar);

        // you don't wanna be pushing the initial souls_counter's value onto this as it can... really make the avg look awesome, when it isn't.
        if n > 1 {
            runs.push(mogrun.souls_delta.clone());
            let median = get_median(&mut runs);
            let mode = get_mode(&runs);
            let mean = get_mean(&runs);
            println!("Median: {:^6}", median);
            println!("Mean: {:^6}", mean);
            println!("Mode: {:#?}", mode);
        }

        println!("Run took : {}ms ", stopwatch.elapsed().as_millis());
        let _ = data_utils::write_to_csv(mogrun, history, &runs);
    }

    println!("see ya tarnished!");
    println!("END_TIME: {:^40}", Utc::now().format("%H:%M:%S %D%m%Y"));
    println!("--------------------------------------------------------------");
    Ok(())
}

fn get_median(v: &mut Vec<usize>) -> f32 {
    if v.len() < 1 {
        return 0.0;
    }

    let mut vec = v.clone();
    vec.sort();
    if vec.len() % 2 == 1 {
        return *vec.get(vec.len() / 2).unwrap() as f32;
    }
    return (*vec.get(vec.len() / 2 - 1).unwrap() + *vec.get(vec.len() / 2).unwrap()) as f32 / 2.0;
}

fn get_mode(slice: &[usize]) -> HashMap<&usize, i32> {
    let mut map = HashMap::with_capacity(slice.len());
    if slice.is_empty() {
        return map;
    }

    for num in slice {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }
    let _max_value: i32 = map.values().map(|v| *v).max().unwrap();
    map
}

fn get_mean(slice: &[usize]) -> f32 {
    if slice.len() < 1 {
        return 0.0;
    }
    let sum: usize = slice.iter().sum();
    return sum as f32 / slice.len() as f32;
}
