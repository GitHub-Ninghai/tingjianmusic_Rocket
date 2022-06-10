mod db;
mod model;

#[macro_use]
extern crate diesel;
use rocket::{catch, catchers, delete, get, put, post, routes, serde::json::{Value, serde_json::json},Request};
use rocket::http::Status;

use rocket::request::{FromRequest,Outcome};

#[database("sqlite_path")]

struct DbConn(diesel::MysqlConnection);


//FromRequest
pub struct BasicAuthStruct{
    pub username:String,
    pub password:String
}

//Basic username:password
impl BasicAuthStruct {
    fn from_header(header: &str) -> Option<BasicAuthStruct> {
        //通过空格分割
        let split_vec = header.split_whitespace().collect::<Vec<_>>();
        if split_vec.len() != 2{
            return None
        }
        if split_vec[0] != "Basic" {
            return None;
        }
        // 将加密的bs64转换成struct
        Self::from_base64(split_vec[1])
    }
    //解密
    fn from_base64(base64_string:&str) -> Option<BasicAuthStruct> {
        let decoded = base64::decode(base64_string).ok()?;
        //将u8的Vec转换成string
        let decode_str = String::from_utf8(decoded).ok()?;
        let split_vec = decode_str.split(":").collect::<Vec<_>>();
        if split_vec.len() != 2 {
            return None;
        }
        let (username,password) = (split_vec[0].to_string(),split_vec[1].to_string());
        Some(BasicAuthStruct{
            username,
            password
        })

    }

}
//登录验证
#[rocket::async_trait]
impl <'r>FromRequest<'r> for BasicAuthStruct{
    type Error = ();
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header_auth = request.headers().get_one("Authorization");
            if let Some(header_auth) = header_auth {
                if let Some(auth) = Self::from_header(header_auth) {
                    return Outcome::Success(auth)
                }
        }
        Outcome::Failure((Status::Unauthorized,()))
    }
}

//主页路由、获取url、导航名
#[get("/")]
fn index() -> &'static str {
    "主界面"
}
#[get("/")]
async fn get_music_url() -> Value {
    json!("url")
}

#[get("/music")]
async fn get_music_nav() -> Value {
    json!("list")
}


//下面是待用功能
#[get("/<id>")]
async fn view_music(id:i32) -> Value{
    json!("get")
}

#[post("/")]
async fn create_music(_auth:BasicAuthStruct) ->Value{json!("post")}

#[put("/<id>")]
async fn put_music(id:i32,_auth:BasicAuthStruct) -> Value { json!("put") }

#[delete("/<id>")]
async fn delete_music(id:i32,_auth:BasicAuthStruct) -> Value{json!("delete")}

#[catch(404)]
async fn not_found_url() -> Value {
    json!("Not found!")
}







#[rocket::main]
async fn main() -> Result<(),Box<dyn std::error::Error>> {
    rocket::build()
    .mount("/",routes![index])
    .mount("/music", routes![get_music_url,get_music_nav,view_music,create_music,put_music,delete_music])
        .register("/",catchers!(not_found_url))
        .attach(DbConn::fairing())
        .launch()
        .await?;
    Ok(())
}