import React, { useEffect, useState } from 'react';
import { Table, Button, Modal, Form, Input, InputNumber, message } from 'antd';
import api from '../utils/api';

interface Account {
  id: string;
  name: string;
  account_type: string;
  balance: number;
  currency: string;
  remark?: string;
}

export default function AccountPage() {
  const [data, setData] = useState<Account[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [form] = Form.useForm();

  const fetchAccounts = async () => {
    setLoading(true);
    try {
  const res = await api.get<Account[]>('/accounts');
      setData(res.data);
    } catch (e) {
      message.error('获取账户失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchAccounts();
  }, []);

  const handleCreate = async () => {
    try {
      const values = await form.validateFields();
  await api.post('/accounts', values);
      message.success('创建成功');
      setModalOpen(false);
      form.resetFields();
      fetchAccounts();
    } catch (e) {
      message.error('创建失败');
    }
  };

  return (
    <div>
      <Button type="primary" onClick={() => setModalOpen(true)} style={{ marginBottom: 16 }}>
        新建账户
      </Button>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={data}
        columns={[
          { title: '名称', dataIndex: 'name' },
          { title: '类型', dataIndex: 'account_type' },
          { title: '余额', dataIndex: 'balance' },
          { title: '币种', dataIndex: 'currency' },
          { title: '备注', dataIndex: 'remark' },
        ]}
      />
      <Modal
        title="新建账户"
        open={modalOpen}
        onOk={handleCreate}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="name" label="名称" rules={[{ required: true }]}> 
            <Input />
          </Form.Item>
          <Form.Item name="account_type" label="类型" rules={[{ required: true }]}> 
            <Input />
          </Form.Item>
          <Form.Item name="balance" label="余额" rules={[{ required: true }]}> 
            <InputNumber style={{ width: '100%' }} />
          </Form.Item>
          <Form.Item name="currency" label="币种" rules={[{ required: true }]}> 
            <Input />
          </Form.Item>
          <Form.Item name="remark" label="备注"> 
            <Input />
          </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
