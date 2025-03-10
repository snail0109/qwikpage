import { useState,useEffect,  MutableRefObject, useImperativeHandle } from "react";
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
    const [visible, setVisible] = useState(false);
    const { set_preferences, get_system_fonts, systemFontFamilys, ...rest } = usePreferencesStore();
    const [form] = Form.useForm();

    useImperativeHandle(props.settingRef, () => ({
        async open() {
            setVisible(true);
            fetchSystemFonts();
        },
    }));

    const fetchSystemFonts = async () => {
        await get_system_fonts();
    };

    useEffect(() => {
        form.setFieldsValue({
            fontfamily: rest.fontFamily,
            dataDir: rest.projectPath,
            update: rest.checkUpdate || true,
        });
    }, [form, rest]);

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
        const values = await form.validateFields();
        await set_preferences({
            ...rest,
            fontFamily: values.fontfamily,
            projectPath: values.dataDir,
            checkUpdate: values.update,
        });
        setVisible(false);
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
                        options={(systemFontFamilys || []).map(font => ({ label: font, value: font }))}
                        // fieldNames={{ label: 'name', value: 'id' }}
                    />
                </Form.Item>
                <Form.Item
                    label="数据存放目录"
                    name="dataDir"
                >
                    <Input placeholder={"数据存放目录"} addonAfter={<EllipsisOutlined onClick={() => {}} />} {...props} />
                </Form.Item>
                <Form.Item
                    label="更新"
                    name="update"
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
