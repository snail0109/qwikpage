import request from '@/utils/request';
import { ProjectCreateParams, ProjectUpdateParams, PageParams } from './types';
import { mockApi } from './mockApi';
export default {
  checkAuth(params: { id: number }) {
    return request.post('/project/checkAuth', params);
  },
  getCategoryList(params: PageParams) {
    // return request.get('/project/category', params);
    const mockCategoryList = {
      list: [
        {
          id: 4356,
          name: '基础管理系统',
          remark: 'Mars基础管理系统提供高效的企业级管理功能，支持数据可视化、数据管理，助力企业精细化管理。',
          userId: 3371,
          userName: '1121988099@qq.com',
          logo: 'https://marsview.cdn.bcebos.com/mars-logo.png',
          updatedAt: '2024-12-27 14:23:55',
          count: 2,
        },
      ],
      total: 1,
      pageSize: 12,
      pageNum: 1,
    };
    return mockApi({ data: mockCategoryList });
  },

  // 新增项目
  addProject(params: ProjectCreateParams) {
    return request.post('/project/create', params);
  },

  // 删除项目
  delProject(params: { id: number; type?: string }) {
    return request.post('/project/delete', params);
  },

  // 获取项目详情
  getProjectDetail(id: number) {
    return request.get(`/project/detail/${id}`, {});
  },

  // 更新项目
  updateProject(params: ProjectUpdateParams) {
    return request.post('/project/update', params);
  },
};
