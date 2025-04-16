use crate::FileTemplate;

pub mod button;
pub mod icon;
pub mod flex;
pub mod grid;
pub mod col;
pub mod form;
pub mod input;
pub mod checkbox;
pub mod material;
pub mod components;
pub mod comp_index;
mod text;
mod link;
mod select;
mod switch;

pub fn get_components() -> Vec<FileTemplate> {
    let mut files = vec![];
    files.push(FileTemplate {
        filename: String::from("src/components/index.ts"),
        content: String::from(comp_index::COM_INDEX),
    });
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
        filename: String::from("src/components/Grid/index.tsx"),
        content: String::from(grid::GRID_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Col/index.tsx"),
        content: String::from(col::COL_INDEX),
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
    files.push(FileTemplate {
        filename: String::from("src/components/Text/index.tsx"),
        content: String::from(text::TEXT_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Link/index.tsx"),
        content: String::from(link::LINK_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Select/index.tsx"),
        content: String::from(select::SELECT_INDEX),
    });
    files.push(FileTemplate {
        filename: String::from("src/components/Switch/index.tsx"),
        content: String::from(switch::SWITCH_INDEX),
    });
    return files;
}