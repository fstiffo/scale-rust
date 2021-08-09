extern crate chrono;
use chrono::NaiveDateTime;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[repr(i32)]
pub enum Account {
    StairsPayment = 1,
    Loan = 2,
    Repayment = 3,
    DuesPayment = 4,
    Expenditure = 5,
    Revenue = 6,
}
impl Account {
    pub fn from_i32(value: i32) -> Account {
        match value {
            1 => Account::StairsPayment,
            2 => Account::Loan,
            3 => Account::Repayment,
            4 => Account::DuesPayment,
            5 => Account::Expenditure,
            6 => Account::Revenue,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[derive(Queryable)]
pub struct JournalEntry {
    pub id: i32,
    pub date: NaiveDateTime,
    pub debit: i32,
    pub credit: i32,
    pub account: i32,
    pub owner_id: Option<i32>,
    pub description: Option<String>,
}

#[derive(Queryable)]
pub struct Owner {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable)]
pub struct Param {
    pub id: i32,
    pub valid_from: NaiveDateTime,
    pub stairs_cleaning_fee: i32,
    pub cleanings_per_month: i32,
    pub monthly_dues_rate: i32,
}
