import usePreferencesStore from "@/stores/preferencesStore";
import { invoke } from "@tauri-apps/api/core";
import { info, warn } from "@tauri-apps/plugin-log";
import { relaunch } from "@tauri-apps/plugin-process";
import { check, Update } from "@tauri-apps/plugin-updater";
import { Modal, Progress, Button } from "antd";
import React, { useCallback, useEffect, useState } from "react";
import styles from "./index.module.less";
import { CloseCircleOutlined } from "@ant-design/icons";
import { beautifyVersion } from "@/utils/util";

type InstallStatus = "Downloading" | "DownloadAndInstall" | "Done" | "Up-to-date" | "Error";

function UpdaterDialog() {
    const [contentLength, setContentLength] = useState(0);
    const [contentDownloaded, setContentDownloaded] = useState(0);
    const [updaterInstancece, setUpdaterInstancece] = useState<Update | null>(null);
    const [needRestart, setNeedRestart] = useState(false);
    const progress = contentDownloaded && contentLength ? (contentDownloaded / contentLength) * 100 : 0;
    const [status, setStatus] = useState<InstallStatus | null>(null);

    const { checkUpdate } = usePreferencesStore((state) => {
        return {
            checkUpdate: state.checkUpdate,
        };
    });


    useEffect(() => {
        const fetchUpdate = async () => {
            if (!checkUpdate) {
                return;
            }
            const update = await check();
            if (!update) {
                return;
            }
            console.log(update);
            setUpdaterInstancece(update);
            const { version } = update;
            // TODO: 优化  
            // 判断 version 是不是正式版本
            const isPreRelease = version.includes("rc") || version.includes("beta") || version.includes("alpha");

            if (isPreRelease) {
                setStatus("Up-to-date");
                return;
            }

            let downloaded = 0;
            setStatus("DownloadAndInstall");
            try {
                await update.downloadAndInstall((event: any) => {
                    switch (event.event) {
                        case "Started":
                            setContentLength(event.data.contentLength!);
                            info(`started downloading ${event.data.contentLength} bytes`);
                            break;
                        case "Progress":
                            downloaded += event.data.chunkLength;
                            setContentDownloaded(downloaded);
                            break;
                        case "Finished":
                            break;
                    }
                });
                setStatus("Done");
            } catch (error) {
                // TODO: 错误给提示手工下载更新
            }
            try {
                await relaunch();
            } catch (e) {
                setNeedRestart(true);
                console.error(e);
                warn("failed to relaunch");
            }
        };

        fetchUpdate();
    }, [checkUpdate]);

    const confirmUpdate = useCallback(async () => {
        let downloaded = 0;
        if (!updaterInstancece) {
            return;
        }
        setStatus("DownloadAndInstall");
        await updaterInstancece.downloadAndInstall((event: any) => {
            switch (event.event) {
                case "Started":
                    setContentLength(event.data.contentLength!);
                    info(`started downloading ${event.data.contentLength} bytes`);
                    break;
                case "Progress":
                    downloaded += event.data.chunkLength;
                    setContentDownloaded(downloaded);
                    break;
                case "Finished":
                    info("download finished");
                    break;
            }
        });
        setStatus("Done");
        try {
            await relaunch();
        } catch (e) {
            setNeedRestart(true);
            console.error(e);
            warn("failed to relaunch");
        }
    }, [updaterInstancece]);

    const restart_app = () => {
        invoke("restart_application");
    };

    const onClose = () => {
        setUpdaterInstancece(null);
        setStatus(null);
    };

    if (!updaterInstancece && status !== "Up-to-date") return null;

    return (
        <div className={styles.updateBannerContainer}>
            <Button className={styles.closeButton} icon={<CloseCircleOutlined />} type="text" onClick={onClose} />
            <h4>
                {status === "Up-to-date" && "新版本可以下载"}
                {status === "DownloadAndInstall" && "下载更新中…"}
                {status === "Done" && "安装完成"}
                {status === "Error" && "Error occurred"}
                {!status && updaterInstancece && "New version available"}
            </h4>
            {status === "Up-to-date" && <Button onClick={confirmUpdate}>更新到 {beautifyVersion(updaterInstancece?.version)}</Button>}
            {status === "DownloadAndInstall" && <Progress percent={Math.round(progress)} />}
            {status === 'Done' && needRestart && <Button onClick={restart_app}>重启</Button>}

        </div>
    );
}

export default UpdaterDialog;
