use chrono::naive::NaiveDate;

use account::Account;
use money::Money;
use parser::{parse_date, parse_money};

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

#[derive(Debug)]
pub struct Transaction {
    pub owner_account: Account,
    pub partner_account: Account,
    pub creation_date: NaiveDate,
    pub validation_date: NaiveDate,
    pub transaction_type: TransactionType,
    pub description: String,
    pub money: Money, // TODO Need proper integer class for money
    pub info: String,
}

impl From<RawTransaction> for Transaction {
    fn from(raw: RawTransaction) -> Self {
        let owner_account = Account {
            name: "Myself".to_owned(),
            iban: raw.owner_account,
            bank: "Sparkasse".to_owned(),
        };
        let partner_account = Account {
            name: raw.partner,
            iban: raw.partner_account,
            bank: raw.partner_bank,
        };
        Transaction {
            owner_account,
            partner_account,
            creation_date: parse_date(&raw.creation_date),
            validation_date: parse_date(&raw.validation_date),
            transaction_type: raw.transaction_type,
            description: raw.description,
            money: parse_money(&raw.currency, &raw.money),
            info: raw.info,
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
