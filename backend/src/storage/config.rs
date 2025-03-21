use std::sync::{RwLock, RwLockReadGuard};

use once_cell::sync::OnceCell;

use super::preferences::Preferences;

pub struct Config {
    preferences: RwLock<Preferences>,
}

impl Config {
    pub fn global() -> &'static Config {
        static INSTANCE: OnceCell<Config> = OnceCell::new();
        INSTANCE.get_or_init(|| {
            let config = Self {
                preferences: RwLock::new(Preferences::new()),
            };
            config
        })
    }

    pub fn preferences(&self) -> RwLockReadGuard<'_, Preferences> {
        self.preferences.read().expect("Failed to read preferences")
    }

    pub fn update_preferences<F>(&self, updater: F)
    where
        F: FnOnce(&mut Preferences),
    {
        let mut prefs = self.preferences.write().unwrap();
        updater(&mut prefs);
    }
    
}
