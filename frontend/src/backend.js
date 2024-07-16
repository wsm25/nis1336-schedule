
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

// return: {categories} | error message
export function userinfo(){
    return {categories: ["哈哈"]};
}

// return: Task[] | error message
export function gettasks(){
    return [
    {
        id: 1,
        title: "鹿乃子乃子",
        content: "虎视眈眈",
        time: "11:45:14",
        date: "1919-08-11",
        priority: "high",
        category: "哈哈",
    },
    {
        id: 2,
        title: "结城友奈",
        content: "是勇者",
        time: "11:45:15",
        date: "1919-08-10",
        priority: "default",
    },
    ]
}

// return: Task | error message
export function addTask(title){
    return {
        title,
        id: Math.floor(Math.random()*114514),
        content: "",
        priority: "default",
    }
}

// return: undefined | error message
export function delTask(id){
    return undefined;
}

// return: undefined | error message
export function modTask(task){
    return undefined;
}

// return: undefined | error message
export function login(username, password){
    return undefined;
}

// return: undefined | error message
export function register(username, password){
    return undefined;
}