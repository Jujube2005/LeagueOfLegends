// @generated automatically by Diesel CLI.

diesel::table! {
    brawlers (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

diesel::table! {
    crew_memberships (mission_id, brawler_id) {
        mission_id -> Int4,
        brawler_id -> Int4,
        joined_at -> Timestamp,
    }
}

diesel::table! {
    missions (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        status -> Varchar,
        chief_id -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        deleted_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    mission_messages (id) {
        id -> Int4,
        mission_id -> Int4,
        user_id -> Nullable<Int4>,
        content -> Text,
        #[max_length = 50]
        #[sql_name = "type"]
        type_ -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(crew_memberships -> brawlers (brawler_id));
diesel::joinable!(crew_memberships -> missions (mission_id));
diesel::joinable!(missions -> brawlers (chief_id));
diesel::joinable!(mission_messages -> missions (mission_id));
diesel::joinable!(mission_messages -> brawlers (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    brawlers,
    crew_memberships,
    missions,
    mission_messages,
);
