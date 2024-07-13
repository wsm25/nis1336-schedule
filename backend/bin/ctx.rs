use std::sync::Arc;
use crossbeam_skiplist::SkipMap;
use crate::{
    error::{Result, Error::*},
    session::SessionID,
};
use sccore::Schedule;

#[derive(Clone)]
pub struct Ctx {
    // todo: expire and login counter
    /// id to login entry
    sessions: Arc<SkipMap<SessionID, Option<Schedule>>>,
    /// schedule must be unique and const
    schedules: Arc<SkipMap<String, Schedule>>, // username is logined
}

impl Ctx {
    pub fn new()->Self {
        Self {
            sessions: Arc::new(SkipMap::new()),
            schedules: Arc::new(SkipMap::new()),
        }
    }

    // not exist / not login / login
    pub fn schedule(&self, id: &SessionID)->Option<Option<Schedule>> {
        self.sessions.get(id)
            .map(|entry|entry.value().as_ref()
                .map(|e|e.clone()))
    }

    pub fn login (
        &self, 
        id: &SessionID, 
        username: &str, 
        password: &str) 
    -> Result<()>
    {
        if let Some(e) = self.sessions.get(&id) {
            if e.value().is_some() {return Err(AlreadyLogin);}
        }
        let schedule = match self.schedules.get(username) {
            None => {
                let schedule = sccore::Schedule::login(username, password)?;
                self.schedules.insert(username.to_string(), schedule.clone());
                schedule
            },
            Some(s) =>  {
                if !s.value().password_verify(password)? {
                    return Err(InvalidPwd);
                }
                s.value().clone()
            }
        };
        
        self.sessions.insert(id.clone(), Some(schedule));
        Err(OK)
    }

    /// register and login
    pub fn register (
        &self, 
        id: &SessionID, 
        username: &str, 
        password: &str) 
    -> Result<()>
    {
        if let Some(e) = self.sessions.get(&id) {
            if e.value().is_some() {return Err(AlreadyLogin);}
        }
        let schedule = sccore::Schedule::register(username, password)?;
        self.schedules.insert(username.to_string(), schedule.clone());
        self.sessions.insert(id.clone(), Some(schedule));
        Err(OK)
    }
}
