import { lazy, Suspense } from 'react';
import { Col, Flex, Row, Space, Tabs, Tooltip } from 'antd';
import {
  AppstoreOutlined,
  PartitionOutlined,
  CodeOutlined,
  ApiOutlined,
  FunctionOutlined,
  ProjectOutlined,
  QuestionCircleOutlined,
} from '@ant-design/icons';
import ComponentPanel from './ComponentPanel';
import SpinLoading from '@/components/SpinLoading';
import styles from './index.module.less';

// 页面列表
const PageList = lazy(() => import('./Pages/PageList'));
// 组件大纲
const OutlinePanel = lazy(() => import('./OutlinePanel'));
// 页面源码
const CodingPanel = lazy(() => import('./CodingPanel'));
// 接口列表
const ApiList = lazy(() => import('./ApiList/ApiList'));
// 页面变量
const VariableList = lazy(() => import('./Variable/VariableList'));
/**
 * 左侧面板类型
 */
const panels = [
  {
    key: 'ComponentPanel',
    icon: <AppstoreOutlined style={{ fontSize: 16 }} />,
    label: '组件',
    title: (
      <Space>
        <span>组件</span>
        <Tooltip title="无需拖拽，直接点击就能添加到画布中。">
          <QuestionCircleOutlined />
        </Tooltip>
      </Space>
    ),
    children: () => {
      return <ComponentPanel />;
    },
  },
  {
    key: 'home',
    icon: <ProjectOutlined style={{ fontSize: 16 }} />,
    label: '页面',
    title: '页面列表',
    children: () => {
      return <PageList />;
    },
  },
  {
    key: 'OutlinePanel',
    icon: <PartitionOutlined style={{ fontSize: 16 }} />,
    label: '大纲',
    title: (
      <Space>
        <span>页面大纲</span>
        <Tooltip title="组件支持拖拽排序">
          <QuestionCircleOutlined />
        </Tooltip>
      </Space>
    ),
    children: () => {
      return <OutlinePanel />;
    },
  },
  {
    key: 'CodingPanel',
    icon: <CodeOutlined style={{ fontSize: 16 }} />,
    label: '代码',
    title: '页面JSON',
    children: () => {
      return <CodingPanel />;
    },
  },
  {
    key: 'ApiList',
    icon: <ApiOutlined style={{ fontSize: 16 }} />,
    label: '接口',
    title: '页面接口',
    children: () => {
      return <ApiList />;
    },
  },
  {
    key: 'Variable',
    icon: <FunctionOutlined style={{ fontSize: 16 }} />,
    label: '变量',
    title: '页面变量',
    children: () => {
      return <VariableList />;
    },
  },
];

/**
 * 生成左侧组件列表
 */

const Menu = () => {
  return (
    <>
      <Tabs
        size={'small'}
        defaultActiveKey={panels[0].key}
        tabPosition="left"
        tabBarStyle={{ width: 50, height: 'calc(100vh - 64px)' }}
        className={styles.leftTool}
        centered={true}
        items={panels.map((item) => {
          return {
            key: item.key,
            label: (
              <Flex vertical justify="center" align="center" gap={5}>
                {item.icon}
                <span style={{ fontSize: 12 }}>{item.label}</span>
              </Flex>
            ),
            children: (
              <div style={{ marginLeft: -10, marginRight: 10 }}>
                <Row style={{ height: 46 }} align={'middle'} justify={'space-between'}>
                  <Col>
                    <span style={{ fontWeight: 'bold' }}>{item.title}</span>
                  </Col>
                </Row>
                <Suspense fallback={<SpinLoading />}>{item.children?.()}</Suspense>
              </div>
            ),
          };
        })}
      />
    </>
  );
};

export default Menu;