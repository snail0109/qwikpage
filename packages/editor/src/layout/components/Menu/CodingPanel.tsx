import { Button, Flex, Row, Space } from "antd";
import Editor, { loader } from "@monaco-editor/react";
import { useRef, useEffect, useState } from "react";
import api from "@/invokeApi/page";
import { usePageStore } from "@/stores/pageStore";
import { message } from "@/utils/AntdGlobal";
import { save } from "@tauri-apps/plugin-dialog";
import { invoke } from "@tauri-apps/api/core";

/**
 * 代码面板
 */
const CodingPanel = () => {
    const editorRef = useRef<any>(null);
    const [loading, setLoading] = useState(false);
    const [exportLoading, setExportLoading] = useState(false);
    const { theme, page, savePageInfo } = usePageStore((state) => ({
        theme: state.theme,
        page: state.page,
        savePageInfo: state.savePageInfo,
    }));

    // 初始化monaco，默认为jsdelivery分发，由于网络原因改为本地cdn
    loader.config({
        paths: {
            vs: `https://cdn.jsdelivr.net/npm/monaco-editor@0.50.0/dev/vs`,
        },
    });

    function handleEditorDidMount(editor: { getValue: () => string }) {
        editorRef.current = editor;
        editorRef.current?.setValue(JSON.stringify({ page }, null, 2));
    }

    useEffect(() => {
        editorRef.current?.setValue(JSON.stringify({ page }, null, 2));
    }, [page]);

    // 保存页面状态
    const handleSave = async (event: React.MouseEvent) => {
        event.stopPropagation();
        let value;

        try {
            value = JSON.parse(editorRef.current?.getValue()) || {};
        } catch (error) {
            value = {};
            message.error("页面数据格式异常，请检查重试");
            return;
        }
        const { name, remark, pageData } = value.page;
        const params = {
            id: page.id,
            name,
            remark,
            pageData: JSON.stringify({ ...pageData, variableData: {}, formData: {} }),
        };
        setLoading(true);
        try {
            await api.updatePageData(params);
            setLoading(false);
            savePageInfo({
                ...params,
                pageData: JSON.parse(params.pageData),
            });
            message.success("保存成功");
        } catch (error) {
            setLoading(false);
        }
    };

    // 导出DSL JSON
    const handleExport = async (event: React.MouseEvent) => {
        event.stopPropagation();
        let value;

        try {
            value = JSON.parse(editorRef.current?.getValue()) || {};
        } catch (error) {
            value = {};
            message.error("页面数据格式异常，请检查重试");
            return;
        }
        const { name, remark, pageData } = value.page;
        const jsonData = {
            id: page.id,
            name,
            remark,
            pageData: JSON.stringify({ ...pageData, variableData: {}, formData: {} }),
        };
        setExportLoading(true);
        try {
            // 弹出保存文件的对话框
            const filePath = await save({
                filters: [
                    {
                        name: "JSON",
                        extensions: ["json"],
                    },
                ],
            });

            if (filePath) {
                // 调用后端命令，将 JSON 数据写入文件
                await invoke("export_json", { filePath, jsonData });
                alert("JSON 文件导出成功！");
                setExportLoading(false);
            } else {
                setExportLoading(false);
                console.log("用户取消了保存操作");
            }
        } catch (error) {
            console.error("导出 JSON 文件失败:", error);
        }
    };

    return (
        <Row style={{ marginLeft: "-14px", marginRight: 1 }}>
            <Editor
                height="calc(100vh - 170px)"
                language="json"
                theme={theme === "dark" ? "vs-dark" : "vs-light"}
                options={{
                    lineNumbers: "on",
                    minimap: {
                        enabled: false,
                    },
                }}
                onMount={handleEditorDidMount}
            />
            <Flex gap="small" style={{ width: "100%" }}>
                <Button
                    block
                    type="primary"
                    loading={loading}
                    style={{ margin: 15 }}
                    onClick={(event) => handleSave(event)}
                >
                    保存
                </Button>
                <Button block loading={exportLoading} style={{ margin: 15 }} onClick={(event) => handleExport(event)}>
                    导出
                </Button>
            </Flex>
        </Row>
    );
};

export default CodingPanel;
