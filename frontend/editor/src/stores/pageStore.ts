import { create } from 'zustand';
import { produce } from 'immer';
import { ComponentType, ApiType, PageVariable, EventType, ComItemType } from '@/packages/types';
import { cloneDeep } from 'lodash-es';
import { createId, getElement, judgeIfInForm, COLUMN_MAP } from '@/utils/util';
import { merge } from 'lodash-es';
import storage from '@/utils/storage';

/**
 * 处理元素ID变更的函数
 */
function handleElementIdChange(state: PageState, oldId: string, newId: string) {
  // 复制元素配置到新ID
  state.page.pageData.elementsMap[newId] = { ...state.page.pageData.elementsMap[oldId] };
  state.page.pageData.elementsMap[newId].id = newId;

  // 递归更新elements中的ID及其引用
  const updateElementsRecursive = (elements: ComponentType[]) => {
    for (let i = 0; i < elements.length; i++) {
      // 更新元素自身ID
      if (elements[i].id === oldId) {
        elements[i].id = newId;
      }

      // 更新parentId引用
      if (elements[i].parentId === oldId) {
        elements[i].parentId = newId;
      }

      // 更新inForm引用
      if (elements[i].inForm === oldId) {
        elements[i].inForm = newId;
      }

      // 递归处理子元素
      if (elements[i].elements && elements[i].elements.length > 0) {
        updateElementsRecursive(elements[i].elements);
      }
    }
  };

  updateElementsRecursive(state.page.pageData.elements);

  // 更新所有元素的parentId和inForm引用
  Object.keys(state.page.pageData.elementsMap).forEach(key => {
    const element = state.page.pageData.elementsMap[key];
    if (element.parentId === oldId) {
      element.parentId = newId;
    }

    if (element.inForm === oldId) {
      element.inForm = newId;
    }
  });

  // 删除旧的元素配置
  delete state.page.pageData.elementsMap[oldId];

  // 如果当前选中的元素是被修改的元素，更新selectedElement
  if (state.selectedElement && state.selectedElement.id === oldId) {
    state.selectedElement.id = newId;
  }

  return newId; // 返回新ID，方便后续使用
}

/**
 * 页面信息存储
 */

export interface PageState {
  mode: 'edit' | 'preview';
  theme: 'light' | 'dark';
  selectedElement: { type: string; id: string } | undefined;
  isUpdateToolbar: boolean; // 更新遮罩
  isEdit: boolean; // 是否编辑了页面
  canvasWidth: string; // 画布宽度
  page: {
    id: string;
    name: string;
    path: string;
    remark: string;
    projectId: string;
    isPublic: 1 | 2;
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
      elements: ComponentType[];
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
  currentTab: string;
}
export interface PageAction {
  savePageInfo: (pageInfo: any) => void;
  updateEditState: (isEdit: boolean) => void;
  addApi: (api: ApiType) => void;
  updateApi: (api: ApiType) => void;
  removeApi: (name: string) => void;
  setMode: (mode: 'edit' | 'preview') => void;
  setTheme: (theme: 'light' | 'dark') => void;
  updateCanvasWidth: (width: string) => void; // 更新画布宽度
  addElement: (element: any) => void;
  addChildElements: (element: any) => void;
  editElement: (payload: any) => void;
  editTableProps: (payload: any) => void;
  editEvents: (payload: any) => void;
  moveElements: (payload: any) => void;
  setSelectedElement: (payload: any) => void;
  removeElements: (payload: any) => void;
  updateGridChildren: (payload: any) => void;
  dragSortElements: (payload: any) => void;
  addVariable: (payload: PageVariable) => void;
  editVariable: (payload: PageVariable) => void;
  removeVariable: (name: string) => void;
  setVariableData: (payload: any) => void;
  setFormData: (payload: any) => void;
  setFormItemData: (payload: any) => void;
  setInterceptor: (payload: any) => void;
  updateToolbar: () => void;
  clearPageInfo: () => void;
  setCurrentTab: (tab: string) => void;
}
export const usePageStore = create<PageState & PageAction>((set) => ({
  mode: 'edit',
  // 是否编辑了页面
  isEdit: false,
  theme: 'light',
  selectedElement: undefined,
  isUpdateToolbar: false,
  canvasWidth: storage.get('canvasWidth') || 'auto',
  page: {
    id: "0",
    name: '',
    path: '',
    remark: '',
    projectId: "0",
    isPublic: 2,
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
        state.isEdit = true; // 标记为编辑状态
        if (payload.type === 'props') {
          state.page.pageData.config.props = payload.props;
        } else if (payload.type === 'style') {
          // 如果是style，则直接更新
          state.page.pageData.config.scopeCss = payload.scopeCss;
          state.page.pageData.config.scopeStyle = payload.scopeStyle;
          state.page.pageData.config.style = payload.style;
        } else if (payload.type === 'events') {
          state.page.pageData.config.events = payload.events || [];
        } else if (payload.type === 'api') {
          state.page.pageData.config.api = payload.api;
        } else {
          state.isEdit = false; // 标记为编辑状态
          state.page = merge({}, state.page, payload);
        }
      }),
    ),
  updateEditState: (isEdit: boolean) => {
    set(
      produce((state) => {
        state.isEdit = isEdit;
      }),
    );
  },
  addApi: (api) => {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        state.page.pageData.apis[api.id] = api;
      }),
    );
  },
  updateApi: (api) => {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        Object.assign(state.page.pageData.apis[api.id], api);
      }),
    );
  },
  removeApi: (id) => {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        delete state.page.pageData.apis[id];
      }),
    );
  },
  // 切换编辑模式
  setMode: (mode: 'edit' | 'preview') => set({ mode }),
  // 切换主题
  setTheme: (theme: 'light' | 'dark') => set({ theme }),
  // 更新画布宽度
  updateCanvasWidth: (width: string) => {
    storage.set('canvasWidth', width);
    set({ canvasWidth: width });
  },
  // 添加组件
  addElement: (element: ComponentType) => {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        state.page.pageData.elements.push({
          id: element.id,
          parentId: element.parentId,
          type: element.type,
          name: element.name,
          elements: element.elements?.map((item) => ({ id: item.id, parentId: element.id, type: item.type, name: item.name })) || [],
        });
        const childElement = cloneDeep({
          ...element,
          elements: undefined,
          remoteUrl: element.remoteUrl,
          remoteConfigUrl: element.remoteConfigUrl,
          remoteCssUrl: element.remoteCssUrl,
        });
        // 添加当前组件对象
        state.page.pageData.elementsMap[element.id] = childElement;
        // 添加子组件对象
        element.elements?.map((item) => {
          state.page.pageData.elementsMap[item.id] = item;
        });
        // 判断当前元素是否为表单内控件
        if (element.config.props.formItem) {
          const inForm = judgeIfInForm(childElement.id, state.page.pageData.elementsMap);
          childElement.inForm = inForm;
          state.page.pageData.elements[state.page.pageData.elements.length - 1].inForm = inForm;
          if (inForm) {
            // 为表单内控件添加name属性
            childElement.config.props.formItem.name = createId(element.type, 6);
          } else {
            delete childElement.config.props.formItem.name;
          }
        }
      }),
    );
  },
  // 添加子组件
  addChildElements(element: ComponentType) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        function deepFind(list: ComItemType[]) {
          for (let i = 0; i < list.length; i++) {
            const item = list[i];
            // 根据parentId先找到当前组件的父组件
            if (item.id == element.parentId) {
              if (item.elements === undefined) {
                item.elements = [];
              }
              item.elements.push({
                id: element.id,
                parentId: element.parentId,
                type: element.type,
                name: element.name,
                elements:
                  element.elements?.map((item) => ({ id: item.id, parentId: element.id, type: item.type, name: item.name, elements: [] })) || [],
              });
              const childElement = cloneDeep({
                ...element,
                elements: undefined,
                remoteUrl: element.remoteUrl,
                remoteConfigUrl: element.remoteConfigUrl,
                remoteCssUrl: element.remoteCssUrl,
              });
              // 添加当前组件对象
              state.page.pageData.elementsMap[element.id] = childElement;
              // 添加子组件对象
              element.elements?.map((item) => {
                state.page.pageData.elementsMap[item.id] = item;
              });
              // 判断当前元素是否为表单内控件
              if (element.type !== 'FormItem' && element.config.props.formItem) {
                const inForm = judgeIfInForm(childElement.id, state.page.pageData.elementsMap);
                childElement.inForm = inForm;
                item.elements[item.elements.length - 1].inForm = inForm;
                if (inForm) {
                  // 为表单内控件添加name属性
                  childElement.config.props.formItem.name = createId(element.type, 6);
                } else {
                  delete childElement.config.props.formItem.name;
                }
              }
              break;
            } else if (item.elements?.length) {
              deepFind(item.elements);
            }
          }
          return list;
        }
        deepFind(state.page.pageData.elements);
        state.isUpdateToolbar = !state.isUpdateToolbar;
      }),
    );
  },
  // 更新组件属性、样式
  editElement(payload: any) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态

        // 处理ID变更
        if (payload.type === 'props' && payload.props.id && payload.props.id !== payload.id) {
          const oldId = payload.id;
          const newId = payload.props.id;

          // 调用ID变更处理函数
          handleElementIdChange(state, oldId, newId);

          // 从props中删除id字段，避免后续处理再次使用
          delete payload.props.id;

          // 更新payload.id为新ID，以便后续处理使用新ID
          payload.id = newId;
        }

        const item = state.page.pageData.elementsMap[payload.id];
        // 属性修改
        if (payload.type === 'props') {
          item.config.props = payload.props;
          // Tabs标签对象需要同步属性值到Tab组件中
          if (item.type === 'Tabs') {
            const { element: parentItem } = getElement(state.page.pageData.elements, payload.id);
            if (parentItem?.elements.length === payload.props.items.length) {
              parentItem?.elements.map((item, index) => {
                Object.assign(state.page.pageData.elementsMap[item.id].config.props, payload.props.items[index]);
              });
            } else {
              parentItem?.elements.map((item, index) => {
                if (!payload.props.items.find((prop: { id: string }) => prop.id === item.id)) {
                  parentItem?.elements.splice(index, 1);
                  delete state.page.pageData.elementsMap[item.id];
                  // 递归删除相互引用的嵌套父子组件
                  const deepRemove = (id: string) => {
                    // 删除子组件
                    Object.values(state.page.pageData.elementsMap).map((item: any) => {
                      if (item.parentId == id) {
                        delete state.page.pageData.elementsMap[item.id];
                        deepRemove(item.id);
                      }
                      return item;
                    });
                  };
                  deepRemove(item.id);
                }
              });
            }
          }
          // Tab对象需要同步属性值到Tabs组件中
          if (item.type === 'Tab') {
            state.page.pageData.elementsMap[item.parentId].config.props.items.map((item: { key: string; label: string }) => {
              if (item.key === payload.id) {
                item.label = payload.props.label;
              }
            });
          }
        } else if (payload.type === 'style') {
          // 如果是style，则直接更新
          item.config.scopeCss = payload.scopeCss;
          item.config.scopeStyle = payload.scopeStyle;
          item.config.style = payload.style;
        } else if (payload.type === 'events') {
          item.config.events = payload.events || [];
        } else if (payload.type === 'api') {
          item.config.api = payload.api;
          if (payload.api.sourceType !== 'api') {
            item.config.api.id = undefined;
          } else {
            // 如果ID存在，更新一下数据源字段即可。
            if (payload.api.id) {
              state.page.pageData.apis[payload.api.id].sourceField = payload.api.sourceField;
            }
          }
        }
        state.isUpdateToolbar = !state.isUpdateToolbar;
      }),
    );
  },
  // 更新表格列表的操作按钮
  editTableProps(payload: any) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        const item = state.page.pageData.elementsMap[payload.id];
        if (payload.type === 'column') {
          if (!item.config.props.columns) item.config.props.columns = [];
          item.config.props.columns[payload.index] = payload.props;
        } else if (payload.type === 'formTable') {
          if (!item.config.props.formWrap.columns) item.config.props.formWrap.columns = [];
          item.config.props.formWrap.columns[payload.index] = payload.props;
        } else if (payload.type === 'items') {
          if (!item.config.props.items) item.config.props.items = [];
          item.config.props.items[payload.index] = payload.props;
        } else {
          if (!item.config.props.bulkActionList) item.config.props.bulkActionList = [];
          item.config.props.bulkActionList[payload.index] = payload.props;
        }
        state.isUpdateToolbar = !state.isUpdateToolbar;
      }),
    );
  },
  // 更新组件事件，比如表格中动态新加的按钮，需要更新事件
  editEvents(payload: any) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        const item = state.page.pageData.elementsMap[payload.id];
        item.events = payload.events;
      }),
    );
  },
  // 组件排序
  moveElements(payload: any) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        const { componentId, direction } = payload;
        function deepFind(list: ComponentType[]) {
          for (let index = 0; index < list.length; index++) {
            const item = list[index];
            if (item.id == componentId) {
              if (direction === 'up' && index > 0) {
                [list[index], list[index - 1]] = [list[index - 1], list[index]];
              } else if (direction === 'down' && list.length - 1 > index) {
                [list[index], list[index + 1]] = [list[index + 1], list[index]];
              }
              break;
            } else if (item.elements?.length) {
              deepFind(item.elements);
            }
          }
        }
        deepFind(state.page.pageData.elements);
      }),
    );
  },
  // 组件大纲拖拽排序
  dragSortElements({ id, list, parentId }: any) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        state.page.pageData.elements = list;
        state.page.pageData.elementsMap[id].parentId = parentId;
      }),
    );
  },
  // 设置选中的组件列表
  setSelectedElement(payload: any) {
    set(() => {
      return { selectedElement: payload };
    });
  },
  removeElements(payload: any) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        const id = payload;
        function deepFind(list: ComponentType[]) {
          for (let i = 0; i < list.length; i++) {
            const item = list[i];
            if (item.id == id) {
              list.splice(i, 1);
              delete state.page.pageData.elementsMap[id];

              // 递归删除相互引用的嵌套父子组件
              const deepRemove = (id: string) => {
                // 删除子组件
                Object.values(state.page.pageData.elementsMap).map((item: any) => {
                  if (item.parentId == id) {
                    delete state.page.pageData.elementsMap[item.id];
                    deepRemove(item.id);
                  }
                  return item;
                });
              };
              deepRemove(id);
              break;
            } else if (item.elements?.length) {
              deepFind(item.elements);
            }
          }
        }
        deepFind(state.page.pageData.elements);
        state.selectedElement = undefined;
      }),
    );
  },
  updateGridChildren(payload: any) {
    const { id, cols = 3 } = payload;
    const colNumArr = COLUMN_MAP[cols as keyof typeof COLUMN_MAP];
    const addCols: any[] = []; // 标记需要新增的cols
    const removeCols: any[] = []; // 标记需要删除的cols
    const editCols: any[] = []; // 标记需要编辑的cols
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        function deepFind(list: ComponentType[]) {
          for (let i = 0; i < list.length; i++) {
            const item = list[i];
            if (item.id == id) {
              const { elements = [] } = item;
              // 先删除多余的cols
              if (elements.length > colNumArr.length) {
                removeCols.push(...elements.slice(colNumArr.length).map((item) => item.id));
                elements.splice(colNumArr.length);
              }
              colNumArr.forEach((colNum, index) => {
                if (elements[index]) {
                  editCols.push({ id: elements[index].id, colNum });
                } else {
                  // 新增cols
                  const newCol: any = {
                    id: createId('Col', 6),
                    type: 'Col',
                    name: '列组件',
                    parentId: id,
                  }
                  elements.push({ ...newCol, elements: [] });
                  addCols.push({
                    ...newCol,
                    config: {
                      props: { span: colNum },
                      style: {},
                      events: [],
                      api: {},
                      source: '',
                    },
                    events: [],
                    methods: [],
                  });
                }
              });
              // 修改elementsMap内的cols
              if (editCols.length) {
                editCols.forEach((col) => {
                  state.page.pageData.elementsMap[col.id].config.props.span = col.colNum;
                });
              }
              if (addCols.length) {
                addCols.forEach((col) => {
                  state.page.pageData.elementsMap[col.id] = col;
                });
              }
              if (removeCols.length) {
                // 递归删除相互引用的嵌套父子组件
                const deepRemove = (id: string) => {
                  Object.values(state.page.pageData.elementsMap).map((item: any) => {
                    if (item.parentId == id) {
                      delete state.page.pageData.elementsMap[item.id];
                      deepRemove(item.id);
                    }
                    return item;
                  });
                }
                removeCols.forEach((col) => {
                  delete state.page.pageData.elementsMap[col];
                  deepRemove(col);
                });
              }
              break;
            } else if (item.elements?.length) {
              deepFind(item.elements);
            }
          }
        }
        deepFind(state.page.pageData.elements);
      }),
    );
  },
  // 添加变量
  addVariable(payload: PageVariable) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        state.page.pageData.variables.push(payload);
      }),
    );
  },
  // 更新变量
  editVariable(payload: PageVariable) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        const index = state.page.pageData.variables.findIndex((item: PageVariable) => item.name == payload.name);
        if (index > -1) {
          state.page.pageData.variableData[payload.name] = payload.defaultValue;
          state.page.pageData.variables[index] = payload;
        }
      }),
    );
  },
  // 删除变量
  removeVariable(name: string) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        state.page.pageData.variables = state.page.pageData.variables.filter((item: PageVariable) => item.name !== name);
      }),
    );
  },
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
  setInterceptor(payload: any) {
    set(
      produce((state) => {
        state.isEdit = true; // 标记为编辑状态
        state.page.pageData.interceptor = payload;
      }),
    );
  },
  // 有些场景下，需要手工更新组件选中的工具条
  updateToolbar: () => {
    set((state) => {
      return {
        isUpdateToolbar: !state.isUpdateToolbar,
      };
    });
  },
  // 清除页面信息
  clearPageInfo() {
    set(
      produce((state) => {
        state.page = {
          id: 0,
          name: '',
          path: '',
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
            // 表单控件数据(控件脱离表单管理)
            formItemData: {},
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
  currentTab: "",
  setCurrentTab: (tab: string) => set({ currentTab: tab }),
}));
