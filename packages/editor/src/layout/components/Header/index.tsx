import { memo, useEffect, useMemo, useState } from 'react';
import { useLocation, useNavigate, useParams } from 'react-router-dom';
import { Layout, Menu, MenuProps, Button, Popover, Space, Switch } from 'antd';
import { ProjectOutlined, CaretDownFilled, SunOutlined, MoonFilled, OneToOneOutlined } from '@ant-design/icons';
import { usePageStore } from '@/stores/pageStore';
import Publish from './PublishPopover';
import styles from './index.module.less';
import storage from '@/utils/storage';

/**
 * 编辑器顶部组件
 */
const Header = memo(() => {
  const [isNav, setNav] = useState(false);
  const [navKey, setNavKey] = useState(['projects']);
  const [pageFrom, setPageFrom] = useState('projects');
  const navigate = useNavigate();
  const { id } = useParams();
  const location = useLocation();

  const { userInfo, page, mode, theme, setMode, setTheme } = usePageStore((state) => {
    return {
      userInfo: state.userInfo,
      page: state.page,
      mode: state.mode,
      theme: state.theme,
      setMode: state.setMode,
      setTheme: state.setTheme,
    };
  });

  // 返回首页
  const goHome = () => {
    setMode('edit');
    // 点击Logo返回最近操作的列表，对用户友好
    const isProject = /projects\/\d+\/\w+/.test(location.pathname);
    const isPage = /editor\/\d+\/(edit|publishHistory)/.test(location.pathname);
    if (isProject) return navigate('/projects');
    if (isPage) return navigate('/pages');
    navigate('/projects');
  };

  // Tab切换项
  const items: MenuProps['items'] = useMemo(
    () => [
      {
        label: '项目列表',
        key: 'projects',
        icon: <ProjectOutlined style={{ fontSize: 16 }} />,
      },
      {
        label: '页面列表',
        key: 'pages',
        icon: <OneToOneOutlined style={{ fontSize: 16 }} />,
      },
    ],
    [],
  );

  useEffect(() => {
    if (['/projects', '/pages'].includes(location.pathname)) {
      setNav(true);
      setNavKey([location.pathname.slice(1)]);
    } else {
      setNav(false);
    }
    setPageFrom(location.pathname.slice(1));
  }, [location]);

  // 设置主题
  useEffect(() => {
    const isDark = storage.get('marsview-theme');
    if (isDark) {
      document.documentElement.setAttribute('data-theme', 'dark');
    } else {
      document.documentElement.setAttribute('data-theme', 'light');
    }
    setTheme(isDark ? 'dark' : 'light');
  }, [userInfo]);

  // Tab切换点击
  const handleTab: MenuProps['onClick'] = (e) => {
    navigate(`/${e.key}`);
  };

  // 退出预览模式
  const handleExitPreview = () => {
    setMode('edit');
  };

  const isEditPage = pageFrom === `editor/${id}/edit` || pageFrom === `editor/${id}/template`;

  return (
    <>
      <Layout.Header className={styles.layoutHeader}>
        <div className={styles.logo} onClick={goHome}>
          <img src={`${theme === 'dark' ? '/imgs/mars-logo-dark.png' : '/imgs/mars-logo.png'}`} width={42} />
          <span>QwikPage</span>
        </div>
        {/* 首页 - 导航菜单 */}
        {isNav && (
          <div className={styles.menu}>
            <Menu onClick={handleTab} selectedKeys={navKey} theme={theme} mode="horizontal" items={items} />
          </div>
        )}

        {/* 用户信息&发布&发布记录 */}
        <div className={styles.user}>
          <Space>
            <Switch
              checkedChildren={<MoonFilled />}
              unCheckedChildren={<SunOutlined />}
              defaultChecked
              checked={theme == 'dark' ? true : false}
              onChange={(val) => {
                storage.set('marsview-theme', val);
                setTheme(val ? 'dark' : 'light');
                document.documentElement.setAttribute('data-theme', val ? 'dark' : 'light');
              }}
            />
            {isEditPage && mode === 'edit' && (
              <>
                <Popover placement="bottom" content={<Publish />} trigger="click">
                  <Button type="primary">
                    发布
                    <CaretDownFilled />
                  </Button>
                </Popover>
              </>
            )}
          </Space>

          {/* 预览模式 */}
          {mode === 'preview' && (
            <Button type="primary" onClick={handleExitPreview}>
              退出预览
            </Button>
          )}

        </div>
      </Layout.Header>
    </>
  );
});

export default Header;
