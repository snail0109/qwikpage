import { create } from "zustand";
import { persist } from "zustand/middleware";
import { invoke } from "@tauri-apps/api/core";

export interface PreferencesState {
    theme: "auto" | "dark" | "light";
    language: "auto" | "en" | "zh";
    fontSize: number;
    fontBold: "normal" | "bold";
    fontFamily: string;
    checkUpdate: boolean;
    projectPath: string;
    systemFontFamilys: string[];
}
interface PreferencesStore extends PreferencesState {
    get_preferences: () => Promise<void>;
    set_preferences: (config: PreferencesState) => Promise<void>;
}

const usePreferencesStore = create<PreferencesStore>()(
    persist(
        (set) => ({
            theme: "auto",
            language: "auto",
            fontSize: 12,
            fontBold: "normal",
            fontFamily: "system",
            checkUpdate: false,
            projectPath: "system",
            systemFontFamilys:[],
            get_preferences: async () => {
                try {
                    const preferences: PreferencesState = await invoke("get_preferences");
                    // 检查 systemFontFamilys 是否为空，如果为空则调用接口获取字体
                    if (preferences.systemFontFamilys.length === 0) {
                        const systemFonts: string[] = await invoke("get_system_fonts");
                        set({ ...preferences, systemFontFamilys: systemFonts }); // 更新状态
                    } else {
                        set(preferences);
                    }
                } catch (error) {
                    console.error("初始化配置失败:", error);
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
        }),
        {
            name: "preferences",
        }
    )
);

export default usePreferencesStore;
