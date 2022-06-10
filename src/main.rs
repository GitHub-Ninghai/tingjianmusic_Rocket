use rocket::{catch,catchers,delete,get,put,post,routes, serde::json::{Value,serde_json::json}};

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/")]
async fn get_music_nav() -> Value {
    json!("list")
}

#[get("/<id>")]
async fn view_music(id:i32) -> Value{
    json!("get")
}

#[post("/")]
async fn create_music() ->Value{json!("post")}

#[put("/<id>")]
async fn put_music(id:i32) -> Value { json!("put") }

#[delete("/<id>")]
async fn delete_music(id:i32) -> Value{json!("delete")}

#[catch(404)]
async fn not_found_url() -> Value {
    json!("Not found!")
}







#[rocket::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    rocket::build()
    .mount("/",routes![index])
    .mount("/music", routes![get_music_nav,view_music,create_music,put_music,delete_music])
        .register("/",catchers!(not_found_url))
        .launch()
        .await?;
    Ok(())
}