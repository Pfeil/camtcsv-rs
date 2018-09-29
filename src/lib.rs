extern crate chrono;
extern crate csv;
#[macro_use]
extern crate serde_derive;

mod account;
mod money;
mod parser;
mod transaction;

use std::path::Path;
use transaction::{RawTransaction, Transaction};

#[allow(dead_code)]
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Transaction>, Box<std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(file);
    let mut transaction_list: Vec<Transaction> = Vec::new();
    for result in rdr.deserialize() {
        // Notice that we need to provide a type hint for automatic
        // deserialization.
        let record: RawTransaction = result?;
        transaction_list.push(record.into());
    }
    Ok(transaction_list)
}
