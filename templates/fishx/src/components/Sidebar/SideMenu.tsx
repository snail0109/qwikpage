import React from 'react';
import { Menu } from 'antd';
import { useSelector, useDispatch } from 'react-redux';
import { Fishx, Link } from 'fishx';
import { MenuConfig, MenuItem } from '@/typings/menu';
import handleConfig from './handleConfig';

const { SubMenu } = Menu;

const getMenuText = (item: MenuItem) => {
  return (
    <span>
      <span>{item.name}</span>
    </span>
  );
};

const renderMenu = (data: MenuConfig) => (
  data.map((item: MenuItem) => {
    if (item.children) {
      return (
        <SubMenu title={getMenuText(item)} key={item.path || item.name}>
          {renderMenu(item.children)}
        </SubMenu>
      );
    }

    return (
      <Menu.Item title={getMenuText(item)} key={item.path || item.name}>
        <Link
          to={item.path}
        >
          {getMenuText(item)}
        </Link>
      </Menu.Item>
    );
  })
);

const SideMenu = () => {
  const mapState = (state: any) => ({
    openKeys: state.sys.openKeys,
  });

  // 获取store中的数据
  const { openKeys } = useSelector(mapState);
  const dispatch = useDispatch();

  const { asideMenuConfig } = Fishx.config;
  const menuConfig = handleConfig(asideMenuConfig);

  return (
    <Menu
      theme="dark"
      mode="inline"
      onSelect={({ key }) => {
        dispatch({ type: 'sys/save', payload: { openKeys: [key] } });
      }}
      selectedKeys={openKeys}
    >
      {renderMenu(menuConfig)}
    </Menu>
  );
};

export default SideMenu;
