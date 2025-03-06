import { create } from "zustand";
import { persist } from "zustand/middleware";
import { homeDir, join } from '@tauri-apps/api/path';
import { Store } from "@tauri-apps/plugin-store";

let diskStore: any;
export async function initAppConfigStore() {
    // app_data_dir
    if (diskStore) {
        return Promise.resolve(diskStore);
    }
    diskStore = await Store.load("config.json", { autoSave: true });
    return diskStore;
}

async function get_store(key: string, defaultValue: any) {
    const store = await initAppConfigStore();
    const value = await store.get(key);
    console.log("get_store", key, value);
    let property;
    if (value === null) {
        property = defaultValue;
        if (key === "dsl_code_dir") {
            const homeDirPath = await homeDir();
            property = await join(homeDirPath, "Library", "Preferences", "QwikPage")
        }
        store.set(key, defaultValue);
        store.save();
    } else {
        property = value;
    }
    return Promise.resolve(property);
}

export interface AppConfigState {
    theme: "system" | "dark" | "light";
    language: "system" | "en" | "zh";
    fontSize: number;
    fontBold: "normal" | "bold";
    fontFamily: string;
    checkUpdate: boolean;
    dsl_code_dir: string;
}
interface AppConfigStore extends AppConfigState {
    initConfig: () => Promise<void>;
    update: (key: string, value : string) => Promise<void>;
}

const useAppConfigStore = create<AppConfigStore>()(
    persist(
        (set) => ({
            theme: "system",
            language: "system",
            fontSize: 12,
            fontBold: "normal",
            fontFamily: "system",
            checkUpdate: false,
            dsl_code_dir: "system",
            initConfig: async () => {
                try {
                    const theme = await get_store("theme", "system");
                    const language = await get_store("language", "system");
                    const fontSize = await get_store("fontSize", 12);
                    const fontBold = await get_store("fontBold", "normal");
                    const fontFamily = await get_store("fontFamily", "system");
                    const checkUpdate = await get_store("check_update", false);
                    const dsl_code_dir = await get_store("dsl_code_dir", undefined);
                    set({
                        theme,
                        language,
                        fontSize,
                        fontBold,
                        fontFamily,
                        checkUpdate,
                        dsl_code_dir,
                    });
                } catch (error) {
                    console.error("初始化配置失败:", error);
                }
            },
            update: async (key: string, value: any) => {
                const store = await initAppConfigStore();
                await store.set(key, value);
                await store.save();
                set({ [key]: value });
            },
        }),
        {
            name: "app-config",
        }
    )
);

export default useAppConfigStore;
