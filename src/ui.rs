use crate::controller::MogRun;
use prettytable::format::Alignment;
use prettytable::*;
use std::iter::Sum;

// from the controller::MogRun struct, print everything using prettytable
pub fn setup_table(m: MogRun) -> Table {
    let mut table = Table::new();
    let format = format::FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separators(
            &[format::LinePosition::Top, format::LinePosition::Bottom],
            format::LineSeparator::new('─', '┬', '┌', '┐'),
        )
        .padding(2, 2)
        .build();
    table.set_format(format);

    table.set_titles(row![
        "run #", // m.num_runs
        "souls_total_all_runs",
        "soul_split",
        "ms elapsed",
        "runs/sec",
        "souls/sec",
    ]);
    // table.set_titles(Row::new(vec![Cell::new_align(
    //     &m.time_app_spartup_utc
    //         .format("%Y-%m-%d %H:%M:%S")
    //         .to_string()[..],
    //     Alignment::CENTER,
    // )
    // .with_hspan(6)]));
    table
}
pub fn update_table(
    m: MogRun,
    table: &mut Table,
    run_number: usize,
    souls_total_all_runs: Vec<i64>,
) -> &mut Table {
    let st_summed = souls_total_all_runs.iter().sum::<i64>() - m.starting_souls as i64;

    if run_number >= 1 && st_summed > 0 {
        let soul_split: i64 = (m.souls_this_run - m.souls_last_run) as i64;
        let secs_elapsed = (m.current_run_end_utc - m.current_run_start_utc).num_milliseconds();
        let sps: i64 = st_summed / secs_elapsed;
        let rps = m.current_run_number as i64 / secs_elapsed * 1000;

        table.add_row(row![
            m.current_run_number,
            st_summed,
            soul_split,
            (secs_elapsed / 100) as f64,
            rps,
            sps,
        ]);

        // clear terminal
        //  print!("{}[2J", 27 as char);
        table
    } else {
        table
    }
}
