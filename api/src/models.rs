﻿// Generated by diesel_ext

#![allow(unused)]
#![allow(clippy::all)]

use crate::schema::sql_types::*;
use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use chrono::offset::Utc;
use chrono::DateTime;
use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "Account")]
pub struct Account {
    pub id: String,
    pub email: String, 
    pub passwordHash: String,
    pub userAuthToken: String,
    pub role: AccountRole,
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "Biome")]
pub struct Biome {
    pub id: String,
    pub name: String,
    pub precipitationMin: f64,
    pub precipitationMax: f64,
    pub temperatureMin: f64,
    pub temperatureMax: f64,
    pub foodModifier: i32,
    pub oreModifier: i32,
    pub plotAreaMax: i32,
    pub plotAreaMin: i32,
    pub plotsMax: i32,
    pub plotsMin: i32,
    pub solarModifier: i32,
    pub stoneModifier: i32,
    pub waterModifier: i32,
    pub windModifier: i32,
    pub woodModifier: i32,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "Plot")]
pub struct Plot {
    pub id: String,
    pub tileId: String,
    pub area: i32,
    pub food: i32,
    pub ore: i32,
    pub solar: i32,
    pub stone: i32,
    pub water: i32,
    pub wind: i32,
    pub wood: i32,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "Profile")]
pub struct Profile {
    pub id: String,
    pub username: String,
    pub picture: String,
    pub accountId: String,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "ProfileServerData")]
pub struct ProfileServerData {
    pub profileId: String,
    pub serverId: String,
    pub id: String,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "Region")]
pub struct Region {
    pub id: String,
    pub name: String,
    pub elevationMap: serde_json::Value,
    pub precipitationMap: serde_json::Value,
    pub temperatureMap: serde_json::Value,
    pub worldId: String,
    pub xCoord: i32,
    pub yCoord: i32,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "Server")]
pub struct Server {
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub port: i32,
    pub status: ServerStatus,
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "Settlement")]
pub struct Settlement {
    pub id: String,
    pub name: String,
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
    pub playerProfileId: String,
    pub plotId: String,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "SettlementStructure")]
pub struct SettlementStructure {
    pub id: String,
    pub settlementId: String,
    pub level: i32,
    pub structureId: String,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "Structure")]
pub struct Structure {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "Tile")]
pub struct Tile {
    pub id: String,
    pub elevation: f64,
    pub temperature: f64,
    pub precipitation: f64,
    pub type_: TileType,
    pub regionId: String,
    pub biomeId: String,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "World")]
pub struct World {
    pub id: String,
    pub name: String,
    pub elevationSettings: serde_json::Value,
    pub precipitationSettings: serde_json::Value,
    pub temperatureSettings: serde_json::Value,
    pub serverId: String,
    pub createdAt: NaiveDateTime,
    pub updatedAt: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Insertable, Queryable, Debug, PartialEq)]
#[diesel(table_name = "_prisma_migrations")]
pub struct PrismaMigration {
    pub id: String,
    pub checksum: String,
    pub finished_at: Option<DateTime<Utc>>,
    pub migration_name: String,
    pub logs: Option<String>,
    pub rolled_back_at: Option<DateTime<Utc>>,
    pub started_at: DateTime<Utc>,
    pub applied_steps_count: i32,
}
