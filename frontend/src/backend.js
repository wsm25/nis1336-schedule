
export class UserInfo {
    categories; // Array<String>
}

// todo: switch to utc
export class Filter {
    date = undefined; // Date
    // from: Date|undefined = undefined; // todo
    // to: Date|undefined = undefined; // todo
    category = undefined; // String | undefined
    priorities = undefined; // Array<String>
}

export class Task {
    id; // u64
    title; // String
    content; // String
    time; // Time
    date;
    notice; // u32 | undefined
    category; // String | undefined
    priority; // string
}

export const pris = [
    {key: "default", label: "默认"},
    {key: "low", label: "低"},
    {key: "mid", label: "中"},
    {key: "high", label: "高"},
]

// sync async, will return string on error, and object on success
function request(ctx, uri, method, data, onFinish){
    var xhr = new XMLHttpRequest();
    xhr.open(method, uri);
    xhr.setRequestHeader("Content-Type", "application/json");
    xhr.onload = function(_){
        if (xhr.readyState === 4) {
            if (xhr.status === 200) {
                try {onFinish(JSON.parse(xhr.response));} 
                catch {onFinish("Bad payload");}
            } else if (xhr.status === 401) { // login is required
                ctx.setLogin(false);
                ctx.messageApi.warning("未登录");
                onFinish("未登录");
            } else {
                try { 
                    var message=JSON.parse(xhr.responseText).message;
                    ctx.messageApi.error("Error: "+message);
                    onFinish(message);
                } catch {
                    ctx.messageApi.error("Error: "+xhr.responseText);
                    onFinish(xhr.responseText);
                }
            }
        }
      };
      xhr.onerror = function(_){
        ctx.messageApi.error("Network error:", xhr.statusText);
        onFinish(xhr.statusText);
      };
      xhr.send(data);
}

// async, will reject on error
function arequest(ctx, uri, method, data){
    return new Promise(function(resolve, reject){
        request(ctx, uri, method, data, function(response){
            switch (typeof response){
                case "string": 
                    reject({message: response});
                    break;
                case "object":
                    resolve(response);
                    break;
                default:
                    reject({message: "bad response"});
            }
        })
    });
    
}

// return: {categories} | undefined
export function userinfo(ctx){
    return arequest(ctx, "/api/user", "GET", null);
}

// return: Task[] | undefined
export function gettasks(ctx){
    return new Promise(function(resolve, reject){
        arequest(
            ctx, "/api/tasks", "POST", 
            JSON.stringify({filter: ctx.filter})
        ).then(
            (v)=>resolve(v.tasks),
            (e)=>reject(e)
        )
    });
}

// return: Task | undefined
export async function addTask(ctx, title){
    var task={
        title,
        content: "",
        priority: "default",
    }
    try {
        var id=(await arequest(ctx, "/api/task", "PUT", JSON.stringify(task))).id;
        task.id=id;
        return task;
    } catch(e) {return Promise.reject(e);}
}

// return: undefined
export async function delTask(ctx, id){
    try {
        await arequest(ctx, "/api/task/"+id, "DELETE", null);
        return;
    } catch(e) {return Promise.reject(e);}
}

// return: undefined
export async function modTask(ctx, task){
    try {
        await arequest(ctx, "/api/task/"+task.id, "PATCH", 
            JSON.stringify(task)
        );
        return;
    } catch(e) {return Promise.reject(e);}
}

// return: undefined
export async function login(ctx, username, password){
    try {
        await arequest(ctx, "/auth/login", "POST", 
            JSON.stringify({username, password})
        );
        ctx.setLogin(true);
        window.location.reload();
        return;
    } catch(e) {return Promise.reject(e);}
}

// return: undefined
export async function register(ctx, username, password){
    try {
        await arequest(ctx, "/auth/register", "POST", 
            JSON.stringify({username, password})
        );
        ctx.setLogin(true);
        window.location.reload();
        return;
    } catch(e) {return Promise.reject(e);}
}