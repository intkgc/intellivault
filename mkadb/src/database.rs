use std::{
    collections::{HashMap, HashSet},
    fs::{self, File},
    io::{self, Read, Write},
    mem,
    sync::{Arc, Mutex, RwLock},
};

use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use tokio::runtime;

use crate::matcher::{Matcher, MatcherRules};

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct FileKeywords {
    pub name: String,
    pub keywords: Vec<String>,
}

pub(crate) struct KeywordStore {
    pub files: Arc<RwLock<HashMap<String, Arc<RwLock<Vec<FileID>>>>>>, // stores keywords by filename
    pub db: Arc<Mutex<Connection>>,
}

type FileID = i64;
impl KeywordStore {
    pub fn new() -> Self {
        let rt = runtime::Runtime::new().unwrap();
        let store = KeywordStore {
            files: Arc::default(),
            db: Arc::new(Mutex::new(Connection::open("mkadb.db").unwrap())),
        };

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
                        name TEXT NOT NULL UNIQUE,
                        keywords TEXT
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

            if path.is_file() && path.to_str().unwrap().ends_with(extension) {
                files_with_extension.push(path.to_str().unwrap().to_string());
            }
        }

        Ok(files_with_extension)
    }

    pub async fn load(&self) {
        Self::files_with_extension("./", ".keyword")
            .unwrap()
            .into_iter()
            .map(|it| it.strip_prefix("./").unwrap_or("").to_string())
            .filter(|it| !it.is_empty())
            .for_each(|name| {
                let keywords = self.files.clone();
                println!("{}", name);
                tokio::spawn(async move {
                    let mut file = File::open(name.as_str()).unwrap();
                    let mut buffer = Vec::new();

                    file.read_to_end(&mut buffer).unwrap();

                    if buffer.len() % 8 != 0 {
                        eprintln!("Error: corrupted file '{}'", name);
                        return;
                    }

                    let ptr = buffer.as_mut_ptr() as *mut FileID;
                    let len = buffer.len() / 8;
                    let capacity = buffer.capacity() / 8;
                    std::mem::forget(buffer);

                    let vec: Vec<FileID> = unsafe { Vec::from_raw_parts(ptr, len, capacity) };

                    keywords.write().unwrap().insert(
                        Self::remove_extension(name.as_str()),
                        Arc::new(RwLock::new(vec)),
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

    pub async fn link_keywords_to_file(&self, file: &FileKeywords) {
        self.add_file_if_not_exist(&file.name.as_str());
        let id = self.get_file_id(file.name.as_str()).unwrap();

        let keywords = self.get_file_keywords(id);
        let keywords: HashSet<&str> = keywords
            .iter()
            .chain(file.keywords.iter())
            .map(|it| it.as_str())
            .collect();
        let keywords = keywords.into_iter().collect::<Vec<&str>>().join(";");

        self.db
            .lock()
            .unwrap()
            .execute(
                "UPDATE files SET keywords = ?1 WHERE id = ?2",
                params![keywords, id],
            )
            .unwrap();

        file.keywords.iter().for_each(|it| {
            let mut map = self.files.write().unwrap();

            if !map.contains_key(it) {
                map.insert(it.to_string(), Arc::default());
            }

            let vec = map.get_mut(it).unwrap().clone();
            vec.write().unwrap().push(id);
            let vec = vec.clone();
            let name = it.to_string();

            tokio::spawn(async move {
                let mut vec = vec.write().unwrap();
                vec.sort();
                let mut file = File::create(format!("{}.keyword", name)).unwrap();
                let byte_slice: &[u8] = unsafe {
                    std::slice::from_raw_parts(
                        vec.as_ptr() as *const u8,
                        vec.len() * mem::size_of::<FileID>(),
                    )
                };

                file.write_all(byte_slice).unwrap();
            });
        });
    }

    pub fn get_file_keywords(&self, id: FileID) -> Vec<String> {
        let db = self.db.lock().unwrap();
        let string: String = db
            .query_row(
                "SELECT keywords FROM files WHERE id = ?1;",
                params![id],
                |row| row.get(0),
            )
            .unwrap_or(String::new());

        string
            .split(";")
            .map(|it| it.to_string())
            .filter(|it| !it.is_empty())
            .collect()
    }

    pub fn add_file_if_not_exist(&self, name: &str) {
        let db = self.db.lock().unwrap();
        db.execute(
            "INSERT INTO files (name) VALUES (?1) ON CONFLICT(name) DO NOTHING;",
            params![name],
        )
        .unwrap();
    }

    pub fn get_file_id(&self, name: &str) -> Result<FileID, rusqlite::Error> {
        let db = self.db.lock().unwrap();

        let id = db.query_row(
            "SELECT id FROM files WHERE name = ?1;",
            params![name],
            |row| row.get(0),
        )?;

        Ok(id)
    }

    pub fn get_files_by_ids(&self, ids: &Vec<FileID>) -> Vec<String> {
        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let query = format!(
            "SELECT name FROM files WHERE id IN ({})",
            placeholders.join(", ")
        )
        .to_string();

        let connection = &self.db.lock().unwrap();
        let mut stmt = connection.prepare(&query).unwrap();
        let file_iter = stmt
            .query_map(rusqlite::params_from_iter(ids.iter()), |row| {
                let name: String = row.get(0)?;
                Ok(name)
            })
            .unwrap();

        file_iter
            .filter(|it| it.is_ok())
            .map(|it| it.unwrap())
            .collect()
    }

    pub fn get_files_ids_by_keywords(
        &self,
        rules: MatcherRules,
        keywords: Vec<String>,
    ) -> Vec<FileID> {
        let locked_keywords = self.files.read().unwrap();
        let keywords: Vec<_> = keywords
            .iter()
            .map(|keyword| locked_keywords.get(keyword))
            .filter(|it| it.is_some())
            .map(|it| unsafe { it.unwrap_unchecked().clone() })
            .collect();
        
         let a = Matcher::new(rules, keywords).find_matches();
         println!("{:?}", a);
         a
    }

    pub fn remove_keywords(&self, id: FileID, keywords: &Vec<String>) {
        keywords.iter().for_each(|it| {
            let hash_map = self.files.read().unwrap();
            let keyword_vec = hash_map.get(it).unwrap();
            let position = keyword_vec.read().unwrap().binary_search(&id).unwrap();

            keyword_vec.write().unwrap().remove(position);
        });
    }

    pub fn remove_file(&self, name: &str) {
        let id = self.get_file_id(name).unwrap();
        let keywords = self.get_file_keywords(id);

        self.remove_keywords(id, &keywords);

        self.db
            .lock()
            .unwrap()
            .execute("DELETE FROM files WHERE id = ?", params![id])
            .unwrap();
    }
}
