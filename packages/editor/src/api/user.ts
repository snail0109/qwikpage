import request from '@/utils/request';
import { mockApi } from './mockApi';

// 用户登录
export const login = async <T>(params: T) => {
  return request.post('/user/login', params);
};

// 发送验证码
export const sendEmail = async (params: { email: string }) => {
  return request.post('/user/sendEmail', params);
};

// 邮箱注册
export const regist = async (params: { userName: string; code?: number; userPwd: string }) => {
  return request.post('/user/regist', params);
};

// 获取用户信息
export const getUserInfo = async () => {
  // return request.get('/user/info', {});
  const mockUserInfo = {
    userId: 3371,
    userName: '1121988099@qq.com',
    nickName: '1121988099',
    avatar: '',
    createdAt: '2024-12-27 14:22:43',
  };
  return mockApi({ data: mockUserInfo });
};

// 搜索用户
export const searchUser = (keyword: string) => {
  return request.post(`/user/search`, { keyword });
};
