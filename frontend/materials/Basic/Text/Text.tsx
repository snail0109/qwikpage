import { useState, useEffect, useImperativeHandle, forwardRef } from "react";
import { Typography } from "antd";
import dayjs from "dayjs";
import { ComponentType } from "@materials/types";
import { formatNumber } from "@materials/utils/util";
import { message } from "@materials/utils/AntdGlobal";
import { omit } from "lodash-es";
/**
 *
 * @param props 组件本身属性
 * @param style 组件样式
 * @returns
 */
const MText = ({ config, onClick }: ComponentType, ref: any) => {
    const [text, setText] = useState("");
    const [visible, setVisible] = useState(true);
    useEffect(() => {
        const originText = config.props?.text?.toString() || "";
        setText(originText);
    }, [config.props.text]);

    // 对外暴露方法
    useImperativeHandle(ref, () => {
        return {
            show() {
                setVisible(true);
            },
            hide() {
                setVisible(false);
            },
        };
    });
    const handleClick = () => {
        onClick?.();
    };

    // 根据 hiddenText 属性设置文本样式
    const getTextStyle = () => {
        const hiddenText = config.props?.hiddenText;
        const style = { display: 'block', ...config.style };

        switch (hiddenText) {
            case "ellipsis":
                style.whiteSpace = "nowrap";
                style.overflow = "hidden";
                style.textOverflow = "ellipsis";
                style.display = "block";
                break;
            case "break":
                style.whiteSpace = "break-spaces";
                style.wordBreak = "break-all";
                break;
            case "wrap":
                style.whiteSpace = "pre-wrap";
                style.wordBreak = "normal";
                break;
            case "nowrap":
                style.whiteSpace = "nowrap";
                break;
            default:
                // 默认不处理
                break;
        }

        return style;
    };

    return (
        visible && (
            <Typography.Text
                style={getTextStyle()}
                {...omit(config.props, ["script", "text", "hiddenText"])}
                onClick={handleClick}
            >
                {text}
            </Typography.Text>
        )
    );
};
export default forwardRef(MText);
