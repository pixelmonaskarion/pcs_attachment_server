use std::sync::{Mutex, Arc};

use rocket::{post, form::Form, FromForm, State};

use rand::distributions::Alphanumeric;
use rand::Rng;

use crate::Server;

#[derive(FromForm)]
pub struct IssueTokenRequest {
    pub number: String,
}

#[post("/issue-token", data="<request>")]
pub fn issue_token(request: Form<IssueTokenRequest>, server_arc: &State<Arc<Mutex<Server>>>) {
    let token: String = rand::thread_rng()
    .sample_iter(&Alphanumeric)
    .take(64)
    .map(char::from)
    .collect();
    
}