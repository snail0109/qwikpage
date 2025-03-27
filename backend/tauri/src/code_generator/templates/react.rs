/// 路由模板
pub const ROUTE: &str = r#"
  {
    path: '{path}',
    component: './{component}',
    exact: true,
  },"#;

/// 菜单模板
pub const MENU: &str = r#"
  {
    path: '{route_path}',
    name: '{route_name}',
  },"#;

/// React组件模板
pub const COMPONENT: &str = r#"
import React, { useState, useEffect } from 'react';
import { PageWrapper } from '@components/PageWrapper';
{antd_import}
import { usePageStore } from '@/stores/pageStore';
import { useShallow } from 'zustand/react/shallow';
import { Spin } from 'antd';

function {compName}() {
  const [loading, setLoading] = useState(true);
  const { savePageInfo } = usePageStore(
    useShallow((state) => ({
      savePageInfo: state.savePageInfo,
    }))
  );

  useEffect(() => {{
    savePageInfo({
      id: "{page_id}",
      pageData: {page_str}
    });
    setLoading(false);
  }}, []);

  if (loading) return <Spin />;

  return (
    <PageWrapper>
      {components}
    </PageWrapper>
  );
}

export default {compName};
"#;
