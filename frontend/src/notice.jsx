import { notification } from 'antd';
import ting from './notification.mp3'
import { useEffect, useRef } from 'react';
import dayjs from 'dayjs';

const Notice = ({tasks}) => {
  const audioRef = useRef(null);
  const [noticer, noticeHolder] = notification.useNotification();
  useEffect(()=>{
      const interval=5000;
      const intervalId = setInterval(()=>{
        if(tasks){
          let now=dayjs();
          for (let task of tasks){
            if (task.date && task.time) {
              let tasktime=dayjs(
                task.date+" "+task.time,
                "YYYY-MM-DD HH:mm:ss"
              );
              let diff=tasktime.diff(now);
              if(diff>=0 && diff<interval){ // alert it!
                noticer.info({
                  message: "该干活啦！",
                  placement: 'topRight',
                  description: <>任务：{task.title} 要开始啦！</>,
                  duration: 30,
                });
                audioRef.current.play();
              }
            }
          }
        }
      }, interval);
      return () => clearInterval(intervalId); 
    }, [tasks]);

  return <>
    {noticeHolder}
    <audio ref={audioRef} src={ting} />
  </>
}

export default Notice;