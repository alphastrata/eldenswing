use crate::controller::MogRun;
use prettytable::*;

// from the controller::MogRun struct, print everything using prettytable
pub fn setup_table(m: MogRun) -> Table {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('-', '+', '+', '+'),
        )
        .padding(1, 1)
        .build();
    table.set_format(format);

    table.set_titles(row![
        "run #", // m.num_runs
        "soul_yield",
        "soul_split",
        "secs elapsed",
        "runs/sec",
        "souls/sec",
    ]);
    table
}
pub fn update_table(m: MogRun, table: &mut Table) -> &mut Table {
    let soul_split = m.newest_reading - m.prev_run;
    let secs_elapsed = (m.current_run_endtime - m.current_run_starttime).num_milliseconds();
    let rps: i64 = m.runs_completed as i64 / secs_elapsed as i64;
    let sps = (m.yield_total) as i64 / secs_elapsed as i64;

    table.add_row(row![
        m.runs_completed,
        m.souls_earned,
        soul_split,
        secs_elapsed / 100,
        rps,
        sps,
    ]);

    // clear terminal
    print!("{}[2J", 27 as char);
    table
}
