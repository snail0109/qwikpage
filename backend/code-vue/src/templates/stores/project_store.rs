pub const PROJECT_STORE_INDEX: &str = r#"
import { defineStore } from 'pinia';
import { reactive } from 'vue';
import type { PageVariable } from '@/types';

export interface ProjectState {
  variables: PageVariable[];
  variableData: { [key: string]: any };
}

export const useProjectStore = defineStore('projectStore', () => {
  const projectState = reactive<ProjectState>({
    variables: [],
    variableData: {},
  });

  // 保存变量信息
  const setVariables = (variables: PageVariable[]) => {
    projectState.variables = variables;
  };

  // 保存变量数据
  const setVariableData = ({ name, value }: any) => {
    projectState.variableData[name] = value;
  };
  return {
    projectState,
    setVariables,
    setVariableData,
  };
})
"#;