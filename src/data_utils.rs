use std::Path::{Path, PathBuf};
use cv_utils::read_soul_counter;

struct PlayerHistory{
	walk1: u32,
	turn_angle: u32,
	walk2: u32,
	wave_wait: f64, // frames or secs?
	grace_wait: f64, // frames or secs?
	_player_lvl: u32,

}
struct Data{
	session_start: u32, // will actually be a timestamp
	soulscount: u32,
	timestamp: String, // will actually be a Utc::Datetime
	run_number: u32,
	playerhistory: PlayerHistory,
	session_end: u32, // will actually be a timestamp

}
impl Data{
	// save all data to disk
	fn write_run_data(){
		todo!()
	}

	//
	// Helpers to get avg yield on runs
	// pass verbose as true to have them print to stdout
	fn running_avg_by_run(verbose: bool)-> u32{
		todo!()
	}
	fn running_avg_by_h(verbose: bool)-> u32{
		todo!()
	}
	fn running_avg_by_m(verbose: bool)-> u32{
		todo!()
	}
	fn running_avg_by_s(verbose: bool)-> u32{
		todo!()
	}
}

struct SoulsCounter{{}
impl SoulsCounter{
	fn new() -> SoulsReader{
		SoulsReader{}
	}
	fn read_souls_counter(img: PathBuf) -> u32{
		cv_utils::read_soul_counter()
	}
}