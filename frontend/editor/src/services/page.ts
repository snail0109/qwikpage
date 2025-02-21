import { invoke } from "@tauri-apps/api/core";
import { IPage } from "@/types";
import { cmd_invoke } from "./cmd_invoke";

export const pageService = {
    // 获取页面列表
    getPageList(params: { keyword?: string, projectId?: string; pageNum: number; pageSize: number }): Promise<{ list: IPage[], total: number }> {
        return invoke("get_page_list", params);
    },

    // 获取页面详情
    getPageDetail(params: { id: string, projectId: string }) {
        return invoke("get_page_detail_with_id", params);
    },

    // 复制页面数据
    copyPageData(params: Partial<IPage>) {
        return cmd_invoke("copy_page", { params });
    },

    // 删除页面数据
    delPageData(params: { id: string, projectId: string }) {
        return cmd_invoke("delete_page", params);
    },

    // 创建页面数据
    createPageData(params: Partial<IPage>) {
        return cmd_invoke("add_page", { params });
    },

    // 保存页面数据
    updatePageData(params: Partial<IPage>) {
        return cmd_invoke("update_page", { params });
    },

};
