import axios, { AxiosError } from 'axios';
import { message } from './AntdGlobal';
import router from '@/router';

const ErrorMsg = '服务异常，请稍后再试';

/**
 * 创建实例
 */
const instance = axios.create({
  timeout: 8000,
  timeoutErrorMessage: '请求超时，请稍后再试',
  withCredentials: true,
  headers: {},
});

// 请求拦截
instance.interceptors.request.use(
  (config) => {
    config.baseURL = import.meta.env.VITE_BASE_API;
    return config;
  },
  (error: AxiosError) => {
    return Promise.reject(error);
  },
);

// 响应拦截
instance.interceptors.response.use(
  async (response) => {
    const res: any = await response.data;
    if (!res) {
      message.error(ErrorMsg);
      return Promise.reject(ErrorMsg);
    }
    return res;
  },
  (error) => {
    if (error.response && error.response.status === 403) {
      router.navigate('/403');
    } else {
      message.error(error.message || ErrorMsg);
    }
    return Promise.reject(error.message);
  },
);

// 调用函数导出
export default {
  get<R = any>(url: string, params?: any): Promise<R> {
    return instance.get(url, {
      params,
    });
  },
  post<R = any>(url: string, params?: any): Promise<R> {
    return instance.post(url, params);
  },
};
