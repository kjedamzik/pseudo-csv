use rayon::prelude::*;
use std::error::Error;
use std::process;
use std::{env, io};

mod pseudonymize;
use pseudonymize::Pseudo;

fn pseudonymize(columns_to_pseudonymize: &[usize]) -> Result<(), Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin());
    let mut wtr = csv::Writer::from_writer(io::stdout());

    wtr.write_record(rdr.headers()?)?;

    let mut record_iter = rdr.records();

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
            .map(|record| record.pseudonymize(columns_to_pseudonymize))
            .collect();

        for record in pseudonymized_records {
            wtr.write_record(&record)?;
        }
    }
    Ok(())
}

fn main() {
    let columns_to_pseudonymize: Vec<usize> = env::args()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect();

    if let Err(err) = pseudonymize(&columns_to_pseudonymize) {
        println!("ERROR: {}", err);
        process::exit(1);
    }
}
