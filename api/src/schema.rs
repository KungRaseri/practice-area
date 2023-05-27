// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(
        diesel::sql_types::SqlType, serde::Serialize, serde::Deserialize, Debug, PartialEq,
    )]
    #[diesel(postgres_type(name = "AccountRole"))]
    pub struct AccountRole;

    #[derive(
        diesel::sql_types::SqlType, serde::Serialize, serde::Deserialize, Debug, PartialEq,
    )]
    #[diesel(postgres_type(name = "ServerStatus"))]
    pub struct ServerStatus;

    #[derive(
        diesel::sql_types::SqlType, serde::Serialize, serde::Deserialize, Debug, PartialEq,
    )]
    #[diesel(postgres_type(name = "TileType"))]
    pub struct TileType;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::AccountRole;

    Account (id) {
        id -> Text,
        email -> Text,
        passwordHash -> Text,
        userAuthToken -> Text,
        role -> AccountRole,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    Biome (id) {
        id -> Text,
        name -> Text,
        precipitationMin -> Float8,
        precipitationMax -> Float8,
        temperatureMin -> Float8,
        temperatureMax -> Float8,
        foodModifier -> Int4,
        oreModifier -> Int4,
        plotAreaMax -> Int4,
        plotAreaMin -> Int4,
        plotsMax -> Int4,
        plotsMin -> Int4,
        solarModifier -> Int4,
        stoneModifier -> Int4,
        waterModifier -> Int4,
        windModifier -> Int4,
        woodModifier -> Int4,
    }
}

diesel::table! {
    Plot (id) {
        id -> Text,
        tileId -> Text,
        area -> Int4,
        food -> Int4,
        ore -> Int4,
        solar -> Int4,
        stone -> Int4,
        water -> Int4,
        wind -> Int4,
        wood -> Int4,
    }
}

diesel::table! {
    Profile (id) {
        id -> Text,
        username -> Text,
        picture -> Text,
        accountId -> Text,
    }
}

diesel::table! {
    ProfileServerData (id) {
        profileId -> Text,
        serverId -> Text,
        id -> Text,
    }
}

diesel::table! {
    Region (id) {
        id -> Text,
        name -> Text,
        elevationMap -> Jsonb,
        precipitationMap -> Jsonb,
        temperatureMap -> Jsonb,
        worldId -> Text,
        xCoord -> Int4,
        yCoord -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::ServerStatus;

    Server (id) {
        id -> Text,
        name -> Text,
        hostname -> Text,
        port -> Int4,
        status -> ServerStatus,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::table! {
    Settlement (id) {
        id -> Text,
        name -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
        playerProfileId -> Text,
        plotId -> Text,
    }
}

diesel::table! {
    SettlementStructure (id) {
        id -> Text,
        settlementId -> Text,
        level -> Int4,
        structureId -> Text,
    }
}

diesel::table! {
    Structure (id) {
        id -> Text,
        name -> Text,
        description -> Text,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::TileType;

    Tile (id) {
        id -> Text,
        elevation -> Float8,
        temperature -> Float8,
        precipitation -> Float8,
        #[sql_name = "type"]
        type_ -> TileType,
        regionId -> Text,
        biomeId -> Text,
    }
}

diesel::table! {
    World (id) {
        id -> Text,
        name -> Text,
        elevationSettings -> Jsonb,
        precipitationSettings -> Jsonb,
        temperatureSettings -> Jsonb,
        serverId -> Text,
        createdAt -> Timestamp,
        updatedAt -> Timestamp,
    }
}

diesel::joinable!(Plot -> Tile (tileId));
diesel::joinable!(Profile -> Account (accountId));
diesel::joinable!(ProfileServerData -> Profile (profileId));
diesel::joinable!(ProfileServerData -> Server (serverId));
diesel::joinable!(Region -> World (worldId));
diesel::joinable!(Settlement -> Plot (plotId));
diesel::joinable!(SettlementStructure -> Settlement (settlementId));
diesel::joinable!(SettlementStructure -> Structure (structureId));
diesel::joinable!(Tile -> Biome (biomeId));
diesel::joinable!(Tile -> Region (regionId));
diesel::joinable!(World -> Server (serverId));

diesel::allow_tables_to_appear_in_same_query!(
    Account,
    Biome,
    Plot,
    Profile,
    ProfileServerData,
    Region,
    Server,
    Settlement,
    SettlementStructure,
    Structure,
    Tile,
    World,
);
