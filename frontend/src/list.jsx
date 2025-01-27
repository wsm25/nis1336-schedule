import { useState, memo } from 'react'
import { 
  Skeleton, Button, Dropdown, 
  DatePicker, Space, Select, 
  Input, List, Checkbox, Empty,
  message, Popconfirm } from 'antd'
import { UnorderedListOutlined, CloseOutlined, PlusOutlined } from '@ant-design/icons'
import * as backend from './backend'

import { pris } from './backend'

import './list.css'
import { priFlags } from './design';

// require: filter, updateFilter, remote categories
const FilterBar = ({ctx}) => {
  // console.log("[Hook] filter reloaded");
  const modFilter = (field, value)=>{
    ctx.filter[field]=value;
    ctx.setFilter(ctx.filter);
    backend.gettasks(ctx).then(
      (tasks)=>{
        ctx.setTasks(tasks);
      }
    )
  }
  const [cat, setCat_] = useState(ctx.filter.category);
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
          {cat ? <>{cat}<CloseOutlined onClick={()=>setCat(undefined)}/></> : "所有分类"}
        </Button>
      </Dropdown>

      <DatePicker onChange={(_, date)=>{modFilter('date', date=="" ? undefined : date)}}/>

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
  // console.log("[Hook] task list reloaded");
  const TaskItem = memo(({task})=>{return (
    <List.Item 
      key={task.id} 
      onClick={()=>{ctx.setCurrentTask(undefined); ctx.setCurrentTask(task)}}
      className={ctx.currentTask ? (ctx.currentTask.id==task.id ? "task-selected task" : "task") : "task"}
    >
      <Space>
        <Popconfirm
          title="删除任务"
          description="确认要删除此任务吗？"
          onConfirm={(e)=>{
            if((ctx.currentTask) && (task.id==ctx.currentTask.id)) {
              ctx.setCurrentTask(undefined);
            }
            ctx.delTask(task);
            e.stopPropagation();
          }}
          onClick={(e)=>{e.stopPropagation();}}
        >
          <Checkbox className={"checkbox-"+task.priority} checked={false} title="删除任务"/>
        </Popconfirm>
        {task.title}
      </Space>
    </List.Item>
  );});
  return <List
      itemLayout="vertical"
      dataSource={ctx.tasks}
      bordered
      locale={{emptyText: <Empty description="没有任务，放松一下~"/>}}
      renderItem={(task)=><TaskItem task={task}/>}
    />
}

const TaskAdder = ({ctx}) => {
  var [newTitle, setNewTitle] = useState("");
  return (
    <Input 
      prefix={<PlusOutlined style={{color: "#ccc"}}/>}
      placeholder="添加任务，回车即可保存"
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
  // console.log("[Hook] list element reloaded");
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