mod database;
mod matcher;
mod summary;

#[macro_use]
extern crate rocket;

use std::path::Path;

use database::{FileKeywords, KeywordStore};
use matcher::KeywordsMatcher;
use rocket::serde::json::Json;
use rocket::State;

#[post("/add_keywords", format = "json", data = "<file_keywords>")]
async fn add_keywords_to_file(file_keywords: Json<FileKeywords>, store: &State<KeywordStore>) {
    store.link_keywords_to_file(&file_keywords.0).await
}

#[get("/get_keywords/<name>")]
fn get_keywords_by_file(name: String, store: &State<KeywordStore>) -> Json<Vec<String>> {
    store
        .get_file_keywords(store.get_file_id(name.as_str()).unwrap())
        .into()
}

#[post("/remove_file/<name>")]
fn remove_file(name: String, store: &State<KeywordStore>) {
    store.remove_file(name.as_str());
}

#[get("/get_files_by_keywords?<keywords..>")]
fn get_files_by_keywords(
    keywords: KeywordsMatcher,
    store: &State<KeywordStore>,
) -> Json<Vec<String>> {
    store
        .get_files_by_ids(&store.get_files_ids_by_keywords(keywords.rules, keywords.keywords))
        .into()
}

#[post("/save_summary?<name>&<text>")]
fn save_summary(name: String, text: String) {
    summary::create_file_and_save_text(name, &text).unwrap()
}

#[post("/get_summary?<name>")]
fn get_summary(name: String) -> String {
    summary::get_text_from_file(name).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build().manage(KeywordStore::new()).mount(
        "/",
        routes![
            add_keywords_to_file,
            get_keywords_by_file,
            get_files_by_keywords,
            remove_file,
            save_summary,
            get_summary
        ],
    )
}
