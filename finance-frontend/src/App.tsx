import React from 'react';
import { Layout, Menu, theme, Typography, Avatar } from 'antd';
import { UserOutlined, WalletOutlined, AppstoreOutlined } from '@ant-design/icons';
import OrderPage from './pages/Order';
import LoginPage from './pages/Login';
import RegisterPage from './pages/Register';

const { Header, Content, Sider } = Layout;
const { Title } = Typography;

const items = [
  { key: 'accounts', icon: <WalletOutlined />, label: '账户管理' },
  // 其它菜单项可后续补充
];

export default function App() {
  // 只保留订单页
  const [authed, setAuthed] = React.useState(() => !!sessionStorage.getItem('token'));
  const [showRegister, setShowRegister] = React.useState(false);
  const { token: { colorBgContainer } } = theme.useToken();

  if (!authed) {
    if (showRegister) {
      return <RegisterPage onRegister={() => { setShowRegister(false); setAuthed(true); }} />;
    }
    return <LoginPage onLogin={() => setAuthed(true)} onShowRegister={() => setShowRegister(true)} />;
  }

  return (
    <Layout style={{ minHeight: '100vh', background: 'linear-gradient(135deg, #e0e7ff 0%, #f0fdfa 100%)' }}>
      <Header style={{ display: 'flex', alignItems: 'center', background: 'rgba(24, 144, 255, 0.95)', boxShadow: '0 2px 8px #0001', zIndex: 10 }}>
        <Avatar size={40} icon={<AppstoreOutlined />} style={{ background: '#fff', color: '#1890ff', marginRight: 16 }} />
        <Title level={3} style={{ color: '#fff', margin: 0, flex: 1 }}>个人理财/资产管理系统</Title>
        <Avatar size={36} icon={<UserOutlined />} style={{ background: '#f0f2f5', color: '#1890ff', marginLeft: 16 }} />
      </Header>
      <Layout>
        <Content style={{ margin: 32, background: '#fff', borderRadius: 16, boxShadow: '0 4px 24px #0001', minHeight: 400, padding: 32 }}>
          <OrderPage 
            onLogout={() => { sessionStorage.removeItem('token'); window.location.reload(); }}
            onSwitchUser={() => { sessionStorage.removeItem('token'); window.location.reload(); }}
          />
        </Content>
      </Layout>
    </Layout>
  );
}

