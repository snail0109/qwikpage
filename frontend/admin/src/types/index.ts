/**
 * 菜单类型
 */

export interface IMenuItem {
  id: string;
  projectId: string;
  name: string;
  parentId: string;
  menuType: number;
  icon: string;
  path: string;
  pageId: string;
  sortNum: number;
  status: number;
  createdAt: string;
  updatedAt?: string;
  buttons?: IMenuItem[];
  children?: IMenuItem[];
}
