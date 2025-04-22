const IconCard = (props: any) => {
    const { width = "20px", height = "20px" } = props;
    return (
        <svg width={width} height={height} viewBox="0 0 20 20" version="1.1" xmlns="http://www.w3.org/2000/svg">
            <g stroke="none" strokeWidth="1" fill="none" fillRule="evenodd">
                <g transform="translate(-65, -552)">
                    <g transform="translate(60, 548)">
                        <g id="表单项" transform="translate(5, 4)">
                            <rect id="矩形" stroke="#777E8C" x="0.5" y="0.5" width="19" height="19" rx="3"></rect>
                            <line
                                x1="4.5"
                                y1="5.5"
                                x2="15.5"
                                y2="5.5"
                                stroke="#216EF7"
                                strokeLinecap="round"
                            ></line>
                            <line
                                x1="4.5"
                                y1="9.5"
                                x2="15.5"
                                y2="9.5"
                                stroke="#777E8C"
                                strokeLinecap="round"
                            ></line>
                            <line
                                x1="4.5"
                                y1="13.5"
                                x2="15.5"
                                y2="13.5"
                                stroke="#777E8C"
                                strokeLinecap="round"
                            ></line>
                        </g>
                    </g>
                </g>
            </g>
        </svg>
    );
};
export default IconCard;
