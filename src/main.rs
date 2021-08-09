extern crate chrono;
extern crate cursive;
extern crate cursive_table_view;
extern crate diesel;

use chrono::NaiveDateTime;
use diesel::prelude::*;
use scale_rust::models::*;
use scale_rust::*;

use std::cmp::Ordering;

use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;

use cursive_table_view::{TableView, TableViewItem};

use scale_rust::models::*;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Date,
    Account,
    Amount,
}

impl BasicColumn {
    fn as_str(&self) -> &str {
        match *self {
            BasicColumn::Date => "Date",
            BasicColumn::Account => "Account",
            BasicColumn::Amount => "Amount",
        }
    }
}

#[derive(Clone, Debug)]
struct Entry {
    date: NaiveDateTime,
    account: Account,
    amount: i32,
}

impl TableViewItem<BasicColumn> for Entry {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Date => self.date.to_string(),
            BasicColumn::Account => format!("{:?}", self.account),
            BasicColumn::Amount => format!("{}", self.amount),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Date => self.date.cmp(&other.date),
            BasicColumn::Account => Ordering::Equal,
            BasicColumn::Amount => self.amount.cmp(&other.amount),
        }
    }
}

fn main() {
    use scale_rust::schema::journal_entries::dsl::*;

    let connection = establish_connection();
    let results = journal_entries
        .load::<JournalEntry>(&connection)
        .expect("Error loading posts");
    // println!("Displaying {} journal entries", results.len());
    // for entry in results {
    //     println!(
    //         "{} {} {} {} {} {:?} {:?}",
    //         entry.id,
    //         entry.date,
    //         entry.debit,
    //         entry.credit,
    //         entry.account,
    //         entry.owner_id,
    //         entry.description
    //     )
    // }
    let mut siv = cursive::default();
    let mut table = TableView::<Entry, BasicColumn>::new()
        .column(BasicColumn::Date, "Date", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(20)
        })
        .column(BasicColumn::Account, "Account", |c| c.align(HAlign::Left))
        .column(BasicColumn::Amount, "Amount", |c| {
            c.width_percent(20).align(HAlign::Right)
        });

    let mut items = Vec::new();
    for entry in results {
        items.push(Entry {
            date: entry.date,
            account: Account::from_i32(entry.account),
            amount: entry.credit - entry.debit,
        });
    }

    table.set_items(items);

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {
        let value = siv
            .call_on_name("table", move |table: &mut TableView<Entry, BasicColumn>| {
                format!("{:?}", table.borrow_item(index).unwrap())
            })
            .unwrap();

        siv.add_layer(
            Dialog::around(TextView::new(value))
                .title(format!("Removing row # {}", row))
                .button("Close", move |s| {
                    s.call_on_name("table", |table: &mut TableView<Entry, BasicColumn>| {
                        table.remove_item(index);
                    });
                    s.pop_layer();
                }),
        );
    });

    siv.add_layer(Dialog::around(table.with_name("table").min_size((50, 20))).title("Table View"));

    siv.run();
}
