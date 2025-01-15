export interface Page {
    id: string,
    name: string,
    remark?: string,
    projectId: string,
    pageData?: string,
    updatedAt?: string,
    previewImg?: string,
}

export interface PaginationInfo {
    pageNum: number,
    pageSize: number,
}

export interface Project {
    id: string;
    name: string;
    logo: string;
    remark: string;
    count: number;
}

export interface MenuItem {
    id: string;
    name: string;
    parentId: string;
    menuType: number;
    projectId: string;
    sortNum: string;
}

export interface EditParams {
    id?: number;
    name: string;
    parent_id?: string;
    code?: string;
    project_id?: number;
    sort_num: string;
}