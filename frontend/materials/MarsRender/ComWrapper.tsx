const TooltipWrapper = ({ children, ...props }: any) => {
    return (
        <span {...props}>{children}</span>
    )
};

export default TooltipWrapper;