import request from '@/utils/request';

// 获取页面详情
export const getPageDetail = (projectId: number, id: number) => {
  return request.get(`/admin/page/detail/${id}?projectId=${projectId}`);
};

// 获取项目对应的菜单列表
export const getProjectMenu = (projectId: string | number) => {
  return request.get('/admin/menu/list/' + projectId);
};

// 获取项目配置
export const getProjectDetail = (projectId: string | number) => {
  return request.get('/admin/getProjectConfig?projectId=' + projectId);
};
