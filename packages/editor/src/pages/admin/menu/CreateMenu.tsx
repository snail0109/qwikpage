import { useImperativeHandle, useRef, useState } from "react";
import { useParams } from "react-router-dom";
import { Modal, Form, TreeSelect, Input, Select, InputNumber, Radio, Spin } from "antd";
import { message } from "@/utils/AntdGlobal";
import { IAction, IModalProp } from "@/pages/types";
import { EditParams, MenuItem, Page } from "@/invokeApi/types";
import { getMenuList, addMenu, updateMenu } from "@/invokeApi/menu";
import api from "@/invokeApi/page";
import { arrayToTree } from "@/utils/util";
import CreatePage, { CreatePageRef } from "@/components/CreatePage";
import CustomIconOptions from "@/components/CustomIconList";

export default function CreateMenu(props: IModalProp<EditParams>) {
    const [form] = Form.useForm();
    const createRef = useRef<CreatePageRef>();
    const [action, setAction] = useState<IAction>("create");
    const [visible, setVisible] = useState(false);
    const [menuList, setMenuList] = useState<MenuItem[]>([]);
    const [pageList, setPageList] = useState<Page[]>([]);
    const [loading, setLoading] = useState(false);
    const [confirmLoading, setConfirmLoading] = useState(false);
    const { id: project_id } = useParams();

    useImperativeHandle(props.mRef, () => ({
        open,
    }));

    // 打开弹框函数
    const open = async (type: IAction, data?: EditParams | { parent_id: string }) => {
        setAction(type);
        setVisible(true);
        setLoading(true);
        // 获取菜单列表
        getMenus();
        type === "edit" && getMyPageList();
        setLoading(false);
        if (data && project_id) {
            form.setFieldsValue({ ...data, project_id });
        }
    };

    // 获取菜单列表，生成菜单树
    const getMenus = async () => {
        if (!project_id) return;
        const res = await getMenuList({
            projectId:project_id,
            status: -1,
        });
        // 菜单编辑时，父菜单不能选择自身子菜单，会产生冲突。
        const parent_id = form.getFieldValue("parent_id");
        const filterList = res.filter((item: MenuItem) => {
            return item.menu_type === 1 && item.parent_id != parent_id;
        });
        const menuData = arrayToTree(filterList);
        setMenuList(menuData);
    };

    // 获取用户页面列表
    const getMyPageList = async () => {
        const res = await api.getPageList({ pageNum: 1, pageSize: 50,  projectId: project_id! });
        setPageList(res.list);
    };

    // 菜单提交
    const handleSubmit = async () => {
        const valid = await form.validateFields();
        if (valid) {
            setConfirmLoading(true);
            const values = form.getFieldsValue();
            try {
                const { id, isCreate, page_id, ...rest } = values;
                const params = { ...rest };
                params.page_id = page_id || "0";
                if (action === "create") {
                    await addMenu({ isCreate, params });
                } else {
                    await updateMenu({ id, params });
                }
                setConfirmLoading(false);
                message.success("操作成功");
                handleCancel();
                props.update();
            } catch (error) {
                setConfirmLoading(false);
            }
        }
    };
    // 关闭和重置弹框
    const handleCancel = () => {
        setVisible(false);
        form.resetFields();
    };
    return (
        <>
            <Modal
                title={action === "create" ? "创建菜单" : "编辑菜单"}
                width={800}
                open={visible}
                okText="确定"
                cancelText="取消"
                confirmLoading={confirmLoading}
                onOk={handleSubmit}
                onCancel={handleCancel}
            >
                <Spin spinning={loading}>
                    <Form
                        form={form}
                        labelAlign="right"
                        labelCol={{ span: 4 }}
                        wrapperCol={{ span: 18 }}
                        initialValues={{ menu_type: 1, status: 1, isCreate: 2 }}
                    >
                        <Form.Item hidden name="id">
                            <Input />
                        </Form.Item>
                        <Form.Item hidden name="project_id">
                            <Input />
                        </Form.Item>
                        <Form.Item label="父级菜单" name="parent_id">
                            <TreeSelect
                                placeholder="请选择父级菜单"
                                allowClear
                                treeDefaultExpandAll
                                fieldNames={{ label: "name", value: "id" }}
                                treeData={menuList}
                            />
                        </Form.Item>
                        <Form.Item label="菜单类型" name="menu_type">
                            <Radio.Group>
                                <Radio value={1}>菜单</Radio>
                                <Radio value={2}>按钮</Radio>
                                <Radio value={3}>页面</Radio>
                            </Radio.Group>
                        </Form.Item>
                        <Form.Item noStyle shouldUpdate>
                            {() => {
                                const menu_type = form.getFieldValue("menu_type");
                                return menu_type === 1 ? (
                                    <Form.Item
                                        label="菜单名称"
                                        name="name"
                                        rules={[{ required: true, message: "请输入菜单名称" }]}
                                    >
                                        <Input placeholder="请输入菜单名称" allowClear maxLength={15} showCount />
                                    </Form.Item>
                                ) : menu_type === 2 ? (
                                    <Form.Item
                                        label="按钮名称"
                                        name="name"
                                        rules={[{ required: true, message: "请输入按钮名称" }]}
                                    >
                                        <Input placeholder="请输入按钮名称" allowClear />
                                    </Form.Item>
                                ) : (
                                    <Form.Item
                                        label="页面名称"
                                        name="name"
                                        rules={[{ required: true, message: "请输入页面名称" }]}
                                    >
                                        <Input placeholder="请输入页面名称" allowClear />
                                    </Form.Item>
                                );
                            }}
                        </Form.Item>
                        <Form.Item noStyle shouldUpdate>
                            {() => {
                                const menu_type = form.getFieldValue("menu_type");
                                return  (
                                    <>
                                        {menu_type === 1 ? (
                                            <Form.Item label="菜单图标" name="icon">
                                                <CustomIconOptions />
                                            </Form.Item>
                                        ) : null}
                                        {action === "edit" ? (
                                            <Form.Item
                                                label="绑定页面"
                                                extra={
                                                    <span>
                                                        该菜单可以解绑、修改、新增绑定页面。暂无页面？
                                                        <a
                                                            onClick={() => {
                                                                createRef.current?.open("create", {
                                                                    id: "0",
                                                                    name: "",
                                                                    // @ts-ignore
                                                                    project_id,
                                                                });
                                                            }}
                                                        >
                                                            去创建
                                                        </a>
                                                    </span>
                                                }
                                                name="page_id"
                                            >
                                                <Select
                                                    placeholder="请选择关联页面"
                                                    allowClear
                                                    showSearch
                                                    filterOption={(input, option) =>
                                                        (option?.name ?? "").toLowerCase().includes(input.toLowerCase())
                                                    }
                                                    options={[...pageList, { name: "空页面", id: "0" }]}
                                                    fieldNames={{ label: "name", value: "id" }}
                                                ></Select>
                                            </Form.Item>
                                        ) : (
                                            <Form.Item
                                                label="生成页面"
                                                name="isCreate"
                                                extra="如果你创建的是末级菜单，请给它生成一个页面，父菜单不需要生成。"
                                            >
                                                <Radio.Group>
                                                    <Radio value={1}>是</Radio>
                                                    <Radio value={2}>否</Radio>
                                                </Radio.Group>
                                            </Form.Item>
                                        )}
                                        <Form.Item
                                            label="页面路由"
                                            name="path"
                                            extra="配置页面路由后，访问时会优先使用页面路由"
                                            rules={[
                                                {
                                                    pattern: /^\/?[^\d][a-zA-Z-_/]+.*$/g,
                                                    message: "页面路由不能以数字开头，且不能包含特殊字符",
                                                },
                                            ]}
                                        >
                                            <Input placeholder="请输入页面路径，例如: /dashboard" />
                                        </Form.Item>
                                    </>
                                );
                            }}
                        </Form.Item>

                        <Form.Item label="排序" name="sort_num" extra="排序值越大越靠后。">
                            <InputNumber placeholder="请输入排序值" />
                        </Form.Item>
                        <Form.Item label="菜单状态" name="status" extra="停用后，菜单不会在admin系统中展示。">
                            <Radio.Group>
                                <Radio value={1}>启用</Radio>
                                <Radio value={2}>停用</Radio>
                            </Radio.Group>
                        </Form.Item>
                    </Form>
                </Spin>
            </Modal>
            {/* 创建和修改页面 */}
            <CreatePage createRef={createRef} update={() => getMyPageList()} />
        </>
    );
}
