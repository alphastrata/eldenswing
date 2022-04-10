use anyhow::Result;
use chrono::prelude::*;
use serde::Serialize;
use std::path::PathBuf;

use crate::controller::MogRun;

// Data specifically pretaining to the RUN, i.e what inputs did we feed the player.
#[derive(Debug, Clone)]
pub struct PlayerHistory {
    pub walk1: usize, // value dictating the ammount of time/frames that the player walks from at spawn
    pub turn_angle: usize, // value dictating the degrees a player turns NOTE: needs to eventually become something the Compass can discern
    pub walk2: usize, // value dictating the ammount of time/frames that the player walks the second time
    pub wave_wait: f64, // frames or secs?
    pub grace_wait: f64, // frames or secs?
    pub player_lvl: u32, // unsure whether to capture this, maybe useful to make a runs for target level feature
}
impl PlayerHistory {
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
#[derive(Debug, Clone)]
pub struct Data {
    pub session_start: DateTime<Utc>, // will actually be a timestamp
    pub soulscount: u32,              // get this from Tesseract with OCR
    pub timestamp: DateTime<Utc>, // representing the UPDATE time this data was last updated will actually be a Utc::Datetime
    pub run_number: usize,        // count this even if infinite
    pub playerhistory: PlayerHistory, // See comments above struct decleration
    pub session_end: u32,         // will actually be a timestamp
    pub prev_run_yeild: u32,      // soul yield previous run
}
impl Data {
    pub fn new(history: PlayerHistory) -> Data {
        Data {
            session_start: Utc::now(),
            soulscount: 0,
            timestamp: Utc::now(),
            run_number: 0,
            playerhistory: history,
            session_end: 0,
            prev_run_yeild: 0,
        }
    }
}

// TODO: Move these somewhere else
pub fn cleanup_tmp_png(run_number: usize) -> Result<()> {
    // remove all png files in dir
    let path = PathBuf::from("./");
    let files = std::fs::read_dir(path)?;
    for file in files {
        let file = file?;
        let file_name = file.file_name();
        let file_name = file_name.to_str().expect("unable to stringify file_name");
        if file_name.ends_with(".png") {
            let output = format!("./screenshots/{}_{}", run_number, file_name);
            std::fs::rename(file.path(), output)?;
            std::fs::remove_file(file.path())?;
        }
    }
    Ok(())
}

#[derive(Serialize)]
pub(crate) struct Row {
    run_number: usize,
    starting_souls: usize,
    souls_this_run: i64,
    app_yield_total: usize,
    best_run: i64,
    worst_run: i64,
    timestamp: String,
    app_startup: String,
    current_run_end_utc: String,
    current_run_start_utc: String,
    walk_one: f64,
    walk_two: f64,
    turn_angle: f64,
    avg_souls_per_run: f64,
    avg_souls_per_second: f64,
}

pub fn write_to_csv(m: MogRun, best: i64, worst: i64, run_number: usize) -> Result<()> {
    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("assets/history.csv"))
        .unwrap();

    let avg_souls_per_second = m.souls_this_run as f64
        / (m.current_run_end_utc.timestamp() - m.current_run_start_utc.timestamp()) as f64;
    let avg_souls_per_run = m.souls_this_run as f64 / m.run_count_total_thusfar as f64;

    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(file);
    wtr.serialize(Row {
        app_startup: m.time_app_spartup_utc.to_string(),
        run_number,
        starting_souls: m.starting_souls,
        souls_this_run: m.souls_this_run - m.starting_souls as i64,
        best_run: best,
        worst_run: worst,
        timestamp: Utc::now().timestamp().to_string(),
        app_yield_total: m.yield_total,
        current_run_start_utc: m.current_run_start_utc.to_string(),
        current_run_end_utc: m.current_run_end_utc.to_string(),
        walk_one: m.walk_one,
        walk_two: m.walk_two,
        turn_angle: m.turn_angle,
        avg_souls_per_run,
        avg_souls_per_second,
    })?;
    Ok(())
}
