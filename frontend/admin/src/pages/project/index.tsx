import { getPageDetailWithPath } from '@/api/index';
import { useLocation, useParams } from 'react-router-dom';
import { useEffect, useState } from 'react';
import { usePageStore } from '@qwikpage/materials/stores/pageStore';
import { message } from '@/utils/AntdGlobal';
import NotFound from './notFound';
import Page from '@qwikpage/materials/Page/Page';
import { useShallow } from 'zustand/react/shallow';
import { ComItemType, ConfigType } from '@qwikpage/materials/types/index';

export default function () {
  const [pageData, setPageData] = useState<{ config: ConfigType; elements: ComItemType[] }>();
  const [notFound, setNotFound] = useState(false);

  const { projectId } = useParams();

  const { initPageData, savePageInfo, clearPageInfo } = usePageStore(
    useShallow((state) => {
      return {
        initPageData: state.page.pageData,
        savePageInfo: state.savePageInfo,
        clearPageInfo: state.clearPageInfo,
      };
    }),
  );
  const { pathname } = useLocation();
  useEffect(() => {
    if (!projectId) return;
    const pageId = pathname.split(projectId)[1].slice(1);
    getPageDetailWithPath(projectId, pageId)
      .then((res: any) => {
        let pageData: any = {};
        try {
          pageData = JSON.parse(res.pageData || '{}');
        } catch (error) {
          console.error(error);
          console.info('【json数据】', res.pageData);
          message.error('页面数据格式错误，请检查');
        }
        clearPageInfo();
        savePageInfo({
          ...res,
          pageData,
        });
        setPageData(pageData);
        setNotFound(false);
      })
      .catch(() => {
        setNotFound(true);
      });
    return () => {
      setPageData({ config: initPageData.config, elements: [] });
    };
  }, [projectId, pathname]);

  if (!pageData?.config) {
    return <></>;
  }

  return <>{notFound ? <NotFound /> : <Page config={pageData?.config} elements={pageData?.elements} />}</>;
}
