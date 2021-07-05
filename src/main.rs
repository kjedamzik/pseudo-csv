use std::error::Error;
use std::io;
use std::process;

use clap::{App, Arg};
use rayon::prelude::*;

use pseudonymize::Pseudo;
use select::SelectColumns;

mod pseudonymize;
mod select;

fn pseudonymize(
    columns_to_pseudonymize: &str,
    no_headers: bool,
    delimiter: u8,
) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(!no_headers)
        .delimiter(delimiter)
        .from_reader(io::stdin());

    let headers = rdr.byte_headers()?.clone();

    let selection: Vec<usize> = SelectColumns::parse(columns_to_pseudonymize)
        .unwrap()
        .selection(&headers, !no_headers)
        .unwrap()
        .to_vec();

    let mut wtr = csv::Writer::from_writer(io::stdout());

    if !no_headers {
        wtr.write_record(rdr.headers()?)?;
    }

    let mut record_iter = rdr.byte_records();

    loop {
        let mut batch = vec![];
        for _ in 0..32768 {
            if let Some(record) = record_iter.next() {
                batch.push(record?);
            } else {
                break;
            }
        }
        if batch.is_empty() {
            break;
        }

        let pseudonymized_records: Vec<_> = batch
            .par_iter()
            .map(|record| record.pseudonymize(&selection))
            .collect();

        for record in pseudonymized_records {
            wtr.write_record(&record)?;
        }
    }
    Ok(())
}

static ABOUT: &'static str = "
this tool let's you pseudonymize csv data from STDIN.

Pseudonymize the first four columns:
$ pseudo-csv 1,2,3,4

...or use header names:
$ pseudo-csv col1,col2

...or ranges:
$ pseudo-csv col1,2-4
";

fn main() {
    let matches = App::new("pseudo-csv")
        .version(version().as_str())
        .about(ABOUT)
        .arg(
            Arg::with_name("selection")
                .help("Set the columns to pseudonymize by index, name or range. [default: all columns]")
                .required(false)
                .index(1),
        )
        .arg(
            Arg::with_name("delimiter")
                .short("d")
                .long("delimiter")
                .multiple(false)
                .takes_value(true)
                .value_name("arg")
                .default_value(",")
                .help("The field delimiter for reading CSV data. Must be a single character."),
        )
        .arg(
            Arg::with_name("no-header")
                .short("n")
                .long("no-header")
                .multiple(false)
                .help("When set, the first row will not be interpreted as headers."),
        )
        .get_matches();

    let no_headers = matches.is_present("no-header");

    let delimiter: u8 = matches.value_of("delimiter").unwrap().as_bytes()[0];

    let columns_to_pseudonymize = matches.value_of("selection").unwrap_or("");

    if let Err(err) = pseudonymize(columns_to_pseudonymize, no_headers, delimiter) {
        println!("ERROR: {}", err);
        process::exit(1);
    }
}

pub fn version() -> String {
    let (maj, min, pat) = (
        option_env!("CARGO_PKG_VERSION_MAJOR"),
        option_env!("CARGO_PKG_VERSION_MINOR"),
        option_env!("CARGO_PKG_VERSION_PATCH"),
    );
    match (maj, min, pat) {
        (Some(maj), Some(min), Some(pat)) => format!("{}.{}.{}", maj, min, pat),
        _ => "".to_owned(),
    }
}
