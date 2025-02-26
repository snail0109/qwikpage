import { type } from "@tauri-apps/plugin-os";

// 获取系统信息
export function useOsInfo() {
    return { osType: type() };
}

