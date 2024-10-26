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
    pub name: String,
    pub keywords: Vec<String>,
}

pub(crate) struct KeywordStore {
    pub files: Arc<Mutex<HashMap<String, Arc<Mutex<Vec<i64>>>>>>, // stores keywords by filename
    pub db: Mutex<Connection>,
}

impl KeywordStore {
    pub fn new() -> Self {
        task::block_on(async {
            let store = KeywordStore {
                files: Arc::default(),
                db: Mutex::new(Connection::open("mkadb.db").unwrap()),
            };
            
            store.load().await;
            
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

    pub fn files_with_extension(dir: &str, extension: &str) -> io::Result<Vec<String>> {
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

    pub async fn load(&self) {
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

    pub fn link_keywords(&self, file: &FileKeywords) {
        task::block_on(async {
            let id = self.get_file_id(file.name.as_str());
            file.keywords.iter().for_each(|it| {
                let mut map = self.files.lock().unwrap();
                
                if !map.contains_key(it) {
                    map.insert(it.to_string(), Arc::default());
                }
                
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


    
    pub fn get_file_id(&self, name: &str) -> i64 {
        let db = self.db.lock().unwrap();
        
        // First, try inserting the file name if it doesn't already exist
        db.execute(
            "INSERT INTO files (name) VALUES (?1) ON CONFLICT(name) DO NOTHING;",
            params![name],
        ).unwrap();
    
        // Then, retrieve the id of the file, whether it was newly inserted or already existed
        db.query_row(
            "SELECT id FROM files WHERE name = ?1;",
            params![name],
            |row| row.get(0),
        ).unwrap()
    }


    pub fn remove_file(&self, name: &str) {
        self.db
            .lock()
            .unwrap()
            .execute("DELETE FROM files WHERE name = ?", params![name])
            .unwrap();
    }
}
