import request from '@/utils/request';
import { invoke } from "@tauri-apps/api/core";

export default {
  // 获取页面列表
  getPageList(params: any) {
    return invoke("get_page_list", params)
  },

  // 获取页面详情
  getPageDetail(id: number) {
    return invoke("get_page_detail", { id })
  },

  // 复制页面数据
  copyPageData(params: any) {
    return request.post('/pages/copy', params);
  },

  // 删除页面数据
  delPageData(params: { id: number }) {
    return request.post('/pages/delete', params);
  },

  // 创建页面数据
  createPageData(params: any) {
    return request.post('/pages/create', params);
  },

  // 保存页面数据
  updatePageData(params: any) {
    return request.post('/pages/update', params);
  },

  // 发布
  publishPage(params: any) {
    return request.post('/page/publish/create', params);
  },

  // 发布记录
  publishList(params: any) {
    return request.post('/page/publish/list', params);
  },

  // 页面回滚
  rollbackPage(params: any) {
    return request.post('/pages/rollback', params);
  },
};
