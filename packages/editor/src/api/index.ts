import request from '@/utils/request';

// 获取菜单列表
export const getMenuList = (params: any) => {
  return request.post('/project/menu/list', params);
};

// 新增菜单
export const addMenu = (params: any) => {
  return request.post('/project/menu/create', params);
};

// 删除菜单
export const delMenu = (params: { id: number }) => {
  return request.post('/project/menu/delete', params);
};

// 更新菜单
export const updateMenu = (params: any) => {
  return request.post('/project/menu/update', params);
};

// 复制菜单
export const copyMenu = (params: { id: number }) => {
  return request.post('/project/menu/copy', params);
};


