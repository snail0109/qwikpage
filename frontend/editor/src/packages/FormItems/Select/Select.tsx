import { ComponentType } from "@/packages/types";
import { Form, Select, FormItemProps, SelectProps } from "antd";
import { forwardRef, useEffect, useImperativeHandle, useState } from "react";
import { handleApi } from "@/packages/utils/handleApi";
import { isNotEmpty } from "@/packages/utils/util";
import { useFormContext } from "@/packages/utils/context";
import { usePageStore } from "@/stores/pageStore";
import { isObject } from "lodash-es";
import { isArray } from "lodash-es";

/* 泛型只需要定义组件本身用到的属性，当然也可以不定义，默认为any */
export interface IConfig {
    defaultValue: string;
    formItem: FormItemProps;
    formWrap: SelectProps;
    field: {
        label: string;
        value: string;
    };
    source: Array<{ label: string; value: any }>;
}
/**
 *
 * @param config 组件配置属性值
 * @param props 系统属性值：componentid、componentname等
 * @returns 返回组件
 */
const MSelect = ({ id, formItemValue, type, config, onChange }: ComponentType<IConfig>, ref: any) => {
    const { initValues, getValue, inForm } = useFormContext();
    const [data, setData] = useState<Array<{ label: string; value: any }>>([]);
    const [visible, setVisible] = useState(true);
    const [disabled, setDisabled] = useState<boolean | undefined>();
    const variableData = usePageStore((state) => state.page.pageData.variableData);
    // 初始化默认值
    useEffect(() => {
        const name: string = config.props.formItem?.name || id;
        // 特殊处理下拉框的 value
        let value: string | number = config.props.defaultValue;
        // 处理被引号包裹的字符串
        if (typeof value === "string") {
            // 去除外层的单引号或双引号
            if ((value.startsWith('"') && value.endsWith('"')) || (value.startsWith("'") && value.endsWith("'"))) {
                value = value.slice(1, -1);
            } else {
                if (value && !isNaN(Number(value))) {
                    value = Number(value);
                }
            }
        }
        initValues(type, name, value);
    }, [JSON.stringify(config.props.defaultValue)]);

    // 启用和禁用
    useEffect(() => {
        if (typeof config.props.formWrap.disabled === "boolean") setDisabled(config.props.formWrap.disabled);
    }, [config.props.formWrap.disabled]);

    useEffect(() => {
        getDataList({});
    }, [config.api, config.api?.sourceType == "variable" ? variableData : ""]);

    // 列表加载
    const getDataList = (data: any) => {
        handleApi(config.api, data).then((res) => {
            if (res?.code === 0) {
                if (!Array.isArray(res.data)) {
                    console.error("[select]", "data数据格式错误，请检查");
                    setData([]);
                } else {
                    // 判断是否需要做数据转换
                    let options = [];
                    if (config.props.field.label === "label" && config.props.field.value === "value") {
                        options = res.data;
                        if (typeof res.data[0] === "string" || typeof res.data[0] === "number") {
                            options = res.data.map((item: string | number) => {
                                return { label: item, value: item };
                            });
                        }
                    } else {
                        options = res.data.map((item: any) => {
                            const label = item[config.props.field.label || "label"];
                            const value = item[config.props.field.value || "value"];
                            return {
                                label: isNotEmpty(label) ? label : "-",
                                value: isNotEmpty(value) ? value : "",
                            };
                        });
                    }
                    setData(options);
                }
            }
        });
    };

    useImperativeHandle(ref, () => {
        return {
            show() {
                setVisible(true);
            },
            hide() {
                setVisible(false);
            },
            enable() {
                setDisabled(false);
            },
            disable() {
                setDisabled(true);
            },
            update: (data: any) => {
                // 重新加载表格数据
                getDataList(data);
            },
            getValue: () => {
                const name = config.props.formItem?.name || id;
                return getValue(name);
            },
            setValue: (value: any) => {
                const name = config.props.formItem?.name || id;
                if (isObject(value) && value[name]) {
                    initValues(type, name, value[name]);
                } else if (isArray(value)) {
                    initValues(type, name, value);
                } else {
                    console.error("[select]", "setValue参数错误，请检查", value);
                }
            },
        };
    });

    const handleChange = (val: any) => {
        const name = config.props.formItem?.name || id;
        if (!inForm) {
            // 控件不在表单内需要自行维护值
            initValues(type, name, val);
        }
        onChange?.(val);
    };

    return (
        visible && (
            <Form.Item {...config.props.formItem} data-id={id} data-type={type}>
                <Select
                    {...config.props.formWrap}
                    disabled={disabled}
                    value={formItemValue}
                    options={data}
                    style={config.style}
                    onChange={(val) => handleChange(val)}
                />
            </Form.Item>
        )
    );
};
export default forwardRef(MSelect);
