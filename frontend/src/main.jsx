import React, {useState, createContext} from 'react'
import ReactDOM from 'react-dom/client'

import ListEle from './list';
import TaskEle from './task';
import * as backend from './backend'

import './main.css'
import Auth from './auth';

const App = () => {
  // lazy load
  var [cats, setCats] = useState(()=>backend.userinfo().categories);
  var [tasks, setTasks] = useState(()=>backend.gettasks());
  var [currentTask, setCurrentTask] = useState(undefined);
  var [login, setLogin] = useState(true);
  var ctx={
    cats, setCats, 
    tasks, setTasks, 
    currentTask, setCurrentTask,
    login, setLogin,
    delTask: function(task){
      backend.delTask(task.id);
      tasks=tasks.filter(t=>t.id!=task.id);
      setTasks(tasks);
    },
    reload: function(){
      setTasks([...tasks]);
    },
    modTaskLazy: function(task){
      backend.modTask(task);
    },
    modTask: function(task){
      this.modTaskLazy(task);
      this.reload();
    },
    addTask: function(title){
      var task=backend.addTask(title);
      console.log("add task", task);
      setCurrentTask(task);
      setTasks([...tasks, task]);
    }
  };
  return (
    <React.StrictMode>
      <Auth ctx={ctx}/>
      <div id="schedule-list"><ListEle ctx={ctx}/></div>
      <div id="schedule-divide"/>
      <div id="schedule-task"><TaskEle ctx={ctx}/></div>
    </React.StrictMode>
  )
}

ReactDOM.createRoot(document.getElementById('root')).render(<App/>)