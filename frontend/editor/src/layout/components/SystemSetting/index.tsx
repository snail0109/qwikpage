import { useState, useEffect, MutableRefObject, useImperativeHandle } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open as oepnDataDir } from "@tauri-apps/plugin-dialog";
import { Modal, Form, Select, Input, Checkbox, Button } from "antd";
import { EllipsisOutlined } from "@ant-design/icons";
import usePreferencesStore from "@/stores/preferencesStore";
import { rest } from "lodash-es";

export type ISystemSettingRef = {
    open: () => void;
};

interface ISystemSettingProps {
    settingRef: MutableRefObject<ISystemSettingRef | undefined>;
}

export function SystemSetting(props: ISystemSettingProps) {
    const [visible, setVisible] = useState(false);
    const { set_preferences, get_system_fonts, systemFontFamilys, restore_preferences, ...rest } = usePreferencesStore();
    const [form] = Form.useForm();

    useImperativeHandle(props.settingRef, () => ({
        async open() {
            setVisible(true);
            fetchSystemFonts();
        },
    }));

    // 获取系统字体列表
    const fetchSystemFonts = async () => {
        await get_system_fonts();
    };

    useEffect(() => {
        form.setFieldsValue({
            fontfamily: rest.fontFamily,
            dataDir: rest.projectPath,
            update: rest.checkUpdate,
        });
    }, [form, rest]);

    // 保存
    const handleOk = async () => {
        await onUpdate();
        setVisible(false);
    };

    // 取消
    const handleCancel = () => {
        setVisible(false);
    };

    // 打开配置目录
    const onOpenPreferencesDir = async () => {
        return await invoke<void>("open_preferences");
    };

    // 修改数据存放目录
    const handleOpenDir = async () => {
        const defaultDir = form.getFieldValue('dataDir');
        const dirPath = await oepnDataDir({
            multiple: false,
            defaultPath: defaultDir,
            directory: true
        });

        if (!dirPath || dirPath?.length === 0) {
            return;
        }
        form.setFieldValue('dataDir', dirPath);
    }

    const onUpdate = async () => {
        const valid = await form.validateFields();
        if (!valid) return;
        const values = form.getFieldsValue();
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
                <Button color="primary" variant="outlined" onClick={onOpenPreferencesDir} style={{ marginRight: '8px' }}>
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
                    />
                </Form.Item>
                <Form.Item
                    label="数据存放目录"
                    name="dataDir"
                >
                    <Input placeholder={"数据存放目录"} addonAfter={<EllipsisOutlined onClick={handleOpenDir} />} {...props} />
                </Form.Item>
                <Form.Item
                    label="更新"
                    name="update"
                    valuePropName="checked"
                >
                    <Checkbox>自动检查更新</Checkbox>
                </Form.Item>
            </Form>
        </Modal>
    );
}
