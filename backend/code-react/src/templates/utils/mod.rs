use crate::FileTemplate;

mod action;
mod handle_api;
mod request;
mod use_component_refs;
mod form_context;
mod util;

pub fn get_utils() -> Vec<FileTemplate> {
    let mut files = vec![];
    files.push(FileTemplate {
        filename: String::from("src/utils/action.ts"),
        content: String::from(action::ACTION_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/utils/handleApi.ts"),
        content: String::from(handle_api::HANDLE_API_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/utils/request.ts"),
        content: String::from(request::REQUEST_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/utils/useComponentRefs.ts"),
        content: String::from(use_component_refs::USE_REF_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/utils/util.ts"),
        content: String::from(util::UTIL_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/utils/context.ts"),
        content: String::from(form_context::FORM_CONTEXT_INDEX),
    });
    return files;
}