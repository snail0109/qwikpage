use std::path::PathBuf;

use anyhow::{Error, Result};
use log::error;
use serde::{de::DeserializeOwned, Serialize};

use crate::utils::file::{read_json_file, write_json_file};

/// 通用的JSON存储管理器，提供增删改查功能
pub struct LocalStorage<T> {
    path: String,
    default_generator: fn() -> T,
}

impl<T> LocalStorage<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    /// 创建一个新的JSON存储管理器
    pub fn new(path: &str, default_generator: fn() -> T) -> Self {
        Self {
            path: path.to_string(),
            default_generator,
        }
    }

    /// 加载数据，如果文件不存在或读取失败则返回默认值
    pub fn load(&self) -> T {
        read_json_file(&self.path).unwrap_or_else(|e| {
            error!("加载JSON文件失败 {}: {}", self.path, e);
            (self.default_generator)()
        })
    }

    /// 保存数据到文件
    pub fn save(&self, data: &T) -> Result<(), Error> {
        write_json_file(&PathBuf::from(&self.path), data)
    }

    /// 更新部分数据
    pub fn update<F>(&self, updater: F) -> Result<T, Error>
    where
        F: FnOnce(&mut T),
    {
        let mut data = self.load();
        updater(&mut data);
        self.save(&data)?;
        Ok(data)
    }
    
}