import React, { useState } from 'react';
import { Button, Checkbox, Form, Input, Col, Row } from 'antd';

import './auth.css'

const AuthCore = ({ctx}) => {
  const [username, setUsername]=useState("");
  const [password, setPassword]=useState("");
  function passwordInvalid(){
    return password.length!=0 && password.length<8;
  }
  return (<>
      <Row gutter={[0, 24]}>
        <Col span={24} id="schedule-auth-title">登录/注册</Col>

        <Col span={6} className="schedule-auth-lable">用户名</Col>
        <Col span={1}/>
        <Col span={14}>
          <Input value={username} onChange={(e)=>{setUsername(e.target.value);}}/>
        </Col>
        <Col span={3}/>

        <Col span={6} className="schedule-auth-lable">密码</Col>
        <Col span={1}/>
        <Col span={14}>
          <Input.Password 
            placeholder="不少于 8 位"
            value={password} 
            onChange={(e)=>{setPassword(e.target.value);}}
            status={passwordInvalid() ? "error" : undefined}
            showCount
          />
        </Col>
        <Col span={3}/>

        <Col span={7}/>
        <Col span={6}><Button type="primary">登录</Button></Col>
        <Col span={6}><Button type="default">注册</Button></Col>
        <Col span={5}/>
      </Row>
  </>)
};

const Auth = ({ctx}) => {
  return ( ctx.login ? <></> :
    <div id="schedule-auth-mask">
      <div id="schedule-auth-back">
        <AuthCore ctx={ctx}/>
      </div>
    </div>
  )
};

export default Auth;