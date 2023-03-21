// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "test_enum"))]
    pub struct TestEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TestEnum;

    motd (id) {
        id -> Int4,
        name -> Varchar,
        description -> Varchar,
        game_mode -> Varchar,
        max_players -> Nullable<Int2>,
        ret_msg -> Nullable<Varchar>,
        start_date_time -> Varchar,
        team_1_gods_csv -> Nullable<Varchar>,
        team_2_gods_csv -> Nullable<Varchar>,
        title -> Varchar,
        mode -> TestEnum,
    }
}
