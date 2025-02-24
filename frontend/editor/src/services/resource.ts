import { cmd_invoke } from "./cmd_invoke";

interface IResourceQueryParams {
    project_id: string;
    resource_type: string; // TODO - enum
    resource_group?: string;
}

export interface IOperResourceGroupParams {
    project_id: string;
    resource_type: string;
    group_name: string;
    new_group_name?: string;
}

interface IUploadParams {
    project_id: string,
    resource_type: string,
    group_name: string,
    file_list: string[],
}

export const resourceService = {
    load_resource: (params: IResourceQueryParams) => {
        return cmd_invoke("load_resource", { params });
    },
    add_resource_group: (params: IOperResourceGroupParams) => {
        return cmd_invoke("add_resource_group", { params });
    },
    delete_resource_group: (params: IOperResourceGroupParams) => {
        return cmd_invoke("delete_resource_group", { params });
    },
    update_resource_group: (params: IOperResourceGroupParams) => {
        return cmd_invoke("update_resource_group", { params });
    },
    import_resource: (params: IUploadParams) => {
        return cmd_invoke("import_resource", { params });
    }
};
