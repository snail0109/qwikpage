import React from 'react';
const TooltipWrapper = ({ children, ...props }: any) => {
    // 克隆子元素并传递所有属性
    // return React.cloneElement(children, props);
    return (
        <span {...props}>{children}</span>
    )
};

export default TooltipWrapper;