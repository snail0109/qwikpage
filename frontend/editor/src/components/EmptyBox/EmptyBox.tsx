import { Button } from 'antd';
import { ExportOutlined } from '@ant-design/icons';
import styles from "./index.module.less";
import UD from "@/assets/image/UD.png";

interface EmptyBoxProps {
  title: string;
  lastCharsCount: number;
  onCreate: () => void;
}

const EmptyBox = ({
  title,
  lastCharsCount = 2,
  onCreate,
}: EmptyBoxProps) => {

  const mainText = title.slice(0, -lastCharsCount);
  const lastTwoChars = title.slice(-lastCharsCount);

  return (
    <div className={styles.emptyItem} onClick={onCreate}>
      <div className={styles.iconTitle}>
        <img src={UD} />
        {mainText}
        <span className={styles.colorTitle}>{lastTwoChars}</span>
      </div>
      <ExportOutlined />
    </div>
  );
};

export default EmptyBox;
