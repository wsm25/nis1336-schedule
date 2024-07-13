use axum::{
    extract::{Extension, Path, Request, State}, 
    routing::{get, post, put, delete}, 
    Json,
    response::Response,
    middleware::{Next, from_fn_with_state},
};
use sccore::{Schedule, task::*};
use crate::{ctx::Ctx, error::{Result, Error::*}, session::SessionID};
use chrono::NaiveTime;

pub fn router(ctx: Ctx)->axum::Router {
    axum::Router::new()
    .route("/categories", get(categories))
    .route("/tasks", post(get_tasks))
    .route("/task", put(insert_task))
    .route("/task/:id", get(get_task))
    .route("/task/:id", put(put_task))
    .route("/task/:id", delete(delete_task))
    .route_layer(from_fn_with_state(ctx.clone(), auth))
    .with_state(ctx)
}

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Cats {names: Vec<String>}

async fn categories (
    Extension(schedule): Extension<Schedule>,
) -> Result<Json<Cats>> 
{
    Ok(Json(Cats{
        names: schedule.categories()
    }))
}

// tasks

#[derive(Deserialize)]
struct NewTaskReq {
    pub title: String, // required
    pub content: String, // required
    pub time: NaiveTime, // required
    pub lop:  Loop, // required
    pub notice: Option<u32>, // default: 0
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
    pub lop:  Option<Loop>,
    pub notice: Option<u32>, // how many minutes before
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
        content: task.content, 
        time: task.time, 
        lop: task.lop, 
        notice: task.notice.unwrap_or(0), 
        category: task.category.unwrap_or("".to_string()), 
        priority: task.priority.unwrap_or(Priority::Default) 
    })?;
    Ok(Json(NewTaskRep{id}))
}

async fn put_task (
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
        time: modtask.time.unwrap_or(task.time),
        lop: modtask.lop.unwrap_or(task.lop),
        notice: modtask.notice.unwrap_or(task.notice),
        category: modtask.category.unwrap_or(task.category),
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
    Ok(Json(
        GetTasksResp{
            tasks: match filter.filter {
                None => schedule.tasks()
                    .filter_map(f).collect(),
                Some(filter) => schedule.tasks_filtered(filter)
                    .filter_map(f).collect(),
            }
    }))
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