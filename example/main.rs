#[macro_use]
extern crate diesel;
use rocket::http::Status;
use rocket::response::status;
use rocket::{
    catch, catchers, delete, get, post, put, routes,
    serde::json::{serde_json::json, Json, Value},
};
use rocket_sync_db_pools::database;

mod basic_auth;
mod models;
mod repositories;
mod schema;

use basic_auth::BasicAuthStruct;
use models::{NewProduct, Product};
use repositories::ProductRepository;

#[database("sqlite_path")]
struct DbConn(diesel::SqliteConnection);

#[get("/")]
async fn get_products(conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(|con| {
        ProductRepository::find_all(con)
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[get("/<id>")]
async fn view_product(id: i32, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(move |con| {
        ProductRepository::find(con, id)
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[post("/", format = "json", data = "<new_product>")]
async fn create_product(
    _auth: BasicAuthStruct,
    conn: DbConn,
    new_product: Json<NewProduct>,
) -> Result<Value, status::Custom<Value>> {
    conn.run({
        |con| {
            ProductRepository::create(con, new_product.into_inner())
                .map(|product| json!(product))
                .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
        }
    })
    .await
}

#[put("/<id>", format = "json", data = "<product>")]
async fn put_product(
    id: i32,
    _auth: BasicAuthStruct,
    conn: DbConn,
    product: Json<Product>,
) -> Result<Value, status::Custom<Value>> {
    conn.run(move |con| {
        ProductRepository::save(con, product.into_inner())
            .map(|product| json!(product))
            .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[delete("/<id>")]
async fn delete_product(id: i32, _auth: BasicAuthStruct, conn: DbConn) -> Result<Value, status::Custom<Value>> {
    conn.run(move |con| {
        ProductRepository::delete(con, id)
        .map(|product| json!(product))
        .map_err(|e| status::Custom(Status::InternalServerError, json!(e.to_string())))
    })
    .await
}

#[catch(404)]
async fn not_found_url() -> Value {
    json!("Not found!")
}

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    rocket::build()
        .mount(
            "/product",
            routes![
                get_products,
                view_product,
                create_product,
                put_product,
                delete_product
            ],
        )
        .register("/", catchers!(not_found_url))
        .attach(DbConn::fairing())
        .launch()
        .await?;
    Ok(())
}
