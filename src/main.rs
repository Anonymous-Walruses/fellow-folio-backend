#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;

use reqwest;
use rocket_contrib::json::{Json, JsonValue};
use std;

#[get("/")]
fn index() -> &'static str {
    "Hello world!"
}

#[get("/bye")]
fn bye() -> &'static str {
    "Goodbye world"
}

#[derive(Deserialize)]
struct CodeTokenRequest {
    code: String,
    username: String,
}

#[derive(Deserialize)]
struct GitHubCodeTokenResponse {
    access_token: String,
    scope: String,
    token_type: String,
}

#[post("/api/v001/oauth/code4token", format = "json", data = "<req>")]
fn handle_oauth_exchange(req: Json<CodeTokenRequest>) -> JsonValue {
    // Grab the client ID and secret from the environment.
    let client_id = std::env::var("GH_CLIENT_ID").unwrap();
    let client_secret = std::env::var("GH_CLIENT_SECRET").unwrap();

    // Build the request.
    let uri = "https://github.com/login/oauth/access_token";
    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("code", req.code.clone()),
    ];

    // Then send it off.
    let client = reqwest::blocking::Client::new();
    let resp = client
        .post(uri)
        .header(reqwest::header::ACCEPT, "application/json")
        .query(&params)
        .send()
        .unwrap();

    let json_result: GitHubCodeTokenResponse = resp.json().unwrap();

    println!(
        "stuff related to: {username}:\n\ttoken: {token}\n\tscope: {scope}\n\ttype: {type}",
        username = req.username,
        token = json_result.access_token,
        scope = json_result.scope,
        type = json_result.token_type
    );

    // TODO: Figure out how to handle errors here.
    return json!({});
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, bye, handle_oauth_exchange])
        .launch();
}
