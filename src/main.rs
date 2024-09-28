use rocket::http::{Status};
use rocket::data::{Data, ToByteUnit};
use rocket::{http::uri::Absolute, tokio::fs::File};

use rocket_dyn_templates::{context, Template};

#[macro_use] 
extern crate rocket;

mod paste_id;

use paste_id::PasteId;

const ID_LENGTH: usize = 3;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[get("/")]
async fn index() -> Result<Template, Status>  {
    // Using the `context! { }` macro.
    Ok(Template::render(
        "index",
        context! {
        },
    ))
}

#[get("/api/v1/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    File::open(id.file_path()).await.ok()
}

#[post("/api/v1", data = "<paste>")]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    paste.open(128.kibibytes()).into_file(id.file_path()).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().attach(Template::fairing()).mount("/", routes![index, retrieve, upload]);
    
    Ok(rocket.into())
}