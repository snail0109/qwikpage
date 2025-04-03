pub const PAGE_STORE_INDEX: &str = r#"
import { defineStore } from 'pinia';
import { reactive } from 'vue';
import type { CSSProperties } from 'vue';
import type { ComponentType, ApiType, PageVariable, EventType, ComItemType } from '@/types';

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
        style: CSSProperties;
        scopeCss: string;
        scopeStyle: CSSProperties;
        events: EventType[];
        api: {
          sourceType: string;
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
      formData: { [key: string | number]: any };
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

const initPage = () => ({
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
})
export const usePageStore = defineStore('pageStore', () => {
  const pageState = reactive<PageState>({
    page: initPage(),
  });

  // 保存页面信息
  const savePageInfo = (payload: any) => {
    const { elementsMap } = payload.pageData || {};
    const formData: any = {};
    Object.keys(elementsMap).forEach(key => {
      if (elementsMap[key].type === 'Form') {
        formData[elementsMap[key].id] = {};
      }
    });
    payload.pageData.formData = formData;
    pageState.page = payload;
  }

  // 保存变量信息
  const setVariableData = ({ name, value }: any) => {
    pageState.page.pageData.variableData[name] = value;
  }

  // 保存表单数据信息
  const setFormData = ({ name, value, type }: any) => {
    if (type === 'override') {
      pageState.page.pageData.formData[name] = value;
    } else {
      Object.keys(value).forEach(key => {
        pageState.page.pageData.formData[name][key] = value[key]
      });
    }
  }

  // 保存普通表单数据信息
  const setFormItemData = ({ name, value }: any) => {
    pageState.page.pageData.formItemData[name] = value;
  }

  // 清除页面信息
  const clearPageInfo = () => {
    pageState.page = initPage()
  }

  return {
    pageState,
    savePageInfo,
    setVariableData,
    setFormData,
    setFormItemData,
    clearPageInfo,
  }
})
"#;