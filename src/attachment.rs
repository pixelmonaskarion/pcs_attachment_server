use std::{path::Path, sync::{Arc, Mutex}};

use rocket::{FromForm, fs::TempFile, post, form::Form, http::Status, State};

use crate::Server;

#[derive(FromForm)]
pub struct Attachment<'a> {
    pub uuid: String,
    pub file: TempFile<'a>,
    pub extension: String,
    // pub token: String,
} 

#[post("/post-attachment",  data = "<attachment>")]
pub async fn post_attachment<'a>(mut attachment: Form<Attachment<'a>>, server_arc: &State<Arc<Mutex<Server>>>) -> Status {
    // if !server_arc.lock().unwrap().tokens.lock().unwrap().contains_key(&attachment.token) {
    //     return Status::Forbidden
    // }
    let path_string = format!("attachments\\{}.{}", attachment.uuid, attachment.extension);
    let path = Path::new(path_string.as_str());
    attachment.file.copy_to(path).await.unwrap();
    Status::Accepted
}