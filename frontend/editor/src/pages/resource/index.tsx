import { resourceService } from "@/services";
import { Button, Form, message, Layout, Divider, Tooltip, Modal } from "antd";
import { useEffect, useRef, useState } from "react";
import styles from "./index.module.less";
import searchBarstyles from "./index.module.less";
import { open } from "@tauri-apps/plugin-dialog";
import { RedoOutlined, PlusOutlined, ExclamationCircleFilled } from "@ant-design/icons";
import SearchBar from "@/components/Searchbar/SearchBar";
import CreateGroup, { IOpenParams } from "./components/CreateGroup";
import { IOperResourceGroupParams } from "@/services/resource";
import { ResourceGroupProvider } from "@/context/resource";
import ResourceGroupList, { IResourceGroup } from "./components/ResourceGroupList";
import ResourceUpload from "@/components/ResourceUpload";

export const RESOURCE_TABS = [
  {
    label: "图片",
    value: "img",
    placeholder: "请输入图片名称",
    extensions: ["jpg", "jpeg", "png", "gif", "bmp", "svg", "webp"],
  },
  {
    label: "字体",
    value: "font",
    placeholder: "请输入字体名称",
    extensions: ["ttf", "otf", "woff", "woff2", "eot", "ttc"],
  },
  {
    label: "第三方JS",
    value: "js",
    placeholder: "请输入JS名称",
    extensions: ["js", "css", "json", "html"],
  },
  {
    label: "附件",
    value: "attachment",
    placeholder: "请输入附件名称",
    extensions: ["zip", "rar", "tar", "gz", "7z"],
  },
  {
    label: "其它",
    value: "other",
    placeholder: "请输入其它资源名称",
    extensions: ["pdf", "doc", "docx", "ppt", "pptx", "txt"],
  },
];

const FILE_LIMITS = 5; // 文件数量
const FILE_LIMIT_SIZE = 20 * 1024 * 1024; // 文件大小
const { confirm } = Modal;

const renameResourceMap = {
  group_name: "",
  resource_name: "",
};

export default function Home() {
  const searchParams = new URLSearchParams(location.search);
  const project_id = searchParams.get("projectId") || "";
  const project_name = searchParams.get("projectName") || "";
  const [data, setData] = useState<IResourceGroup[]>([]);
  const [resource_type, setResourceType] = useState(RESOURCE_TABS[0].value);
  const [loading, setLoading] = useState(true);
  const [placeholder, setPlaceholder] = useState(RESOURCE_TABS[0].placeholder);

  const createGroupRef = useRef<{ open: (params: IOpenParams) => void }>();
  const uploadfileRef = useRef<{ open: () => void }>();

  const [form] = Form.useForm();

  const fetchResource = async (sourceType: string) => {
    setLoading(true);
    try {
      const res = await resourceService.load_resource({
        project_id: project_id!,
        resource_type: sourceType,
      });
      if (res) {
        console.log("resourceGroup List: ", res);
        setData(res);
      }
    } catch (err) {
      console.log(err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    fetchResource(resource_type);
  }, []);

  const onChangTab = async (tab: any) => {
    await fetchResource(tab.value);
    setResourceType(tab.value);
    setPlaceholder(tab.placeholder);
  };

  // 新建资源分组
  const handleAddResGroup = () => {
    createGroupRef.current?.open({ action: "create" });
  };

  // 上传资源
  const onImportClick = async (name: string) => {
    // uploadfileRef.current?.open();
    const filters = RESOURCE_TABS.find((item) => item.value === resource_type)?.extensions || [];
    const filtersUp = filters.map(v => v.toUpperCase())
    const filePaths = await open({
      title: "Select File",
      multiple: true,
      filters: [
        {
          name: "Files",
          extensions: filters.concat(filtersUp),
        },
      ],
    });

    if (!filePaths || filePaths?.length === 0) {
      return;
    }

    if (filePaths.length > FILE_LIMITS) {
      message.warning(`单次最多可上传${FILE_LIMITS}个文件`);
      return;
    }

    // const MAX_SIZE =

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
        project_id: project_id,
        resource_type: resource_type,
      };
      await resourceService.update_resource_group(cmdParams);
      refresh();
      return true;
    } catch (error) {
      message.error("修改失败,请重试");
      console.error("修改失败", error);
    }
    return false;
  };

  // 删除分组
  const onDeleteGroupClick = (name: string) => {
    resourceService
      .delete_resource_group({
        project_id,
        resource_type,
        group_name: name,
      })
      .then(() => {
        message.success("删除成功");
        refresh();
      });
  };

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

  // 编辑资源
  const onEditResourceClick = (groupName: string, resourceName: string) => {
    renameResourceMap.group_name = groupName;
    renameResourceMap.resource_name = resourceName;
    const [name] = resourceName.split(".");
    createGroupRef.current?.open({
      action: "renameResource",
      group_name: name,
    });
  };

  const renameResource = async (newName: string) => {
    const [, type] = renameResourceMap.resource_name.split(".");
    const result = await resourceService.rename_resource({
      project_id,
      resource_type,
      group_name: renameResourceMap.group_name,
      resource_name: renameResourceMap.resource_name,
      new_resource_name: `${newName}.${type}`,
    });
    if (result) {
      message.success("重命名成功");
      refresh();
    }
    return result;
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
    <Layout.Content className={searchBarstyles.resourceContainer}>
      {/* 搜索工具条 */}
      <SearchBar
        className={searchBarstyles.searchBar}
        showGroup={false}
        noNeedCreate
        noNeedFresh
        form={form}
        searchPlaceholder={placeholder}
        projectName={project_name}
        submit={refresh}
        refresh={refresh}
      />
      <Divider />
      <div className={searchBarstyles.topContainer}>
        <div>
          {RESOURCE_TABS.map((tab) => (
            <Button
              key={tab.value}
              type={resource_type === tab.value ? "primary" : "default"}
              className={styles.tabButton}
              onClick={() => onChangTab(tab)}
            >
              {tab.label}
            </Button>
          ))}
        </div>
        <div>
          <Button
            type="dashed"
            className={searchBarstyles.createGroupBtn}
            icon={<PlusOutlined />}
            onClick={handleAddResGroup}
          >
            创建分组
          </Button>
          <Tooltip title="刷新">
            <Button icon={<RedoOutlined />} onClick={refresh}></Button>
          </Tooltip>
        </div>
      </div>
      <div className={styles.pagesContent}>
        <ResourceGroupProvider
          resource_type={resource_type}
          onImport={onImportClick}
          onEditGroup={onEditGroupClick}
          onDeleteGroup={onDeleteGroupClick}
          onEditResource={onEditResourceClick}
          onDeleteResource={onDeleteResourceClick}
        >
          <ResourceGroupList data={data} />
        </ResourceGroupProvider>
      </div>
      {/* 创建分组弹框 */}
      <CreateGroup
        createRef={createGroupRef}
        update={refresh}
        customConfirm={renameResource}
        project_id={project_id!}
        resource_type={resource_type}
      />
      {/* 上传资源 */}
      <ResourceUpload uploadRef={uploadfileRef} />
    </Layout.Content>
  );
}
