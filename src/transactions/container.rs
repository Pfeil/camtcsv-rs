use chrono::naive::NaiveDate;

use account::Account;
use util::{EuroDateFormat, SymbolFromIsoCode, CompressWhitespace};
// Use serdes serialize and deserialize derive macros. Requires Rust 1.30 or higher.
use serde_derive::*;

pub use crate::claude::Currency;

#[derive(Debug, Deserialize)]
pub enum TransactionType {
    ABSCHLUSS,
    UMBUCHUNG,
    UEBERTRAG,
    GUTSCHRIFT,
    DAUERAUFTRAG,
    KARTENZAHLUNG,
    FOLGELASTSCHRIFT,
    #[serde(rename = "ONLINE-UEBERWEISUNG")]
    ONLINE,
    BARGELDAUSZAHLUNG,
    #[serde(rename = "EINMAL LASTSCHRIFT")]
    EINMALLASTSCHRIFT,
    #[serde(rename = "SEPA-ELV-LASTSCHRIFT")]
    SepaElvLastschrift,
    #[serde(rename = "ECHTZEIT-UEBERWEISUNG")]
    EchtzeitUeberweisung,
}

// TODO directly serialize Transaction from CSV by using a deserialize method and stuff.

#[derive(Debug)]
pub struct Transaction {
    pub owner_account: Account,
    pub partner_account: Account,
    pub creation_date: NaiveDate,
    pub validation_date: NaiveDate,
    pub transaction_type: TransactionType,
    pub description: String,
    pub money: Currency,
    pub info: String,
}

impl From<RawTransaction> for Transaction {
    fn from(raw: RawTransaction) -> Self {
        let owner_account = Account {
            name: "Myself".to_owned().compress_whitespace(),
            iban: raw.owner_account.compress_whitespace(),
            bank: "Sparkasse".to_owned().compress_whitespace(),
        };
        let partner_account = Account {
            name: raw.partner.compress_whitespace(),
            iban: raw.partner_account.compress_whitespace(),
            bank: raw.partner_bank.compress_whitespace(),
        };
        Transaction {
            owner_account,
            partner_account,
            creation_date: NaiveDate::from_eu_date_str(&raw.creation_date),
            validation_date: NaiveDate::from_eu_date_str(&raw.validation_date),
            transaction_type: raw.transaction_type,
            description: raw.description.compress_whitespace(),
            money: Currency::from_str_and_code(raw.money, raw.currency),
            info: raw.info.compress_whitespace(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawTransaction {
    #[serde(rename = "Auftragskonto")]
    owner_account: String,
    #[serde(rename = "Buchungstag")]
    creation_date: String,
    #[serde(rename = "Valutadatum")]
    validation_date: String,
    #[serde(rename = "Buchungstext")]
    transaction_type: TransactionType,
    #[serde(rename = "Verwendungszweck")]
    description: String,

    #[serde(rename = "Glaeubiger ID")]
    creditor: String,
    #[serde(rename = "Mandatsreferenz")]
    mandate_ref: String,
    #[serde(rename = "Kundenreferenz (End-to-End)")]
    customer_ref: String,
    #[serde(rename = "Sammlerreferenz")]
    collector_ref: String,
    #[serde(rename = "Lastschrift Ursprungsbetrag")]
    debit_original_amount: String,
    #[serde(rename = "Auslagenersatz Ruecklastschrift")]
    expense_chargeback: String,

    #[serde(rename = "Beguenstigter/Zahlungspflichtiger")]
    partner: String,
    #[serde(rename = "Kontonummer/IBAN")]
    partner_account: String,
    #[serde(rename = "BIC (SWIFT-Code)")]
    partner_bank: String,
    #[serde(rename = "Betrag")]
    money: String,
    #[serde(rename = "Waehrung")]
    currency: String, // TODO enum EUR usw
    #[serde(rename = "Info")]
    info: String,
}
