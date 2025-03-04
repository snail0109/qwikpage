import { useEffect, useMemo } from 'react';
import { Outlet, useParams, useNavigate, useLocation } from 'react-router-dom';
import { ConfigProvider, Layout } from 'antd';
// import Header from '../components/Header/Header';
// import Menu from '../components/Menu/Menu';
import { useProjectStore } from '@/stores/projectStore';
import { getProjectDetail } from '@/api/index';
// import Tab from '../components/Tab';
// import Logo from '@/components/Logo/Logo';
// import BreadList from '@/components/BreadList/BreadList';
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
  const navigate = useNavigate();
  const { projectId } = useParams();
  const { pathname } = useLocation();

  // 获取项目信息
  useEffect(() => {
    // 判断项目ID是否合法
    // if (projectId && isNaN(+projectId)) return navigate('/404?type=project');
    const fetchProjectDetail = async () => {
      if (projectId) {
        const detail = await getProjectDetail(projectId).catch(() => {
          return navigate('/403?type=project');
        });
        // 如果项目不存在，跳转到404
        if (!detail.id) {
          return navigate('/404?type=project');
        }
        const paths = pathname.split("/").filter(v => v)
        if (paths.length <= 2 || pathname.endsWith('welcome')) {
          navigate(`/project/${projectId}/welcome`);
        }
        const { menuTree, buttons, pageMap, menuMap } = arrayToTree([]);
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
  // const calcHeight = useMemo(() => {
  //   return projectInfo.tag ? `calc(100vh - 114px)` : `100vh`;
  // }, [projectInfo.tag]);

  // 定义Footer
  const Footer = () => (
    <Layout.Footer>
      <div className="footnote">
        <span>QwikPage</span>
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
      {/* <Layout> */}
      {/* 左右布局 */}
      {/* {projectInfo.layout === 1 && ( */}
      <Layout style={{ flexDirection: 'row' }}>
        {/* 左侧Sider渲染 */}
        {/* <div style={{ width: collapsed ? 80 : 256, borderRight: '1px solid #e8e9eb' }}>
            <Logo />
            <Menu />
          </div> */}
        {/* 右侧内容渲染 */}
        <div style={{ width: '100vw' }}>
          {/* <Header /> */}
          {/* 加载页签 */}
          {/* {projectInfo.tag && <Tab />} */}
          {/* 加载内容 */}
          <div style={{ height: '100vh', overflow: 'auto' }}>
            <Outlet></Outlet>
            {projectInfo.footer === 1 && <Footer />}
          </div>
        </div>
      </Layout>
      {/* )} */}
      {/* 上下布局 */}
      {/* {projectInfo.layout === 2 && ( */}
      {/* <> */}
      {/* <Header /> */}
      {/* 加载页签 */}
      {/* {projectInfo.tag ? <Tab /> : null} */}
      {/* <Layout style={{ padding: 20, backgroundColor: '#f3f5f9', height: calcHeight, overflow: 'auto' }}> */}
      {/* 加载面包屑 */}
      {/* {projectInfo.breadcrumb && <BreadList />} */}
      {/* <Outlet></Outlet> */}
      {/* {projectInfo.footer === 1 && <Footer />} */}
      {/* </Layout> */}
      {/* </> */}
      {/* )} */}
      {/* </Layout> */}
    </ConfigProvider>
  );
};

export default AdminLayout;
