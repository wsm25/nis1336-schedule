import React, {useState, useEffect, useMemo} from 'react'
import ReactDOM from 'react-dom/client'
import { ConfigProvider, message } from 'antd';
import zhCN from 'antd/locale/zh_CN';

import ListEle from './list';
import TaskEle from './task';
import Auth from './auth';
import * as backend from './backend'

import './main.css'

const App = function(){
  // console.log("[Hook] page rendering...")
  // lazy load
  var [cats, setCats] = useState(()=>[]);
  var [tasks, setTasks] = useState(()=>undefined);
  var [currentTask, setCurrentTask] = useState(undefined);
  var [login, setLogin] = useState(true);
  var [filter, setFilter] = useState(new backend.Filter());
  var lazysave = useMemo(()=>({task: undefined, timeout: undefined}), []);
  const [messageApi, contextHolder] = message.useMessage();
  var ctx={
    cats, setCats, 
    tasks, setTasks, 
    currentTask, setCurrentTask,
    login, setLogin,
    filter, setFilter,
    messageApi,
    superReload: function(){
      backend.gettasks(ctx).then(
        (tasks)=>{ctx.setTasks(tasks);}
      );
    },
    delTask: function(task){
      backend.delTask(ctx, task.id);
      tasks=tasks.filter(t=>t.id!=task.id);
      setTasks(tasks);
    },
    reload: function(){
      setTasks([...tasks]);
    },
    modTaskLazy: function(task){
      if(lazysave.task===task) {
        clearTimeout(lazysave.timeout); // can be undefined
      }
      else {
        lazysave.task=task;
      }
      lazysave.timeout=setTimeout(function(){
        console.log("[lazy save] uploading...");
        backend.modTask(ctx, task);
        lazysave.timeout=undefined;
        lazysave.task=undefined;
      }, 1000)
    },
    modTask: function(task){
      this.modTaskLazy(task);
      this.reload();
    },
    addTask: function(title){
      backend.addTask(ctx, title).then(
        (task)=>{
          setCurrentTask(task);
          this.superReload();
        },
      )
    }
  };
  // init
  useEffect(function(){
    backend.userinfo(ctx).then(function(info){
      ctx.setCats(info.categories);
      ctx.superReload();
    }, ()=>{});
  }, [])
  return (
    <React.StrictMode>
      <ConfigProvider locale={zhCN}>
        {contextHolder}
        <Auth ctx={ctx}/>
        <div id="schedule-list"><ListEle ctx={ctx}/></div>
        <div id="schedule-divide"/>
        <div id="schedule-task"><TaskEle ctx={ctx}/></div>
      </ConfigProvider>
    </React.StrictMode>
  )
}

ReactDOM.createRoot(document.getElementById('root')).render(<App/>);
