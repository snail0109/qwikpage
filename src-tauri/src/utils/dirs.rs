use once_cell::sync::Lazy;
use std::{borrow::Cow, fs, path::PathBuf};
use anyhow::Result;
use dirs::{config_dir, data_local_dir};

pub const APP_NAME: &str = "Qwikpage";


/// Suggest a directory for configuration files.
/// * - Linux: Resolves to $XDG_CONFIG_HOME/{placeholder} or $HOME/.config/{placeholder}.
/// * - macOS: Resolves to $HOME/Library/Application Support/{placeholder}/config.
/// * - Windows: Resolves to {FOLDERID_RoamingAppData}/{placehholder}/config.
pub fn suggest_config_dir(placeholder: &str) -> Option<PathBuf> {
    let path = config_dir()?;
    #[cfg(target_os = "linux")]
    {
        Some(path.join(placeholder))
    }
    #[cfg(not(target_os = "linux"))]
    {
        Some(path.join(placeholder).join("config"))
    }
}


/// Suggest a directory for configuration files.
/// * - Linux: Resolves to $XDG_DATA_HOME/{placeholder} or $HOME/.local/share/{placeholder}.
/// * - macOS: Resolves to $HOME/Library/Application Support/{placeholder}/data.
/// * - Windows: Resolves to {FOLDERID_LocalAppData}/{placehholder}/data.
pub fn suggest_data_dir(placeholder: &str) -> Option<PathBuf> {
    let path = data_local_dir()?;
    #[cfg(target_os = "linux")]
    {
        Some(path.join(placeholder))
    }
    #[cfg(not(target_os = "linux"))]
    {
        Some(path.join(placeholder).join("data"))
    }
}


/// App Dir placeholder
/// It is used to create the config and data dir in the filesystem
/// For windows, the style should be similar to `C:/Users/nyanapasu/AppData/Roaming/Qwikpage`
/// For macos, it should be similar to `/Users/xxx/Library/Application Support/Qwikpage`
/// For other platforms, it should be similar to `/home/xxx/.config/Qwikpage`
pub static APP_DIR_PLACEHOLDER: Lazy<Cow<'static, str>> = Lazy::new(|| {
    use convert_case::{Case, Casing};
    if cfg!(any(target_os = "windows", target_os = "macos")) {
        Cow::Owned(APP_NAME.to_case(Case::Title))
    } else {
        Cow::Borrowed(APP_NAME)
    }
});

#[cfg(target_os = "windows")]
pub static IS_PORTABLE: Lazy<bool> = Lazy::new(|| {
    if cfg!(windows) {
        let dir = crate::utils::dirs::app_install_dir().unwrap();
        let portable_file = dir.join(".config/PORTABLE");
        portable_file.exists()
    } else {
        false
    }
});

#[allow(unused)]
#[cfg(target_os = "windows")]
pub fn get_portable_flag() -> bool {
    *IS_PORTABLE
}

pub fn app_config_dir() -> Result<PathBuf> {
    let path: Option<PathBuf> = {
        #[cfg(target_os = "windows")]
        {
            if get_portable_flag() {
                let app_dir = app_install_dir()?;
                Some(app_dir.join(".config").join(APP_NAME))
            } else if let Ok(Some(path)) = super::winreg::get_app_dir() {
                Some(path)
            } else {
                None
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            None
        }
    };

    match path {
        Some(path) => Ok(path),
        None => suggest_config_dir(&APP_DIR_PLACEHOLDER)
            .ok_or(anyhow::anyhow!("failed to get the app config dir")),
    }
    .and_then(|dir| {
        fs::create_dir_all(&dir)?;
        Ok(dir)
    })
}


pub fn app_data_dir() -> Result<PathBuf> {
    let path: Option<PathBuf> = {
        #[cfg(target_os = "windows")]
        {
            if get_portable_flag() {
                let app_dir = app_install_dir()?;
                Some(app_dir.join(".data").join(APP_NAME))
            } else {
                None
            }
        }
        #[cfg(not(target_os = "windows"))]
        {
            None
        }
    };

    match path {
        Some(path) => Ok(path),
        None => suggest_data_dir(&APP_DIR_PLACEHOLDER)
            .ok_or(anyhow::anyhow!("failed to get the app data dir")),
    }
    .and_then(|dir| {
        fs::create_dir_all(&dir)?;
        Ok(dir)
    })
}

pub fn app_install_dir() -> Result<PathBuf> {
    let exe = tauri::utils::platform::current_exe()?;
    let exe = dunce::canonicalize(exe)?;
    let dir = exe
        .parent()
        .ok_or(anyhow::anyhow!("failed to get the app install dir"))?;
    Ok(PathBuf::from(dir))
}
