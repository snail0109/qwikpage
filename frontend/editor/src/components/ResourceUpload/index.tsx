
import { useImperativeHandle, useState, forwardRef, memo, useMemo } from "react";
import { InboxOutlined } from '@ant-design/icons';
import type { UploadProps, UploadFile } from 'antd';
import { message, Upload, Modal } from 'antd';

interface IResourceUploadProps {
  uploadRef: any;
}

const { Dragger } = Upload;
export default function ResourceUpload(props: IResourceUploadProps) {
  const [visible, setVisible] = useState(false);
  const [fileList, setFileList] = useState<UploadFile[]>([]);
  const [uploading, setUploading] = useState(false);

  // 暴露方法
  useImperativeHandle(props.uploadRef, () => ({
    open() {
      setVisible(true);
    },
    close() {
      setVisible(false);
    }
  }));

  const uploadProps: UploadProps = {
    name: 'file',
    maxCount: 50,
    multiple: true,
    // showUploadList: false,
    beforeUpload(file) {
      const isValidSize = file.size / 1024 / 1024 < 20;
      if (!isValidSize) {
        message.error(`${file.name} 文件体积不能超过20MB！`);
        setFileList([])
      } else {
        setFileList([...fileList, file]);
      }
      return false;
    },
    onChange(info) {
      // const { status } = info.file;
      // if (status !== 'uploading') {
      //   console.log(info.file, info.fileList);
      // }
      // if (status === 'done') {
      //   message.success(`${info.file.name} 文件上传成功.`);
      // } else if (status === 'error') {
      //   message.error(`${info.file.name} 文件上传失败.`);
      // }
      console.log(">>>>aaaaaaa", info)
    },
    onDrop(e) {
      console.log('Dropped files', e.dataTransfer.files);
    }
  }

  return (
    <Modal
      title="上传资源"
      open={visible}
      footer={null}
      centered
      width={500}
      onCancel={() => setVisible(false)}
    >
      <Dragger {...uploadProps}>
        <p className="ant-upload-drag-icon">
          <InboxOutlined />
        </p>
        <p className="ant-upload-text">拖拽静态资源到这里，或<a>点此上传</a></p>
        <p className="ant-upload-hint">
          每次最多上传50个文件，单张图片体积不超过20MB
        </p>
      </Dragger>
    </Modal>
  )
}