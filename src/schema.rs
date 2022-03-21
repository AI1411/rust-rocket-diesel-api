use diesel::table;
table! {
    todo (id) {
        id -> Int4,
        title -> Varchar,
        checked -> Bool,
    }
}
