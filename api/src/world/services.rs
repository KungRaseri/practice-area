use crate::{DbPool};
use actix_web::{delete, get, post, put, web, error, HttpResponse, Responder, Result};
use super::functions::{get_all_worlds};

#[get("/world")]
async fn world_get(pool: web::Data<DbPool>) -> Result<impl Responder> {
    let worlds = web::block(move || {
        let mut conn = pool.get().expect("Failed to acquire database connection from pool");

        get_all_worlds(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(worlds.to_vec()))
}

#[post("/world")]
async fn world_new(pool: web::Data<DbPool>, params: web::Json<World>) -> Result<impl Responder> {
    let mut worlds = pool.World.lock().unwrap();
    let mut id: String = cuid::cuid2();

    worlds.push(World {
        id: id.to_string(),
        serverId: params.serverId.to_string(),
        name: params.name.to_string(),
        elevationSettings: params.elevationSettings.clone(),
        precipitationSettings: params.precipitationSettings.clone(),
        temperatureSettings: params.temperatureSettings.clone(),
        createdAt: params.createdAt,
        updatedAt: params.updatedAt,
    });

    Ok(HttpResponse::Ok().json(worlds.to_vec()))
}

#[put("/world/{id}")]
async fn world_update(
    pool: web::Data<DbPool>,
    path: web::Path<String>,
    params: web::Json<World>,
) -> impl Responder {
    let id = path.into_inner();
    let mut worlds = data.worlds.lock().unwrap();

    for index in 0..worlds.len() {
        if worlds[index].id == id {
            worlds[index] = params.clone();
        }
    }

    HttpResponse::Ok().json(worlds.to_vec())
}

#[delete("/world/{id}")]
async fn world_delete(data: web::Data<AppState>, path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    let mut worlds = data.worlds.lock().unwrap();

    *worlds = worlds
        .to_vec()
        .into_iter()
        .filter(|world| world.id != id)
        .collect();

    HttpResponse::Ok().json(worlds.to_vec())
}

pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(world_get)
        .service(world_new)
        .service(world_update)
        .service(world_delete);
}
