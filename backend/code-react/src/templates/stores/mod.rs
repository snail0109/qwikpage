use crate::FileTemplate;

pub const PAGE_STORE: &str = r#"
import { create } from 'zustand';
import { produce } from 'immer';
import { ComponentType, ApiType, PageVariable, EventType, ComItemType } from '@/types';
export interface PageState {
  page: {
    id: string;
    name: string;
    remark: string;
    projectId: string;
    previewImg?: string;
    userId: number;
    userName: string;
    pageData: {
      // 页面配置数据
      config: {
        props: any;
        // 页面综合样式(scopeCss + scopeStyle)
        style: React.CSSProperties;
        scopeCss: string;
        scopeStyle: React.CSSProperties;
        events: EventType[];
        api: {
          sourceType: 'json' | 'api';
          id: string;
          source: any;
          sourceField: string | { type: 'variable' | 'static'; value: string };
        };
      };
      events: Array<{ name: string; value: string }>;
      // 页面全局接口
      apis: { [key: string]: ApiType };
      elements: ComItemType[];
      elementsMap: { [key: string]: ComponentType };
      // 页面变量
      variables: PageVariable[];
      variableData: { [key: string]: any };
      // 表单数据
      formData: { [key: string]: any };
      // 表单控件数据
      formItemData: { [key: string]: any };
      // 全局拦截器
      interceptor: {
        headers?: {
          key: string;
          value: string;
        }[];
        timeout: number;
        timeoutErrorMessage: string;
        requestInterceptor?: string;
        responseInterceptor?: string;
      };
    };
  };
}
export interface PageAction {
  savePageInfo: (pageInfo: any) => void;
  setVariableData: (payload: any) => void;
  setFormData: (payload: any) => void;
  setFormItemData: (payload: any) => void;
  clearPageInfo: () => void;
}
export const usePageStore = create<PageState & PageAction>((set) => ({
  selectedElement: undefined,
  page: {
    id: "0",
    name: '',
    remark: '',
    projectId: "0",
    userId: 0,
    userName: '',
    previewImg: '',
    pageData: {
      config: {
        props: {},
        style: {},
        scopeCss: '',
        scopeStyle: {},
        events: [],
        api: {
          sourceType: 'json',
          id: '',
          source: {},
          sourceField: '',
        },
      },
      events: [],
      // 页面全局接口
      apis: {},
      elements: [],
      elementsMap: {},
      // 页面变量定义列表
      variables: [],
      // 页面变量数据
      variableData: {},
      // 表单数据
      formData: {},
      // 表单控件数据
      formItemData: {},
      // 全局拦截器
      interceptor: {
        headers: [{ key: '', value: '' }],
        timeout: 8,
        timeoutErrorMessage: '请求超时，请稍后再试',
      },
    },
  },
  // 保存页面信息
  savePageInfo: (payload: any) =>
    set(
      produce((state) => {
        state.page = Object.assign(state.page, payload);
      }),
    ),
  setVariableData({ name, value }: any) {
    set(
      produce((state) => {
        state.page.pageData.variableData[name] = value;
      }),
    );
  },
  setFormData({ name, value, type }: any) {
    set(
      produce((state) => {
        if (type === 'override') {
          state.page.pageData.formData[name] = value;
        } else {
          state.page.pageData.formData[name] = { ...state.page.pageData.formData[name], ...value };
        }
      }),
    );
  },
  setFormItemData({ name, value }: any) {
    set(
      produce((state) => {
        state.page.pageData.formItemData[name] = value;
      }),
    );
  },
  // 清除页面信息
  clearPageInfo() {
    set(
      produce((state) => {
        state.page = {
          id: 0,
          name: '',
          remark: '',
          projectId: 0,
          userId: 0,
          userName: '',
          previewImg: '',
          pageData: {
            config: {
              props: {},
              style: {},
              scopeCss: '',
              scopeStyle: {},
              events: [],
              api: {
                sourceType: 'json',
                id: '',
                source: {},
                sourceField: '',
              },
            },
            events: [],
            // 页面全局接口
            apis: {},
            elements: [],
            elementsMap: {},
            // 页面变量定义列表
            variables: [],
            // 页面变量数据
            variableData: {},
            // 表单数据
            formData: {},
            // 全局拦截器
            interceptor: {
              headers: [{ key: '', value: '' }],
              timeout: 8,
              timeoutErrorMessage: '请求超时，请稍后再试',
            },
          },
        };
      }),
    );
  },
}));
"#;


pub fn get_store() -> Vec<FileTemplate> {
    let mut files = vec![];
    files.push(FileTemplate {
        filename: String::from("src/stores/pageStore.ts"),
        content: String::from(PAGE_STORE),
    });
    return files;
}