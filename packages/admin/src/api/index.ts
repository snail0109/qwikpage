import request from '@/utils/request';

// 获取页面详情
export const getPageDetail = (pageId: string) => {
  return request.get(`/page/detail/${pageId}`);
};

// 获取菜单详情
// export const getMenuDetail = (projectId: string, menuId: string) => {
//   return request.get(`/menu/detail/${projectId}/${menuId}`);
// }

// 获取项目对应的菜单列表
export const getProjectMenu = (projectId: string) => {
  return request.get(`/project/menus/${projectId}`);
};

// 获取项目配置
export const getProjectDetail = (projectId: string) => {
  return request.get(`/project/detail/${projectId}`);
};
