mod sni;

pub use sni::SniService;

use async_trait::async_trait;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TrayItemStatus {
    Active,
    Passive,
    NeedsAttention,
}

#[derive(Debug, Clone, Serialize)]
pub struct IconPixmap {
    pub width: u32,
    pub height: u32,
    #[serde(skip_serializing)]
    pub data: Vec<u8>,
}

impl IconPixmap {
    /// Convert the pixmap data to RGBA format
    pub fn to_rgba_data(&self) -> Vec<u8> {
        let mut rgba_data = Vec::with_capacity(self.data.len());
        for pixel in self.data.chunks(4) {
            rgba_data.push(pixel[1]); // R
            rgba_data.push(pixel[2]); // G
            rgba_data.push(pixel[3]); // B
            rgba_data.push(pixel[0]); // A
        }
        rgba_data
    }

    /// Display the pixmap by kitty's graphics protocol
    pub fn display_kitty(&self) -> String {
        use base64::{Engine, engine::general_purpose::STANDARD};

        let rgba_data = self.to_rgba_data();
        let base64_data = STANDARD.encode(&rgba_data);
        let bytes = base64_data.as_bytes();

        const CHUNK_SIZE: usize = 4096;
        let mut result = String::new();
        let total_len = bytes.len();
        let mut offset = 0;

        while offset < total_len {
            let end = std::cmp::min(offset + CHUNK_SIZE, total_len);
            let chunk = &bytes[offset..end];
            let is_last_chunk = end == total_len;

            if !is_last_chunk {
                result.push_str(&format!(
                    "\x1b_Ga=T,f=32,s={width},v={height},m=1;{data};\x1b\\",
                    width = self.width,
                    height = self.height,
                    data = std::str::from_utf8(chunk).unwrap()
                ));
            } else {
                result.push_str(&format!(
                    "\x1b_Ga=T,f=32,s={width},v={height},m=0;{data};\x1b\\",
                    width = self.width,
                    height = self.height,
                    data = std::str::from_utf8(chunk).unwrap()
                ));
            }

            offset += CHUNK_SIZE;
        }

        result
    }
}

/// A service that provides access to system tray items
#[async_trait]
pub trait TrayService: Send + Sync {
    type Error;

    async fn get_all_items(&self) -> Result<Vec<TrayItem>, Self::Error>;
    async fn get_item_by_id(&self, id: &str) -> Result<Option<TrayItem>, Self::Error>;
}
