import { Button } from "antd";
import usePreferencesStore from "@/stores/preferencesStore";
import { invoke } from "@tauri-apps/api/core";
import { Modal } from "antd";
import React, { MutableRefObject, useImperativeHandle } from "react";

export type ISystemSettingRef = {
    open: () => void;
};

interface ISystemSettingProps {
    settingRef: MutableRefObject<ISystemSettingRef | undefined>;
}

export function SystemSetting(props: ISystemSettingProps) {
    const [visible, setVisible] = React.useState(false);
    const { set_preferences, ...rest } = usePreferencesStore();

    useImperativeHandle(props.settingRef, () => ({
        async open() {
            setVisible(true);
        },
    }));

    const handleOk = () => {
        setVisible(false);
    };

    const handleCancel = () => {
        setVisible(false);
    };

    const onOpenDirClick = async () => {
        return await invoke<void>("open_folder");
    };

    const onUpdate = async (key: string, value: any) => {
        await set_preferences(key, value)
    };

    return (
        <Modal
            title="系统设置"
            open={visible}
            onOk={handleOk}
            onCancel={handleCancel}
            width={500}
            okText="确定"
            cancelText="取消"
        >
            <div>系统设置</div>
            {JSON.stringify(rest)}
            <Button
                onClick={() => {
                    onUpdate("fontSize", 10 * Math.random());
                }}
            >
                修改字体大小
            </Button>
            <Button
                onClick={() => {
                    onUpdate("codeBuildPath", "");
                }}
            >
                修改主题
            </Button>
        </Modal>
    );
}
