import styles from "./index.module.less";

interface ProjectLogoProps {
  logoUrl: string;
  handleUpload: () => void;
}

const ProjectLogo = ({ logoUrl, handleUpload }: ProjectLogoProps) => {
 
  return (
    <div className={styles.imageContainer}>
      <img width={100} height={100} src={logoUrl} />
      <div className={styles.mask} onClick={handleUpload}>
        上传
      </div>
    </div>
  );
};

export default ProjectLogo;
