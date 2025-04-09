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