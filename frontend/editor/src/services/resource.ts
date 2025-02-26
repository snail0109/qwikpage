import { rename } from "fs";
import { cmd_invoke } from "./cmd_invoke";

interface IResourceQueryParams {
    project_id: string;
    resource_type: string; // TODO - enum
    resource_group?: string;
    keyword?: string;
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

interface IUpdateResourceParams {
    project_id: string,
    resource_type: string,
    group_name: string,
    resource_name: string,
    new_resource_name: string,
}

interface IDeleteResourceParams {
    project_id: string,
    resource_type: string,
    group_name: string,
    resource_name: string,
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
    },
    rename_resource: (params: IUpdateResourceParams) => {
        return cmd_invoke("rename_resource", { params });
    },
    delete_resource: (params: IDeleteResourceParams) => {
        return cmd_invoke("delete_resource", { params });
    }
};
