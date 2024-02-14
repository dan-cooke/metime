use regex::Regex;
use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let rows = parse_input(&buffer);

    println!("Date,Notes");
    for row in rows {
        println!("{},{}", row.date, row.notes)
    }
}

struct TimesheetRow {
    date: String,
    notes: String,
}

fn parse_input(input: &str) -> Vec<TimesheetRow> {
    let regex = Regex::new(r"Date:\s+([^\n]*)\n+(.*)").unwrap();

    regex
        .captures_iter(input)
        .map(|x| {
            let (_, [date, msg]) = x.extract::<2>();
            TimesheetRow {
                date: date.to_owned(),
                notes: msg.to_owned(),
            }
        })
        .collect()
}
