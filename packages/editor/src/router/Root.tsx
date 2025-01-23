import { Outlet } from 'react-router-dom';
import Header from '@/layout/components/Header';
import { useEffect } from 'react';
import useAppConfigStore from '@/stores/appConfigStore';
export default function Root() {

  const { initConfig }  = useAppConfigStore();
  useEffect(() => {
    // initConfig();
  }, [])

  return (
    <>
      <Header />
      <Outlet />
    </>
  );
}
