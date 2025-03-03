import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import React, { useState } from "react";
import { useOsInfo } from "@/utils/os";
import { Button, Flex } from "antd";
import { CloseOutlined, MinusOutlined } from "@ant-design/icons";
import { useLocation } from "react-router-dom";
export const WINDOW_CONTROLS_WIDTH = "8rem";

interface Props {
    className?: string;
    onlyX?: boolean;
    macos?: boolean;
}

export function WindowControls({ className, onlyX }: Props) {
    const [maximized, setMaximized] = useState<boolean>(false);
    const osInfo = useOsInfo();
    const location = useLocation();


    // Never show controls on macOS
    if (osInfo.osType === 'macos') {
        return null;
    }

    return (
        <Flex justify="end" style={{ width: WINDOW_CONTROLS_WIDTH }} data-tauri-drag-region>
            <Button type="text" onClick={() => getCurrentWebviewWindow().minimize()}>
                <MinusOutlined style={{ color: location.pathname === '/project/pages' ? '#fff' : '#000' }} />
            </Button>
            <Button
                type="text"
                onClick={async () => {
                    const w = getCurrentWebviewWindow();
                    await w.toggleMaximize();
                    setMaximized(await w.isMaximized());
                }}
                style={{ color: location.pathname === '/project/pages' ? '#fff' : '#000' }}
            >
                {maximized ? (
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
                        <g fill="currentColor">
                            <path d="M3 5v9h9V5zm8 8H4V6h7z" />
                            <path fillRule="evenodd" d="M5 5h1V4h7v7h-1v1h2V3H5z" clipRule="evenodd" />
                        </g>
                    </svg>
                ) : (
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16">
                        <path fill="currentColor" d="M3 3v10h10V3zm9 9H4V4h8z" />
                    </svg>
                )}
            </Button>
            <Button type="text" onClick={() => getCurrentWebviewWindow().close()}>
                <CloseOutlined style={{ color: location.pathname === '/project/pages' ? '#fff' : '#000' }} />
            </Button>
        </Flex>
    );
}
