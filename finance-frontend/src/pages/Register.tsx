import React, { useState } from 'react';
import { Form, Input, Button, message, Card } from 'antd';
import api from '../utils/api';

export default function RegisterPage({ onRegister }: { onRegister: () => void }) {
  const [loading, setLoading] = useState(false);
  const [form] = Form.useForm();

  const handleRegister = async () => {
    try {
      const values = await form.validateFields();
      setLoading(true);
      // 假设后端注册接口为 /register，返回 { token: string }
      const res = await api.post('/register', values);
      sessionStorage.setItem('token', res.data.token);
      message.success('注册成功');
      onRegister();
    } catch (e: any) {
      message.error(e?.response?.data?.message || '注册失败');
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ display: 'flex', justifyContent: 'center', alignItems: 'center', height: '100vh', background: '#f0f2f5' }}>
      <Card title="注册" style={{ width: 320 }}>
        <Form form={form} layout="vertical" onFinish={handleRegister}>
          <Form.Item name="username" label="用户名" rules={[{ required: true, message: '请输入用户名' }]}> 
            <Input autoFocus />
          </Form.Item>
          <Form.Item name="password" label="密码" rules={[{ required: true, message: '请输入密码' }]}> 
            <Input.Password />
          </Form.Item>
          <Form.Item name="confirm" label="确认密码" dependencies={["password"]} rules={[
            { required: true, message: '请确认密码' },
            ({ getFieldValue }) => ({
              validator(_, value) {
                if (!value || getFieldValue('password') === value) {
                  return Promise.resolve();
                }
                return Promise.reject('两次输入的密码不一致');
              },
            }),
          ]}>
            <Input.Password />
          </Form.Item>
          <Form.Item>
            <Button type="primary" htmlType="submit" loading={loading} block>注册</Button>
          </Form.Item>
        </Form>
      </Card>
    </div>
  );
}
