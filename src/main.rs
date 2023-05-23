mod attachment;
mod login;
use std::{
    collections::HashMap,
    fs::{self, create_dir, OpenOptions},
    path::Path,
    sync::{Arc, Mutex}, io::Write,
};

use attachment::post_attachment;
use login::issue_token;
use rocket::{fs::FileServer, routes};

fn create_or_open_file(path: &str) -> Result<std::fs::File, std::io::Error> {
    return OpenOptions::new()
        .write(true)
        .create(!Path::new(path).exists())
        .truncate(true)
        .open(path);
}

pub struct Server {
    pub tokens: Mutex<HashMap<String, String>>,
    pub pending_tokens: Mutex<HashMap<String, String>>,
}

impl Server {
    pub fn new() -> Self {
        if Path::new("save/token.json").exists() {
            let tokens = Mutex::new(
                serde_json::from_str(
                    fs::read_to_string("save/tokens.json")
                        .expect("Should have been able to read the file")
                        .as_str(),
                )
                .expect("couldn't parse tokens"),
            );
            Self { 
                tokens,
                pending_tokens: Mutex::new(HashMap::new()),
            }
        } else {
            println!("SAVE NOT FOUND! NEW DATABASE STARTED!");
            Self {
                tokens: Mutex::new(HashMap::new()),
                pending_tokens: Mutex::new(HashMap::new()),
            }
        }
    }

    pub fn write_data(&self) -> std::io::Result<()> {
        println!("saving data...");
        if !Path::new("save").exists() {
            create_dir("save")?;
        }
        let mut users_file = create_or_open_file("save/tokens.json")?;
        users_file.write_all(
            serde_json::to_string(&self.tokens.lock().unwrap().clone())
                .expect("could not write to users file")
                .as_bytes(),
        )?;
        println!("saved data!");
        Ok(())
    }
}

#[rocket::main]
async fn main() {
    let server = Arc::new(Mutex::new(Server::new()));
    rocket::build()
        .mount("/", FileServer::from("src"))
        .manage(server.clone())
        .mount("/", routes![post_attachment, issue_token])
        .launch()
        .await
        .unwrap();
    server.lock().unwrap().write_data().expect("saving data failed!");
}
