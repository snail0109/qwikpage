import { Input, Modal, Form, Button } from "antd";
import { useImperativeHandle, useState, forwardRef, memo } from "react";
import { resourceService, IOperResourceGroupParams } from "@/services/resource";
import { message } from "@/utils/AntdGlobal";

export interface IOpenParams {
    action: "create" | "edit";
    group_name?: string;
}

/**
 * 创建分组
 */
const CreateGroup = (
    props: { createRef: any; update?: () => void; project_id: string; resource_type: string },
    ref: any
) => {
    const [form] = Form.useForm();
    const [visible, setVisible] = useState(false);
    const [loading, setLoading] = useState(false);
    const [type, setType] = useState<"create" | "edit">("create");
    const [oldGroupName, setOldGroupName] = useState<string>("");

    // 暴露方法
    useImperativeHandle(props.createRef, () => ({
        open(params: IOpenParams) {
            const { action, group_name } = params;
            setType(action);
            if (action === "edit") {
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
                    message.success(type === "create" ? "新建分组成功" : "编辑分组成功");
                    props.update?.();
                })
                .finally(() => {
                    setLoading(false);
                    setVisible(false);
                });
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
            title="创建分组"
            open={visible}
            confirmLoading={loading}
            onOk={handleOk}
            onCancel={handleCancel}
            width={500}
        >
            <Form layout="vertical" form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 24 }}>
                <Form.Item label="分组名称" name="group_name" rules={[{ required: true, message: "请输入分组名称" }]}>
                    <Input placeholder="请输入分组名称" maxLength={15} showCount />
                </Form.Item>
            </Form>
        </Modal>
    );
};

export default memo(forwardRef(CreateGroup));
