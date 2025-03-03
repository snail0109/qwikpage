import { Layout } from 'antd';
import { Outlet } from 'react-router-dom';
import styles from './index.module.less';
export default function Admin() {
  return (
    <Layout>
      <Layout.Content className={styles.content}>
        <Outlet></Outlet>
      </Layout.Content>
    </Layout>
  );
}
