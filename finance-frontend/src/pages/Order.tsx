import React, { useEffect, useState } from 'react';
import numeral from 'numeral';
import { Table, Button, Modal, Form, Input, InputNumber, message, Select, DatePicker, Popconfirm, Card, Typography, Space, Row, Col } from 'antd';
import { Bar, Column } from '@ant-design/charts';
import api from '../utils/api';
import dayjs from 'dayjs';
import isBetween from 'dayjs/plugin/isBetween';
dayjs.extend(isBetween);
import { PlusOutlined, LogoutOutlined, SwapOutlined } from '@ant-design/icons';
import './Order.css';

// 固定类型和币种前端出现一些问题
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
  const [groupChartType, setGroupChartType] = useState<'day'|'month'>('day');
  const [groupChartData, setGroupChartData] = useState<any[]>([]);
  const [data, setData] = useState<Order[]>([]);
  const [loading, setLoading] = useState(false);
  const [modalOpen, setModalOpen] = useState(false);
  const [form] = Form.useForm();
  const [filter, setFilter] = useState<{name?: string; type?: string; date?: [any, any]}>({});
  const [page, setPage] = useState(1);
  const [pageSize, setPageSize] = useState(8);
  const [statData, setStatData] = useState<any[]>([]);
  const [statGroup, setStatGroup] = useState<'type'|'currency'|'month'>('type');
  const [statMonth, setStatMonth] = useState<'this' | 'last'>('this');

  const [total, setTotal] = useState(0);
  // 获取本月/上月的起止日期
  const getMonthRange = (type: 'this' | 'last') => {
    const now = dayjs();
    if (type === 'this') {
      return [now.startOf('month'), now.endOf('month')];
    } else {
      const last = now.subtract(1, 'month');
      return [last.startOf('month'), last.endOf('month')];
    }
  };

  // 获取订单统计分组key
  const getGroupKey = (item: Order) => {
    if (statGroup === 'type') return item.type;
    if (statGroup === 'currency') return item.currency;
    if (statGroup === 'month') return dayjs(item.date).format('YYYY-MM');
    return '';
  };

  const fetchOrders = async () => {
    setLoading(true);
    try {
  // 查询订单列表
      const res = await api.get<any>('/order_query/orders/query');
      // 后端返回对象结构
      if (res.data && Array.isArray(res.data.orders)) {
        const currencyMap: Record<string, string> = { '人民币': 'CNY', '美元': 'USD', '欧元': 'EUR' };
        let mapped = res.data.orders.map((item: any) => ({
          id: item.id || item._id || '',
          name: item.name || '',
          type: item.type || item.order_type || '',
          amount: item.amount,
          currency: currencyMap[item.currency] || item.currency,
          remark: item.remark,
          date: item.date ? (typeof item.date === 'string' ? item.date : dayjs(item.date).format('YYYY-MM-DD')) : '',
        }));
        setTotal(res.data.total || mapped.length);
        setData(mapped);
        // 分类统计（按本月/上月过滤）
        const [start, end] = getMonthRange(statMonth);
        const monthOrders = mapped.filter((o: Order) => dayjs(o.date).isBetween(start, end, null, '[]'));
        // 动态分组统计
        type StatItem = { group: string; amount: number };
        const statMap = new Map<string, number>();
        monthOrders.forEach((cur: Order) => {
          const key = getGroupKey(cur);
          statMap.set(key, (statMap.get(key) || 0) + cur.amount);
        });
        const statArr: StatItem[] = Array.from(statMap.entries()).map(([group, amount]) => ({ group, amount }));
        statArr.sort((a: StatItem, b: StatItem) => a.amount - b.amount); // 金额升序
        setStatData(statArr);
      } else {
        setData([]);
        setStatData([]);
        setTotal(0);
        message.error('返回数据格式错误');
      }
    } catch (e) {
      message.error('获取订单失败');
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchOrders();
    // eslint-disable-next-line
  }, [filter, page, pageSize, statMonth]);

  // 生成分组堆叠图数据
  useEffect(() => {
    // 只统计本月/上月数据
    const [start, end] = getMonthRange(statMonth);
    const dateFmt = groupChartType === 'day' ? 'YYYY-MM-DD' : 'YYYY-MM';
    const groupMap = new Map<string, {[type: string]: number}>();
    data.forEach((item: Order) => {
      const d = dayjs(item.date);
      if (!d.isValid() || !d.isBetween(start, end, null, '[]')) return;
      const dateKey = d.format(dateFmt);
      if (!groupMap.has(dateKey)) groupMap.set(dateKey, {});
      const typeMap = groupMap.get(dateKey)!;
      typeMap[item.type] = (typeMap[item.type] || 0) + item.amount;
    });
    // 转为图表数据
    const chartArr: { date: string; type: string; amount: number }[] = [];
    groupMap.forEach((typeMap, date) => {
      Object.entries(typeMap).forEach(([type, amount]) => {
        chartArr.push({ date, type, amount });
      });
    });
    setGroupChartData(chartArr);
  }, [data, statMonth, groupChartType]);

  const handleCreate = async () => {
    try {
      const values = await form.validateFields();
      values.date = values.date.format('YYYY-MM-DD');
  // 创建订单，走 /order
  await api.post('/order', values);
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
  // 删除订单，走 /order/{id}
  await api.delete(`/order/${id}`);
      message.success('删除成功');
      fetchOrders();
    } catch (e) {
      message.error('删除失败');
    }
  };

  return (
    <div className="order-bg">
  <Card className="order-card" variant="outlined" style={{ borderRadius: 16, boxShadow: '0 4px 24px #e6f7ff', margin: '32px auto', maxWidth: 1000 }}>
        <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 24 }}>
          <Typography.Title level={3} style={{ margin: 0, color: '#1890ff', fontWeight: 700, letterSpacing: 2 }}>记账本</Typography.Title>
          <Space>
            <Button type="primary" icon={<PlusOutlined />} onClick={() => setModalOpen(true)} size="large" style={{ borderRadius: 20, fontWeight: 500 }}>新建订单</Button>
            <Button icon={<SwapOutlined />} onClick={onSwitchUser} size="large" style={{ borderRadius: 20, background: '#f0f5ff', color: '#1890ff', fontWeight: 500, border: 0 }}>切换账户</Button>
            <Button icon={<LogoutOutlined />} danger onClick={onLogout} size="large" style={{ borderRadius: 20, fontWeight: 500 }}>退出登录</Button>
          </Space>
        </div>
        {/* 筛选区 */}
        <Row gutter={16} style={{ marginBottom: 16 }}>
          <Col span={6}>
            <Input placeholder="按名称搜索" allowClear onChange={e => setFilter(f => ({ ...f, name: e.target.value }))} style={{ borderRadius: 12 }} />
          </Col>
          <Col span={6}>
            <Select options={[{label:'全部',value:''},...ORDER_TYPES]} placeholder="按类型筛选" allowClear style={{ width: '100%', borderRadius: 12 }} onChange={v => setFilter(f => ({ ...f, type: v||undefined }))} />
          </Col>
          <Col span={8}>
            <DatePicker.RangePicker style={{ width: '100%', borderRadius: 12 }} onChange={v => setFilter(f => ({ ...f, date: v ? [v[0], v[1]] : undefined }))} />
          </Col>
        </Row>
        {/* 账单表格在上 */}
        {/* 账单筛选过滤 */}
        <Table
          rowKey="id"
          loading={loading}
          dataSource={
            data
              .filter(item => {
                // 名称筛选
                if (filter.name && !item.name.includes(filter.name)) return false;
                // 类型筛选
                if (filter.type && item.type !== filter.type) return false;
                // 日期筛选
                if (filter.date && filter.date.length === 2) {
                  const d = dayjs(item.date);
                  if (!d.isBetween(filter.date[0], filter.date[1], null, '[]')) return false;
                }
                return true;
              })
              .slice((page-1)*pageSize, page*pageSize)
          }
          pagination={{
            current: page,
            pageSize,
            total,
            showSizeChanger: true,
            pageSizeOptions: [8, 16, 32],
            onChange: (p, ps) => { setPage(p); setPageSize(ps); },
            style: { borderRadius: 12 }
          }}
          bordered
          style={{ background: '#fff', borderRadius: 12 }}
          columns={[
            { title: '名称', dataIndex: 'name', align: 'center' },
            { title: '类型', dataIndex: 'type', align: 'center', render: v => <span style={{ color: '#52c41a', fontWeight: 500 }}>{v}</span> },
            { title: '金额', dataIndex: 'amount', align: 'center', render: v => <span style={{ color: '#faad14', fontWeight: 700 }}>{v}</span> },
            { title: '币种', dataIndex: 'currency', align: 'center' },
            { title: '日期', dataIndex: 'date', align: 'center', render: v => v ? dayjs(v).format('YYYY-MM-DD') : '' },
            { title: '备注', dataIndex: 'remark', align: 'center' },
            {
              title: '操作',
              dataIndex: 'action',
              align: 'center',
              render: (_, record) => (
                <Popconfirm title="确定删除？" onConfirm={() => handleDelete(record.id)}>
                  <Button danger size="small" style={{ borderRadius: 12 }}>删除</Button>
                </Popconfirm>
              ),
            },
          ]}
        />
  {/* 统计图表在下 */}
        {/* 分组堆叠柱状图 */}
        <div style={{ marginTop: 32, background: '#fffbe6', borderRadius: 12, padding: 16 }}>
          <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: 8 }}>
            <Typography.Title level={5} style={{ color: '#faad14', margin: 0 }}>日期-类别-金额分布</Typography.Title>
            <Space>
              <Select
                size="small"
                style={{ width: 100 }}
                value={groupChartType}
                onChange={v => setGroupChartType(v)}
                options={[
                  { label: '按天', value: 'day' },
                  { label: '按月', value: 'month' },
                ]}
              />
            </Space>
          </div>
          <Column
            data={groupChartData}
            xField="date"
            yField="amount"
            seriesField="type"
            isGroup={true}
            isStack={true}
            color={[ '#1890ff', '#52c41a', '#faad14', '#eb2f96', '#722ed1', '#13c2c2', '#fa541c' ]}
            legend={{ position: 'top' }}
            height={320}
            xAxis={{
              label: { style: { fill: '#888', fontSize: 13 } },
              title: { text: groupChartType==='day'?'日期':'月份', style: { fontWeight: 600, fontSize: 14 } },
            }}
            yAxis={{
              label: { style: { fill: '#888', fontSize: 13 } },
              title: { text: '金额', style: { fontWeight: 600, fontSize: 14 } },
            }}
            tooltip={{ formatter: (d: any) => ({ name: d.type, value: numeral(d.amount).format('0,0.##') }) }}
            label={false}
          />
        </div>
  {/* 已删除统计（本月/上月）Bar 图表区域 */}
      </Card>
      <Modal
        title={<span style={{ color: '#1890ff', fontWeight: 600 }}>新建订单</span>}
        open={modalOpen}
        onOk={handleCreate}
        onCancel={() => setModalOpen(false)}
        destroyOnHidden
        styles={{ body: { padding: 24, background: '#f6faff', borderRadius: 16 } }}
        style={{ borderRadius: 16 }}
        okButtonProps={{ style: { borderRadius: 20, fontWeight: 500 } }}
        cancelButtonProps={{ style: { borderRadius: 20, fontWeight: 500 } }}
      >
        <Form form={form} layout="vertical">
          <Form.Item name="name" label="名称" rules={[{ required: true }]}> 
            <Input placeholder="如：早餐、交通..." style={{ borderRadius: 12 }} />
          </Form.Item>
          <Form.Item name="type" label="类型" rules={[{ required: true }]}> 
            <Select options={ORDER_TYPES} placeholder="请选择类型" style={{ borderRadius: 12 }} />
          </Form.Item>
          <Form.Item name="amount" label="金额" rules={[{ required: true }]}> 
            <InputNumber style={{ width: '100%', borderRadius: 12 }} placeholder="请输入金额" min={0.01} step={0.01} />
          </Form.Item>
          <Form.Item name="currency" label="币种" initialValue="CNY" rules={[{ required: true }]}> 
            <Select options={CURRENCIES} style={{ borderRadius: 12 }} />
          </Form.Item>
          <Form.Item name="date" label="日期" rules={[{ required: true }]}> 
            <DatePicker style={{ width: '100%', borderRadius: 12 }} />
          </Form.Item>
          <Form.Item name="remark" label="备注"> 
            <Input placeholder="可填写备注" style={{ borderRadius: 12 }} />
          </Form.Item>
        </Form>
      </Modal>
      <div className="order-footer">Copyright © 2025 记账系统</div>
    </div>
  );
}
