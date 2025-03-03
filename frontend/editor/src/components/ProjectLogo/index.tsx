import { convertFileSrc } from "@tauri-apps/api/core";
import styles from "./index.module.less";

interface ProjectLogoProps {
    disabled?: boolean;
    logoUrl: string;
    handleUpload: () => void;
}

const ProjectLogo = ({ disabled = false, logoUrl, handleUpload }: ProjectLogoProps) => {
    const src = logoUrl.includes("com.qwikpage.desktop/resources/project_logo") ? convertFileSrc(logoUrl) : logoUrl;

    return (
        <div className={styles.imageContainer}>
            <img width={100} height={100} src={src} />
            <div className={styles.mask} onClick={handleUpload}>
                上传
            </div>
        </div>
    );
};

export default ProjectLogo;
