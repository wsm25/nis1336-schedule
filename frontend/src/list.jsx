import { useState, memo } from 'react'
import { 
  Skeleton, Button, Dropdown, 
  DatePicker, Space, Select, 
  Input, List, Checkbox, 
  message, } from 'antd'
import { UnorderedListOutlined, CloseOutlined } from '@ant-design/icons'
import dayjs from 'dayjs';

import { Filter, pris } from './backend'

import './list.css'
import { priFlags } from './design';

// require: filter, updateFilter, remote categories
const FilterBar = ({ctx}) => {
  var [filter, setFilter] = useState(new Filter());
  console.log("[Hook] filter reloaded");
  const modFilter = (field, value)=>{
    filter[field]=value;
    console.log("filter is now", filter);
    setFilter(filter);
  }
  const [cat, setCat_] = useState(filter.category);
  const setCat = (cat)=>{setCat_(cat); modFilter('category', cat)};
  var categoryitems = []
  for (const category of ctx.cats) {
    categoryitems.push({key: category, label: (
      <div onClick={()=>setCat(category)}>{category}</div>
    )});
  }
  // pris
  let prioptions = pris.map(({key, label})=>({value: key, label: <Space>{priFlags[key]}{label}</Space>}));
  return (
    <Space>
      <Dropdown menu={{items :categoryitems}}>
        <Button type="text">
          <UnorderedListOutlined />
          {cat ? <>{cat}<CloseOutlined onClick={()=>setCat(undefined)}/></> : "所有类型"}
        </Button>
      </Dropdown>

      <DatePicker onChange={(date)=>{modFilter('date', date)}}/>

      <Select 
        placeholder="优先级"
        style={{ minWidth: 120 }} 
        onChange={(ps)=>{modFilter('priorities', ps)}} 
        mode="multiple"
        options={prioptions}
      />
    </Space>
  )
}

// only filter will rerender all tasks
const TasksList = ({ctx}) =>{
  console.log("[Hook] task list reloaded");
  const TaskItem = memo(({task})=>{return (
    <List.Item 
      key={task.id} 
      onClick={()=>{ctx.setCurrentTask(undefined); ctx.setCurrentTask(task)}}
      className={ctx.currentTask ? (ctx.currentTask.id==task.id ? "task-selected task" : "task") : "task"}
    >
      <Space>
        <Checkbox onClick={(e)=>{
          if(ctx.currentTask && task.id==ctx.currentTask.id) {
            ctx.setCurrentTask(undefined);
          }
          ctx.delTask(task);
          e.stopPropagation();
          }}/>
        {task.title}
      </Space>
    </List.Item>
  );});
  return <List
    itemLayout="vertical"
    dataSource={ctx.tasks}
    bordered
    renderItem={(task)=><TaskItem task={task}/>}
  />
}

const TaskAdder = ({ctx}) => {
  var [newTitle, setNewTitle] = useState("");
  return (
    <Input 
      placeholder="添加任务" 
      style={{ height: "48px" }} 
      value={newTitle}
      onChange={(e)=>{setNewTitle(e.target.value)}}
      onKeyDown={(e)=>{if(e.key==='Enter') {
        var title=e.target.value;
        if(title=="") {message.error("任务名称不能为空"); return;}
        setNewTitle("");
        ctx.addTask(title);
      }}}
    />
  ) 
}

// require: cat
const ListEle = ({ctx}) => {
  console.log("[Hook] list element reloaded");
  return (<>
    <div id="schedule-filter-bar"><FilterBar ctx={ctx}/></div>
    <div id="schedule-taskadder-bar"><TaskAdder ctx={ctx}/></div>
    <div id="schedule-task-list">
      <Skeleton active loading={!ctx.tasks}>
        <TasksList ctx={ctx}/>
      </Skeleton>
    </div>
  </>)
}

export default ListEle;