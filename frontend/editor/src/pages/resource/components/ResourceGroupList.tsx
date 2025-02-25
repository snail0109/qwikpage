import { useMemo, useState, useEffect, createContext } from "react";
import { ImageViewer, FontViewer } from "@/components/ResourcePreview";
import { Col, Row, message, Collapse } from "antd";
import { PlusOutlined } from "@ant-design/icons";
import { open } from "@tauri-apps/plugin-dialog";
import { resourceService } from "@/services";
import EmptyBox from "@/components/EmptyBox/EmptyBox";
import GroupTitle from "@/components/GroupTitle";
import styles from "./resource.module.less";

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
  onImport: (name: string) => void;
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
    } else {
      return <div>None Viewer</div>;
    }
  }, [file_type]);

  return <Col xs={12} sm={12} md={8} lg={8} xl={6} xxl={4}>{resourceInfo}</Col>;
}

function ResourceGroup(props: IResourceGroupProps) {
  const { name, resources, resource_type, project_id, refresh, onImport } = props;

  const onEditGroupClick = () => {
    props.handleEditResGroup(name);
  };

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
  };

  return (
    <div className="resource-group">
      {/* <Flex justify="space-between" style={{ height: "20px", borderBottom: "1px solid #F3F3F3" }}>
                {name}
                <span onClick={onEditGroupClick}>编辑</span>
                <span onClick={onDeleteGroupClick}>删除</span>
                <div onClick={onImportClick}>上传</div>
            </Flex> */}
      <div style={{ margin: 14 }}>
        {resources.length > 0 ? (
          <Row gutter={16} justify="start">
            {resources.map((item, index) => {
              return <ResourceInfo key={index} {...item} />;
            })}
          </Row>
        ) : (
          <EmptyBox title="该分组下暂无静态资源，请上传" lastCharsCount={2} onCreate={() => onImport(name)} />
        )}
      </div>
    </div>
  );
}

const ResourceGroupContext = createContext<IResourceGroup | null>(null);

function ResourceGroupList(props: IResourceGroupListProps) {
  const { data, ...rest } = props;
  const { project_id, resource_type, refresh } = rest;
  const [activeKeys, setActiveKeys] = useState<string[]>([]);

  useEffect(() => {
    setActiveKeys(data.map((item) => item.path));
  }, [data])
  const onChange = (key: string[]) => {
    setActiveKeys(key);
  };

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

  const updateGroupName = async () => {
    return false;
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
            groupItem={{ id: item.path, name: item.name }}
            createText="上传"
            onCreate={() => onImportClick(item.name)}
            onUpdateGroup={updateGroupName}
          />
        ),
        children: <ResourceGroup {...rest} {...item} onImport={onImportClick} />,
      }))}
    ></Collapse>
  );
}

export default ResourceGroupList;
