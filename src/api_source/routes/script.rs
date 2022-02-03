use crate::db;
use crate::models::script::{NewScript, Script, UpdateScript};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use sbox::tasks::Sender;

#[openapi(tag = "Script")]
#[get("/script/<id>")]
pub async fn read_script(conn: db::Conn, id: i32, sender: Sender) -> Result<Json<Script>, Status> {
    sender.send(|| sbox::tasks::add::new(1, 2));

    let res = conn.run(move |c| db::script::read(c, &id)).await;
    match res {
        Ok(script) => Ok(Json(script)),
        Err(_err) => Err(Status::NotFound),
    }
}

#[openapi(tag = "Script")]
#[post("/script", data = "<new_script>")]
pub async fn create_script(
    conn: db::Conn,
    new_script: Json<NewScript>,
) -> Result<Json<Script>, Status> {
    let res = conn.run(move |c| db::script::create(c, &new_script)).await;
    match res {
        Ok(script) => Ok(Json(script)),
        Err(_err) => Err(Status::InternalServerError),
    }
}

#[openapi(tag = "Script")]
#[put("/script/<id>", data = "<update_script>")]
pub async fn update_script(
    conn: db::Conn,
    id: i32,
    update_script: Json<UpdateScript>,
) -> Result<Json<Script>, Status> {
    let res = conn
        .run(move |c| db::script::update(c, &id, &update_script))
        .await;
    match res {
        Ok(script) => Ok(Json(script)),
        Err(_err) => Err(Status::InternalServerError),
    }
}

#[openapi(tag = "Script")]
#[delete("/script/<id>")]
pub async fn delete_script(conn: db::Conn, id: i32) -> Result<Status, Status> {
    let res = conn.run(move |c| db::script::delete(c, &id)).await;
    match res {
        Ok(_num) => Ok(Status::Ok),
        Err(_err) => Err(Status::InternalServerError),
    }
}

#[openapi(tag = "test")]
#[get("/test")]
pub async fn test() {
    println!("TEST")
}
