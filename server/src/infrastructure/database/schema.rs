// @generated automatically by Diesel CLI.

diesel::table! {
    achievements (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 255]
        icon_url -> Nullable<Varchar>,
        #[max_length = 50]
        condition_type -> Nullable<Varchar>,
        condition_value -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

diesel::table! {
    brawler_achievements (brawler_id, achievement_id) {
        brawler_id -> Int4,
        achievement_id -> Int4,
        earned_at -> Timestamp,
    }
}

diesel::table! {
    brawlers (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 255]
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        #[max_length = 50]
        display_name -> Varchar,
        #[max_length = 512]
        avatar_url -> Nullable<Varchar>,
        #[max_length = 255]
        avatar_public_id -> Nullable<Varchar>,
        mission_success_count -> Int4,
        mission_join_count -> Int4,
        #[max_length = 255]
        email -> Nullable<Varchar>,
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

diesel::joinable!(brawler_achievements -> achievements (achievement_id));
diesel::joinable!(brawler_achievements -> brawlers (brawler_id));
diesel::joinable!(crew_memberships -> brawlers (brawler_id));
diesel::joinable!(crew_memberships -> missions (mission_id));
diesel::joinable!(missions -> brawlers (chief_id));

diesel::allow_tables_to_appear_in_same_query!(
    achievements,
    brawler_achievements,
    brawlers,
    crew_memberships,
    missions,
);
