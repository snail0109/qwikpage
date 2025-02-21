import { ImageViewer } from '@/components/resourceViewers/ImageViewer';
import { appConfigDir, appDataDir, join } from '@tauri-apps/api/path';
import { set } from 'lodash-es';
import { useEffect, useState } from 'react';

export default function Home() {
    const [path, setPath] = useState<string>('');
    useEffect(async () => {
        console.log('Home page mounted');
        const appConfigDirPath = await appConfigDir();
        console.log('appConfigDirPath:', appConfigDirPath);
        const appDataDirPath = await appDataDir();
        console.log('appDataDirPath:', appDataDirPath);
        // 项目根目录下的 resources/imgs/ss/logo.png
        const filePath = await join(appDataDirPath, 'resources/imgs/ss/logo.png');
        setPath(filePath);
    }
    , []);

    return <ImageViewer bodyPath={path} />;
}