import { Outlet } from 'react-router-dom';
import Header from '@/layout/components/Header';
export default function Root() {

  return (
    <>
      <Header />
      <Outlet />
    </>
  );
}
