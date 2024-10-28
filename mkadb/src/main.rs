mod database;
mod matcher;


#[macro_use]
extern crate rocket;

use database::{FileKeywords, KeywordStore};
use matcher::KeywordsMatcher;
use rocket::{form::FromFormField, serde::json::Json};
use rocket::State;



#[post("/add_keywords", format = "json", data = "<file_keywords>")]
async fn add_keywords_to_file(file_keywords: Json<FileKeywords>, store: &State<KeywordStore>) {
    store.link_keywords_to_file(&file_keywords.0).await
}


#[get("/get_keywords/<name>")]
fn get_keywords_by_file(name: String, store: &State<KeywordStore>) -> Json<Vec<String>> {
    Json::default().unwrap()
}

#[get("/get_files_by_keywords?<keywords..>")]
fn get_files_by_keywords(keywords: KeywordsMatcher, store: &State<KeywordStore>) -> Json<Vec<String>> {
    store.get_files_by_ids(&store.get_files_ids_by_keywords(keywords.rules, keywords.keywords)).into()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(KeywordStore::new())
        .mount("/", routes![add_keywords_to_file, get_keywords_by_file, get_files_by_keywords])
}