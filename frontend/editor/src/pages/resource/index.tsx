import { resourceService } from "@/services";
import { Button, Form, Input, Layout, Divider, Tooltip } from "antd";
import { useEffect, useRef, useState } from "react";
import styles from "./index.module.less";
import searchBarstyles from "./index.module.less";
import { RedoOutlined, PlusOutlined } from "@ant-design/icons";
import SearchBar from "@/components/Searchbar/SearchBar";
import CreateGroup, { IOpenParams } from "./components/CreateGroup";
import ResourceGroupList, { IResourceGroup } from "./components/ResourceGroupList";

const tabs = [
  {
    label: "图片",
    value: "img",
    placeholder: "请输入图片名称"
  },
  {
    label: "字体",
    value: "font",
    placeholder: "请输入字体名称"
  },
  {
    label: "第三方JS",
    value: "js",
    placeholder: "请输入JS名称"
  },
  {
    label: "附件",
    value: "attachment",
    placeholder: "请输入附件名称"
  },
  {
    label: "其它",
    value: "other",
    placeholder: "请输入其它资源名称"
  },
];

export default function Home() {
  const searchParams = new URLSearchParams(location.search);
  const project_id = searchParams.get('projectId') || undefined;
  const project_name = searchParams.get('projectName') || undefined;
  const [data, setData] = useState<IResourceGroup[]>([]);
  const [resource_type, setResourceType] = useState(tabs[0].value);
  const [loading, setLoading] = useState(true);
  const [placeholder, setPlaceholder] = useState(tabs[0].placeholder);

  const createGroupRef = useRef<{ open: (params: IOpenParams) => void }>();

  const [form] = Form.useForm();

  useEffect(() => {
    setLoading(true);
    resourceService
      .load_resource({
        project_id: project_id!,
        resource_type,
      })
      .then((res) => {
        setData(res);
        console.log("resourceGroup List: ", res);
        setLoading(false);
      })
      .catch((err) => {
        console.log(err);
        setLoading(false);
      });
  }, []);

  const searchSubmit = () => { };

  // 新建资源分组
  const handleAddResGroup = () => {
    createGroupRef.current?.open({ action: "create" });
  };

  const handleEditResGroup = (group_name: string) => {
    createGroupRef.current?.open({
      action: "edit",
      group_name,
    });
  };

  const refresh = () => {
    const keyword = form.getFieldValue("keyword");
    resourceService
      .load_resource({
        // @ts-ignore
        project_id: project_id,
        resource_type,
        keyword,
      })
      .then((res) => {
        setData(res);
        console.log(res);
      });
  };

  return (
    // TODO 抽取公共组件
    <Layout.Content className={searchBarstyles.resourceContainer}>
      {/* 搜索工具条 */}
      <SearchBar className={searchBarstyles.searchBar} showGroup={false} noNeedCreate noNeedFresh form={form} searchPlaceholder={placeholder} projectName={project_name} submit={searchSubmit} refresh={refresh} onCreate={searchSubmit} />
      <Divider />
      <div className={searchBarstyles.topContainer}>
        <div>
          {tabs.map((tab) => (
            <Button
              key={tab.value}
              type={resource_type === tab.value ? "primary" : "default"}
              className={styles.tabButton}
              onClick={() => {
                setResourceType(tab.value)
                setPlaceholder(tab.placeholder)
              }}
            >
              {tab.label}
            </Button>
          ))}
        </div>
        <div>
          <Button type="dashed" className={searchBarstyles.createGroupBtn} icon={<PlusOutlined />} onClick={handleAddResGroup}>
            创建分组
          </Button>
          <Tooltip title="刷新">
            <Button icon={<RedoOutlined />} onClick={refresh}></Button>
          </Tooltip>
        </div>
      </div>
      <div className={styles.pagesContent}>
        <ResourceGroupList
          refresh={refresh}
          data={data}
          project_id={project_id!}
          resource_type={resource_type}
          handleEditResGroup={handleEditResGroup}
        />
      </div>
      <CreateGroup
        createRef={createGroupRef}
        update={refresh}
        project_id={project_id!}
        resource_type={resource_type}
      />
    </Layout.Content>
  );
}
