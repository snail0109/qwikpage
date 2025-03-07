import { create } from "zustand";
import { persist } from "zustand/middleware";
import { invoke } from "@tauri-apps/api/core";

export interface PreferencesState {
    theme: "system" | "dark" | "light";
    language: "system" | "en" | "zh";
    fontSize: number;
    fontBold: "normal" | "bold";
    fontFamily: string;
    checkUpdate: boolean;
    codeBuildPath: string;
}
interface PreferencesStore extends PreferencesState {
    get_preferences: () => Promise<void>;
    set_preferences: (key: string, value : any) => Promise<void>;
}

const usePreferencesStore = create<PreferencesStore>()(
    persist(
        (set) => ({
            theme: "system",
            language: "system",
            fontSize: 12,
            fontBold: "normal",
            fontFamily: "system",
            checkUpdate: false,
            codeBuildPath: "system",
            get_preferences: async () => {
                try {
                    const preferences: PreferencesState = await invoke("get_preferences");
                    set(preferences);
                } catch (error) {
                    console.error("初始化配置失败:", error);
                }
            },
            set_preferences: async (key: string, value : any) => {
                try {
                    await invoke("set_preferences", { key ,value});
                } catch (error) {
                    console.error("初始化配置失败:", error);
                }
            },
        }),
        {
            name: "preferences",
        }
    )
);

export default usePreferencesStore;
