import { notification } from "@/utils/AntdGlobal";
import { invoke } from "@tauri-apps/api/core";

/**
 * Small wrapper on top of tauri api invoke for a light abstraction.
 */
export async function cmd_invoke(method: string, params?: any): Promise<any> {
    try {
        const response : any = await invoke(method, params);
        if (!response.success) {
            throw new Error(response.message);
        } else {
            return response.data;
        }
    } catch (err: any) {
        // 如果 err 是字符串，直接显示
        if (typeof err === "string") {
            notification.error({
                message: "Error",
                description: err,
            });
            throw new Error(err);
        } else if (err instanceof Error) {
            // 如果 err 是 Error 对象，显示 err.message
            notification.error({
                message: "Error",
                description: err.message,
            });
            throw new Error(err.message);
        }
        throw new Error(err);
    }
}
