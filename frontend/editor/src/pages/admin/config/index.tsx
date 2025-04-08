import { useEffect, useState, memo } from "react";
import { useNavigate, useParams } from "react-router-dom";
import { Form, Input, Button, Space, Radio, Switch, Modal } from "antd";
import { open } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";
import { message } from "@/utils/AntdGlobal";
import { ArrowLeftOutlined, EllipsisOutlined } from "@ant-design/icons";
import ColorPicker from "@/components/ColorPicker";
import ColorRadioGroup from "@/components/RadioColorGroup/RadioColorGroup";
import RadioButtonGroup from "@/components/RadioButtonGroup";
import ProjectLogo from "@/components/ProjectLogo";
import { projectService } from "@/services";
import styles from "./index.module.less";
import LR from "@/assets/image/LR.png";
import UD from "@/assets/image/UD.png";
import project from "@/pages/home/project";
import { GlobalHotKeys } from "react-hotkeys";
import { keyMap } from "@/constants/hotKeys";

const MenuModeOptions = [
    [
        {
            label: "垂直",
            value: "vertical",
        },
        {
            label: "内嵌",
            value: "inline",
        },
    ],
    [
        {
            label: "水平",
            value: "horizontal",
        },
    ],
];

const MenyThemeColor = [
    {
        label: "深色",
        value: "dark",
    },
    {
        label: "浅色",
        value: "light",
    },
];

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
            const { menuMode, menuThemeColor, systemThemeColor, codeExportPath, ...rest } = value;

            await projectService.updateProject({
                ...rest,
                theme_color: selectedColor,
                system_theme_color: systemThemeColor,
                menu_mode: menuMode,
                menu_theme_color: menuThemeColor,
                code_export_path: codeExportPath,
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
    // 删除项目
    const handleOk = async () => {
        setDelLoading(true);
        try {
            if (id) {
                await projectService.delProject({ id, groupId, logoUrl });
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
    } = {
        disabled: type === "detail",
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
                project_id: id,
                old_file_path: logoUrl,
            },
        })
            .then((res) => {
                form.setFieldValue("logo", res);
                setLogoUrl(res as string);
            })
            .catch((error) => {
                console.log(error);
            });
    };

    // 切换自定义radiobutton的事件
    const handleChangeTab = (fieldName: string) => (selectedOption: { label: string; value: string }) => {
        form.setFieldsValue({ [fieldName]: selectedOption.value });
    };

    // 修改导出目录
    const changeCodeExportDir = async (defaultDir: string) => {
        const dirPath = await open({
            multiple: false,
            defaultPath: defaultDir,
            directory: true,
        });

        if (!dirPath || dirPath?.length === 0) {
            return;
        }
        form.setFieldValue("codeExportPath", dirPath);
    };

    const handlers = {
        ESC: (e: KeyboardEvent | undefined) => {
            e?.preventDefault();
            e?.stopPropagation();
            console.log("esc");
            history.back();
        },
    };

    return (
        <GlobalHotKeys keyMap={keyMap} handlers={handlers} allowChanges={true}>
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
                <div className={styles.titleWrap}>
                    <div className={styles.prefixIcon}></div>
                    <div className={styles.title}>项目配置</div>
                </div>
                <Form.Item label="项目名称" name="name" rules={[{ required: true, message: "请输入项目名称" }]}>
                    <Input placeholder={"项目名称: Mars"} {...props} maxLength={15} showCount />
                </Form.Item>
                <Form.Item label="导出目录" name="codeExportPath">
                    <Input
                        placeholder={"导出项目所在的目录"}
                        addonAfter={
                            <EllipsisOutlined
                                onClick={() => changeCodeExportDir(form.getFieldValue("codeExportPath"))}
                            />
                        }
                        {...props}
                    />
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
                        const menuMode = form.getFieldValue("menuMode");
                        const targetOptions = layout === 1 ? MenuModeOptions[0] : MenuModeOptions[1];
                        return (
                            <Form.Item label="菜单模式" name="menuMode">
                                <RadioButtonGroup
                                    disabled={type === "detail"}
                                    options={targetOptions}
                                    selected={menuMode}
                                    onChangeTab={handleChangeTab("menuMode")}
                                />
                            </Form.Item>
                        );
                    }}
                </Form.Item>
                <Form.Item noStyle shouldUpdate>
                    {(form: any) => {
                        const menuThemeColor = form.getFieldValue("menuThemeColor");
                        return (
                            <Form.Item label="菜单主题" name="menuThemeColor">
                                <RadioButtonGroup
                                    disabled={type === "detail"}
                                    options={MenyThemeColor}
                                    selected={menuThemeColor}
                                    onChangeTab={handleChangeTab("menuThemeColor")}
                                />
                            </Form.Item>
                        );
                    }}
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
                            <Button icon={<ArrowLeftOutlined />} onClick={() => navigate("/projects")}></Button>
                            <Button
                                type="primary"
                                onClick={() => {
                                    setType("edit");
                                }}
                            >
                                编辑
                            </Button>
                            <Button color="danger" variant="outlined" onClick={handleDelConfirm} loading={delLoading}>
                                删除项目
                            </Button>
                        </Space>
                    ) : (
                        <Space>
                            <Button icon={<ArrowLeftOutlined />} onClick={() => navigate("/projects")}></Button>
                            <Button type="primary" loading={loading} onClick={handleSubmit}>
                                保存
                            </Button>
                            <Button color="danger" variant="outlined" onClick={handleDelConfirm} loading={delLoading}>
                                删除项目
                            </Button>
                        </Space>
                    )}
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
        </GlobalHotKeys>
    );
});

export default Config;
