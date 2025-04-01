import { RouterProvider } from "react-router-dom";
import { ConfigProvider, App as AntdApp, theme, Skeleton, Spin } from "antd";
import router from "./config/router";
import AntdGlobal from "@/utils/AntdGlobal";
import locale from "antd/locale/zh_CN";
import dayjs from "dayjs";
import "dayjs/locale/zh-cn";
import weekday from "dayjs/plugin/weekday";
import localeData from "dayjs/plugin/localeData";
import relativeTime from "dayjs/plugin/relativeTime";
dayjs.extend(relativeTime);
dayjs.extend(weekday);
dayjs.extend(localeData);
dayjs.locale("zh-cn");
import "./App.less";
import { useEffect, useState } from "react";
import usePreferencesStore from "./stores/preferencesStore";
import UpdaterDialog from "./components/UpdaterDialog";
import "@/styles/global.less";
import { attachConsole } from "@tauri-apps/plugin-log";

function App() {
    const [loading, setLoading] = useState(true);
    const { get_preferences, fontFamily, fontSize } = usePreferencesStore();
    useEffect(() => {
        const fetchData = async () => {
            const detach = await attachConsole();
            // call detach() if you do not want to print logs to the console anymore

            get_preferences().then(() => {
                setLoading(false);
            });
        };

        fetchData();
    }, []);
    useEffect(() => {
        if (fontFamily !== null) {
            document.documentElement.style.fontFamily = `"${fontFamily === "default" ? "sans-serif" : fontFamily}"`;
        }
        if (fontSize !== null) {
            document.documentElement.style.fontSize = `${fontSize}px`;
        }
    }, [fontFamily, fontSize]);

    if (loading) {
        return <Spin spinning={loading} fullscreen />;
    }

    return (
        <ConfigProvider
            locale={locale}
            theme={{
                cssVar: true,
                hashed: false,
                token: {
                    colorPrimary: "#216EF7",
                    colorLink: "#216EF7",
                    colorInfo: "#216EF7",
                    controlHeight: 28,
                    borderRadius: 4,
                    fontFamily: fontFamily,
                },
                components: {
                    Button: {
                        defaultBorderColor: "#D0DAE8",
                        fontWeight: 300,
                        defaultShadow: "none",
                        primaryShadow: "none",
                        dangerShadow: "none",
                        boxShadow: "none",
                    },
                    Input: {
                        activeShadow: "none",
                        boxShadow: "none",
                    },
                    Menu: {
                        darkItemBg: "#000",
                        darkItemHoverColor: "#216EF7",
                    },
                    Form: {
                        itemMarginBottom: 15,
                        verticalLabelPadding: 0,
                    },
                },
                // algorithm: marsTheme === 'dark' ? theme.darkAlgorithm : theme.defaultAlgorithm,
            }}
        >
            <AntdApp>
                <AntdGlobal />
                <RouterProvider router={router} />
            </AntdApp>
            {import.meta.env.MODE !== "development" && <UpdaterDialog />}
        </ConfigProvider>
    );
}

export default App;
