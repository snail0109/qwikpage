import React, { MutableRefObject, useImperativeHandle } from "react";
import { invoke } from "@tauri-apps/api/core";
import { Modal, Form, Select, Input, Checkbox, Button } from "antd";
import { EllipsisOutlined } from "@ant-design/icons";
import usePreferencesStore from "@/stores/preferencesStore";

export type ISystemSettingRef = {
    open: () => void;
};

interface ISystemSettingProps {
    settingRef: MutableRefObject<ISystemSettingRef | undefined>;
}

export function SystemSetting(props: ISystemSettingProps) {
    const [visible, setVisible] = React.useState(false);
    const { set_preferences, ...rest } = usePreferencesStore();
    const { systemFontFamilys } = usePreferencesStore.getState();
    const [form] = Form.useForm();

    useImperativeHandle(props.settingRef, () => ({
        async open() {
            setVisible(true);
        },
    }));

    const handleOk = () => {
        setVisible(false);
        onUpdate();
    };

    const handleCancel = () => {
        setVisible(false);
    };

    const onOpenDirClick = async () => {
        return await invoke<void>("open_folder");
    };

    const onUpdate = async () => {
        await set_preferences({
            theme: "light",
            language: "zh",
            fontSize: 20,
            fontBold: "bold",
            fontFamily: "PingFang SC",
            checkUpdate: true,
            projectPath: "/Users/dxy/Download",
            systemFontFamilys,
        });
    };

    const customFooter = () => (
        <div style={{ display: 'flex', justifyContent: 'space-between' }}>
            <div>
                <Button color="primary" variant="outlined" onClick={onOpenDirClick} style={{ marginRight: '8px' }}>
                    打开配置目录
                </Button>
                <Button color="primary" variant="outlined" onClick={onUpdate}>
                    重置为默认
                </Button>
            </div>

            <div>
                <Button onClick={handleCancel} style={{ marginRight: '8px' }}>
                    取消
                </Button>
                <Button type="primary" onClick={handleOk}>
                    保存
                </Button>
            </div>
        </div>
    );

    return (
        <Modal
            title="系统设置"
            open={visible}
            onCancel={handleCancel}
            width={500}
            footer={customFooter}
        >
            <Form form={form} layout="vertical" autoComplete="off">
                <Form.Item
                    label="字体"
                    name="fontfamily"
                >
                    <Select
                        placeholder="请选择字体"
                        options={systemFontFamilys}
                        fieldNames={{ label: 'name', value: 'id' }}
                    // optionRender={(option) => (

                    // )}
                    />
                </Form.Item>
                <Form.Item
                    label="数据存放目录"
                    name="fontfamily"
                >
                    <Input placeholder={"数据存放目录"} addonAfter={<EllipsisOutlined onClick={() => {}} />} {...props} />
                </Form.Item>
                <Form.Item
                    label="更新"
                    name="fontfamily"
                    valuePropName="checked"
                >
                    <Checkbox>自动检查更新</Checkbox>
                </Form.Item>
            </Form>
            {/* {JSON.stringify(rest)}
            <Button
                onClick={() => {
                    onUpdate();
                }}
            >
                修改字体大小
            </Button>
            <Button
                onClick={() => {
                    onUpdate();
                }}
            >
                修改主题
            </Button> */}
        </Modal>
    );
}
