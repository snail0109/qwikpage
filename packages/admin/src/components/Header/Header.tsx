import React, { memo } from 'react';
import { Flex } from 'antd';
import { MenuFoldOutlined, MenuUnfoldOutlined } from '@ant-design/icons';
import { useProjectStore } from '@/stores/projectStore';
import Logo from '../Logo/Logo';
import BreadList from '../BreadList/BreadList';
import Menu from '../Menu/Menu';
import styles from './index.module.less';

/**
 * 编辑器顶部组件
 */
const Header = memo(() => {
  const { projectInfo, collapsed, updateCollapsed } = useProjectStore();

  // 控制菜单图标关闭和展开
  const toggleCollapsed = () => {
    updateCollapsed();
  };

  const isLight = projectInfo.menuThemeColor === 'light' || projectInfo.layout === 1;
  const style: React.CSSProperties = {
    backgroundColor: isLight ? '#fff' : '#001529',
    color: projectInfo.menuThemeColor === 'light' ? '#000' : '#fff',
  };
  return (
    <div className={styles.header} style={style}>
      {/* 加载面包屑，左右布局时，面包屑在顶部 */}
      {projectInfo.layout === 1 ? (
        <Flex align="center">
          <div onClick={toggleCollapsed} style={{ cursor: 'pointer' }}>
            {collapsed ? (
              <MenuUnfoldOutlined style={{ color: isLight ? '#000' : '#fff' }} />
            ) : (
              <MenuFoldOutlined style={{ color: isLight ? '#000' : '#fff' }} />
            )}
          </div>
          {projectInfo.breadcrumb && <BreadList />}
        </Flex>
      ) : null}

      {projectInfo.layout === 2 ? (
        <Flex align="center">
          <Logo />
          <Menu />
        </Flex>
      ) : null}
    </div>
  );
});

export default Header;
