import { notification } from "@/utils/AntdGlobal";
import { invoke } from "@tauri-apps/api/core";

/**
 * Small wrapper on top of tauri api invoke for a light abstraction.
 */
export async function cmd_invoke(method: string, params?: any): Promise<any> {
    try {
        const response : any = await invoke(method, params);
        if (!response.success) {
            console.log("ERROR - cmd_invoke error", response);
            notification.error({
                message: "Error",
                description: response.message,
            });
            throw new Error(response.message);
        } else {
            return response.data;
        }
    } catch (err: any) {
        console.log("ERROR - cmd_invoke error", err);
        notification.error({
            message: "Error",
            description: err,
        });
        throw new Error(err);
    }
}
