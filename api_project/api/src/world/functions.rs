use diesel::PgConnection;

use crate::models::World;

pub fn get_all_worlds(conn: &mut PgConnection) -> diesel::QueryResult<World> {
    use crate::schema::World::dsl::*;

    let results = World
        .load::<World>(conn)
        .expect("Error when loading worlds");

    Ok(results)
}
