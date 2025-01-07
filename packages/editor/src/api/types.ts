export interface PageItem {
  id: number;
  name: string;
  // 其他必要的页面属性
}

export namespace Menu {
  export interface MenuItem {
    id: string;
    name: string;
    parentId: string;
    type: number;
  }

  export interface EditParams {
    id?: number;
    name: string;
    parentId?: string;
    code?: string;
    projectId?: number;
  }
}
