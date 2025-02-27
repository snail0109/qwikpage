import { useEffect, useMemo, useState } from "react";
import { Tooltip, Flex, Divider, theme } from 'antd';
import { convertFileSrc, invoke } from "@tauri-apps/api/core";
import { DeleteOutlined } from '@ant-design/icons';
import { useResource } from "@/context/resource";
import EditIcon from "@/assets/icons/EditIcon.svg?react";
import LoadError from "@/assets/icons/loadError.svg?react";
import styles from './index.module.less';

const { useToken } = theme;

interface IProps {
  resource_name: string;
  name: string;
  path: string;
  file_type: string;
  last_modified_time: string;
}

interface FontProp {
  postscript_name: string;
  family: string;
  full_name: string;
  fail?: boolean;
}

const FontViewer = (props: IProps) => {
  const { token } = useToken();
  const { onEditResource, onDeleteResource } = useResource();
  const { name, path, resource_name } = props;

  const [font, setFont] = useState<FontProp>();

  // const id = useMemo(() => {
  //   return `${name}-${file_type}`;
  // }, [name, file_type]);


  useEffect(() => {
    invoke("parse_font_metadata", { path })
      .then((res: FontProp) => {
        console.log(res);
        setFont(res);
        const url = convertFileSrc(path);
        const fontFace = new FontFace(res.full_name, `url(${url})`);
        fontFace
          .load()
          .then((loadedFont) => {
            document.fonts.add(loadedFont);
            console.log(`字体 ${res.full_name} 加载完成`);
          })
          .catch((error) => {
            console.error(`加载字体 ${res.full_name} 失败:`, error)
            const [fullName] = name.split('.');
            setFont({
              fail: true,
              postscript_name: '',
              family: 'inherit',
              full_name: fullName
            })
          });
      })
      .catch((error) => {
        console.log(error);
        const [fullName] = name.split('.');
        setFont({
          fail: true,
          postscript_name: '',
          family: 'inherit',
          full_name: fullName
        })
      });
  }, []);

  if (!font) {
    return null;
  }

  const handleEdit = () => {
    onEditResource(resource_name, name)
  }
  const handleDelete = () => {
    onDeleteResource(resource_name, name);
  }

  return (
    <div className={styles.fontPreview} key={font.full_name} style={{ fontFamily: font.full_name }}>
      <div className={styles.fontContainer}>
        {font.fail ? (
          <div className={styles.loadFail}>
            <LoadError />
            <span>加载失败</span>
          </div>
        ) : <>汉</>}
        <Flex className={styles.fontTool} style={{
          color: token.colorPrimaryBg,
        }} justify="center">
          <EditIcon onClick={handleEdit} />
          <Divider type="vertical" style={{ borderColor: token.colorPrimaryBg }} />
          <DeleteOutlined onClick={handleDelete} />
        </Flex>
      </div>
      <Tooltip title={font.full_name} placement="bottom">
        <div className={styles.fontBottom}>{font.full_name}</div>
      </Tooltip>

    </div>
  )
};

export default FontViewer;
