use crate::controller::MogRun;
use anyhow::Result;
use chrono::prelude::*;
use csv::*;
use serde::Serialize;
use std::fs::OpenOptions;
use std::path::PathBuf;

// Data specifically pretaining to the RUN, i.e what inputs did we feed the player.
#[derive(Debug, Clone, Copy)]
pub struct PlayerHistory {
    pub walk1: usize, // value dictating the ammount of time/frames that the player walks from at spawn
    pub turn_angle: usize, // value dictating the degrees a player turns NOTE: needs to eventually become something the Compass can discern
    pub walk2: usize, // value dictating the ammount of time/frames that the player walks the second time
    pub wave_wait: f64, // frames or secs?
    pub grace_wait: f64, // frames or secs?
    pub player_lvl: u32, // unsure whether to capture this, maybe useful to make a runs for target level feature
}
impl PlayerHistory {
    pub fn new() -> PlayerHistory {
        PlayerHistory {
            walk1: 0,
            turn_angle: 0,
            walk2: 0,
            wave_wait: 0.0,
            grace_wait: 0.0,
            player_lvl: 0,
        }
    }
    pub fn new_from(
        walk1: usize,
        walk2: usize,
        turn_angle: usize,
        wave_wait: f64,
        grace_wait: f64,
        player_lvl: u32,
    ) -> PlayerHistory {
        PlayerHistory {
            walk1,
            turn_angle,
            walk2,
            wave_wait,
            grace_wait,
            player_lvl,
        }
    }
}

// representing all data we wish to capture from the game and interact with/present

#[derive(Serialize)]
struct Row {
    timestamp: String,
    starting_souls: usize,
    souls_this_run: usize,
    app_yield_total: usize,
    best_run: usize,
    worst_run: usize,
    app_startup: String,
    current_run_end_utc: String,
    current_run_start_utc: String,
    walk_one: usize,
    walk_two: usize,
    turn_angle: usize,
}

pub fn write_to_csv(m: MogRun, best: usize, worst: usize, p: PlayerHistory) -> Result<()> {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("assets/history.csv"))
        .unwrap();

    let w1 = p.walk1;
    let w2 = p.walk2;
    let turn_angle = p.turn_angle;

    let mut wtr = WriterBuilder::new().has_headers(true).from_writer(file);
    wtr.serialize(Row {
        app_startup: m.time_app_spartup_utc.to_string(),
        timestamp: Utc::now().timestamp().to_string(), // This is the machine parsable one (well, easier..)
        best_run: best,
        worst_run: worst,
        current_run_start_utc: m.current_run_start_utc.to_string(),
        current_run_end_utc: m.current_run_end_utc.to_string(),
        app_yield_total: m.yield_total,
        starting_souls: m.starting_souls,
        souls_this_run: m.starting_souls - m.souls_this_run,
        walk_one: w1,
        walk_two: w2,
        turn_angle: turn_angle,
    })?;
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
