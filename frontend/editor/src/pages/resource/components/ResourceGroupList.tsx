import { useMemo, useState, useEffect } from "react";
import { ImageViewer, FontViewer, FileViewer } from "@/components/ResourcePreview";
import { Col, Flex, Collapse } from "antd";
import EmptyBox from "@/components/EmptyBox/EmptyBox";
import GroupTitle from "@/components/GroupTitle";
import { RESOURCE_TABS } from "../index";
import { useResource } from "@/context/resource";
import styles from "./resource.module.less";
import type { IResource } from '@/types';
import ExpandIcon from "@/assets/icons/ExpandIcon.svg?react";

interface IResourceInfoProp extends IResource {
  resource_name: string;
}

export interface IResourceGroup {
  name: string;
  path: string;
  last_modified_time: string;
  default_group: boolean;
  resources: Array<IResource>;
}

interface IResourceGroupProps {
  name: string;
  path: string;
  resources: Array<IResource>;
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

  // const colConfig = useMemo(() => {
  //   let config: any = {
  //     xs: 12,
  //     sm: 12,
  //     md: 8,
  //     lg: 8,
  //     xl: 6,
  //     xxl: 4,
  //   };
  //   switch (resource_type) {
  //     case RESOURCE_TABS[1].value:
  //       config = { xs: 8, sm: 6, md: 4, lg: 3, xl: 2, xxl: 2 };
  //       break;
  //     case RESOURCE_TABS[0].value:
  //     default:
  //       break;
  //   }
  //   return config;
  // }, [resource_type]);

  return <div>{resourceInfo}</div>;
}

function ResourceContainer(props: IResourceGroupProps) {
  const { name, path, resources } = props;
  const { resource_type } = useResource();
  return (
    [RESOURCE_TABS[0].value, RESOURCE_TABS[1].value].includes(resource_type) ? (
      <Flex gap={25} wrap={true} justify="flex-start">
        {resources.map((item, index) => {
          return <ResourceInfo key={index} {...item} resource_name={name} />;
        })}
      </Flex>
    ) : <FileViewer resource_name={name} resource_path={path} data={resources} />
  )
}

function ResourceGroup(props: IResourceGroupProps) {
  const { name, resources } = props;
  const { onImport } = useResource();

  return (
    <div className="resource-group">
      <div style={{ margin: '14px 0' }}>
        {resources.length > 0 ? <ResourceContainer {...props} /> : (
          <EmptyBox title="该分组下暂无静态资源，请上传" lastCharsCount={2} onCreate={() => onImport(name)} />
        )}
      </div>
    </div>
  );
}

function ResourceGroupList(props: IResourceGroupListProps) {
  const { data, ...rest } = props;
  const { onImport, onEditGroup, onDeleteGroup } = useResource();
  const [activeKeys, setActiveKeys] = useState<string[]>([]);

  useEffect(() => {
    setActiveKeys(data.map((item) => item.path));
  }, [data]);
  const onChange = (key: string[]) => {
    setActiveKeys(key);
  };

  const handleEdit = (oldName: string, newName: string) => {
    return onEditGroup(oldName, newName)
  }

  return (
    <Collapse
      className={styles.resourceGroup}
      collapsible="icon"
      ghost
      activeKey={activeKeys}
      expandIcon={({ isActive }) => (
        <ExpandIcon
          width={20}
          height={20}
          style={{
            transform: isActive ? "rotate(0deg)" : "rotate(-90deg)",
            transition: "transform 0.3s ease",
          }}
        />
      )}
      onChange={onChange}
      items={data.map((item) => ({
        key: item.path,
        label: (
          <GroupTitle
            groupItem={{ id: item.default_group ? '-1' : item.name, name: item.name }}
            createText="上传"
            onCreate={() => onImport(item.name)}
            onDelete={onDeleteGroup}
            onUpdateGroup={(_, newName) => handleEdit(item.name, newName)}
          />
        ),
        children: <ResourceGroup {...rest} {...item} />,
      }))}
    ></Collapse>
  );
}

export default ResourceGroupList;
