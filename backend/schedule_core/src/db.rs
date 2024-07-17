// we create tables:
// - Main task table indexed with ID(usize).
//   currently every select performs a full table search.
// - metadata tree
//
// we generate id with db::generate_id

use std::path::PathBuf;

use sled::Tree;

use crate::filter::Filter;
use crate::task::Task;
use crate::user::User;
use crate::error::{Error, Result};

#[derive(Clone)]
pub struct Db {
    db: sled::Db,
    tasks: Tree, // stores tasks
}

impl Db {
    fn userpath(username: &str)->PathBuf{
        let mut buf = PathBuf::from("db/");
        buf.push(hex::encode(username));
        buf
    }
    pub fn open(username: &str)->Result<Self>{
        let path = Self::userpath(username);
        if !path.as_path().exists(){return Err(Error::UserNotExist);}
        let db = sled::open(path)?;
        let tasks = db.open_tree("tasks")?;
        let db = Self{db, tasks};
        let _user = db.user()?;
        Ok(db)
    }
    pub fn register(user: &User)->Result<Self>{
        let path = Self::userpath(&user.name);
        if path.as_path().exists(){return Err(Error::UserExists);}
        let db = sled::open(path)?;
        let tasks = db.open_tree("tasks")?;
        let db = Self{db, tasks};
        db.set_user(user)?;
        Ok(db)
    }
    pub fn user(&self)->Result<User>{
        match self.read("meta")?{
            None => Err(Error::BrokenDB),
            Some(u) => Ok(u)
        }
    }
    // used in register
    pub fn set_user(&self, user: &User)->Result<Option<User>>{
        let raw = self.db.insert("meta", bincode::serialize(user)?)?;
        if let Some(raw) = raw {Ok(Some(bincode::deserialize(&raw)?))}
        else {Ok(None)}
    }

    pub fn generate_id(&self)->Result<u64> {
        Ok(self.db.generate_id()?)
    }
    pub fn get_task(&self, id: u64) -> Result<Task> {
        match self.tasks.get(id.to_be_bytes())? {
            None => Err(Error::TaskNotFound(id)),
            Some(t) => Ok(bincode::deserialize(&t)?)
        }
    }
    pub fn contains_task(&self, id: u64) -> Result<bool> {
        Ok(self.tasks.contains_key(id.to_be_bytes())?)
    }
    pub fn insert_task(&self, task: &Task)->Result<Option<Task>> {
        if let Some(raw) = self.tasks.insert(
            task.id.to_be_bytes(), 
            bincode::serialize(task)?,
        )? {Ok(Some(bincode::deserialize(&raw)?))}
        else {Ok(None)}
    }
    pub fn iter_tasks(&self)->impl Iterator<Item = Result<Task>>{
        self.tasks.iter().map(|raw|{
            let raw=raw?;
            Ok(bincode::deserialize(&raw.1)?)
        })
    }
    pub fn select_tasks(&self, filter: Filter)->impl Iterator<Item = Result<Task>>{
        self.iter_tasks().filter(move |t|{
            if let Ok(task)=t {filter.matches(task)}
            else {true} // errors should be catched
        })
    }
    pub fn remove_task(&self, id: u64)->Result<Task> {
        match self.tasks.remove(id.to_be_bytes())?{
            None=>Err(Error::TaskNotFound(id)),
            Some(task)=>Ok(bincode::deserialize(&task)?),
        }
    }

    fn read<T: serde::de::DeserializeOwned>(&self, k: &str)->Result<Option<T>>{
        Ok(match self.db.get(k)? {
            None=>None,
            Some(v)=>Some(bincode::deserialize(&v)?)
        })
    }
}

#[cfg(test)]
mod test {
    use std::path::Path;
    use std::fs::{remove_dir_all, create_dir};
    use super::*;

    // cannot run in multiple thread.
    fn _test_db()->std::result::Result<(), Box<dyn std::error::Error>>{
        let path = Path::new("db/");
        remove_dir_all(path)?;
        create_dir(path)?;
        let db=Db::register(&User::new("wsm", "114514"))?;
        drop(db);
        let db=Db::open("wsm")?;
        let user = db.user()?;
        db.generate_id()?;
        assert!(user.verify_password("114514"));
        Ok(())
    }

}