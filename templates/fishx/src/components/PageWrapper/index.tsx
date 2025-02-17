import { Layout } from "antd";
import React, { useMemo } from "react";

function PageWrapper(props) {
    const calcHeight = useMemo(() => {
        return `calc(100vh - 64px)`;
    }, []);
    return (
        <Layout
            style={{ padding: 20, backgroundColor: "#f3f5f9", height: calcHeight, overflow: "auto", ...props.style }}
        >
            {props.children}
        </Layout>
    );
    
}


export {
    PageWrapper
}