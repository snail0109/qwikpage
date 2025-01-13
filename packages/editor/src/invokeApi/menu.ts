import { invoke } from "@tauri-apps/api/core";

// 获取菜单列表
export const getMenuList = (params: any) => {
  return invoke("get_menu_list", params)
};

// 新增菜单
export const addMenu = (params: any) => {
  return invoke("add_menu", params)
};

// 删除菜单
export const delMenu = (params: any) => {
  return invoke("delete_menu", params)
};

// 更新菜单
export const updateMenu = (params: any) => {
  return invoke("update_menu", params)
};

// 复制菜单
export const copyMenu = (params: { projectId: string, id : string }) => {
  return invoke("copy_menu", params)
};


