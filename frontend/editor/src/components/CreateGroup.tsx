import { Input, Modal, Form, Button } from "antd";
import { useImperativeHandle, useState, forwardRef, memo } from "react";
import { projectService } from "@/services";
import { message } from "@/utils/AntdGlobal";
import { cmd_invoke } from "@/services/cmd_invoke";

/**
 * 创建分组
 */
const CreateGroup = (props: { createRef: any; update?: () => void }, ref: any) => {
    const [form] = Form.useForm();
    const [visible, setVisible] = useState(false);
    const [loading, setLoading] = useState(false);

    // 暴露方法
    useImperativeHandle(props.createRef, () => ({
        open() {
            form.resetFields();
            setVisible(true);
        },
    }));

    // 提交
    const handleOk = async () => {
        try {
            await form.validateFields();
            const values = form.getFieldsValue();
            setLoading(true);
            cmd_invoke("add_group", { groupName: values.name })
                .then(() => {
                    message.success("新建分组成功");
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
            title="新建分组"
            open={visible}
            confirmLoading={loading}
            onCancel={handleCancel}
            width={500}
            footer={null}
        >
            <Form layout="vertical" form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 24 }}>
                <Form.Item label="名称" name="name" rules={[{ required: true, message: "请输入分组名称" }]}>
                    <Input placeholder="请输入分组名称" maxLength={15} showCount />
                </Form.Item>
                <Form.Item>
                    <Button block type="primary" onClick={handleOk} loading={loading}>
                        提交
                    </Button>
                </Form.Item>
            </Form>
        </Modal>
    );
};

export default memo(forwardRef(CreateGroup));
