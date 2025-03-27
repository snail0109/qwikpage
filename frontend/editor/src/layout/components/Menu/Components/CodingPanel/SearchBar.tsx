import { useState, useRef, useEffect } from "react";
import { Row, Input, Button, Space, message, Tooltip } from "antd";
import MatchCaseIcon from "@/assets/icons/MatchCase.svg?react";
import WholeWordIcon from "@/assets/icons/MatchWhole.svg?react";
import ReGexIcon from "@/assets/icons/ReGex.svg?react";
import {
    UpOutlined,
    DownOutlined,
    CaretDownOutlined,
    CaretUpOutlined,
    CloseOutlined,
} from "@ant-design/icons";
import styles from "./searchbar.module.less";

interface SearchBarProps {
    editor: any;
    visible: boolean;
    initialSearchText?: string;
    onClose: () => void;
}

// 搜索栏组件
const SearchBar: React.FC<SearchBarProps> = ({ editor, visible, initialSearchText = '', onClose }) => {
    const [searchText, setSearchText] = useState(initialSearchText);
    const [replaceText, setReplaceText] = useState("");
    const [showReplace, setShowReplace] = useState(false);
    const [matchCase, setMatchCase] = useState(false);
    const [wholeWord, setWholeWord] = useState(false);
    const [useRegex, setUseRegex] = useState(false);
    const [currentMatch, setCurrentMatch] = useState(0);
    const [totalMatches, setTotalMatches] = useState(0);
    const [decorations, setDecorations] = useState<string[]>([]);
    const searchInputRef = useRef<any>(null);

    useEffect(() => {
        if (visible && searchInputRef.current) {
            searchInputRef.current.focus();
        }
    }, [visible]);

    // 清除所有高亮装饰
    const clearDecorations = () => {
        if (editor && decorations.length > 0) {
            editor.deltaDecorations(decorations, []);
            setDecorations([]);
        }
    };

    // 执行查找操作
    const findMatches = () => {
        if (!editor || !searchText) {
            clearDecorations();
            setTotalMatches(0);
            return;
        }

        try {
            // 使用 Monaco 编辑器的模型来查找匹配项
            const model = editor.getModel();
            if (!model) return;

            // 准备搜索选项
            const searchOptions = {
                caseSensitive: matchCase,
                isRegex: useRegex,
                matchWholeWord: wholeWord,
            };

            // 定义单词分隔符
            const wordSeparators = ' ,;.(){}[]<>/"\'`~!@#$%^&*-+=|\\?:';

            // 使用 Monaco 的 API 查找所有匹配
            let matches;
            try {
                matches = model.findMatches(
                    searchText,
                    true, // 搜索整个模型
                    searchOptions.isRegex,
                    searchOptions.caseSensitive,
                    searchOptions.matchWholeWord ? wordSeparators : null,
                    true
                );
            } catch (regexError) {
                console.error("Regex error:", regexError);
                message.error("正则表达式无效，请检查语法");
                clearDecorations();
                setTotalMatches(0);
                return;
            }

            setTotalMatches(matches.length);

            // 高亮所有匹配项
            const decorationsArray = matches.map((match: any) => ({
                range: match.range,
                options: {
                    className: 'search-match',
                    stickiness: 1,
                }
            }));

            // 为当前选中项添加不同的样式
            if (matches.length > 0) {
                const currentIdx = currentMatch % matches.length;
                decorationsArray[currentIdx] = {
                    range: matches[currentIdx].range,
                    options: {
                        className: 'search-match-current',
                        stickiness: 1,
                    }
                };

                // 选中并滚动到当前匹配项
                const range = matches[currentIdx].range;
                editor.setSelection(range);
                editor.revealRangeInCenter(range);
            }

            // 应用所有装饰
            const ids = editor.deltaDecorations(decorations, decorationsArray);
            setDecorations(ids);
        } catch (error) {
            console.error("Search error:", error);
            message.error("搜索出错，请重试");
            clearDecorations();
        }
    };

    // 查找下一个匹配项
    const findNext = () => {
        if (!editor || !searchText || totalMatches === 0) return;

        try {
            const model = editor.getModel();
            if (!model) return;

            const searchOptions = {
                caseSensitive: matchCase,
                isRegex: useRegex,
                matchWholeWord: wholeWord,
            };

            const wordSeparators = ' ,;.(){}[]<>/"\'`~!@#$%^&*-+=|\\?:';

            let matches;
            try {
                matches = model.findMatches(
                    searchText,
                    true,
                    searchOptions.isRegex,
                    searchOptions.caseSensitive,
                    searchOptions.matchWholeWord ? wordSeparators : null,
                    true
                );
            } catch (regexError) {
                console.error("Regex error:", regexError);
                message.error("正则表达式无效，请检查语法");
                return;
            }

            if (matches.length > 0) {
                const nextMatch = (currentMatch + 1) % matches.length;
                setCurrentMatch(nextMatch);

                // 更新高亮状态
                const decorationsArray = matches.map((match: any, idx: number) => ({
                    range: match.range,
                    options: {
                        className: idx === nextMatch ? 'search-match-current' : 'search-match',
                        stickiness: 1,
                    }
                }));

                const range = matches[nextMatch].range;
                editor.setSelection(range);
                editor.revealRangeInCenter(range);

                // 更新装饰
                const ids = editor.deltaDecorations(decorations, decorationsArray);
                setDecorations(ids);
            }
        } catch (error) {
            console.error("Next search error:", error);
        }
    };

    // 查找上一个匹配项
    const findPrevious = () => {
        if (!editor || !searchText || totalMatches === 0) return;

        try {
            const model = editor.getModel();
            if (!model) return;

            const searchOptions = {
                caseSensitive: matchCase,
                isRegex: useRegex,
                matchWholeWord: wholeWord,
            };

            const wordSeparators = ' ,;.(){}[]<>/"\'`~!@#$%^&*-+=|\\?:';

            const matches = model.findMatches(
                searchText,
                true,
                searchOptions.isRegex,
                searchOptions.caseSensitive,
                searchOptions.matchWholeWord ? wordSeparators : null,
                true
            );

            if (matches.length > 0) {
                const prevMatch = (currentMatch - 1 + matches.length) % matches.length;
                setCurrentMatch(prevMatch);

                // 更新高亮状态
                const decorationsArray = matches.map((match: any, idx: number) => ({
                    range: match.range,
                    options: {
                        className: idx === prevMatch ? 'search-match-current' : 'search-match',
                        stickiness: 1,
                    }
                }));

                const range = matches[prevMatch].range;
                editor.setSelection(range);
                editor.revealRangeInCenter(range);

                // 更新装饰
                const ids = editor.deltaDecorations(decorations, decorationsArray);
                setDecorations(ids);
            }
        } catch (error) {
            console.error("Previous search error:", error);
        }
    };

    // 替换当前匹配项
    const replaceOne = () => {
        if (!editor || !searchText || totalMatches === 0) return;

        try {
            const selection = editor.getSelection();
            const model = editor.getModel();

            if (!selection || !model) return;

            // 获取选中文本
            const selectedText = model.getValueInRange(selection);

            // 检查选中文本是否匹配搜索文本
            let matches;
            if (useRegex) {
                try {
                    const flags = matchCase ? 'g' : 'gi';
                    const regex = new RegExp(searchText, flags);
                    matches = selectedText.match(regex);
                } catch (e) {
                    console.error("Invalid regex:", e);
                    return;
                }
            } else if (matchCase) {
                matches = selectedText === searchText;
            } else {
                matches = selectedText.toLowerCase() === searchText.toLowerCase();
            }

            if (matches) {
                // 执行替换操作
                editor.executeEdits('searchReplace', [
                    { range: selection, text: replaceText }
                ]);

                // 重新查找匹配项，更新高亮
                setTimeout(() => {
                    findMatches();
                    if (totalMatches > 0) {
                        findNext();
                    }
                }, 0);
            } else {
                // 如果当前选择不匹配，重新查找
                findMatches();
            }
        } catch (error) {
            console.error("Replace error:", error);
        }
    };

    // 替换所有匹配项
    const replaceAll = () => {
        if (!editor || !searchText || totalMatches === 0) return;

        try {
            const model = editor.getModel();
            if (!model) return;

            const searchOptions = {
                caseSensitive: matchCase,
                isRegex: useRegex,
                matchWholeWord: wholeWord,
            };

            const wordSeparators = ' ,;.(){}[]<>/"\'`~!@#$%^&*-+=|\\?:';

            // 验证正则表达式
            let matches;
            try {
                matches = model.findMatches(
                    searchText,
                    true,
                    searchOptions.isRegex,
                    searchOptions.caseSensitive,
                    searchOptions.matchWholeWord ? wordSeparators : null,
                    true
                );
            } catch (regexError) {
                console.error("Regex error:", regexError);
                message.error("正则表达式无效，请检查语法");
                return;
            }

            // 处理正则表达式中的捕获组
            let processedReplaceText = replaceText;
            if (searchOptions.isRegex) {
                // Monaco 编辑器支持 $1, $2 等替换模式
                // 这里我们不需要特殊处理，Monaco 内部会处理
            }

            // 倒序替换，避免替换时位置偏移
            const edits = matches.reverse().map((match: any) => ({
                range: match.range,
                text: processedReplaceText
            }));

            if (edits.length > 0) {
                editor.executeEdits('searchReplaceAll', edits);
                message.success(`已替换 ${edits.length} 处匹配项`);

                // 清除高亮，因为原始文本已经被替换
                clearDecorations();
                setCurrentMatch(0);
                setTotalMatches(0);
            } else {
                message.info('没有找到匹配项');
            }
        } catch (error) {
            console.error("Replace all error:", error);
        }
    };

    // 关闭三件套
    const handleClose = () => {
        clearDecorations();
        // 清理搜索框
        setSearchText('');
        onClose();
    };

    const handleKeyDown = (e: React.KeyboardEvent) => {
        if (e.key === "Enter") {
            e.preventDefault();
            if (e.shiftKey) {
                findPrevious();
            } else {
                findNext();
            }
        } else if (e.key === "Escape") {
            handleClose();
        }
    };

    // 当搜索文本或搜索选项改变时重置搜索
    useEffect(() => {
        setCurrentMatch(0);
        if (searchText) {
            findMatches();
        } else {
            clearDecorations();
            setTotalMatches(0);
        }
    }, [searchText, matchCase, wholeWord, useRegex]);

    // 当 initialSearchText 变化时更新搜索文本
    useEffect(() => {
        if (initialSearchText) {
            setSearchText(initialSearchText);
        }
    }, [initialSearchText]);

    // 添加对编辑器内容变化的监听
    useEffect(() => {
        if (!editor) return;

        // 监听编辑器内容变化事件
        const disposable = editor.onDidChangeModelContent(() => {
            // 当编辑器内容变化且搜索框有内容时，重新执行搜索
            if (searchText) {
                findMatches();
            }
        });

        // 清理函数，移除事件监听
        return () => {
            disposable.dispose();
        };
    }, [editor, searchText]);

    // 组件卸载时清除装饰
    useEffect(() => {
        return () => {
            clearDecorations();
        };
    }, []);

    if (!visible) {
        clearDecorations();
        return null;
    }

    // 搜索选项图标按钮
    const searchSuffix = (
        <Space className={styles.searchOptions}>
            <Tooltip title="区分大小写">
                <Button
                    type="text"
                    size="small"
                    style={{ fontSize: '20px' }}
                    className={matchCase ? styles.optionActive : styles.optionButton}
                    onClick={() => setMatchCase(!matchCase)}
                    icon={<MatchCaseIcon />}
                />
            </Tooltip>
            <Tooltip title="全词匹配">
                <Button
                    type="text"
                    size="small"
                    style={{ fontSize: '20px' }}
                    className={wholeWord ? styles.optionActive : styles.optionButton}
                    onClick={() => setWholeWord(!wholeWord)}
                    icon={<WholeWordIcon />}
                />
            </Tooltip>
            <Tooltip title="正则表达式">
                <Button
                    type="text"
                    size="small"
                    style={{ fontSize: '20px' }}
                    className={useRegex ? styles.optionActive : styles.optionButton}
                    onClick={() => setUseRegex(!useRegex)}
                    icon={<ReGexIcon />}
                />
            </Tooltip>
        </Space>
    );

    return (
        <div className={styles.searchBar}>
            <Row className={styles.searchRow}>
                {/* <Tooltip title={showReplace ? "隐藏替换" : "显示替换"}> */}
                <Button
                    icon={!showReplace ? <CaretDownOutlined /> : <CaretUpOutlined style={{ color: 'transparent' }} />}
                    onClick={() => setShowReplace(!showReplace)}
                    size="small"
                    type="text"
                > 查找</Button>
                {/* </Tooltip> */}
                <Input
                    ref={searchInputRef}
                    value={searchText}
                    onChange={(e) => setSearchText(e.target.value)}
                    onKeyDown={handleKeyDown}
                    placeholder="查找"
                    className={styles.searchInput}
                    suffix={searchSuffix}
                />
                <span className={styles.matchCount}>
                    {totalMatches > 0 ? `${currentMatch + 1}/${totalMatches}` : '0/0'}
                </span>
                <Space className={styles.buttonGroup}>
                    <Tooltip title="上一个匹配项 (Shift+Enter)">
                        <Button
                            icon={<UpOutlined />}
                            onClick={findPrevious}
                            type="text"
                            size="small"
                        />
                    </Tooltip>
                    <Tooltip title="下一个匹配项 (Enter)">
                        <Button
                            icon={<DownOutlined />}
                            onClick={findNext}
                            type="text"
                            size="small"
                        />
                    </Tooltip>
                    {/* 关闭按钮 */}
                    <Tooltip title="关闭">
                        <Button
                            icon={<CloseOutlined />}
                            onClick={handleClose}
                            type="text"
                            size="small"
                        />
                    </Tooltip>
                </Space>
            </Row>

            {showReplace && (
                <Row className={styles.replaceRow}>
                    {/* <Tooltip title={showReplace ? "隐藏替换" : "显示替换"}> */}
                    <Button
                        icon={<CaretUpOutlined />}
                        onClick={() => setShowReplace(!showReplace)}
                        size="small"
                        type="text"
                    >替换</Button>
                    {/* </Tooltip> */}
                    <Input
                        value={replaceText}
                        onChange={(e) => setReplaceText(e.target.value)}
                        placeholder="替换为"
                        className={styles.replaceInput}
                    />
                    <Space className={styles.buttonGroup}>
                        <Tooltip title="替换当前">
                            <Button
                                onClick={replaceOne}
                                size="small"
                            >
                                替换
                            </Button>
                        </Tooltip>
                        <Tooltip title="替换所有">
                            <Button
                                onClick={replaceAll}
                                size="small"
                            >
                                全部
                            </Button>
                        </Tooltip>
                    </Space>
                </Row>
            )}

        </div>
    );
};

export default SearchBar; 