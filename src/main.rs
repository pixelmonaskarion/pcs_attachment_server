use std::path::Path;

use rocket::{post, form::Form, fs::{TempFile, FileServer}, FromForm, routes};

#[derive(FromForm)]
struct Attachment<'a> {
    pub uuid: String,
    pub file: TempFile<'a>,
} 

#[post("/post-attachment",  data = "<attachment>")]
async fn post_attachment<'a>(mut attachment: Form<Attachment<'a>>) {
    let path_string = format!("\\attachments\\{}", attachment.uuid);
    let path = Path::new(path_string.as_str());
    attachment.file.copy_to(path).await.unwrap();
}

#[rocket::main]
async fn main() {
    rocket::build().mount(
        "/",
        FileServer::from("src"),
    ).mount("/", routes![post_attachment]).launch().await.unwrap();
}