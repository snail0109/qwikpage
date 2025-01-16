import { create } from 'zustand';
import _ from 'lodash-es';
import { IMenuItem } from '@/types/index';

export interface ProjectInfo {
  id?: string;
  footer?: number;
  layout?: number;
  logo?: string;
  menuMode: 'inline' | 'horizontal' | 'vertical';
  menuThemeColor?: string;
  name: string;
  systemThemeColor?: string;
  breadcrumb?: boolean;
  tag?: boolean;
}

export interface PageState {
  projectInfo: ProjectInfo;
  collapsed: boolean;
  menuTree: any[];
  buttons: any[];
  pageMap: { [key: string]: IMenuItem };
  menuMap: { [key: string]: IMenuItem };
}

export interface ProjectAction {
  setProjectInfo: (payload: any) => void;
  updateCollapsed: () => void;
}

export const useProjectStore = create<PageState & ProjectAction>((set, get) => ({
  projectInfo: {
    name: '',
    menuMode: 'inline',
  },
  collapsed: false,
  menuTree: [],
  buttons: [],
  pageMap: {},
  menuMap: {},
  setProjectInfo: ({ projectInfo, menuTree, buttons, pageMap, menuMap }: any) => {
    set({
      projectInfo,
      menuTree,
      buttons,
      pageMap,
      menuMap,
    });
  },
  updateCollapsed: () => {
    set((state) => {
      return {
        collapsed: !state.collapsed,
      };
    });
  },
}));