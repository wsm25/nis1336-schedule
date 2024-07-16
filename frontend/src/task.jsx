/// Task: includes task showing and editing.
/// Will automatically save if editted and no action in 10s;
/// or exiting the page.

import { Checkbox, Divider, DatePicker, Button, AutoComplete, Input, TimePicker, Dropdown, Space } from 'antd';
import './task.css'
import { pris } from './backend'
import { priFlags } from './design';
import { UnorderedListOutlined } from '@ant-design/icons';
import dayjs from 'dayjs';
import { useState } from 'react';

// require: cat_state
const Task = ({ctx}) => {
  console.log("[Hook] task reloaded");
  let task = ctx.currentTask;
  if (!task) {return <></>;}
  // cats
  let cats = [];
  for (let cat of ctx.cats) 
    cats.push({value: cat, label: cat});
  // cat
  function submitCat(cat){
    console.log("submit cat", cat);
    if (cat && !ctx.cats.includes(cat)){
      console.log("inserting new cat", cat);
      ctx.cats.push(cat);
      ctx.setCats(ctx.cats);
    }
    task.category=cat;
    ctx.modTask(task);
  }
  // priority
  const priItems=pris.map(({key, label})=>({
    key,
    label: <a key={key} onClick={()=>{
      console.log("priority set to", key);
      task.priority=key;
      ctx.modTask(task);
    }}><Space>{priFlags[key]}{label}</Space></a>
  }))
  // content
  const Content = ({ctx})=>{
    const [content, setContent]=useState(task.content);
    return (
      <textarea
        id="schedule-task-content-box"
        value={content}
        onChange={(e)=>{
          task.content=e.target.value;
          setContent(task.content);
          ctx.modTaskLazy(task);
        }}
      />
    );
  };
  return (<>
    <div id="schedule-task-meta"  key={"taskdetail"+task.id}>
        <Checkbox onClick={()=>{
          ctx.delTask(task);
          ctx.setCurrentTask(undefined);
        }}/>
        <Divider type="vertical" />
        <DatePicker 
          onChange={(_, date)=>{
            task.date=date;
            ctx.setCurrentTask(task);
          }} 
          defaultValue={task.date?dayjs(task.date):undefined}
        />
        <Divider type="vertical" />
        <TimePicker  
          onChange={(_, time)=>{
            task.time=time;
            ctx.setCurrentTask(task);
          }}
          defaultValue={task.time?dayjs(task.time, "HH:mm:ss"):undefined}
        />
        <Divider type="vertical" />
        <Dropdown menu={{items: priItems}}>
          <Button icon={priFlags[ctx.currentTask.priority]}></Button>
        </Dropdown>
    </div>
    <Divider/>
    <div id="schedule-task-title">
      <input
        id="schedule-task-title-box"
        value={task.title}
        onChange={(e)=>{
          task.title=e.target.value;
          ctx.modTask(task)
        }}
      />
    </div>
    <div id="schedule-task-content">
        <Content ctx={ctx}/>  
    </div>
    <div id="schedule-task-category" >
      <AutoComplete key={"taskdetail"+task.id} options={cats} defaultValue={task.category} >
        <Input 
          prefix={<UnorderedListOutlined/>}
          placeholder="类型" 
          defaultValue={task.category}
          onKeyDown={(e)=>{if(e.key==='Enter') submitCat(e.target.value);}}
        />
      </AutoComplete>
    </div>
  </>)
}

export default Task;