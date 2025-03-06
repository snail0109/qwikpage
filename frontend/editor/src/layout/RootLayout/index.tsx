import { Outlet } from 'react-router-dom';
import Header from '../components/Header';
import { useEffect } from 'react';
import useAppConfigStore from '@/stores/appConfigStore';
export default function Root() {

  
  const { initConfig, ...rest }  = useAppConfigStore();
  useEffect(() => {
    initConfig();
  }, [])

  console.log("config value", rest)

  return (
    <>
      <Header />
      <Outlet />
    </>
  );
}

