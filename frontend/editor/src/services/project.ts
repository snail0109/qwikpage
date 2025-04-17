import { invoke } from "@tauri-apps/api/core";
import { cmd_invoke } from "./cmd_invoke";

export const projectService = {
  getProjectList(params: any): Promise<any> {
    return invoke("get_project_list", params);
  },

  // 新增项目
  addProject(params: any) {
    return cmd_invoke("add_project", { params });
  },

  // 更新项目
  updateProject(params: any) {
    return cmd_invoke("update_project", { params });
  },

  // 删除项目
  delProject(params: { id: string; groupId?: string; logoUrl: string }) {
    return cmd_invoke("delete_project", params);
  },

  // 获取项目详情
  getProjectDetail(id: string): Promise<any> {
    return cmd_invoke("get_project_detail", { id });
  },

  // 修改项目变量（增删改）
  updateProVariables(params: { id: string; variables: string }) {
    return cmd_invoke("update_project_variables", params);
  },
};
