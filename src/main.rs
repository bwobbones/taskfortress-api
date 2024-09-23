use rocket::http::{Status, ContentType};
use rocket::data::{Data, ToByteUnit};
use rocket::{http::uri::Absolute, tokio::fs::File};

#[macro_use] 
extern crate rocket;

mod paste_id;

use paste_id::PasteId;

const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[get("/")]
fn json() -> (Status, (ContentType, &'static str)) {
    (Status::ImATeapot, (ContentType::JSON, "{ \"hi\": \"world\" }"))
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[post("/", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

// #[shuttle-runtime::main];
// async fn main() -> shuttle_runtime::ShuttleApp {
//     rocket::build().mount("/", routes![json, retrieve, upload])
// }

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![json, retrieve, upload]);

    Ok(rocket.into())
}