import request from "@/utils/request";
import { invoke } from "@tauri-apps/api/core";
import { Page } from "@/invokeApi/types";

export default {
    // 获取页面列表
    getPageList(params: { keyword?: string, projectId?: string; pageNum: number; pageSize: number }) : Promise<{ list: Page[], total: number }> {
        return invoke("get_page_list", params);
    },

    // 获取页面详情
    getPageDetail(params: { id: string }) {
        return invoke("get_page_detail", params);
    },

    // 复制页面数据
    copyPageData(params: Partial<Page>) {
        return invoke("copy_page", params);
    },

    // 删除页面数据
    delPageData(params: { id: string }) {
        return invoke("delete_page", params);
    },

    // 创建页面数据
    createPageData(params: Partial<Page>) {
        return invoke("add_page", params);
    },

    // 保存页面数据
    updatePageData(params: Partial<Page>) {
        return invoke("update_page", params);
    },

};
