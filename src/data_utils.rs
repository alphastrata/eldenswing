use anyhow::Result;
use chrono::prelude::*;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

use crate::controller::MogRun;

/// PlayerHistory holds all the paramaters dictating how far the player walks or turns automatically during a MogRun.
/// MogRuns could potentially expand in the future with more timed walks, more turns etc and of course: Bird Shots.
/// walks one and two can be updated during runtime -- see the README.md for keybindings.
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
    pub session_start: DateTime<Utc>,
    pub soulscount: u32,
    pub timestamp: DateTime<Utc>,
    pub run_number: usize,
    pub playerhistory: PlayerHistory,
    pub session_end: u32,
    pub prev_run_yeild: u32,
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

/// Move the .pngs taken during play to the screenshots folder
/// Arguments:
/// * run_number - the number of the run being moved -- these timestamps help for diagnosing issues etc later.
/// For Example -- if you notice a death occured, these screenshots should help you find out how it is that your
/// character fell off a ledge and where it was etc.
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

/// Row holds all the data we wish to capture from the game and write to csv.
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

/// Writes the data from the Row struct to a csv file
/// Arguments:
/// * MogRun the struct containing most of the data to be written
/// * best: the best run
/// * worst: the worst run
/// * data: the data to write
/// * run_number: the run number to write to
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
fn get_mean(slice: &[usize]) -> f32 {
    if slice.len() < 1 {
        return 0.0;
    }
    let sum: usize = slice.iter().sum();
    return sum as f32 / slice.len() as f32;
}
