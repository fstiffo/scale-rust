extern crate diesel;

use diesel::prelude::*;
use scale_rust::models::*;
use scale_rust::*;

fn main() {
    use scale_rust::schema::journal_entries::dsl::*;

    let connection = establish_connection();
    let results = journal_entries
        .load::<JournalEntry>(&connection)
        .expect("Error loading posts");
    println!("Displaying {} journal entries", results.len());
    for entry in results {
        println!(
            "{} {} {} {} {} {:?} {:?}",
            entry.id,
            entry.date,
            entry.debit,
            entry.credit,
            entry.account,
            entry.owner_id,
            entry.description
        )
    }
}
