use crate::FileTemplate;
mod lazy_load;
mod spin_loading;
mod button;
mod icon;
mod flex;
mod form;
mod input;
mod checkbox;
mod material;
mod components;


pub fn get_components() -> Vec<FileTemplate> {
    let mut files = vec![];
    // 路由组件
    files.push(FileTemplate {
        filename: String::from("src/components/LazyLoad.tsx"),
        content: String::from(lazy_load::LAZY_LOAD),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/SpinLoading.tsx"),
        content: String::from(spin_loading::SPIN_LOADING),
    });

    // 渲染器组件
    // files.push(FileTemplate {
    //     filename: String::from("src/components/index.ts"),
    //     content: String::from(components::COMPONENTS_INDEX),
    // });
    files.push(FileTemplate {
        filename: String::from("src/components/components.ts"),
        content: String::from(components::COMPONENTS_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Button/index.tsx"),
        content: String::from(button::BUTTON_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Icon/index.tsx"),
        content: String::from(icon::ICON_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Icon/BaseIcon.tsx"),
        content: String::from(icon::ICON_BASE_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Form/index.tsx"),
        content: String::from(form::FORM_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Flex/index.tsx"),
        content: String::from(flex::FLEX_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Input/index.tsx"),
        content: String::from(input::INPUT_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/CheckBox/index.tsx"),
        content: String::from(checkbox::CHECKBOX_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/MarsRender/index.tsx"),
        content: String::from(material::MATERIAL_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/MarsRender/Material.tsx"),
        content: String::from(material::MATERIAL_ITEM_INDEX),
    });
    return files;
}