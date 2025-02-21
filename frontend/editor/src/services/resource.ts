import { cmd_invoke } from "./cmd_invoke";

interface ResourceQueryParams {
    project_id: string;
    resouce_type: string; // TODO - enum
    resouce_group?: string;
}

export const resourceService = {
    load_resource: (params: ResourceQueryParams) => {
        return cmd_invoke("load_resource", { params });
    },
    add_resource_group: (params: any) => {
    },
    delete_resource_group: (params: any) => {
    },
    update_resource_group: (params: any) => {
    },
    upload_resource: (params: any) => {
    }
};
