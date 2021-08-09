table! {
    journal_entries (id) {
        id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
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
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
        name -> Text,
    }
}

table! {
    params (id) {
        id -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        deleted_at -> Nullable<Timestamp>,
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
