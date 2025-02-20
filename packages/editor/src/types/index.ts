export interface IProject {
  id: string;
  name: string;
  logo: string;
  remark: string;
  count: number;
}
export interface IMenuItem {
  id: string;
  projectId: string;
  name: string;
  parentId: string;
  menuType: number;
  icon: string;
  path: string;
  pageId: string;
  pageName: string;
  sortNum: number;
  status: number;
  createdAt: string;
  buttons?: IMenuItem[];
  children?: IMenuItem[];
}

export interface IPage {
  id: string,
  name: string,
  path: string,
  remark?: string,
  projectId: string,
  project_id: string,
  pageData?: string,
  updatedAt?: string,
  previewImg?: string,
}

export interface PaginationInfo {
  pageNum: number,
  pageSize: number,
}

export interface MenuEditParams {
  id?: number;
  name: string;
  parent_id?: string;
  code?: string;
  project_id?: number;
  sort_num: string;
}