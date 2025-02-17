import React, { useEffect, useState } from 'react';
import { Layout, Select } from 'antd';
import { MenuFoldOutlined, MenuUnfoldOutlined } from '@ant-design/icons';
import { getInitLocale, Outlet, setLang, utils } from 'fishx';
import Sidebar from '@/components/Sidebar';
import styles from './index.module.less';

const { cookieUtils } = utils;
const { getCookie } = cookieUtils;
const { Header, Content, Sider } = Layout;
const { Option } = Select;

const BasicLayout = (props: any) => {
  const [collapsed, setCollapsed] = useState(false);
  const [initLocale, setInitLocale] = useState('zh-CN');

  const toggle = () => {
    setCollapsed(!collapsed);
  };

  const handleChange = (value: string) => {
    setLang(value);
    setInitLocale(value);
  };

  const getContentStyle = () => {
    if (collapsed) {
      return { paddingLeft: '80px' };
    }
    return { paddingLeft: '200px' };
  };

  const getDefaultLocale = () => {
    const lang = getCookie('userLocale');
    if (!lang) {
      setInitLocale(getInitLocale());
    } else {
      setInitLocale(lang);
    }
  };

  useEffect(() => {
    getDefaultLocale();
  }, []);

  return (
    <Layout className={styles.layout}>
      <Sider trigger={null} collapsible collapsed={collapsed} className={`${styles.sider} ${styles.fixSiderbar}`}>
        <Sidebar />
      </Sider>
      <Layout style={getContentStyle()}>
        <Header style={{ background: '#fff', padding: 0 }}>
          {collapsed ?
            (<MenuUnfoldOutlined className={styles.trigger} onClick={toggle} />)
            : (<MenuFoldOutlined className={styles.trigger} onClick={toggle} />)
          }
          <div style={{ display: 'inline-block', float: 'right', marginRight: '20px' }}>
            <Select value={initLocale} style={{ width: 120, marginRight: '10px' }} onChange={handleChange}>
              <Option key="en-US" value="en-US">
                英文
              </Option>
              <Option key="zh-CN" value="zh-CN">
                中文
              </Option>
            </Select>
          </div>
        </Header>
        <Content
          className="main-layout-content"
        >
          <Outlet />
        </Content>
      </Layout>
    </Layout>
  );
};

export default BasicLayout;
