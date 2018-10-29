extern crate chrono;
extern crate csv;
extern crate serde_derive;

mod account;
mod money;
mod parser;
mod transaction;

use std::io::Read;
use std::path::Path;
use transaction::RawTransaction;

pub use transaction::Transaction;

#[allow(dead_code)]
pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Vec<Transaction>, Box<std::error::Error>> {
    let mut file = std::fs::File::open(path)?;

    // Transcode content into UTF-8 like this:
    //   Read file as bytes, cast into chars and then collect to a utf8 String.
    let mut bytes: Vec<u8> = Vec::new();
    file.read_to_end(&mut bytes)?;
    let utf8: String = bytes.into_iter().map(|byte| byte as char).collect();
    // While it may cause garbage in some cases, it formally should now be valid utf-8.
    // At least it works for ISO-8859-1 containing german umlauts.

    // now use the String to deserialize.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(true)
        .from_reader(utf8.as_bytes());
    let mut transaction_list: Vec<Transaction> = Vec::new();
    for result in rdr.deserialize() {
        // type hint
        let record: RawTransaction = result?;
        transaction_list.push(record.into());
    }
    Ok(transaction_list)
}
