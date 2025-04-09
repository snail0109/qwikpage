import { ComponentType, ComItemType } from '../packages/types';

/**
 * 生成组件ID
 * @param name 组件类型名称
 * @returns 新名称
 */
export const createId = (name: string, len: number = 10) => {
  return (
    name +
    '_' +
    Number(Math.random().toString().substring(2, 12) + Date.now())
      .toString(36)
      .slice(0, len)
  );
};


/**
 * 递归查找组件
 * element：返回当前元素
 * index：返回当前元素在父级中的索引
 * elements：返回父级列表
 */
export const getElement = (elements: ComItemType[], id?: string): { element: ComItemType | null; index: number; elements: ComItemType[] } => {
  if (!id) return { element: null, index: -1, elements: [] };
  for (let i = 0; i < elements.length; i++) {
    const item = elements[i];
    if (item.id == id) {
      return { element: item, index: i, elements };
    } else if (item.elements?.length) {
      const result = getElement(item.elements, id);
      if (result.element) return result;
    }
  }
  return { element: null, index: -1, elements: [] };
};



/**
 * 判断是否在表单中。
 * @param elementId - 元素的唯一标识符id
 * @param elementsMap - 元素elementsMap对象。
 */
export function judgeIfInForm(elementId: string, elementsMap: { [key: string]: ComponentType<any> }, deepth = 0) {
  const currentElement = elementsMap[elementId];
  if (!currentElement) return false;
  if (deepth === 0) {
    const { formItem } = currentElement.config.props || {};
    if (!formItem) {
      return false;
    }
  }
  const { parentId } = currentElement;
  if (!parentId) return false;
  const parentElement = elementsMap[parentId];
  if (!parentElement) return false;
  const { type, id } = parentElement;
  if (type === 'Form') {
    return id;
  }
  const deep = deepth + 1;
  return judgeIfInForm(parentId, elementsMap, deep);
}
