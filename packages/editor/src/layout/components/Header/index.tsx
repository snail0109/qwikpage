import { memo, useEffect, useMemo, useState } from 'react';
import { useLocation, useNavigate, useParams } from 'react-router-dom';
import { Layout, Menu, MenuProps, Button, Popover, Dropdown, Space, Flex, Switch } from 'antd';
import { ProjectOutlined, CaretDownFilled, DownOutlined, SunOutlined, MoonFilled, OneToOneOutlined } from '@ant-design/icons';
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
  function goToPath(path: string) {
    if (path === 'edit') {
      // 跳转编辑页面时，编辑器已经被销毁，导致page对象为空，此时从浏览器中获取页面ID参数
      navigate(`/editor/${id}/${path}`);
    } else {
      navigate(`/editor/${page.id}/${path}`);
    }
  }

  const pathItems: MenuProps['items'] = [
    {
      label: (
        <a
          target="_blank"
          rel="noopener noreferrer"
          onClick={(e) => {
            e.preventDefault();
            goToPath('edit');
          }}
        >
          编辑器
        </a>
      ),
      key: 'edit',
    },
    {
      label: (
        <a
          target="_blank"
          rel="noopener noreferrer"
          onClick={(e) => {
            e.preventDefault();
            goToPath('publishHistory');
          }}
        >
          发布记录
        </a>
      ),
      key: 'publishHistory',
    },
  ];

  const isEditPage = pageFrom === `editor/${id}/edit` || pageFrom === `editor/${id}/template`;
  const isPublishPage = pageFrom === `editor/${id}/publishHistory`;

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
          {/* 编辑和发布 */}
          {(isEditPage || isPublishPage) && mode === 'edit' && (
            <Dropdown menu={{ items: pathItems, selectable: true, defaultSelectedKeys: [...pageFrom.split('/').slice(-1)] }}>
              <a onClick={(e) => e.preventDefault()}>
                <Space>
                  {isEditPage ? '编辑' : '发布记录'}
                  <DownOutlined />
                </Space>
              </a>
            </Dropdown>
          )}
          {/* 预览模式 */}
          {mode === 'preview' && (
            <Button type="primary" onClick={handleExitPreview}>
              退出预览
            </Button>
          )}

          {/* 用户头像 */}
          <div className={styles.avatar}>
            <Dropdown
              menu={{
                items: [
                  {
                    key: 'logout',
                    label: '退出',
                  },
                ],
                onClick: (e) => {
                  localStorage.clear();
                  navigate(`/login?callback=${window.location.href}`);
                },
                selectable: true,
              }}
            >
              <Flex align="center" style={{ height: 64 }}>
                <a onClick={(e) => e.preventDefault()} style={{ marginInline: 5 }}>
                  {`${userInfo.nickName}` || '开发者'}
                </a>
                {userInfo.avatar && <img width={25} style={{ verticalAlign: 'sub', borderRadius: '100%' }} src={userInfo.avatar} />}
              </Flex>
            </Dropdown>

          </div>
        </div>
      </Layout.Header>
    </>
  );
});

export default Header;
