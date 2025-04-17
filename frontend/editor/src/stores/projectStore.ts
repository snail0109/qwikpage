import { create } from 'zustand';
import { produce } from 'immer';
import { PageVariable } from '@/packages/types';


export interface ProjectState {
  variables: PageVariable[];
  variableData: { [key: string]: any };
}

export interface ProjectAction {
  setVariables: (payload: PageVariable[]) => void;
  addVariable: (payload: PageVariable) => void;
  editVariable: (payload: PageVariable) => void;
  removeVariable: (name: string) => void;
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
  // 添加变量
  addVariable(payload: PageVariable) {
    set(
      produce((state) => {
        state.variables.push(payload);
      }),
    );
  },
  // 更新变量
  editVariable(payload: PageVariable) {
    set(
      produce((state) => {
        const index = state.variables.findIndex((item: PageVariable) => item.name == payload.name);
        if (index > -1) {
          state.variableData[payload.name] = payload.defaultValue;
          state.variables[index] = payload;
        }
      }),
    );
  },
  // 删除变量
  removeVariable(name: string) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        state.variables = state.variables.filter((item: PageVariable) => item.name !== name);
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
