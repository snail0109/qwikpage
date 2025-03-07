import { Input, Modal, Form } from "antd";
import { useImperativeHandle, useState, forwardRef, memo, useMemo } from "react";
import { resourceService, IOperResourceGroupParams } from "@/services/resource";
import { message } from "@/utils/AntdGlobal";
import { error } from "console";

export interface IOpenParams {
    action: "create" | "edit" | "renameResource";
    group_name?: string;
}

/**
 * 创建分组
 */
const CreateGroup = (
    props: { createRef: any; update?: () => void; project_id: string; resource_type: string; customConfirm?: (name: string) => Promise<boolean> },
    ref: any
) => {
    const [form] = Form.useForm();
    const [visible, setVisible] = useState(false);
    const [loading, setLoading] = useState(false);
    const [type, setType] = useState<"create" | "edit" | "renameResource">("create");
    const [oldGroupName, setOldGroupName] = useState<string>("");

    const getTitle = useMemo(() => {
        if (type === 'create') {
            return '创建分组'
        }
        if (type === 'edit') {
            return '编辑分组'
        }
        return '重命名'
    }, [type]);

    const getLabel = useMemo(() => {
        if (['create', 'edit'].includes(type)) {
            return '分组名称'
        }
        return '资源名称'
    }, [type]);

    // 暴露方法
    useImperativeHandle(props.createRef, () => ({
        open(params: IOpenParams) {
            const { action, group_name } = params;
            setType(action);
            if (['edit', 'renameResource'].includes(action)) {
                setOldGroupName(group_name!);
                form.setFieldsValue({
                    group_name,
                });
            } else {
                form.resetFields();
            }
            setVisible(true);
        },
    }));

    // 提交
    const handleOk = async () => {
        try {
            const valid = await form.validateFields();
            if (!valid) {
                return;
            }
            const values = form.getFieldsValue();
            setLoading(true);
            if (props.customConfirm && type === 'renameResource') {
                const result = await props.customConfirm(values.group_name);
                if (result) {
                    setLoading(false);
                    setVisible(false);
                } else {
                    setLoading(false);
                }
                return;
            }
            const cmdParams: IOperResourceGroupParams = {
                group_name: values.group_name,
                project_id: props.project_id,
                resource_type: props.resource_type,
            };
            let msg = "新建分组成功"
            let serviceFunc = resourceService.add_resource_group;
            if (type === "edit") {
                serviceFunc = resourceService.update_resource_group;
                msg = "编辑分组成功";
                cmdParams.group_name = oldGroupName
                cmdParams.new_group_name = values.group_name
            }

            serviceFunc(cmdParams)
                .then(() => {
                    message.success(msg);
                    props.update?.();
                    setLoading(false);
                    setVisible(false);
                })
                .catch(error => {
                    setLoading(false);
                })

        } catch (error) {
            setLoading(false);
        }
    };

    // 关闭
    const handleCancel = () => {
        form.resetFields();
        setVisible(false);
    };
    return (
        <Modal
            title={getTitle}
            open={visible}
            confirmLoading={loading}
            onOk={handleOk}
            onCancel={handleCancel}
            width={500}
        >
            <Form layout="vertical" form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 24 }}>
                <Form.Item label={getLabel} name="group_name" rules={[{ required: true, message: `请输入${getLabel}` }]}>
                    <Input placeholder={`请输入${getLabel}`} maxLength={55} showCount />
                </Form.Item>
            </Form>
        </Modal>
    );
};

export default memo(forwardRef(CreateGroup));
