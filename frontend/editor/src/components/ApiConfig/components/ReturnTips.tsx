import { Form, Input, Switch, Col, Row } from "antd";
import styles from '../index.module.less';
const ReturnTips = function () {
  return (
    <>
      <Form.Item label="默认成功提示" name={["tips", "success"]}>
        <Input placeholder="默认不提示" maxLength={50} showCount />
      </Form.Item>
      <Form.Item label="默认失败提示" name={["tips", "fail"]}>
        <Input placeholder="默认不提示" maxLength={50} showCount />
      </Form.Item>
      <Row>
        <Col span={12}>
          <Form.Item
            label="接口成功提示"
          >
            <Form.Item name={["tips", "isSuccess"]}
              valuePropName="checked" noStyle>
              <Switch />
            </Form.Item>
            <span className={styles.corsExtra}>开启后，会优先使用接口返回成功信息。</span>
          </Form.Item>
        </Col>
        <Col span={12}>
          <Form.Item
            label="接口报错提示"
          >
            <Form.Item name={["tips", "isError"]}
              valuePropName="checked" noStyle>
              <Switch />
            </Form.Item>
            <span className={styles.corsExtra}>开启后，会优先使用接口返回错误信息。</span>
          </Form.Item>
        </Col>
      </Row>
    </>
  );
};

export default ReturnTips;
