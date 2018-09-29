#[derive(Debug)]
pub struct Account {
    pub name: String,
    pub iban: String, // TODO enum IBAN und Kontonr
    pub bank: String, // TODO enum BIC, SWIFT oder BLZ
}
