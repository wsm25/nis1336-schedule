use std::{collections::BTreeMap, sync::{Arc, RwLock}};

// export
pub use error::{Error, Result};
pub use task::Task;
pub use filter::Filter;

#[derive(Clone)]
pub struct Schedule{
    db: db::Db,
    cats: Arc<RwLock<BTreeMap<String, usize>>>, // dynamicly maintained category set
}

impl Schedule{
    // user related
    pub fn login(username: &str, password: &str)->Result<Self> {
        let db = db::Db::open(username)?;
        let user = db.user()?;
        if !user.verify_password(password) {
            return Err(Error::IncorrectPassword);
        }
        let mut cats=BTreeMap::new();
        for task in db.iter_tasks(){
            if let Some(cat) = task?.category{
                cats.entry(cat)
                    .and_modify(|curr| *curr += 1)
                    .or_insert(1);
            }
        }
        Ok(Self{db, cats: Arc::new(RwLock::new(cats))})
    }
    pub fn register(username: &str, password: &str)->Result<Self> {
        Ok(Self{
            db: db::Db::register(
                &user::User::new(username, password)
            )?,
            cats: Arc::new(RwLock::new(BTreeMap::new())),
        })
    }

    pub fn password_set(&self, password: &str)->Result<()> {
        let mut user=self.db.user()?;
        user.set_password(password);
        self.db.set_user(&user)?;
        Ok(())
    }

    pub fn password_verify(&self, password: &str)->Result<bool> {
        let user=self.db.user()?;
        Ok(user.verify_password(password))
    }

    // category
    pub fn categories(&self)->Vec<String>
        {self.cats.read().unwrap().keys().cloned().collect()}
    
    fn category_inc(&self, cat: &Option<String>) {
        let Some(cat) = cat else {return;};
        let mut cats = self.cats.write().unwrap();
        cats.entry(cat.to_string())
            .and_modify(|curr| *curr += 1)
            .or_insert(1);
    }

    fn category_dec(&self, cat: &Option<String>) {
        let Some(cat) = cat else {return;};
        let mut cats = self.cats.write().unwrap();
        if let Some(count) = cats.get_mut(cat){
            if *count==1 {cats.remove(cat);}
            else {*count-=1;}
        }
    }

    // tasks

    pub fn generate_id(&self)->Result<u64> 
        {self.db.generate_id()}
    
    /// should call Schedule::generate_id to generate new task id;
    pub fn task_insert(&self, task: &Task)->Result<()> {
        if self.db.contains_task(task.id)? {
            return Err(Error::TaskExists(task.id));
        }
        self.category_inc(&task.category);
        self.db.insert_task(task)?;
        Ok(())
    }

    pub fn task_modify(&self, task: &Task)->Result<()> {
        if !self.db.contains_task(task.id)? {
            return Err(Error::TaskNotFound(task.id));
        }
        let oldtask = self.db.insert_task(task)?.unwrap();
        if oldtask.category != task.category {
            self.category_dec(&oldtask.category);
            self.category_inc(&task.category);
        }
        Ok(())
    }
    
    pub fn task_remove(&self, id: u64)->Result<Task> {
        let task = self.db.remove_task(id)?;
        self.category_dec(&task.category);
        Ok(task)
    }

    pub fn task(&self, id: u64) -> Result<Task> {
        Ok(self.db.get_task(id)?)
    }

    pub fn tasks(&self) ->impl Iterator<Item = Result<Task>>
        {self.db.iter_tasks()}
    
    pub fn tasks_filtered(&self, filter: Filter) 
        ->impl Iterator<Item = Result<Task>>
        {self.db.select_tasks(filter)}
}

mod db;
mod user;
pub mod task;
pub mod filter;
pub mod error;

#[cfg(test)]
mod tests{
    type R<T> = std::result::Result<T, Box<dyn std::error::Error>>;
    use chrono::{NaiveDate, NaiveTime};

    use super::*;

    fn bootstrap() -> R<()>{
        use std::path::Path;
        use std::fs::{remove_dir_all, create_dir};
        let path = Path::new("db/");
        remove_dir_all(path).unwrap();
        create_dir(path).unwrap();
        Ok(())
    }
    
    #[test]
    fn test_lib() -> R<()>{
        bootstrap()?;
        let schedule = Schedule::register("wsm", "114514")?;
        let date =  NaiveDate::from_ymd_opt(1919, 8, 10);
        let date2 =  NaiveDate::from_ymd_opt(1919, 8, 11);
        let task = Task{
            id:schedule.generate_id()?,
            title: "结城友奈".into(),
            content: "是勇者".into(),
            time: NaiveTime::from_hms_opt(11,45,14),
            date,
            category: Some("10".into()),
            priority: task::Priority::Default,
        };
        let task2 = Task{
            id: schedule.generate_id()?,
            title: "結城友奈".into(),
            content: "は勇者である".into(),
            time: NaiveTime::from_hms_opt(11,45,14),
            date: date2,
            category: Some("10".into()),
            priority: task::Priority::Default,
        };
        schedule.task_insert(&task)?;
        drop(schedule);
        let schedule = Schedule::login("wsm", "114514")?;
        schedule.task_insert(&task2)?;
        assert_eq!(schedule.tasks().count(), 2);
        let t = schedule.tasks().next().unwrap()?;
        assert_eq!(&t.title, &task.title);
        let mut it = schedule.tasks_filtered(Filter{
            date: None,
            from: None,
            to: date,
            category: None,
            priorities: Some(vec![task::Priority::Default]),
        });
        assert_eq!(&it.next().unwrap()?.content, &task.content);
        assert!(&it.next().is_none());
        Ok(())
    }
}