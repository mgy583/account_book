import React, { useState } from 'react';
import { Form, Input, Button, message, Card } from 'antd';
import api from '../utils/api';

export default function LoginPage({ onLogin, onShowRegister }: { onLogin: () => void; onShowRegister?: () => void }) {
  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm();

  const handleLogin = async () => {
    try {
      const values = await form.validateFields();
      setLoading(true);
      // 假设后端登录接口为 /login，返回 { token: string }
      const res = await api.post('/login', values);
      sessionStorage.setItem('token', res.data.token);
      message.success('登录成功');
      onLogin();
    } catch (e: any) {
      message.error(e?.response?.data?.message || '登录失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh', background: '#f0f2f5' }}>
      <Card title="登录" style={{ width: 320 }}>
        <Form form={form} layout="vertical" onFinish={handleLogin}>
          <Form.Item name="username" label="用户名" rules={[{ required: true, message: '请输入用户名' }]}> 
            <Input autoFocus />
          </Form.Item>
          <Form.Item name="password" label="密码" rules={[{ required: true, message: '请输入密码' }]}> 
            <Input.Password />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit" loading={loading} block>登录</Button>
          </Form.Item>
        </Form>
        <div style={{ textAlign: 'right', marginTop: 8 }}>
          <Button type="link" onClick={onShowRegister}>没有账号？注册</Button>
        </div>
      </Card>
    </div>
  );
}
