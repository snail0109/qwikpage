import request from '@/utils/request';
import { PageParams, PageReqParams, CreatePageParams, PublishPageParams, PublishListParams } from './types';
import { mockApi } from './mockApi';
export default {
  // 获取页面列表
  getPageList(params: PageParams) {
    const mockPageListData = {
      list: [
        {
          id: 8553,
          name: '数据看板页面',
          userId: 3371,
          remark: '数据看板页面',
          isPublic: 2,
          isEdit: 2,
          previewImg: null,
          stgPublishId: 0,
          prePublishId: 0,
          prdPublishId: 0,
          stgState: 1,
          preState: 1,
          prdState: 1,
          projectId: 4356,
          updatedAt: '2024-12-27 14:23:55',
          userName: '1121988099',
        },
        {
          id: 8554,
          name: '用户列表',
          userId: 3371,
          remark: '',
          isPublic: 2,
          isEdit: 2,
          previewImg: null,
          stgPublishId: 0,
          prePublishId: 0,
          prdPublishId: 0,
          stgState: 1,
          preState: 1,
          prdState: 1,
          projectId: 4356,
          updatedAt: '2024-12-27 14:23:55',
          userName: '1121988099',
        },
      ],
      total: 2,
      pageSize: 12,
      pageNum: 1,
    };
    // return request.get('/page/list', params);
    return mockApi({ data: mockPageListData });
  },

  // 获取页面模板列表
  getPageTemplateList(params: Omit<PageParams, 'type'>) {
    return request.get('/page/getPageTemplateList', params);
  },
  // 获取页面详情
  getPageDetail(id: number) {
    return request.get(`/page/detail/${id}`);
  },

  // 复制页面数据
  copyPageData(params: PageReqParams) {
    return request.post('/page/copy', params);
  },

  // 删除页面数据
  delPageData(params: { id: number }) {
    return request.post('/page/delete', params);
  },

  // 创建页面数据
  createPageData(params: CreatePageParams) {
    return request.post('/page/create', params);
  },

  // 保存页面数据
  updatePageData(params: any) {
    return request.post('/page/update', params);
  },

  // 发布
  publishPage(params: PublishPageParams) {
    return request.post('/publish/create', params);
  },

  // 发布记录
  publishList(params: PublishListParams) {
    return request.post('/publish/list', params);
  },

  // 页面回滚
  rollbackPage(params: { pageId: number; env: string; lastPublishId: number }) {
    return request.post('/page/rollback', params);
  },
};
