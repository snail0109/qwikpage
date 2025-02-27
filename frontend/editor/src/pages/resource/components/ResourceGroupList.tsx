import { useMemo, useState, useEffect } from "react";
import { ImageViewer, FontViewer } from "@/components/ResourcePreview";
import { Col, Row, Collapse } from "antd";
import { PlusOutlined } from "@ant-design/icons";
import EmptyBox from "@/components/EmptyBox/EmptyBox";
import GroupTitle from "@/components/GroupTitle";
import { RESOURCE_TABS } from "../index";
import { useResource } from "@/context/resource";
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

interface IResourceGroupProps {
  name: string;
  resources: Array<IResourceInfo>;
}

interface IResourceGroupListProps {
  data: IResourceGroup[];
}

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
  }, [resource_type, props.name, props.last_modified_time]);

  const colConfig = useMemo(() => {
    let config: any = {
      xs: 12,
      sm: 12,
      md: 8,
      lg: 8,
      xl: 6,
      xxl: 4,
    };
    switch (resource_type) {
      case RESOURCE_TABS[1].value:
        config = { xs: 8, sm: 6, md: 4, lg: 3, xl: 2, xxl: 2 };
        break;
      case RESOURCE_TABS[0].value:
      default:
        break;
    }
    return config;
  }, [resource_type]);

  return <Col {...colConfig}>{resourceInfo}</Col>;
}

function ResourceGroup(props: IResourceGroupProps) {
  const { name, resources } = props;
  const { onImport } = useResource();

  return (
    <div className="resource-group">
      <div style={{ margin: 14 }}>
        {resources.length > 0 ? (
          <Row gutter={[16, 24]} justify="start">
            {resources.map((item, index) => {
              return <ResourceInfo key={index} {...item} resource_name={name} />;
            })}
          </Row>
        ) : (
          <EmptyBox title="该分组下暂无静态资源，请上传" lastCharsCount={2} onCreate={() => onImport(name)} />
        )}
      </div>
    </div>
  );
}

function ResourceGroupList(props: IResourceGroupListProps) {
  const { data, ...rest } = props;
  const { onImport, onEditGroup } = useResource();
  const [activeKeys, setActiveKeys] = useState<string[]>([]);

  useEffect(() => {
    setActiveKeys(data.map((item) => item.path));
  }, [data]);
  const onChange = (key: string[]) => {
    setActiveKeys(key);
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
            onCreate={() => onImport(item.name)}
            onUpdateGroup={onEditGroup}
          />
        ),
        children: <ResourceGroup {...rest} {...item} />,
      }))}
    ></Collapse>
  );
}

export default ResourceGroupList;
