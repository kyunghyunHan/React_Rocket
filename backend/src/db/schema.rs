// @generated automatically by Diesel CLI.

diesel::table! {
    heroes (id) {
        id -> Integer,
        name -> Varchar,
        identity -> Varchar,
        hometown -> Varchar,
        age -> Integer,
    }
}

diesel::table! {
    plant (PlantID) {
        PlantID -> Integer,
        BotanicalName -> Nullable<Varchar>,
        CommonName -> Nullable<Varchar>,
        Family -> Nullable<Varchar>,
        PlantType -> Nullable<Varchar>,
        SoilType -> Nullable<Varchar>,
        SoilpH -> Nullable<Varchar>,
        SunExposure -> Nullable<Varchar>,
    }
}

diesel::table! {
    students (id) {
        id -> Integer,
        firstname -> Varchar,
        lastname -> Text,
        age -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(heroes, plant, students,);
