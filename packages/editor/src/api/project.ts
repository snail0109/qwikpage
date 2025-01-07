import { invoke } from "@tauri-apps/api/core";

export default {

  getProjectList(params: any) {
    return invoke("get_project_list", params)
  },

  // 新增项目
  addProject(params: any) {
    return invoke("add_project", params)
  },

  // 更新项目
  updateProject(params: any) {
    return invoke("get_project_list", params)
  },

  // 删除项目
  delProject(params: { id: number; type?: string }) {
    return invoke("get_project_list", params)
  },

  // 获取项目详情
  getProjectDetail(id: number) {
    return invoke("get_project_detail", { id})
  },

};
