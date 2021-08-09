enum Account {
    StairsPayment = 1,
    Loan = 2,
	Repayment= 3,
	DuesPayment = 4,
	Expenditure = 5,
	Revenue = 6
}

#[derive(Queryable)]
pub struct JournalEntry {
    pub id: i32,
    pub date: NativeDateTime,
    pub debit: i32,
    pub credit: i32,
    pub account: Account,
    pub owner_id: i32,
    pub description: String

#[derive(Queryable)]
pub struct Owner {
    pub id: i32,
    pub name: String
}

#[derive(Queryable)]
pub struct Param {
    pub id: i32,
    pub valid_from: NaiveDateTime,
    pub stairs_cleaning_fee: i32,
    pub cleanings_per_month: i32,
    pub monthly_dues_rate: i32, 
}
