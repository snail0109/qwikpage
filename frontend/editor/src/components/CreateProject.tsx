import { Input, Modal, Form, Button, Image, Radio } from "antd";
import { useImperativeHandle, useState, forwardRef, memo } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import { projectService } from "@/services";
import { message } from "@/utils/AntdGlobal";
import TextArea from "antd/es/input/TextArea";
import ColorRadioGroup from "@/components/RadioColorGroup/RadioColorGroup";
import styles from "./index.module.less";

/**
 * 创建项目
 */
const CreateProject = (props: { createRef: any; update?: () => void }, ref: any) => {
  const [form] = Form.useForm();
  const [visible, setVisible] = useState(false);
  const [loading, setLoading] = useState(false);
  const [selectedColor, setSelectedColor] = useState("blue");
  const [groupId, setGroupId] = useState<string | undefined>(undefined);
  const [logoUrl, setLogoUrl] = useState("");

  // 暴露方法
  useImperativeHandle(props.createRef, () => ({
    open(type: string, groupId?: string) {
      form.resetFields();
      setGroupId(groupId);
      setVisible(true);
      setLogoUrl("/imgs/qwikpage-logo.svg");
    },
  }));

  // 提交
  const handleOk = async () => {
    try {
      await form.validateFields();
      const values = form.getFieldsValue();
      setLoading(true);
      await projectService.addProject({ ...values, group_id: groupId });
      message.success("项目初始化成功");
      props.update?.();
      setLoading(false);
      setVisible(false);
    } catch (error) {
      setLoading(false);
    }
  };

  // 关闭
  const handleCancel = () => {
    form.resetFields();
    setVisible(false);
  };

  const handleColorChange = (value: string) => {
    setSelectedColor(value);
    form.setFieldsValue({ theme_color: value });
  };

  // 上传图标
  const handleUpload = async () => {
    const filePath = await open({
      title: "Select File",
      multiple: false,
      filters: [
        {
          name: "Files",
          extensions: ["jpg", "jpeg", "png", "svg"],
        },
      ],
    });

    if (!filePath || filePath?.length === 0) {
      return;
    }

    form.setFieldValue('logo', filePath)
    setLogoUrl(filePath); 
  };

  return (
    <Modal
      title="新增项目"
      open={visible}
      confirmLoading={loading}
      onCancel={handleCancel}
      width={500}
      footer={null}
    >
      <Form
        layout="vertical"
        form={form}
        labelCol={{ span: 5 }}
        wrapperCol={{ span: 24 }}
        initialValues={{
          logo: "/imgs/qwikpage-logo.svg",
          theme_color: "blue",
        }}
      >
        <Form.Item label="名称" name="name" rules={[{ required: true, message: "请输入页面名称" }]}>
          <Input placeholder="请输入项目名称" maxLength={15} showCount />
        </Form.Item>
        <Form.Item label="描述" name="remark">
          <TextArea
            autoSize={{ minRows: 4, maxRows: 6 }}
            placeholder="请输入描述"
            maxLength={100}
            showCount
          />
        </Form.Item>
        <Form.Item label="选择主题色" name="theme_color">
          <ColorRadioGroup selectedValue={selectedColor} onChange={handleColorChange} />
        </Form.Item>
        <Form.Item label="图标" name="logo" rules={[{ required: true, message: "请上传项目Logo" }]}>
          <div className={styles.imageContainer}>
            <Image
              width={100}
              src={logoUrl}
              preview={false}
            />
            <div className={styles.mask} onClick={handleUpload}>
              上传
            </div>
          </div>
        </Form.Item>
        <Form.Item>
          <Button block type="primary" onClick={handleOk} loading={loading}>
            快速初始化
          </Button>
        </Form.Item>
      </Form>
    </Modal>
  );
};

export default memo(forwardRef(CreateProject));
