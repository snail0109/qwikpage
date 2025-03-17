import { memo } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { Button, Form, Input, Space, Tooltip, Divider } from 'antd';
import { ArrowLeftOutlined, ReloadOutlined } from '@ant-design/icons';
import styles from './index.module.less';

const SearchBar = (props: any) => {
  const { pathname } = useLocation();
  const navigate = useNavigate();
  const { form, from, projectName, submit, refresh, onCreate, onCreateGroup, openDir, className = '', searchPlaceholder = '', noNeedCreate = false, noNeedFresh = false, needOpenDir = false } = props;

  return (
    <div className={`${styles.searchBar} ${className}`}>
      {['/project/pages', '/resources'].includes(pathname) && (
        <div className={styles.projectName}>
          <div className={styles.prefixIcon}></div>
          <div className={styles.projectNameText}>{projectName}</div>
        </div>)}
      <div className={styles.searchBarContent} style={{ width: ['/project/pages', '/resources'].includes(pathname) ? 'auto' : '100%' }}>
        {['/project/pages', '/resources'].includes(pathname) && (
          <Tooltip title="返回" className={styles.backButton}>
            <Button icon={<ArrowLeftOutlined />} onClick={() => navigate('/projects')}></Button>
          </Tooltip>
        )}
        <Form form={form} layout="inline" initialValues={{ type: 1 }}>
          <Form.Item name="keyword" style={{ width: 200, marginInlineEnd: '8px' }} >
            <Input placeholder={searchPlaceholder || `请输入${from === '分组' ? '项目' : from}名称`} onPressEnter={submit} />
          </Form.Item>
          <Form.Item>
            <Button type="primary" className={styles.searchBtn} onClick={submit} size="middle" autoInsertSpace={false}>
              搜索
            </Button>
          </Form.Item>
        </Form>
        {['/project/pages', '/resources'].includes(pathname) && <div className={styles.divider}></div>}
        <Space>
          {
            !noNeedCreate && (
              <Button onClick={from === '分组' ? onCreateGroup : onCreate}>
                创建{from}
              </Button>
            )
          }
          {
            !noNeedFresh && (
              <Tooltip title="刷新">
                <Button icon={<ReloadOutlined className={styles.refreshButton} />} onClick={refresh}></Button>
              </Tooltip>
            )
          }
           {
            needOpenDir && (
              <Button onClick={openDir}>
               打开目录
              </Button>
            )
          }
        </Space>
      </div>
    </div>
  );
};

export default memo(SearchBar);
