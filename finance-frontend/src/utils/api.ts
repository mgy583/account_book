import axios from 'axios';

// 获取 token 的工具函数（可根据实际登录逻辑调整）
export function getToken() {
  return sessionStorage.getItem('token') || '';
}

// 创建 axios 实例，自动加上 Authorization 头
// 注意：所有业务接口都需加模块前缀，如 /user/login、/order/xxx、/order_query/orders/query
const api = axios.create({
  baseURL: '/api',
});

api.interceptors.request.use(config => {
  const token = getToken();
  if (token) {
    config.headers = config.headers || {};
    config.headers['Authorization'] = `Bearer ${token}`;
  }
  return config;
});

export default api;
