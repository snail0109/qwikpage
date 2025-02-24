import { convertFileSrc } from "@tauri-apps/api/core";
import { Flex } from "antd";
import React from "react";

interface IProps {
    name: string;
    path: string;
    file_type: string;
    last_modified_time: string;
}

export default function ImageViewer(props: IProps) {
    const { name, path, file_type, last_modified_time } = props;
    const src = convertFileSrc(path);
    return (
        <div style={{ width: '100%', height:'100%', display: 'flex', flexDirection: 'column'}} >
            <img src={src} alt="Response preview" style={{ width: '100%', height:'100%'}}/>
            <Flex justify="space-between">
                <div>{name}</div>
                <div>{last_modified_time}</div>
            </Flex>
        </div>
    );
}
