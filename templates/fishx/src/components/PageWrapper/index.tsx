import { Layout } from "antd";
import React, { useMemo, useCallback } from "react";
import { usePageStore } from '@/stores/pageStore';
import { isNotEmpty, getInitValue } from '@/utils/util';
import { FormContext } from '@/utils/context';

function PageWrapper(props: any) {
    const { formItemData, setFormItemData } = usePageStore((state) => {
        return {
            formItemData: state.page.pageData.formItemData,
            setFormItemData: state.setFormItemData,
        };
    });
    const calcHeight = useMemo(() => {
        return `100vh`;
    }, []);

    const initValues = useCallback((type: string, name: string, value: any) => {
        if (name && isNotEmpty(value)) {
            const initValue = getInitValue(type, value);
            setFormItemData({
                name,
                value: initValue,
            });
        }
    }, []);

    const getValue = useCallback((name: string) => {
        const value = formItemData[name];
        return value;
    }, [formItemData]);
    return (
        <Layout
            style={{ padding: 20, backgroundColor: "#f3f5f9", height: calcHeight, overflow: "auto", ...props.style }}
        >
            <FormContext.Provider value={{ initValues, getValue, inForm: false }}>
                {props.children}
            </FormContext.Provider>
        </Layout>
    );

}


export {
    PageWrapper
}