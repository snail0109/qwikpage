import { invoke } from "@tauri-apps/api/core";

export const menuService =  {
    // 获取菜单列表
    getMenuList(params: any) : Promise<any> {
        return invoke("get_menu_list", params);
    },
    // 新增菜单
    addMenu(params: any) : Promise<any> {
        return invoke("add_menu", params);
    },
    // 删除菜单
    delMenu(params: any) : Promise<any> {
        return invoke("delete_menu", params);
    },
    // 更新菜单
    updateMenu(params: any) : Promise<any> {
        return invoke("update_menu", params);
    },
    // 复制菜单
    copyMenu(params: any) : Promise<any> {
        return invoke("copy_menu", params);
    },
}




