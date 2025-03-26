import { Form, Input, InputNumber, Alert, Col, Row } from "antd";
import { InfoCircleOutlined } from "@ant-design/icons";
const ReturnStructure = function () {
    return (
        <>
            <Alert message="用来定义接口返回结构，推荐结构：{`{ code: 0, data: {}, msg: '' }`}" type="info" showIcon />
            <Row gutter={80}>
                <Col span={12}>
                    <Form.Item
                        label="业务码"
                        name={["result", "code"]}
                        tooltip={
                            <>
                                <p>接口返回业务状态码，默认是：code</p>
                            </>
                        }
                    >
                        <Input placeholder="默认为：code" maxLength={15} showCount />
                    </Form.Item>
                </Col>
                <Col span={12}>
                    <Form.Item
                        label="成功值"
                        name={["result", "codeValue"]}
                        wrapperCol={{ span: 24 }}
                        tooltip={
                            <>
                                <p>接口返回成功时对应的状态码值，默认是：0</p>
                            </>
                        }
                    >
                        <InputNumber style={{ width: '100%' }} placeholder="默认为：0" />
                    </Form.Item>
                </Col>
            </Row>
            <Row gutter={80}>
                <Col span={12}>
                    <Form.Item
                        label="结果字段"
                        name={["result", "data"]}
                        tooltip={
                            <>
                                <p>接口返回结果字段，默认是：data</p>
                            </>
                        }
                    >
                        <Input placeholder="默认为：data" maxLength={15} showCount />
                    </Form.Item>
                </Col>
                <Col span={12}>
                    <Form.Item
                        label="报错字段"
                        name={["result", "msg"]}
                        tooltip={
                            <>
                                <p>接口返回报错字段，默认是：msg</p>
                            </>
                        }
                    >
                        <Input placeholder="默认为：msg" maxLength={15} showCount />
                    </Form.Item>
                </Col>
            </Row>
        </>
    );
};

export default ReturnStructure;
