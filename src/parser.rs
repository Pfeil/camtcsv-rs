use chrono::naive::NaiveDate;
use claude::Currency;

use std::str::FromStr;

/// Parses a date in this specific format: dd.mm.yyyy or dd.mm.yy
/// TODO make this more generic or use a library.
pub fn parse_date(text: &str) -> NaiveDate {
    let vec: Vec<&str> = text.split('.').collect();
    if let [ref day, ref month, ref year] = vec[..] {
        let mut date = if year.len() == 2 {
            format!("20{}-{}-{}", year, month, day)
        } else {
            format!("{}-{}-{}", year, month, day)
        };
        NaiveDate::from_str(&date).unwrap()
    } else {
        panic!()
    }
}

pub fn parse_currency(string: String, currency: String) -> Currency {
    let mut money =
        Currency::from_string(&string).expect(&format!("Could not parse money string: {}", string));
    let symbol = match currency.as_str() {
        "EUR" => Some('€'),
        "USD" => Some('$'),
        "YEN" => Some('¥'),
        _ => None,
    };
    money.symbol = symbol;
    money
}
