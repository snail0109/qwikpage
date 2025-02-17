import React, { useEffect } from 'react';
import { Fishx } from 'fishx';
import './global.less';

interface IAppProps {
  children: React.ReactNode;
}

function App(props: IAppProps) {
  const [location, setLocation] = React.useState('/');
  useEffect(() => {
    setLocation(Fishx.history.location.pathname);
    Fishx.history.listen((data: any) => {
      setLocation(data.location.pathname);
    });
  }, []);
 
  return <>{props.children}</>;
}

export default App;
