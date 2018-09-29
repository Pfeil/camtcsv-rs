use chrono::naive::NaiveDate;

use std::borrow::Borrow;
use std::fmt::Debug;
use std::str::FromStr;

use money::Money;

#[allow(dead_code)]
pub fn parse_date(text: &str) -> NaiveDate {
    let vec: Vec<&str> = text.split('.').collect();
    vec_to_date(&vec)
}

#[allow(dead_code)]
pub fn parse_money(currency: &str, value: &str) -> Money {
    let (unit, subunit): (i64, u64) = parse_tuple(&value, ",");
    match currency {
        "EUR" => Money::EUR(unit, subunit),
        "USD" => Money::USD(unit, subunit),
        _ => panic!(format!("unknown currency: {}", currency)),
    }
}

fn parse_tuple<T1, T2>(s: &str, delimiter: &str) -> (T1, T2)
where
    T1: FromStr,
    T1::Err: Debug,
    T2: FromStr,
    T2::Err: Debug,
{
    let vec: Vec<&str> = s.split(delimiter).collect();
    assert_eq!(vec.len(), 2);
    (T1::from_str(vec[0]).unwrap(), T2::from_str(vec[1]).unwrap())
}

fn vec_to_date(vec: &[&str]) -> NaiveDate {
    if let [ref day, ref month, ref year] = vec.borrow()[..] {
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
