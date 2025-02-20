import { Input, Modal, Form, Select, Space, Flex, Button, message } from 'antd';
import { useImperativeHandle, useState, MutableRefObject } from 'react';
import { projectService, pageService } from '@/services';
import { IPage, IProject } from '@/types';
import { useSearchParams } from 'react-router-dom';
import TextArea from 'antd/es/input/TextArea';
import { usePageStore } from '@/stores/pageStore';
/**
 * 创建页面
 */
export interface CreatePageRef {
  open: (action: 'create' | 'edit' | 'copy', record?: IPage) => void;
}
export interface IModalProp {
  createRef: MutableRefObject<{ open: (action: 'create' | 'edit' | 'copy', record?: IPage) => void } | undefined>;
  update?: (status?: string) => void;
  copy?: (record: IProject) => void;
}

const CreatePage = (props: IModalProp) => {
  const [form] = Form.useForm();
  const [visible, setVisible] = useState(false);
  const [type, setType] = useState<'create' | 'edit' | 'copy'>('create');
  const [recordId, setRecordId] = useState("0");
  const [loading, setLoading] = useState(false);
  const [projectList, setProjectList] = useState<IProject[]>([]);
  const [searchParams] = useSearchParams();
  const savePageInfo = usePageStore((state) => state.savePageInfo);
  // 暴露方法
  useImperativeHandle(props.createRef, () => ({
    async open(action: 'create' | 'edit' | 'copy', record?: IPage) {
      const { list = [] } = await projectService.getProjectList({
        pageNum: 1,
        pageSize: 100,
      });
      setProjectList(
        list.map((item: IProject) => {
          return {
            name: item.name,
            id: item.id,
            logo: item.logo,
            remark: item.remark,
          };
        }),
      );
      setType(action);
      if (action === 'edit') {
        record && setRecordId(record.id!);
        form.setFieldsValue(record);
      } else if (action === 'copy') {
        record && setRecordId(record.id!);
        form.setFieldsValue({ ...record, name: `${record?.name}-副本` });
      } else {
        const projectId = searchParams.get('projectId') || record?.projectId;
        setType('create');
        setRecordId("0");
        if (projectId) form.setFieldValue('projectId', projectId);
      }
      setVisible(true);
    },
  }));

  // const handleNameBlur = () => {
  //   const pageName = form.getFieldValue('name');
  //   const pagePath = form.getFieldValue('path');
  //   if (pageName && !pagePath) {
  //     // 默认路由
  //     const defaultPath = pageName.split(' ').map((word: string, index: number) => {
  //       if (index === 0) {
  //         return word.charAt(0).toLowerCase() + word.slice(1);
  //       }
  //       return word.charAt(0).toUpperCase() + word.slice(1);
  //     });
  //     form.setFieldValue('path', `/${defaultPath}`)
  //   }
  // }

  // 提交
  const handleOk = async () => {
    const params = form.getFieldsValue();
    const valid = await form.validateFields();
    if (valid) {
      setLoading(true);
      try {
        if (type === 'create') {
          await pageService.createPageData(params);
          message.success('页面创建成功');
        } else if (type === 'edit') {
          await pageService.updatePageData({
            ...params,
            id: recordId,
          });
          message.success('页面修改成功');
          savePageInfo({
            name: params?.name,
            remark: params?.remark,
            projectId: params?.projectId,
            path: params?.path,
          });
        } else {
          const param = {
            ...params,
            id: recordId,
          }

          await pageService.copyPageData(param);
          message.success('页面复制成功');
        }
        // 编辑器界面 - 左侧菜单修改后刷新
        props.update?.('success');
        form.resetFields();
        setVisible(false);
        setLoading(false);
      } catch (error) {
        setLoading(false);
      }
    }
  };

  // 关闭
  const handleCancel = () => {
    form.resetFields();
    setVisible(false);
    props.update?.('cancel');
  };
  return (
    <Modal
      title={type === 'create' ? '初始页面信息' : type === 'edit' ? '编辑页面' : '复制页面'}
      open={visible}
      confirmLoading={loading}
      onOk={handleOk}
      onCancel={handleCancel}
      width={500}
      okText="确定"
      cancelText="取消"
      footer={type === 'create' ? null : undefined}
    >
      <Form layout="vertical" form={form} labelCol={{ span: 5 }} wrapperCol={{ span: 24 }}>
        <Form.Item label="名称" name="name" rules={[{ required: true, message: '请输入页面名称' }]}>
          <Input placeholder="请输入页面名称" maxLength={15} showCount />
        </Form.Item>
        <Form.Item label="描述" name="remark">
          <TextArea autoSize={{ minRows: 4, maxRows: 6 }} placeholder="请输入描述" maxLength={100} showCount />
        </Form.Item>
        <Form.Item label="页面路由" name="path" rules={[{ required: true, message: '请输入页面路由' }]}>
          <Input placeholder="请输入页面路径，例如: /dashboard" />
        </Form.Item>
        <Form.Item label="所属项目" name="projectId" rules={[{ required: true, message: '请选择所属项目' }]}>
          <Select
            placeholder="请选择所属项目"
            options={projectList}
            fieldNames={{ label: 'name', value: 'id' }}
            optionRender={(option) => (
              <Space>
                <img src={option.data.logo} style={{ maxWidth: 50, maxHeight: 50 }} />
                <Flex vertical>
                  <span>{option.data.name}</span>
                  <span>{option.data.remark}</span>
                </Flex>
              </Space>
            )}
          />
        </Form.Item>

        {type == 'create' ? (
          <Form.Item>
            <Button block type="primary" onClick={handleOk} loading={loading}>
              快速初始化
            </Button>
          </Form.Item>
        ) : null}
      </Form>
    </Modal>
  );
};

export default CreatePage;
