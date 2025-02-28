import styles from "./index.module.less";
import EmptyFolder from "@/assets/image/emptyFolder.png";
import ArrowIcon from "@/assets/icons/ArrowIcon.svg?react";

interface EmptyBoxProps {
  title: string;
  lastCharsCount: number;
  groupId: string;
  onCreate: (groupId: string) => void;
}

const EmptyBox = ({
  groupId,
  title,
  lastCharsCount = 2,
  onCreate,
}: EmptyBoxProps) => {

  const mainText = title.slice(0, -lastCharsCount);
  const lastTwoChars = title.slice(-lastCharsCount);

   // 调用父组件方法
   const handleEmptyClick = (event: any) => {
    onCreate(groupId);
};

  return (
    <div className={styles.emptyItem} onClick={handleEmptyClick}>
      <div className={styles.iconTitle}>
        <img src={EmptyFolder} />
        {mainText}
        <span className={styles.colorTitle}>{lastTwoChars}</span>
      </div>
      <ArrowIcon />
    </div>
  );
};

export default EmptyBox;
