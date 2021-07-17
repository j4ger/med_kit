table! {
    use diesel::sql_types::*;
    use crate::models::Stage;
    use crate::models::Role;

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
    use crate::models::Role;

    profiles (id) {
        id -> Int4,
        user_id -> Int4,
        submit_time -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::Stage;
    use crate::models::Role;

    users (id) {
        id -> Int4,
        username -> Nullable<Varchar>,
        wechat_id -> Nullable<Varchar>,
        user_role -> Role,
        password_hashed -> Nullable<Varchar>,
        phone_number -> Nullable<Int4>,
        sign_up_time -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    products,
    profiles,
    users,
);
