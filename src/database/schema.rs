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
        report_id -> Nullable<Uuid>,
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
        name -> Varchar,
        age -> Int4,
        sample_time -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::Stage;
    use crate::models::Role;

    reports (id) {
        id -> Uuid,
        uploader_id -> Int4,
        filename -> Nullable<Varchar>,
        download_url -> Varchar,
        upload_time -> Timestamp,
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

joinable!(products -> profiles (profile_id));
joinable!(products -> reports (report_id));
joinable!(profiles -> users (user_id));
joinable!(reports -> users (uploader_id));

allow_tables_to_appear_in_same_query!(
    products,
    profiles,
    reports,
    users,
);
