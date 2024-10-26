use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Write},
    mem,
    sync::{Arc, Mutex},
};

use async_std::task;
use memmap2::Mmap;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct FileKeywords {
    name: String,
    keywords: Vec<String>,
}

pub(crate) struct KeywordStore {
    files: Arc<Mutex<HashMap<String, Arc<Mutex<Vec<i64>>>>>>, // stores keywords by filename
    db: Mutex<Connection>,
}

impl KeywordStore {
    pub fn new() -> Self {
        task::block_on(async {
            let store = KeywordStore {
                files: Arc::default(),
                db: Mutex::new(Connection::open("mkadb.db").unwrap()),
            };

            store
                .db
                .lock()
                .unwrap()
                .execute(
                    "CREATE TABLE IF NOT EXISTS files (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL UNIQUE
                    )",
                    [],
                )
                .unwrap();

            store
        })
    }

    fn files_with_extension(dir: &str, extension: &str) -> io::Result<Vec<String>> {
        let mut files_with_extension = Vec::new();

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == extension {
                        if let Some(file_name) = path.to_str() {
                            files_with_extension.push(file_name.to_string());
                        }
                    }
                }
            }
        }

        Ok(files_with_extension)
    }

    async fn load(&self) {
        Self::files_with_extension("./", ".keyboard")
            .unwrap()
            .into_iter()
            .for_each(|name| {
                let keywords = self.files.clone();
                task::spawn(async move {
                    let file = File::open(name.as_str()).unwrap();
                    let mmap = unsafe { Mmap::map(&file).unwrap() };
                    if mmap.len() % mem::size_of::<i64>() != 0 {
                        panic!("")
                    }

                    let len = mmap.len() / mem::size_of::<i64>();

                    let vec: Vec<i64> = unsafe {
                        std::slice::from_raw_parts(mmap.as_ptr() as *const i64, len).to_vec()
                    };

                    keywords
                        .lock()
                        .unwrap()
                        .insert(name, Arc::new(Mutex::new(vec)))
                        .unwrap()
                });
            });
    }

    fn link_keywords(&self, file: &FileKeywords) {
        task::block_on(async {
            let id = self.get_file_id(file.name.as_str());
            file.keywords.iter().for_each(|it| {
                let mut map = self.files.lock().unwrap();
                let vec = map.get_mut(it).unwrap().clone();
                vec.lock().unwrap().push(id);
                let vec = vec.clone();
                let name = it.to_string();

                task::spawn(async move {
                    let mut vec = vec.lock().unwrap();
                    vec.sort();
                    let mut file = File::create(format!("{}.keyword", name)).unwrap();
                    let byte_slice: &[u8] = unsafe {
                        std::slice::from_raw_parts(
                            vec.as_ptr() as *const u8,
                            vec.len() * mem::size_of::<i64>(),
                        )
                    };

                    file.write_all(byte_slice).unwrap();
                });
            });
            self.store();
        });
    }

    fn store(&self) {}

    fn get_file_id(&self, name: &str) -> i64 {
        self.db
            .lock()
            .unwrap()
            .execute(
                "
        WITH new_file AS (
            INSERT INTO files (filename) VALUES (?1)
            ON CONFLICT(filename) DO NOTHING
            RETURNING id
        )
        SELECT id FROM new_file
        UNION ALL
        SELECT id FROM files WHERE filename = 'example.txt'
        LIMIT 1;",
                params![name],
            )
            .unwrap() as i64
    }

    fn remove_file(&self, name: &str) {
        self.db
            .lock()
            .unwrap()
            .execute("DELETE FROM files WHERE name = ?", params![name])
            .unwrap();
    }
}
