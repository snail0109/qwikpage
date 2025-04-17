import { create } from 'zustand';
import { produce } from 'immer';
import { PageVariable } from '@materials/types';


export interface ProjectState {
  variables: PageVariable[];
  variableData: { [key: string]: any };
}

export interface ProjectAction {
  setVariables: (payload: PageVariable[]) => void;
  setVariableData: (payload: any) => void;
}

export const useProjectStore = create<ProjectState & ProjectAction>((set, get) => ({
  // 项目变量定义列表
  variables: [],
  // 项目变量数据
  variableData: {},
  // 初始化变量列表
  setVariables(payload: PageVariable[]) {
    set(
      produce((state) => {
        state.variables = payload;
      }),
    );
  },
  // 设置变量数据
  setVariableData({ name, value }: any) {
    set(
      produce((state) => {
        state.variableData[name] = value;
      }),
    );
  },
}));
