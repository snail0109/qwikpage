import { useMemo, useState, useEffect } from "react";
import { ImageViewer, FontViewer } from "@/components/ResourcePreview";
import { Col, Row, message, Collapse, Modal } from "antd";
import { PlusOutlined, ExclamationCircleFilled } from "@ant-design/icons";
import { open } from "@tauri-apps/plugin-dialog";
import { resourceService } from "@/services";
import EmptyBox from "@/components/EmptyBox/EmptyBox";
import GroupTitle from "@/components/GroupTitle";
import { IOperResourceGroupParams } from "@/services/resource";
import { RESOURCE_TABS } from "../index";
import { ResourceGroupProvider, useResource } from "@/context/resource";
import styles from "./resource.module.less";

interface IResourceInfo {
  name: string;
  path: string;
  file_type: string;
  last_modified_time: string;
}

interface IResourceInfoProp extends IResourceInfo {
  resource_name: string;
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
}

interface IResourceGroupProps extends ICommonProps {
  name: string;
  resources: Array<IResourceInfo>;
}

interface IResourceGroupListProps extends ICommonProps {
  data: IResourceGroup[];
}

const { confirm } = Modal;

function ResourceInfo(props: IResourceInfoProp) {
  // 根据 resource_type 使用不同的展示组件,
  const { resource_type } = useResource();
  const resourceInfo = useMemo(() => {
    switch (resource_type) {
      case RESOURCE_TABS[0].value:
        return <ImageViewer {...props} />;
      case RESOURCE_TABS[1].value:
        return <FontViewer {...props} />;
      default:
        return <div>None Viewer</div>;
    }
  }, [resource_type]);

  return (
    <Col xs={12} sm={12} md={8} lg={8} xl={6} xxl={4}>
      {resourceInfo}
    </Col>
  );
}

function ResourceGroup(props: IResourceGroupProps) {
  // const context = useContext(ResourceGroupContext);
  const { name, resources } = props;
  const { onImport } = useResource();

  return (
    <div className="resource-group">
      <div style={{ margin: 14 }}>
        {resources.length > 0 ? (
          <Row gutter={16} justify="start">
            {resources.map((item, index) => {
              return <ResourceInfo key={index} {...item} resource_name={name} />;
            })}
          </Row>
        ) : (
          <EmptyBox
            title="该分组下暂无静态资源，请上传"
            lastCharsCount={2}
            onCreate={() => onImport(name)}
          />
        )}
      </div>
    </div>
  );
}

function ResourceGroupList(props: IResourceGroupListProps) {
  const { data, ...rest } = props;
  const { project_id, resource_type, refresh } = rest;
  const [activeKeys, setActiveKeys] = useState<string[]>([]);

  useEffect(() => {
    setActiveKeys(data.map((item) => item.path));
  }, [data]);
  const onChange = (key: string[]) => {
    setActiveKeys(key);
  };

  // 上传资源
  const onImportClick = async (name: string) => {
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
        resource_type,
        group_name: name,
        file_list: filePaths!,
      })
      .then(() => {
        message.success("导入成功");
        refresh();
      });
  };

  // 编辑分组
  const onEditGroupClick = async (oldName: string, newName: string) => {
    try {
      const cmdParams: IOperResourceGroupParams = {
        group_name: oldName,
        new_group_name: newName,
        project_id: rest.project_id,
        resource_type: rest.resource_type,
      };
      await resourceService.update_resource_group(cmdParams);
      props.refresh();
      return true;
    } catch (error) {
      message.error("修改失败,请重试");
      console.error("修改失败", error);
    }
    return false;
  };

  // 删除分组
  // const onDeleteGroupClick = (name: string) => {
  //   resourceService
  //     .delete_resource_group({
  //       project_id,
  //       resource_type: resource_type,
  //       group_name: name,
  //     })
  //     .then(() => {
  //       message.success("删除成功");
  //       refresh();
  //     });
  // };

  // 删除资源
  const onDeleteResourceClick = async (groupName: string, resourceName: string) => {
    const fileType = RESOURCE_TABS.find((item) => item.value === resource_type)?.label;
    confirm({
      title: `确认要删除该${fileType}吗?`,
      icon: <ExclamationCircleFilled />,
      onOk() {
        resourceService
          .delete_resource({
            project_id,
            resource_type,
            group_name: groupName,
            resource_name: resourceName,
          })
          .then(() => {
            message.success("删除成功");
            refresh();
          });
      },
    });
  };

  return (
    <Collapse
      className={styles.resourceGroup}
      ghost
      activeKey={activeKeys}
      expandIcon={({ isActive }) => <PlusOutlined rotate={isActive ? 90 : 0} />}
      onChange={onChange}
      items={data.map((item) => ({
        key: item.path,
        label: (
          <GroupTitle
            groupItem={{ id: item.name, name: item.name }}
            createText="上传"
            onCreate={() => onImportClick(item.name)}
            onUpdateGroup={onEditGroupClick}
          />
        ),
        children: (
          <ResourceGroupProvider
            resource_type={resource_type}
            onImport={onImportClick}
            onDelete={onDeleteResourceClick}
          >
            <ResourceGroup {...rest} {...item} />
          </ResourceGroupProvider>
        ),
      }))}
    ></Collapse>
  );
}

export default ResourceGroupList;
