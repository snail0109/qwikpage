import { RouterProvider } from 'react-router-dom';
import { ConfigProvider, App as AntdApp, theme } from 'antd';
import router from './config/router';
import AntdGlobal from '@/utils/AntdGlobal';
import locale from 'antd/locale/zh_CN';
import dayjs from 'dayjs';
import 'dayjs/locale/zh-cn';
import weekday from 'dayjs/plugin/weekday';
import localeData from 'dayjs/plugin/localeData';
import relativeTime from 'dayjs/plugin/relativeTime';
dayjs.extend(relativeTime);
dayjs.extend(weekday);
dayjs.extend(localeData);
dayjs.locale('zh-cn');
import './App.less';
import storage from './utils/storage';
import { usePageStore } from './stores/pageStore';

function App() {
  const isDark = storage.get('marsview-theme');
  const marsTheme = usePageStore((state) => state.theme);
  return (
    <ConfigProvider
      locale={locale}
      theme={{
        cssVar: true,
        hashed: false,
        token: {
          colorPrimary: '#216EF7',
          colorLink: '#216EF7',
          colorInfo: '#216EF7',
        },
        components: {
          Button: {
            defaultBorderColor: '#D0DAE8',
            fontWeight: 300,
            defaultShadow: 'none',
            boxShadow: 'none'
          },
          Input: {
            activeShadow: 'none',
            boxShadow: 'none'
          },
          Menu: {
            darkItemBg: '#000',
            darkItemHoverColor: '#216EF7',
          },
        },
        algorithm: marsTheme === 'dark' ? theme.darkAlgorithm : theme.defaultAlgorithm,
      }}
    >
      <AntdApp>
        <AntdGlobal />
        <RouterProvider router={router} />
      </AntdApp>
    </ConfigProvider>
  );
}

export default App;
