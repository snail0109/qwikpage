import VariableBind from "@/components/VariableBind/VariableBind";
import { MinusCircleOutlined, PlusOutlined } from "@ant-design/icons";
import { Form, Radio, Input, Switch, Space, Col, Row } from "antd";
import styles from '../index.module.less';

const SettingForm = function () {
    const formLayout = {
        labelCol: { span: 12 },
        wrapperCol: { span: 12 },
    };
    return (
        <>
            <Form.Item label="接口ID" name="id" hidden>
                <Input />
            </Form.Item>
            <Row gutter={80}>
                <Col span={12} >
                    <Form.Item
                        label="接口名称"
                        name="name"
                        rules={[
                            {
                                required: true,
                            },
                        ]}
                    >
                        <Input placeholder="请输入接口中文名称，eg: 用户列表" maxLength={20} showCount />
                    </Form.Item>
                </Col>
                <Col span={12} >
                    <Form.Item
                        label="请求方式"
                        name="method"
                        rules={[
                            {
                                required: true,
                            },
                        ]}
                    >
                        <Radio.Group buttonStyle="solid">
                            <Radio.Button value="GET">GET</Radio.Button>
                            <Radio.Button value="POST">POST</Radio.Button>
                            <Radio.Button value="PUT">PUT</Radio.Button>
                            <Radio.Button value="PATCH">PATCH</Radio.Button>
                            <Radio.Button value="DELETE">DELETE</Radio.Button>
                        </Radio.Group>
                    </Form.Item>
                </Col>
            </Row>
            <Form.Item label="接口地址" extra="支持模板语法：${id}，前提是事件流中有该字段。">
                <Space direction="vertical" style={{ width: "100%" }}>
                    <Form.Item
                        label="STG地址"
                        name="stgApi"
                        noStyle
                        rules={[
                            {
                                required: true,
                            },
                        ]}
                    >
                        <Input placeholder="http://mars-api.marsview.cc/api/user" />
                    </Form.Item>
                </Space>
            </Form.Item>
            <Row gutter={80}>
                <Col span={12}>
                    <Form.Item label="数据格式" name="contentType">
                        <Radio.Group buttonStyle="solid">
                            <Radio.Button value="application/json">JSON</Radio.Button>
                            <Radio.Button value="multipart/form-data">FormData</Radio.Button>
                            <Radio.Button value="application/x-www-form-urlencoded">Form</Radio.Button>
                        </Radio.Group>
                    </Form.Item>
                </Col>
                <Col span={12}>
                    <Form.Item
                        label="参数替换"
                        name="replaceData"
                        tooltip={
                            <>
                                <p>1. 默认会把事件流参数和自定义参数合并；</p>
                                <p>2. 覆盖参数是事件流中上下文的参数覆盖自定义参数；</p>
                                <p>3. 保留参数为只取自定义参数，参数支持模板语法：{"${id}"}。</p>
                            </>
                        }
                    >
                        <Radio.Group buttonStyle="solid">
                            <Radio.Button value="merge">合并参数</Radio.Button>
                            <Radio.Button value="cover">覆盖参数</Radio.Button>
                            <Radio.Button value="reserve">保留参数</Radio.Button>
                        </Radio.Group>
                    </Form.Item>
                </Col>
            </Row>


            <Form.Item label="发送参数" wrapperCol={{ span: 24 }}>
                <Form.List name="params">
                    {(fields, { add, remove }) => (
                        <div style={{ maxHeight: 180, overflowY: "auto" }}>
                            {fields.map(({ name }, index) => (
                                <div
                                    style={{ marginBottom: fields.length === index + 1 ? 0 : 10, alignItems: "center", display: "flex", gap: 10, paddingRight: index === 0 ? 38 : 0 }}
                                    key={`header-${index}`}
                                >
                                    <Form.Item name={[name, "key"]} noStyle >
                                        <Input placeholder="请输入参数名" />
                                    </Form.Item>
                                    <Form.Item name={[name, "value"]} noStyle>
                                        <VariableBind placeholder="请输入参数值" />
                                    </Form.Item>
                                    <PlusOutlined className={styles.variableIcon} onClick={() => add({ key: "", value: "" })} />
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
            {/* <Form.Item name="sendOn" extra={'用表达式来设置该请求的发送条件'} label="发送条件">
        <Input />
      </Form.Item> */}

            <Form.Item label="开启代理" name="isCors">
                <Switch />
                <span className={styles.corsExtra}>开启接口代理对解决跨域问题很有用</span>
            </Form.Item>
        </>
    );
};

export default SettingForm;
