import { useEffect } from 'react';
import { Outlet, useParams, useNavigate, useLocation } from 'react-router-dom';
import { ConfigProvider, Layout } from 'antd';
import { useProjectStore } from '@/stores/projectStore';
import { useProjectStore as useMProjectStore } from '@materials/stores/projectStore'
import { getProjectDetail } from '@/api/index';
import locale from 'antd/locale/zh_CN';
import 'dayjs/locale/zh-cn';
import './layout.less';

const AdminLayout = () => {
  const { projectInfo } = useProjectStore((state) => {
    return {
      projectInfo: state.projectInfo,
    };
  });
  const { setProjectVariables } = useMProjectStore((state) => {
    return {
      setProjectVariables: state.setVariables,
    };
  });
  const navigate = useNavigate();
  const { projectId } = useParams();
  const { pathname } = useLocation();

  // 获取项目信息
  useEffect(() => {
    const fetchProjectDetail = async () => {
      if (projectId) {
        const detail = await getProjectDetail(projectId).catch(() => {
          return navigate('/403?type=project');
        });
        // 如果项目不存在，跳转到404
        if (!detail.id) {
          return navigate('/404?type=project');
        }
        if (pathname.endsWith('welcome')) {
          return navigate(`/project/${projectId}/welcome`);
        }
        if (detail.variables) {
          try {
            const variables = JSON.parse(detail.variables);
            setProjectVariables(variables);
          } catch(error) {
            console.error(error);
            console.info('解析项目变量失败，【variables】', detail.variables);
          }
        }
      }
    };
    fetchProjectDetail();
  }, [projectId]);

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
        components: {
          Form: {
            inlineItemMarginBottom: 15,
          },
        }
      }}
    >
      <Layout style={{ flexDirection: 'row' }}>
        <div style={{ width: '100vw' }}>
          <div style={{ height: '100vh', overflow: 'auto' }}>
            <Outlet></Outlet>
          </div>
        </div>
      </Layout>
    </ConfigProvider>
  );
};

export default AdminLayout;
