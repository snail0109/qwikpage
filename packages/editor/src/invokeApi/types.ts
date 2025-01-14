// FIXME
export interface PageItem {
    id: number;
    name: string;
    // 其他必要的页面属性
}

// FIXME
export namespace Menu {
    export interface MenuItem {
        id: string;
        name: string;
        parent_id: string;
        menu_type: string;
        project_id: string;
        sort_num: string;
    }

    export interface EditParams {
        id?: number;
        name: string;
        parent_id?: string;
        code?: string;
        project_id?: number;
        sort_num: string;
    }
}

// FIXME
export interface CreatePageParams {
    id?: string;
    name: string;
    project_id?: string;
    // 其他必要的页面属性
}

export namespace Project {
    export interface CreateParams {
        name: string;
    }

    export interface ProjectItem {
        id: number;
        name: string;
        logo: string;
        remark: string;
    }
}

// 新类型从下面开始

export interface Page {
    id: string,
    name: string,
    remark?: string,
    projectId: string,
    pageData?: string
}