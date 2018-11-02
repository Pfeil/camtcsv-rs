use chrono::naive::NaiveDate;
use claude::Currency;
use std::fmt::Display;

use std::str::FromStr;

pub trait SymbolFromIsoCode {
    fn from_str_and_code<S>(amount: S, isocode: S) -> Currency
    where
        S: AsRef<str> + Display;
}

// TODO This traits name is too ugly.
impl SymbolFromIsoCode for Currency {
    /// Parse from string using Currency::from_string,
    /// but also setting the symbol by a given ISO-4217 code.
    ///
    /// # Supported ISO-4217 codes
    ///
    /// Unsupported codes result in a `Currency` instance without symbol!
    ///
    /// - "EUR" -> '€'
    /// - "USD" -> '$'
    /// - "YEN" -> '¥'
    ///
    /// ```
    /// use camtcsv::prelude::*;
    ///
    /// let euro = Currency::from_str_and_code("-13,37", "EUR");
    /// assert_eq!(euro.value, -1337_i64);
    /// assert_eq!(euro.symbol, Some('€'));
    ///
    /// let usd = Currency::from_str_and_code("13,37", "USD");
    /// assert_eq!(usd.value, 1337_i64);
    /// assert_eq!(usd.symbol, Some('$'));
    ///
    /// let yen = Currency::from_str_and_code("13.37", "JPY");
    /// assert_eq!(yen.value, 1337_i64);
    /// assert_eq!(yen.symbol, Some('¥'));
    ///
    /// let unknown = Currency::from_str_and_code("-13.37", "other");
    /// assert_eq!(unknown.value, -1337_i64);
    /// assert_eq!(unknown.symbol, None);
    ///
    /// let mut euro2 = Currency::from_str_and_code("13,37", "EUR");
    /// let euro2 = euro + euro2;
    /// assert_eq!(euro2.value, 0_i64);
    /// assert_eq!(euro2.symbol, Some('€'));
    /// ```
    fn from_str_and_code<S>(amount: S, isocode: S) -> Currency
    where
        S: AsRef<str> + Display,
    {
        let mut money = Currency::from_string(amount.as_ref())
            .unwrap_or_else(|| panic!("Parse failed: {}", amount));
        money.symbol = match isocode.as_ref() {
            "EUR" => Some('€'),
            "USD" => Some('$'),
            "JPY" => Some('¥'),
            _ => None,
        };
        money
    }
}

pub trait EuroDateFormat {
    fn from_eu_date_str(text: &str) -> NaiveDate;
}

impl EuroDateFormat for NaiveDate {
    /// Parses a date in this specific format: dd.mm.yyyy or dd.mm.yy
    fn from_eu_date_str(text: &str) -> NaiveDate {
        let vec: Vec<&str> = text.split('.').collect();
        if let [ref day, ref month, ref year] = vec[..] {
            let mut date = if year.len() == 2 {
                format!("20{}-{}-{}", year, month, day)
            } else {
                format!("{}-{}-{}", year, month, day)
            };
            NaiveDate::from_str(&date).unwrap()
        } else {
            panic!("Could not parse Date.")
        }
    }
}
