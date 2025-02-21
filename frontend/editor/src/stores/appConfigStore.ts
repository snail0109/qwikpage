
import { create } from 'zustand'
import { persist } from 'zustand/middleware'
import { invoke } from '@tauri-apps/api/core';

export interface AppConfigState {
    theme: "system" | "dark" | "light";
}

interface AppConfigStore extends AppConfigState {
    setTheme: (theme: AppConfigState['theme']) => void;
    initConfig: () => Promise<void>;
}

const useAppConfigStore = create<AppConfigStore>()(
    persist(
        (set) => ({
            theme: 'system',
            setTheme: (theme) => set({ theme }),
            initConfig: async () => {
                try {
                    const config = await invoke<AppConfigState>('get_app_conf');
                    set(config);
                } catch (error) {
                    console.error('初始化配置失败:', error);
                }
            }
        }),
        {
            name: 'app-config',
        }
    )
)

export default useAppConfigStore


