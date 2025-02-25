import {useMemo } from "react";
import { ImageViewer, FontViewer } from "@/components/ResourcePreview";
import { Col, Row, Flex, message } from "antd";
import { open } from "@tauri-apps/plugin-dialog";
import { resourceService } from "@/services";
import EmptyGroup from "./EmptyGroup";

interface IResourceInfo {
    name: string;
    path: string;
    file_type: string;
    last_modified_time: string;
}

export interface IResourceGroup {
    name: string;
    path: string;
    last_modified_time: string;
    resources: Array<IResourceInfo>;
}

interface ICommonProps {
    project_id: string;
    resource_type: string;
    refresh: () => void;
    handleEditResGroup: (group_name: string) => void;
}

interface IResourceGroupProps extends ICommonProps {
    name: string;
    resources: Array<IResourceInfo>;
}

interface IResourceGroupListProps extends ICommonProps {
    data: IResourceGroup[];
}

function ResourceInfo(props: IResourceInfo) {
    // 根据 file_type 使用不同的展示组件,
    const { file_type } = props;

    // TODO
    const resourceInfo = useMemo(() => {
        // 匹配所有的图片格式
        if (["png", "jpg", "jpeg", "gif"].includes(file_type)) {
            return <ImageViewer {...props} />;
        } else if (["otf", "ttf"].includes(file_type)) {
            return <FontViewer {...props} />;
        }
        else {
            return <div>None Viewer</div>;
        }
    }, [file_type]);

    return <Col span={4}>{resourceInfo}</Col>;
}

function ResourceGroup(props: IResourceGroupProps) {
    const { name, resources, resource_type, project_id, refresh } = props;

    // 导入资源
    const onImportClick = async () => {
        const filePaths = await open({
            title: "Select File",
            multiple: true,
        });

        if (!filePaths || filePaths?.length === 0) {
            return;
        }

        resourceService
            .import_resource({
                project_id,
                resource_type: resource_type,
                group_name: name,
                file_list: filePaths!,
            })
            .then(() => {
                message.success("导入成功");
                refresh();
            });
    };

    const onEditGroupClick = () => {
        props.handleEditResGroup(name)
    }

    const onDeleteGroupClick = () => {
        resourceService
            .delete_resource_group({
                project_id,
                resource_type: resource_type,
                group_name: name,
            })
            .then(() => {
                message.success("删除成功");
                refresh();
            });
    }

    return (
        <div className="resource-group">
            <Flex justify="space-between" style={{ height: "20px", borderBottom: "1px solid #F3F3F3" }}>
                {name} 
                <span onClick={onEditGroupClick}>编辑</span>
                <span onClick={onDeleteGroupClick}>删除</span>
                <div onClick={onImportClick}>上传</div>
            </Flex>
            <div style={{ margin: 14 }}>
                {resources.length > 0 ? (
                    <Row gutter={16}>
                        {resources.map((item, index) => {
                            return <ResourceInfo key={index} {...item} />;
                        })}
                    </Row>
                ) : (
                    <EmptyGroup />
                )}
            </div>
        </div>
    );
}

function ResourceGroupList(props: IResourceGroupListProps) {
    const { data, ...rest } = props;
    return (
        <div className="resource-group-list">
            {data.map((item, index) => {
                return <ResourceGroup key={index}  {...rest} {...item} />;
            })}
        </div>
    );
}


export default ResourceGroupList;