table! {
    journal_entries (id) {
        id -> Nullable<Integer>,
        date -> Timestamp,
        debit -> Integer,
        credit -> Integer,
        account -> Integer,
        owner_id -> Nullable<Integer>,
        description -> Nullable<Text>,
    }
}

table! {
    owners (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

table! {
    params (id) {
        id -> Nullable<Integer>,
        valid_from -> Timestamp,
        stairs_cleaning_fee -> Integer,
        cleanings_per_month -> Integer,
        monthly_dues_rate -> Integer,
    }
}

joinable!(journal_entries -> owners (owner_id));

allow_tables_to_appear_in_same_query!(
    journal_entries,
    owners,
    params,
);
