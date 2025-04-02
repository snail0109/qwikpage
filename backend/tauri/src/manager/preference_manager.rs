use anyhow::Error;
use once_cell::sync::OnceCell;
use std::path::PathBuf;
use std::sync::{RwLock, RwLockReadGuard};

use crate::storage::local_storage::LocalStorage;
use crate::{
    types::preferences::Preferences,
    utils::dirs::{app_preferences_path, get_default_code_path},
};
const DEFAULT_FONT_SIZE: u32 = 12;
const DEFAULT_FONT_BOLD: &str = "normal";

// 全局配置单例
pub struct PreferencesManager {
    preferences: RwLock<Preferences>,
    storage: LocalStorage<Preferences>,
}

impl PreferencesManager {
    pub fn global() -> &'static PreferencesManager {
        static INSTANCE: OnceCell<PreferencesManager> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let path = app_preferences_path();
            let storage = LocalStorage::new(&path.to_string_lossy(), Preferences::default);
            let config = Self {
                preferences: RwLock::new(storage.load()),
                storage,
            };
            config
        })
    }

    pub fn preferences(&self) -> RwLockReadGuard<'_, Preferences> {
        self.preferences.read().expect("读取配置文件失败")
    }

    pub fn get_preferences() -> RwLockReadGuard<'static, Preferences> {
        Self::global().preferences()
    }

    pub fn update_preferences<F>(&self, updater: F) -> Result<(), Error>
    where
        F: FnOnce(&mut Preferences),
    {
        let mut prefs = self.preferences.write().unwrap();
        updater(&mut prefs);
        self.storage.save(&prefs)
    }

    // 设置偏好配置
    fn set_preferences_impl(&self, preferences: Preferences) -> Result<(), Error> {
        self.update_preferences(|current| {
            *current = preferences;
        })
    }

    pub fn set_preferences(preferences: Preferences) -> Result<(), Error> {
        Self::global().set_preferences_impl(preferences)
    }

    // 重命名为内部实例方法
    fn get_project_path_impl(&self) -> PathBuf {
        let prefs = self.preferences();
        log::debug!("PreferencesManager::get_project_path_impl: 项目路径: {:?}", prefs.project_path);
        PathBuf::from(prefs.project_path.clone())
    }

    // 使用原名称作为静态方法
    pub fn get_project_path() -> PathBuf {
        Self::global().get_project_path_impl()
    }
}

impl Default for Preferences {
    fn default() -> Self {
        let font_family = if cfg!(target_os = "macos") {
            "PingFang SC".to_string()
        } else {
            "Microsoft YaHei Mono".to_string()
        };
        Self {
            font_family,
            theme: "auto".to_string(),
            language: "auto".to_string(),
            font_size: DEFAULT_FONT_SIZE,
            font_bold: DEFAULT_FONT_BOLD.to_string(),
            check_update: true,
            project_path: get_default_code_path(),
        }
    }
}
