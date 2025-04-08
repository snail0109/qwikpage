import { useCallback, useEffect, useState } from 'react';
import styles from './index.module.less';
import { Badge, Button, Tooltip, theme } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import { usePageStore } from '@/stores/pageStore';
import { getComponentRef } from '@/packages/utils/useComponentRefs';
import DrawerIcon from '@/assets/icons/drawer.svg?react';
import ModalIcon from '@/assets/icons/modal.svg?react';
import DeleteIcon from '@/assets/icons/Delete.svg?react';
import { getComponent } from '@/packages/index';
import { createId } from '@/utils/util';

type CollectorItem = {
  id: string;
  name: string;
  title: string;
};
const ELE_MAP: any = {
  1: {
    name: '弹框',
    type: 'Modal'
  },
  2: {
    name: '抽屉',
    type: 'Drawer'
  }
}
const { useToken } = theme;
const FloatingCollector = () => {
  const { token } = useToken();
  const [isExpanded, setIsExpanded] = useState(false);
  const [selectedItem, setSelectedItem] = useState<string | null>(null);
  const [modalList, setModalList] = useState<CollectorItem[]>([]);
  const [drawerList, setDrawerList] = useState<CollectorItem[]>([]);
  const [currentItems, setCurrentItems] = useState<CollectorItem[]>([]);
  const [currentType, setCurrentType] = useState<number>(1);

  const { elementsMap, removeElements, addElement } = usePageStore((state) => {
    return {
      elementsMap: state.page.pageData.elementsMap,
      removeElements: state.removeElements,
      addElement: state.addElement
    };
  });

  // 过滤弹框和抽屉组件
  useEffect(() => {
    setModalList([]);
    setDrawerList([]);
    const floats = Object.keys(elementsMap)
      .filter((id) => id.startsWith('Modal') || id.startsWith('Drawer'));
    const modals = floats.filter(v => v.startsWith('Modal'));
    const drawers = floats.filter(v => v.startsWith('Drawer'));
    modals.forEach((id, index) => {
      const modal = elementsMap[id];
      if (modal) {
        setModalList((prevList) => [...prevList, { id, name: `Modal(${id})`, title: `弹窗${index + 1}` }]);
      }
    });
    drawers.forEach((id, index) => {
      const drawer = elementsMap[id];
      if (drawer) {
        setDrawerList((prevList) => [...prevList, { id, name: `Drawer(${id})`, title: `侧边弹窗${index + 1}` }]);
      }
    });
  }, [elementsMap]);

  // 更新当前显示的列表
  useEffect(() => {
    setCurrentItems(currentType === 1 ? modalList : drawerList);
    // setIsExpanded(isExpanded && currentType === 1 ? modalList.length > 0 : drawerList.length > 0);
  }, [currentType, modalList, drawerList]);

  // 切换类型
  const handleTypeClick = (type: number) => {
    const changed = currentType !== type;
    setCurrentType(type);
    if (changed) {
      setIsExpanded(true)
    } else {
      setIsExpanded(!isExpanded);
    }
  };

  // 打开弹框或抽屉
  const handleItemClick = useCallback((item: CollectorItem) => {
    if (selectedItem && selectedItem !== item.id) {
      // 先关闭上一次弹框
      const prevRef = getComponentRef(selectedItem);
      if (prevRef) {
        prevRef.close();
      }
    }
    setSelectedItem(item.id);
    const ref = getComponentRef(item.id);
    ref.open({});
    // setTimeout(() => {
    //   setIsExpanded(false);
    // }, 300);
  }, [selectedItem]);

  const handleCreate = useCallback(async () => {
    const ele = ELE_MAP[currentType];
    const { config, events, methods = [] } = (await getComponent(ele.type + 'Config'))?.default || {};
    const newId = createId(ele.type);
    addElement({
      type: ele.type,
      name: ele.name,
      id: newId,
      elements: [],
      config,
      events,
      methods,
    });
  }, [currentType])

  // 删除弹框或抽屉
  const handleDelete = useCallback((targetId: string) => {
    setSelectedItem(null);
    removeElements(targetId);
  }, []);

  return (
    <div className={styles.container}>
      {/* 展开内容区域 */}
      {isExpanded && (
        <div className={`${styles.collectorContent} ${styles.expanded}`}>
          <div className={styles.itemList}>
            {currentItems.map((item, index) => (
              <div
                onClick={() => handleItemClick(item)}
                className={`${styles.item} ${selectedItem === item.id ? styles.active : ''}`}
              >
                <span className={styles.title}>{item.title}</span>
                <span className={styles.action}>
                  <DeleteIcon
                    style={{ fontSize: 17 }}
                    onClick={(e) => {
                      e.stopPropagation();
                      handleDelete(item.id);
                    }}
                  />
                </span>
              </div>
            ))}
          </div>
          <Button block variant="dashed" color="primary" icon={<PlusOutlined />} onClick={handleCreate}>创建</Button>
        </div>
      )}

      {/* <div className={`${styles.iconContainer} ${isExpanded ? styles.expanded : ''}`}>
        <Tooltip title="弹框" placement="right">
          <Button className={styles.iconButton} onClick={() => handleTypeClick(1)}>
            <Badge count={modalList.length} size="small" color={token.colorPrimary} showZero>
              <ModalIcon />
            </Badge>
          </Button>
        </Tooltip>
        <Tooltip title="抽屉" placement="right">
          <Button className={styles.iconButton} onClick={() => handleTypeClick(2)}>
            <Badge count={drawerList.length} size="small" color={token.colorPrimary} showZero>
              <DrawerIcon />
            </Badge>
          </Button>
        </Tooltip>
      </div> */}
    </div>
  );
};

export default FloatingCollector;
