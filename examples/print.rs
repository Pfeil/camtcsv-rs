extern crate camtcsv;

use camtcsv::prelude::*;
use std::env;

fn main() -> Result<(), Box<std::error::Error>> {
    let args = env::args().collect::<Vec<String>>();
    let path = args.get(1);
    let path = path.expect("You need to give a path to a camt csv file as first argument.");

    match TransactionManager::vec_from_file(path) {
        Ok(transactions) => {
            for t in &transactions {
                println!("{:?}\n", t);
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}
