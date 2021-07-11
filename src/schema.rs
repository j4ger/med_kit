table! {
    use diesel::sql_types::*;
    use crate::models::Stage;

    products (id) {
        id -> Int4,
        product_barcode -> Varchar,
        profile_id -> Nullable<Int4>,
        init_time -> Timestamp,
        current_stage -> Stage,
        report_id -> Nullable<Int4>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::Stage;

    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        product_id -> Int4,
        submit_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(products, profiles,);
