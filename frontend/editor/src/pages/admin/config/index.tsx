import { useEffect, useState, memo } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Form, Input, Button, Space, Radio, Switch, Modal, Image } from "antd";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@/utils/AntdGlobal";
import { RollbackOutlined, EditOutlined, SaveOutlined } from "@ant-design/icons";
import ColorPicker from "@/components/ColorPicker";
import ColorRadioGroup from "@/components/RadioColorGroup/RadioColorGroup";
import ProjectLogo from "@/components/ProjectLogo";
import { projectService } from "@/services";
import styles from "./index.module.less";
import LR from "@/assets/image/LR.png";
import UD from "@/assets/image/UD.png";

/**
 * 项目配置
 */
const Config: React.FC = memo(() => {
    const [loading, setLoading] = useState<boolean>(false);
    const [delLoading, setDelLoading] = useState<boolean>(false);
    const [openModal, setOpenModal] = useState(false);
    const [type, setType] = useState<"detail" | "edit" | "create">("detail");
    const [selectedColor, setSelectedColor] = useState("");
    const [groupId, setGroupId] = useState("");
    const [logoUrl, setLogoUrl] = useState("");

    const { id } = useParams();
    const [form] = Form.useForm();
    const navigate = useNavigate();

    // 项目加载
    useEffect(() => {
        if (!id) return;
        projectService.getProjectDetail(id).then((res) => {
            form.setFieldsValue(res);
            setGroupId(res.groupId);
            setSelectedColor(res.themeColor);
            setLogoUrl(res.logo);
        });
    }, []);

    // 项目提交
    const handleSubmit = async () => {
        try {
            await form.validateFields();
            const value = form.getFieldsValue();
            setLoading(true);
            // value 转化成 snake_case格式
            const { menuMode, menuThemeColor, systemThemeColor, ...rest } = value;

            await projectService.updateProject({
                ...rest,
                theme_color: selectedColor,
                system_theme_color: systemThemeColor,
                menu_mode: menuMode,
                menu_theme_color: menuThemeColor,
            });
            message.success("更新成功");
            setLoading(false);
            setType("detail");
        } catch (error) {
            setLoading(false);
        }
    };

    // 删除确认
    const handleDelConfirm = () => {
        setOpenModal(true);
    };
    // 删除提交
    const handleOk = async () => {
        setDelLoading(true);
        try {
            if (id) {
                await projectService.delProject({ id, groupId });
                message.success("删除成功");
                navigate("/projects");
            }
        } catch (error) {
            console.error("删除项目失败:", error);
            message.error("删除失败，请稍后重试");
        } finally {
            setOpenModal(false);
            setDelLoading(false);
        }
    };

    // 属性设置：默认只读，编辑模式下可输入
    const props: {
        disabled: boolean;
        variant: "outlined" | "borderless";
    } = {
        disabled: type === "detail",
        variant: type === "detail" ? "borderless" : "outlined",
    };

    // 设置项目主题色
    const handleColorChange = (value: string) => {
        setSelectedColor(value);
        form.setFieldsValue({ theme_color: value });
    };

    // 上传项目图标
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

        invoke("upload_project_resource", {
            params: {
                file_path: filePath,
            }
        }).then((res) => {
            form.setFieldValue('logo', res)
            setLogoUrl(res as string);
        }).catch(error => {
            console.log(error)
        })
    };

    return (
        <>
            <Form
                form={form}
                initialValues={{
                    isPublic: 1,
                    layout: 1,
                    menuMode: "inline",
                    menuThemeColor: "dark",
                    systemThemeColor: "#1677ff",
                    breadcrumb: true,
                    tag: true,
                    footer: false,
                    logo: `${import.meta.env.VITE_CDN_URL}/mars-logo.png`,
                }}
                labelCol={{ span: 4 }}
                wrapperCol={{ span: 12 }}
                className={styles.form}
                size="middle"
                onFinish={handleSubmit}
            >
                <Form.Item label="项目ID" name="id" hidden>
                    <Input />
                </Form.Item>
                <h3>基础配置</h3>
                <Form.Item label="项目名称" name="name" rules={[{ required: true, message: "请输入项目名称" }]}>
                    <Input placeholder={"项目名称: Mars"} {...props} maxLength={15} showCount />
                </Form.Item>
                <Form.Item label="项目描述" name="remark">
                    <Input.TextArea
                        placeholder={"请输入项目描述"}
                        rows={3}
                        maxLength={100}
                        showCount={type !== "detail"}
                        {...props}
                    />
                </Form.Item>
                <Form.Item label="选择主题色" name="theme_color">
                    <ColorRadioGroup
                        selectedValue={selectedColor}
                        onChange={handleColorChange}
                        disabled={type === "detail"}
                    />
                </Form.Item>
                <Form.Item label="LOGO" name="logo" rules={[{ required: true, message: "请上传项目Logo" }]}>
                    <ProjectLogo disabled={type === "detail"} logoUrl={logoUrl} handleUpload={handleUpload} />
                </Form.Item>
                <h3>系统配置</h3>
                <Form.Item label="系统布局" name="layout">
                    <Radio.Group
                        {...props}
                        onChange={(event) =>
                            form.setFieldValue("menuMode", event.target.value === 1 ? "inline" : "horizontal")
                        }
                    >
                        <Radio value={1}>
                            <img style={{ width: 100 }} src={LR} alt="左右布局" />
                        </Radio>
                        <Radio value={2}>
                            <img style={{ width: 100 }} src={UD} alt="上左右下布局" />
                        </Radio>
                    </Radio.Group>
                </Form.Item>
                <Form.Item noStyle shouldUpdate>
                    {(form: any) => {
                        const layout = form.getFieldValue("layout");
                        return layout === 1 ? (
                            <Form.Item label="菜单模式" name="menuMode">
                                <Radio.Group {...props} buttonStyle="solid">
                                    <Radio.Button value="vertical">垂直</Radio.Button>
                                    <Radio.Button value="inline">内嵌</Radio.Button>
                                </Radio.Group>
                            </Form.Item>
                        ) : (
                            <Form.Item label="菜单模式" name="menuMode">
                                <Radio.Group {...props} buttonStyle="solid">
                                    <Radio.Button value="horizontal">水平</Radio.Button>
                                </Radio.Group>
                            </Form.Item>
                        );
                    }}
                </Form.Item>
                <Form.Item label="菜单主题" name="menuThemeColor">
                    <Radio.Group {...props}>
                        <Radio value="dark">深色</Radio>
                        <Radio value="light">浅色</Radio>
                    </Radio.Group>
                </Form.Item>
                <Form.Item label="系统主题" name="systemThemeColor">
                    <ColorPicker {...props} />
                </Form.Item>
                <Form.Item label="面包屑" name="breadcrumb" valuePropName="checked">
                    <Switch {...props} />
                </Form.Item>
                <Form.Item label="多页签" name="tag" valuePropName="checked">
                    <Switch {...props} />
                </Form.Item>
                <Form.Item label="页脚" name="footer" valuePropName="checked">
                    <Switch {...props} />
                </Form.Item>
                <div className={styles.editBtn}>
                    {type === "detail" ? (
                        <Space>
                            <Button
                                type="primary"
                                icon={<EditOutlined />}
                                onClick={() => {
                                    setType("edit");
                                }}
                            >
                                编辑
                            </Button>
                            <Button icon={<RollbackOutlined />} onClick={() => navigate("/projects")}>
                                返回
                            </Button>
                        </Space>
                    ) : (
                        <Space>
                            <Button type="primary" icon={<SaveOutlined />} loading={loading} onClick={handleSubmit}>
                                保存
                            </Button>
                            <Button icon={<RollbackOutlined />} onClick={() => setType("detail")}>
                                取消
                            </Button>
                        </Space>
                    )}
                </div>
                <h3>危险区域</h3>
                <div className={styles.delBtn}>
                    <Button danger type="primary" onClick={handleDelConfirm} loading={delLoading}>
                        删除项目
                    </Button>
                </div>
            </Form>

            {/* 项目删除弹框 */}
            <Modal
                open={openModal}
                title="项目删除确认"
                centered
                onOk={() => handleOk()}
                onCancel={() => setOpenModal(false)}
                footer={[
                    <Button key="back" onClick={() => setOpenModal(false)}>
                        关闭
                    </Button>,
                    <Button key="link" type="primary" danger loading={delLoading} onClick={() => handleOk()}>
                        删除所有数据
                    </Button>,
                ]}
            >
                <p>1. 删除项目，会彻底删除项目本身、菜单列表以及归属页面列表。</p>
                <p>2. 删除项目后，您将无法找回，请慎重操作！</p>
            </Modal>
        </>
    );
});

export default Config;
