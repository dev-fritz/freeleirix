// @generated automatically by Diesel CLI.

diesel::table! {
    projects (id) {
        id -> Int4,
        #[max_length = 255]
        title -> Varchar,
        description -> Nullable<Text>,
        subscriber_id -> Int4,
        freelancer_id -> Nullable<Int4>,
        value -> Nullable<Int4>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        removed -> Nullable<Bool>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        description -> Nullable<Text>,
        contractor -> Nullable<Bool>,
        freelancer -> Nullable<Bool>,
        #[max_length = 255]
        doc -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        removed -> Nullable<Bool>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    projects,
    users,
);
