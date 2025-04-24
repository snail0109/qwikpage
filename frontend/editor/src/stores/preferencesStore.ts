import { create } from "zustand";
import { persist } from "zustand/middleware";
import { invoke } from "@tauri-apps/api/core";
import { getVersion } from '@tauri-apps/api/app';

export interface PreferencesState {
    theme: "auto" | "dark" | "light";
    language: "auto" | "en" | "zh";
    fontSize: number;
    fontBold: "normal" | "bold";
    fontFamily: string;
    checkUpdate: boolean;
    projectPath: string;
    systemFontFamilys?: string[]; 
    version: string;
}
interface PreferencesStore extends PreferencesState {
    get_preferences: () => Promise<PreferencesState>;
    set_preferences: (config: PreferencesState) => Promise<void>;
    get_system_fonts: () => Promise<void>;
}

const usePreferencesStore = create<PreferencesStore>()(
    persist(
        (set) => ({
            theme: "auto",
            language: "auto",
            fontSize: 12,
            fontBold: "normal",
            fontFamily: "system",
            checkUpdate: true,
            projectPath: "system",
            systemFontFamilys: [],
            version: "", 
            get_preferences: async () => {
                try {
                    const preferences: PreferencesState = await invoke("get_preferences");
                    const origVersion = await getVersion();
                    const version = origVersion.match(/^v/i) ? origVersion : `v${origVersion}`;
                    set({ ...preferences, version });
                    return { ...preferences, version };
                } catch (error) {
                    console.error("初始化配置失败:", error);
                    throw error;
                }
            },
            set_preferences: async (preferences: PreferencesState) => {
                try {
                    await invoke("set_preferences", { preferences });
                    // 更新本地状态
                    set(preferences);
                } catch (error) {
                    console.error("Failed to update preferences:", error);
                    throw error;
                }
            },
            get_system_fonts: async () => {
                try {
                    const systemFonts: string[] = await invoke("get_system_fonts");
                    set({ systemFontFamilys: systemFonts });
                } catch (error) {
                    console.error("获取系统字体失败:", error);
                }
            }
        }),
        {
            name: "preferences",
        }
    )
);

export default usePreferencesStore;
