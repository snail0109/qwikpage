import { Form, Input, Space, InputNumber, Col, Row } from 'antd';
import { MinusCircleOutlined, PlusOutlined } from '@ant-design/icons';
import VsEditor from '@/components/VsEditor';
import VariableBind from '@/components/VariableBind/VariableBind';
import styles from '../index.module.less';

const SettingForm = function () {
  return (
    <>
      <Form.Item label="请求头">
        <Form.List name="headers">
          {(fields, { add, remove }) => (
            <div style={{ maxHeight: 180, overflowY: "auto" }}>
              {fields.map(({ name }, index) => (
                <div style={{ marginBottom: fields.length === index + 1 ? 0 : 10, display: "flex", gap: 10, paddingRight: index === 0 ? 38 : 0 }} key={`header-${index}`}>
                  <Form.Item name={[name, 'key']} noStyle>
                    <Input placeholder="请输入Key" />
                  </Form.Item>
                  <Form.Item name={[name, 'value']} noStyle>
                    <VariableBind placeholder="请输入Value" />
                  </Form.Item>
                  <PlusOutlined className={styles.variableIcon} onClick={() => add({ key: '', value: '' })} />
                  {index > 0 && (
                    <MinusCircleOutlined
                      className={styles.variableIcon}
                      onClick={() => {
                        remove(name);
                      }}
                    />
                  )}
                </div>
              ))}
            </div>
          )}
        </Form.List>
      </Form.Item>
      <Form.Item label="超时时间" name="timeout">
        <InputNumber style={{ width: '100%' }} addonAfter="秒" />
      </Form.Item>
      <Form.Item label="超时提示" name="timeoutErrorMessage">
        <Input placeholder="请输入超时提示" />
      </Form.Item>
      <Form.Item label="请求适配" name="requestInterceptor">
        <VsEditor />
      </Form.Item>
      <Form.Item label="返回适配" name="responseInterceptor">
        <VsEditor />
      </Form.Item>
    </>
  );
};

export default SettingForm;
