use chrono::NaiveDate;
use crate::task::*;

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct Filter {
    pub date: Option<NaiveDate>,
    // pub from: Option<NaiveDate>,
    // pub to: Option<NaiveDate>,
    pub category: Option<String>,
    pub priorities: Option<Vec<Priority>>,
}

impl Filter {
    pub fn matches(&self, task: &Task)->bool {
        if self.date.is_some() {
            if self.date!=task.date {return false;}
        }
        // check date range

        // todo: bug: fix from,to 
        /*
        let from=match self.from {
            None => Some(task.lop.next()),
            Some(d) => task.lop.next_since(d),
        };
        match from {
            None => {return false;},
            Some(d_from) => match self.to {
                None => (),
                Some(d_to) => if d_from>d_to {return false;}
            },
        };
        */
        if self.category.is_some() {
            if self.category!=task.category {return false;}
        }
            
        if let Some(ps) = &self.priorities {
            if !ps.is_empty() && !ps.contains(&task.priority) {return false;}
        }
        true
    }
}