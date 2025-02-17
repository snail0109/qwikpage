import React, { useEffect } from "react";
import { Fishx } from "fishx";
import "./global.less";
import { ConfigProvider } from "antd";

interface IAppProps {
    children: React.ReactNode;
}

function App(props: IAppProps) {
    const [location, setLocation] = React.useState("/");
    useEffect(() => {
        setLocation(Fishx.history.location.pathname);
        Fishx.history.listen((data: any) => {
            setLocation(data.location.pathname);
        });
    }, []);

    return (
        <ConfigProvider
            theme={{
                token: {
                    // colorPrimary: systemThemeColor || "#1677ff",
                    colorPrimary: "#1677ff",
                },
                hashed: false,
            }}
        >
            {props.children}
        </ConfigProvider>
    );
}

export default App;
