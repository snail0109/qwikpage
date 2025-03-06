import { invoke } from "@tauri-apps/api/core";
import { Modal } from "antd";
import React, { MutableRefObject, useImperativeHandle } from "react";

export type ISystemSettingRef = {
    open: () => void 
}

interface ISystemSettingProps {
    settingRef: MutableRefObject<ISystemSettingRef | undefined>
}

export function SystemSetting(props: ISystemSettingProps) {
    const [visible, setVisible] = React.useState(false);

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
        </Modal>
    );
}
