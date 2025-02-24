import { Input, Modal, Form, Button, Image, Radio } from 'antd';
import { useImperativeHandle, useState, forwardRef, memo } from 'react';
import { projectService } from '@/services';
import { message } from '@/utils/AntdGlobal';
import TextArea from 'antd/es/input/TextArea';
import styles from "./index.module.less";

const colorOptions: { label: string; value: string }[] = [
  { label: 'blue', value: 'blue' },
  { label: 'purple', value: 'purple' },
  { label: 'red', value: 'red' },
  { label: 'green', value: 'green' },
]

/**
 * 创建项目
 */
const CreateProject = (props: { createRef: any; update?: () => void }, ref: any) => {
  const [form] = Form.useForm();
  const [visible, setVisible] = useState(false);
  const [loading, setLoading] = useState(false);
  const [color, setColor] = useState('blue');
  const [groupId, setGroupId] = useState<string | undefined>(undefined);

  // 暴露方法
  useImperativeHandle(props.createRef, () => ({
    open(type: string, groupId?: string) {
      form.resetFields();
      setGroupId(groupId);
      setVisible(true);
    },
  }));

  // 提交
  const handleOk = async () => {
    try {
      await form.validateFields();
      const values = form.getFieldsValue();
      setLoading(true);
      await projectService.addProject({ ...values, group_id: groupId });
      message.success('项目初始化成功');
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
  return (
    <Modal title="新增项目" open={visible} confirmLoading={loading} onCancel={handleCancel} width={500} footer={null}>
      <Form
        layout="vertical"
        form={form}
        labelCol={{ span: 5 }}
        wrapperCol={{ span: 24 }}
        initialValues={{
          logo: '/imgs/qwikpage-logo.png',
          theme_color: 'blue',
        }}
      >
        <Form.Item label="名称" name="name" rules={[{ required: true, message: '请输入页面名称' }]}>
          <Input placeholder="请输入项目名称" maxLength={15} showCount />
        </Form.Item>
        <Form.Item label="描述" name="remark">
          <TextArea autoSize={{ minRows: 4, maxRows: 6 }} placeholder="请输入描述" maxLength={100} showCount />
        </Form.Item>
        <Form.Item label="选择主题色" name="theme_color">
          <Radio.Group optionType='button' className={styles.themeColor}>
            {colorOptions.map((option) => {
              const selectedColor = form.getFieldValue('theme_color');
              return (
                <Radio key={option.value} value={option.value}>
                  <div
                    style={{
                      backgroundColor: option.value,
                      padding: '15px',
                      borderWidth: '2px',
                      borderStyle: 'solid',
                      borderColor: selectedColor === option.value ? '#D3E4FF' : option.value,
                      borderRadius: '4px',
                    }}
                  />
                </Radio>
              );
            })}
          </Radio.Group>
        </Form.Item>
        <Form.Item label="图标" name="logo" rules={[{ required: true, message: '请上传项目Logo' }]}>
          <Image width={100} src="/imgs/qwikpage-logo.png" />
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
