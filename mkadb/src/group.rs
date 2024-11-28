use std::{collections::HashMap, fs::File, io::Write, sync::RwLock};

use rocket::futures::io;

struct Group(Vec<String>);

pub struct Groups(RwLock<HashMap<String, Group>>);

trait IntoOk: Sized {
    fn into_ok<E>(self) -> Result<Self, E> {
        Ok(self)
    }
}
impl<T> IntoOk for T {}

impl Groups {
    fn group_list(&self) -> Vec<String> {
        self.0
            .read()
            .unwrap()
            .keys()
            .map(|it| it.to_string())
            .collect()
    }

    fn add_keywords_to_group(
        &mut self,
        name: &str,
        mut keywords: Vec<String>,
    ) -> Result<(), io::Error> {
        let mut write_guard = self
            .0
            .write()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to acquire write lock"))?;

        if !write_guard.contains_key(name) {
            write_guard
                .insert(name.to_string(), Group(keywords))
                .unwrap();
        } else {
            write_guard.get_mut(name).unwrap().0.append(&mut keywords);
        }

        let content = write_guard.get(name).unwrap().0.join("\n");

        drop(write_guard);

        let filename = format!("{}.group", name);
        let mut file = File::create(filename)?;

        file.write_all(content.as_bytes())?;

        Ok(())
    }

    fn get_keywords_from_group(&self, name: &str) -> Result<Vec<String>, io::Error> {
        self.0
            .read()
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "Failed to acquire read lock"))?
            .get(name)
            .map(|it| it.0.clone())
            .unwrap_or(Vec::new())
            .into_ok()
    }
}
