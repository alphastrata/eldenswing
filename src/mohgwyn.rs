use crate::controller::{MogRun, PlayerController};
use crate::cv_utils::GameWindow;
use crate::data_utils::{cleanup_tmp_png, write_to_csv, Data, PlayerHistory};
use anyhow::Result;
use chrono::prelude::*;
use enigo::Enigo;
use enigo::*;
use std::path::PathBuf;
use std::time::Duration;

pub fn run(
    enigo: &mut Enigo,
    player: &PlayerController,
    // _gamemenu: GameWindow,
    data: &mut Data,
    mogrun: &mut MogRun,
) -> Result<()> {
    // start at Mog
    println!("App running.");
    mogrun.time_app_spartup_utc = Utc::now();
    data.session_start = mogrun.time_app_spartup_utc;
    std::thread::sleep(Duration::from_secs(5)); // needs to be long enough for initial read..

    // get our initial ingame screengrab
    let _ = GameWindow::screengrab("starting_souls".into(), "png".into(), "".into())?;

    // crop it
    let _ = GameWindow::crop_souls_counter(PathBuf::from("starting_souls.png"))?;

    // allow the user some alt-tab time
    mogrun.teleport(enigo, &player);
    // allow them a moment to cancel/move their char etc before control of their keyboard is snatched
    std::thread::sleep(Duration::from_secs(3));

    // read it
    mogrun.starting_souls =
        GameWindow::external_tesseract_call("current_souls_cropped.png".into(), "eng".into())?;
    println!("{:#?} starting_souls", mogrun.starting_souls);
    let _ = GameWindow::crop_souls_counter(PathBuf::from(r"starting_souls.png"))?;
    // let mut souls_total_all_runs = vec![1];

    // let mut table = ui::setup_table(mogrun);
    let mut best = 0;
    let mut worst = 999999;

    // ----------------- MAIN LOOP ------------------
    // How many runs do you wanna do?
    mogrun.run_count_total_absolute = 101;
    for n in 1..mogrun.run_count_total_absolute {
        data.run_number = n as usize;
        mogrun.current_run_number = n as usize;

        // this is being recreated here because I cannot work out how to solve a lifetime issue with the Copy thing...
        let history: PlayerHistory = PlayerHistory::new_from(77, 40, 90, 0.0, 0.0, 0);
        // let history = *data.playerhistory.clone();

        // the actual run
        enigo.key_down(Key::Space);
        mogrun.run_count_total_thusfar += 1;
        mogrun.run(enigo, &player, history);
        enigo.key_up(Key::Space);

        mogrun.current_run_end_utc = Utc::now();

        let _ = GameWindow::crop_souls_counter(PathBuf::from(r"starting_souls.png"))?;
        mogrun.souls_this_run = (GameWindow::external_tesseract_call(
            "current_souls_cropped.png".to_string(),
            "eng".to_string(),
        )?) as i64;

        let _ = cleanup_tmp_png(n);
        let delta = mogrun.souls_this_run - mogrun.souls_last_run;

        std::thread::sleep(Duration::from_millis(4500));
        if delta > best && delta < 99999 {
            best = delta;
        }

        if delta < worst {
            worst = delta;
        }

        // STDOUT for user feedback...
        println!("--------------------------------------------------------------");
        println!("Starting Souls: {:^12}", mogrun.starting_souls);
        println!(
            "Souls from bot: {:^12}",
            mogrun.souls_this_run - mogrun.starting_souls as i64
        );
        println!("Souls vs last : {:^12}", delta);
        println!("Run# :{}/{}", n, mogrun.run_count_total_absolute);
        println!("Best run : {:^6}", best);
        println!("Worst run: {:^6}", worst);
        println!("--------------------------------------------------------------");

        mogrun.souls_last_run = mogrun.souls_this_run;
        mogrun.souls_this_run = 0;
        let _ = write_to_csv(*mogrun, best, worst, n);
    }
    Ok(())
}
