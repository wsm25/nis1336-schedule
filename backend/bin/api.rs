use axum::{
    extract::{Extension, Path, Request, State}, 
    routing::{get, post, put, delete, patch}, 
    Json,
    response::Response,
    middleware::{Next, from_fn_with_state},
};
use sccore::{Schedule, task::*};
use crate::{ctx::Ctx, error::{Result, Error::*}, session::SessionID};
use chrono::{NaiveDate, NaiveTime};

pub fn router(ctx: Ctx)->axum::Router {
    axum::Router::new()
    .route("/user", get(user))
    .route("/tasks", post(get_tasks))
    .route("/task", put(insert_task))
    .route("/task/:id", get(get_task))
    .route("/task/:id", patch(patch_task))
    .route("/task/:id", delete(delete_task))
    .route_layer(from_fn_with_state(ctx.clone(), auth))
    .with_state(ctx)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct User {
    categories: Vec<String>
}

// user info
async fn user (
    Extension(schedule): Extension<Schedule>,
) -> Result<Json<User>> 
{
    Ok(Json(User{
        categories: schedule.categories()
    }))
}

// tasks

#[derive(Deserialize)]
struct NewTaskReq {
    pub title: String, // required
    pub content: Option<String>,
    pub time: Option<NaiveTime>,
    pub date: Option<NaiveDate>,
    pub category: Option<String>, // default: ""
    pub priority: Option<Priority>, // default: Default
    // status: Option<Status>,
}

#[derive(Serialize)]
struct NewTaskRep {id: u64}

#[derive(Deserialize)]
struct ModTask {
    pub title: Option<String>,
    pub content: Option<String>,
    pub time: Option<NaiveTime>,
    pub date: Option<NaiveDate>,
    pub category: Option<String>, // "" is default
    pub priority: Option<Priority>, // Default is default
    // status: Option<Status>,
}

async fn get_task (
    Extension(schedule): Extension<Schedule>,
    Path(id): Path<u64>,
) -> Result<Json<Task>> 
{
    Ok(Json(schedule.task(id)?))
}

async fn insert_task (
    Extension(schedule): Extension<Schedule>,
    Json(task): Json<NewTaskReq>
) -> Result<Json<NewTaskRep>> 
{
    let id = schedule.generate_id()?;
    schedule.task_insert(&Task{ 
        id,
        title: task.title, 
        content: task.content.unwrap_or("".to_string()), 
        time: task.time, 
        date: task.date,
        category: task.category, 
        priority: task.priority.unwrap_or(Priority::Default) 
    })?;
    Ok(Json(NewTaskRep{id}))
}

async fn patch_task (
    Extension(schedule): Extension<Schedule>,
    Path(id): Path<u64>,
    Json(modtask): Json<ModTask>,
) -> Result<()> 
{
    let mut task = schedule.task(id)?;
    task = Task {
        id: task.id,
        title: modtask.title.unwrap_or(task.title),
        content: modtask.content.unwrap_or(task.content),
        time: modtask.time,
        date: modtask.date,
        category: modtask.category,
        priority: modtask.priority.unwrap_or(task.priority),
    };
    schedule.task_modify(&task)?;
    Err(OK)
}

async fn delete_task (
    Extension(schedule): Extension<Schedule>,
    Path(id): Path<u64>,
) -> Result<()> 
{
    schedule.task_remove(id)?;
    Err(OK)
}

// tasks
#[derive(Deserialize)]
struct GetTasks {
    filter: Option<sccore::Filter>,
}

// todo: paging
#[derive(Serialize)]
struct GetTasksResp {
    tasks: Vec<Task>,
}

async fn get_tasks (
    Extension(schedule): Extension<Schedule>,
    Json(filter): Json<GetTasks>,
) -> Result<Json<GetTasksResp>> 
{
    // iter filter
    let f = |t|match t {
        Ok(t)=>Some(t),
        Err(_) => None,
    };
    let tasks:Vec<_> = match filter.filter {
        None => schedule.tasks()
            .filter_map(f).collect(),
        Some(filter) => schedule.tasks_filtered(filter)
            .filter_map(f).collect(),
    };
    // todo: sort
    /*
    tasks.sort_by_key(|a| chrono::NaiveDateTime::new(a.lop.next(), a.time));
    */
    Ok(Json(GetTasksResp{tasks}))
}

/// middleware that authenticate id and inserts its schedule
pub async fn auth(
    State(ctx): State<Ctx>,
    Extension(id): Extension<SessionID>,
    mut req: Request,
    next: Next
) -> Result<Response> 
{
    let Some(Some(schedule)) = ctx.schedule(&id) else 
        {return Err(Unauthorized);};
    req.extensions_mut().insert(schedule);
    Ok(next.run(req).await)
}