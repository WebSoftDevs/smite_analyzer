use super::DbPool;

use actix_web::{get, web, Error, HttpResponse};
use diesel::prelude::*;

use crate::motd::Motd;

type DbError = Box<dyn std::error::Error + Send + Sync>;

#[get("/motds")]
async fn motds(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let motds = web::block(move || {
        let mut conn = pool.get()?;
        find_all(&mut conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(motds))
}

fn find_all(conn: &mut PgConnection) -> Result<Vec<Motd>, DbError> {
    use crate::schema::motd::dsl::*;
    let items = motd.load::<Motd>(conn)?;
    Ok(items)
}
