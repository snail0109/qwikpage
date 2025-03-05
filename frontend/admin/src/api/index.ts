import request from '@/utils/request';

// 获取页面详情
export const getPageDetail = (pageId: string) => {
  return request.get(`/page/detail/${pageId}`);
};

// 根据projectId和page path获取页面详情
export const getPageDetailWithPath = (projectId: string, pageUrl: string) => {
  return request.get(`/page/detail/${projectId}/${pageUrl || "*"}`);
}


// 获取项目配置
export const getProjectDetail = (projectId: string) => {
  return request.get(`/project/detail/${projectId}`);
};
