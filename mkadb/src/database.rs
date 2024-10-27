use std::{
    collections::HashMap,
    fs::{self, File},
    io::{self, Read, Write},
    mem,
    sync::{Arc, Mutex},
};

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tokio::runtime;

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
        let rt = runtime::Runtime::new().unwrap();
        let store = KeywordStore {
            files: Arc::default(),
            db: Mutex::new(Connection::open("mkadb.db").unwrap()),
        };

        //let store = Arc::new(store);
        rt.block_on(async {
            store.load().await;
        });

        store
            .db
            .lock()
            .unwrap()
            .execute(
                "CREATE TABLE IF NOT EXISTS files (
                        id INTEGER PRIMARY KEY AUTOINCREMENT,
                        name TEXT NOT NULL UNIQUE
                );
                CREATE INDEX IF NOT EXISTS idx_name ON files(name);
                ",
                [],
            )
            .unwrap();

        store
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
                tokio::spawn(async move {
                    let mut file = File::open(name.as_str()).unwrap();
                    let mut buffer = Vec::new();

                    file.read_to_end(&mut buffer).unwrap();

                    if buffer.len() % 8 != 0 {
                        eprintln!("Error: corrupted file '{}'", name);
                        return;
                    }

                    let ptr = buffer.as_mut_ptr() as *mut i64;
                    let len = buffer.len() / 8;
                    let capacity = buffer.capacity() / 8;
                    std::mem::forget(buffer);

                    let vec: Vec<i64> = unsafe { Vec::from_raw_parts(ptr, len, capacity) };

                    keywords.lock().unwrap().insert(
                        Self::remove_extension(name.as_str()),
                        Arc::new(Mutex::new(vec)),
                    );
                });
            });
    }

    fn remove_extension(file_name: &str) -> String {
        match file_name.rfind('.') {
            Some(pos) => file_name[..pos].to_string(),
            None => file_name.to_string(),
        }
    }
    pub async fn link_keywords(&self, file: &FileKeywords) {
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

            tokio::spawn(async move {
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
    }

    fn store(&self) {}

    pub fn get_file_id(&self, name: &str) -> i64 {
        let db = self.db.lock().unwrap();

        // First, try inserting the file name if it doesn't already exist
        db.execute(
            "INSERT INTO files (name) VALUES (?1) ON CONFLICT(name) DO NOTHING;",
            params![name],
        )
        .unwrap();

        // Then, retrieve the id of the file, whether it was newly inserted or already existed
        db.query_row(
            "SELECT id FROM files WHERE name = ?1;",
            params![name],
            |row| row.get(0),
        )
        .unwrap()
    }
    
    fn get_files_by_ids(&self, ids: &[i32]) -> Vec<String> {
        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let query = format!("SELECT name FROM files WHERE id IN ({})", placeholders.join(", ")).to_string();
    
 
        let connection = &self.db.lock().unwrap();
        let mut stmt = connection.prepare(&query).unwrap();
        let file_iter = stmt.query_map(rusqlite::params_from_iter(ids.iter()), |row| {
            let name: String = row.get(1)?;
            Ok(name)
        }).unwrap();
    
 
        file_iter.filter(|it| it.is_ok()).map(|it| it.unwrap()).collect()
    }

    pub fn remove_file(&self, name: &str) {
        self.db
            .lock()
            .unwrap()
            .execute("DELETE FROM files WHERE name = ?", params![name])
            .unwrap();
    }
}
