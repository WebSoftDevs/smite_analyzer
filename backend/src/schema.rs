// @generated automatically by Diesel CLI.

diesel::table! {
    motds (id) {
        id -> Int4,
        description -> Varchar,
        game_mode -> Varchar,
        max_players -> Nullable<Int4>,
        name -> Varchar,
        ret_msg -> Nullable<Varchar>,
        start_date_time -> Varchar,
        team_1_gods_csv -> Nullable<Varchar>,
        team_2_gods_csv -> Nullable<Varchar>,
        mode -> Text,
    }
}
