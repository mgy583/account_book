import React, { useEffect, useState } from 'react';
import { Table, Button, Modal, Form, Input, InputNumber, message, Select, DatePicker, Popconfirm } from 'antd';
import api from '../utils/api';
import dayjs from 'dayjs';

// 固定类型和币种
const ORDER_TYPES = [
  { label: '餐饮', value: '餐饮' },
  { label: '购物', value: '购物' },
  { label: '交通', value: '交通' },
  { label: '娱乐', value: '娱乐' },
  { label: '医疗', value: '医疗' },
  { label: '其他', value: '其他' },
];
const CURRENCIES = [
  { label: '人民币', value: 'CNY' },
  { label: '美元', value: 'USD' },
  { label: '欧元', value: 'EUR' },
];

interface Order {
  id: string;
  name: string;
  type: string;
  amount: number;
  currency: string;
  remark?: string;
  date: string;
}

export default function OrderPage({ onLogout, onSwitchUser }: { onLogout: () => void; onSwitchUser: () => void }) {
  const [data, setData] = useState<Order[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [form] = Form.useForm();

  const fetchOrders = async () => {
    setLoading(true);
    try {
      const res = await api.get<Order[]>('/orders');
      setData(res.data);
    } catch (e) {
      message.error('获取订单失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchOrders();
  }, []);

  const handleCreate = async () => {
    try {
      const values = await form.validateFields();
      values.date = values.date.format('YYYY-MM-DD');
      await api.post('/orders', values);
      message.success('创建成功');
      setModalOpen(false);
      form.resetFields();
      fetchOrders();
    } catch (e) {
      message.error('创建失败');
    }
  };

  const handleDelete = async (id: string) => {
    try {
      await api.delete(`/orders/${id}`);
      message.success('删除成功');
      fetchOrders();
    } catch (e) {
      message.error('删除失败');
    }
  };

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 16 }}>
        <Button type="primary" onClick={() => setModalOpen(true)}>新建订单</Button>
        <div>
          <Button onClick={onSwitchUser} style={{ marginRight: 8 }}>切换账户</Button>
          <Button danger onClick={onLogout}>退出登录</Button>
        </div>
      </div>
      <Table
        rowKey="id"
        loading={loading}
        dataSource={data}
        columns={[
          { title: '名称', dataIndex: 'name' },
          { title: '类型', dataIndex: 'type' },
          { title: '金额', dataIndex: 'amount' },
          { title: '币种', dataIndex: 'currency' },
          { title: '日期', dataIndex: 'date' },
          { title: '备注', dataIndex: 'remark' },
          {
            title: '操作',
            dataIndex: 'action',
            render: (_, record) => (
              <Popconfirm title="确定删除？" onConfirm={() => handleDelete(record.id)}>
                <Button danger size="small">删除</Button>
              </Popconfirm>
            ),
          },
        ]}
      />
      <Modal
        title="新建订单"
        open={modalOpen}
        onOk={handleCreate}
        onCancel={() => setModalOpen(false)}
        destroyOnClose
      >
        <Form form={form} layout="vertical">
          <Form.Item name="name" label="名称" rules={[{ required: true }]}> <Input /> </Form.Item>
          <Form.Item name="type" label="类型" rules={[{ required: true }]}> <Select options={ORDER_TYPES} /> </Form.Item>
          <Form.Item name="amount" label="金额" rules={[{ required: true }]}> <InputNumber style={{ width: '100%' }} /> </Form.Item>
          <Form.Item name="currency" label="币种" initialValue="CNY" rules={[{ required: true }]}> <Select options={CURRENCIES} /> </Form.Item>
          <Form.Item name="date" label="日期" rules={[{ required: true }]}> <DatePicker style={{ width: '100%' }} /> </Form.Item>
          <Form.Item name="remark" label="备注"> <Input /> </Form.Item>
        </Form>
      </Modal>
    </div>
  );
}
