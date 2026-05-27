mod sni;

pub use sni::SniService;

use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct TrayItem {
    pub id: String,
    pub service_name: String,
    pub object_path: String,
    pub title: String,
    pub icon_name: String,
    pub icon_pixmaps: Vec<IconPixmap>,
    pub status: TrayItemStatus,
    pub menu_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrayItemStatus {
    Active,
    Passive,
    NeedsAttention,
}

#[derive(Debug, Clone)]
pub struct IconPixmap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

/// A service that provides access to system tray items
#[async_trait]
pub trait TrayService: Send + Sync {
    type Error;

    async fn get_all_items(&self) -> Result<Vec<TrayItem>, Self::Error>;
}
