import { useState, useEffect, useImperativeHandle, forwardRef } from "react";
import { Typography } from "antd";
import { ComponentType } from "@materials/types";
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
            setValue: (value: any) => {
                // 判断 value 类型 是否为字符串
                if (typeof value !== "string") {
                  console.error("setValue 方法的参数必须是字符串", value);
                  setText("setValue 方法的参数必须是字符串")
                  return;
                }
                setText(value)
            },
            getValue: () => {
                return text;
            },
        };
    });
    const handleClick = () => {
        onClick?.();
    };

    // 根据 hiddenText 属性设置文本样式
    const getTextStyle = () => {
        const hiddenText = config.props?.hiddenText;
        const style = { ...config.style };

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
                {...omit(config.props, ["script", "text", "hiddenText", 'formItem'])}
                onClick={handleClick}
            >
                {text}
            </Typography.Text>
        )
    );
};
export default forwardRef(MText);
