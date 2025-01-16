import { useEffect, useMemo } from 'react';
import { Outlet, useParams, useNavigate, useLoaderData, useLocation } from 'react-router-dom';
import { ConfigProvider, Layout } from 'antd';
import Header from '../components/Header/Header';
import Menu from '../components/Menu/Menu';
import { useProjectStore } from '@/stores/projectStore';
import { UserInfoStore, usePageStore } from '@marsview/materials/stores/pageStore';
import { getProjectDetail, getProjectMenu } from '@/api/index';
import Tab from '../components/Tab';
import Logo from '@/components/Logo/Logo';
import BreadList from '@/components/BreadList/BreadList';
import { arrayToTree } from '@/utils/util';
import storage from '@/utils/storage';
import locale from 'antd/locale/zh_CN';
import 'dayjs/locale/zh-cn';
import './layout.less';

const AdminLayout = () => {
  const { collapsed, setProjectInfo, projectInfo } = useProjectStore((state) => {
    return {
      collapsed: state.collapsed,
      setProjectInfo: state.setProjectInfo,
      projectInfo: state.projectInfo,
    };
  });
  const saveUserInfo = usePageStore((state) => state.saveUserInfo);
  const loaderData = useLoaderData();
  const navigate = useNavigate();
  const { projectId } = useParams();
  const { pathname } = useLocation();

  // 初始化用户信息
  useEffect(() => {
    if (!loaderData) return;
    saveUserInfo(loaderData as UserInfoStore);
  }, []);

  // 获取项目信息
  useEffect(() => {
    // 判断项目ID是否合法
    if (projectId && isNaN(+projectId)) return navigate('/404?type=project');
    const fetchProjectDetail = async () => {
      if (projectId) {
        const detail = await getProjectDetail(projectId).catch(() => {
          return navigate('/403?type=project');
        });
        // 如果项目不存在，跳转到404
        if (!detail.id) {
          return navigate('/404?type=project');
        }
        const menus = await getProjectMenu(projectId).catch(() => {
          return navigate('/403?type=project');
        });
        if (!menus) return;

        // 如果没有页面路径，跳转到欢迎页
        if (!/project\/\d+\/\w+/.test(pathname)) navigate(`/project/${projectId}/welcome`);
        const { menuTree, buttons, pageMap, menuMap } = arrayToTree(menus.list);
        storage.set('buttons', buttons);
        storage.set('pageMap', pageMap);
        setProjectInfo({
          projectInfo: detail,
          menuTree,
          buttons,
          pageMap,
          menuMap,
        });
      }
    };
    fetchProjectDetail();
  }, [projectId]);

  // 计算渲染区容器实际高度
  const calcHeight = useMemo(() => {
    return projectInfo.tag ? `calc(100vh - 114px)` : `calc(100vh - 64px)`;
  }, [projectInfo.tag]);

  // 定义Footer
  const Footer = () => (
    <Layout.Footer>
      <div className="footnote">
        <span>Copyright © 2024 Marsview. All Rights Reserved. </span>
      </div>
    </Layout.Footer>
  );

  return (
    <ConfigProvider
      locale={locale}
      theme={{
        token: {
          colorPrimary: projectInfo.systemThemeColor || '#1677ff',
        },
        hashed: false,
      }}
    >
      <Layout>
        {/* 左右布局 */}
        {projectInfo.layout === 1 && (
          <Layout style={{ flexDirection: 'row' }}>
            {/* 左侧Sider渲染 */}
            <div style={{ width: collapsed ? 80 : 256, borderRight: '1px solid #e8e9eb' }}>
              <Logo />
              <Menu />
            </div>
            {/* 右侧内容渲染 */}
            <div style={{ width: collapsed ? 'calc(100vw - 80px)' : 'calc(100vw - 256px)' }}>
              <Header />
              {/* 加载页签 */}
              {projectInfo.tag === 1 && <Tab />}
              {/* 加载内容 */}
              <div style={{ height: calcHeight, overflow: 'auto' }}>
                <Outlet></Outlet>
                {projectInfo.footer === 1 && <Footer />}
              </div>
            </div>
          </Layout>
        )}
        {/* 上下布局 */}
        {projectInfo.layout === 2 && (
          <>
            <Header />
            {/* 加载页签 */}
            {projectInfo.tag ? <Tab /> : null}
            <Layout style={{ padding: 20, backgroundColor: '#f3f5f9', height: calcHeight, overflow: 'auto' }}>
              {/* 加载面包屑 */}
              {projectInfo.breadcrumb === 1 && <BreadList />}
              <Outlet></Outlet>
              {projectInfo.footer === 1 && <Footer />}
            </Layout>
          </>
        )}
      </Layout>
    </ConfigProvider>
  );
};

export default AdminLayout;
