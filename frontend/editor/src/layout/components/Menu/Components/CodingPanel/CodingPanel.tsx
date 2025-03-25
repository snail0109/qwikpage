import { Row } from "antd";
import Editor, { loader } from "@monaco-editor/react";
import { useRef, useEffect, useState, useCallback } from "react";
import { usePageStore } from "@/stores/pageStore";
import styles from "./index.module.less";
import SearchBar from "./SearchBar";

/**
 * 代码面板
 */
const CodingPanel = () => {
    const editorRef = useRef<any>(null);
    const monacoRef = useRef<any>(null);
    const [loading, setLoading] = useState(false);
    const [exportLoading, setExportLoading] = useState(false);
    const [initialSearchText, setInitialSearchText] = useState('');
    const [editorReady, setEditorReady] = useState(false);
    const { theme, page, savePageInfo } = usePageStore((state) => ({
        theme: state.theme,
        page: state.page,
        savePageInfo: state.savePageInfo,
    }));

    // 初始化monaco，默认为jsdelivery分发，由于网络原因改为本地cdn
    loader.config({
        paths: {
            vs: window.location.origin + '/monaco-editor/0.52.2/min/vs',
        },
        'vs/nls': { availableLanguages: { '*': 'zh-cn' } }
    });

    function handleEditorDidMount(editor: any, monaco: any) {
        editorRef.current = editor;
        monacoRef.current = monaco;
        editorRef.current?.setValue(JSON.stringify({ page }, null, 2));

        // 设置编辑器已准备好
        setEditorReady(true);

        // 添加自定义快捷键
        editor.addCommand(monaco.KeyMod.CtrlCmd | monaco.KeyCode.KeyF, () => {
            // 获取当前选中的文本作为搜索初始值
            const selection = editor.getSelection();
            let selectedText = '';

            if (selection && !selection.isEmpty()) {
                const model = editor.getModel();
                if (model) {
                    selectedText = model.getValueInRange(selection);
                }
            }

            // 始终设置初始搜索文本，无论是否为空
            setInitialSearchText(selectedText);
        });
    }

    useEffect(() => {
        editorRef.current?.setValue(JSON.stringify({ page }, null, 2));
    }, [page]);

    return (
        <Row style={{ marginRight: 1, position: 'relative' }}>
            <Editor
                height="calc(100vh - 36px)"
                language="json"
                className={styles.dslEditor}
                theme={theme === "dark" ? "vs-dark" : "vs-light"}
                options={{
                    lineNumbers: "on",
                    minimap: {
                        enabled: false,
                    },
                    find: {
                        addExtraSpaceOnTop: false, // 搜索时不要添加额外的空格
                        seedSearchStringFromSelection: 'never', // 搜索时不要从选中的文本开始
                        autoFindInSelection: 'never', // 搜索时不要在选中的文本中自动查找
                    },
                }}
                onMount={handleEditorDidMount}
            />
            {editorReady && (
                <SearchBar
                    editor={editorRef.current}
                    visible={true}
                    initialSearchText={initialSearchText}
                />
            )}
        </Row>
    );
};

export default CodingPanel;
