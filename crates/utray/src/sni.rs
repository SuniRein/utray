use crate::{IconPixmap, TrayItem, TrayItemStatus, TrayService};
use async_trait::async_trait;
use zbus::{Connection, proxy, zvariant::OwnedObjectPath};

#[proxy(
    interface = "org.kde.StatusNotifierWatcher",
    default_service = "org.kde.StatusNotifierWatcher",
    default_path = "/StatusNotifierWatcher"
)]
trait StatusNotifierWatcher {
    #[zbus(property, name = "RegisteredStatusNotifierItems")]
    fn registered_items(&self) -> zbus::Result<Vec<String>>;
}

#[proxy(
    interface = "org.kde.StatusNotifierItem",
    default_path = "/StatusNotifierItem"
)]
trait StatusNotifierItem {
    #[zbus(property)]
    fn id(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn title(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn status(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn icon_name(&self) -> zbus::Result<String>;

    #[zbus(property)]
    fn icon_pixmap(&self) -> zbus::Result<Vec<(i32, i32, Vec<u8>)>>;

    #[zbus(property)]
    fn menu(&self) -> zbus::Result<OwnedObjectPath>;
}

pub struct SniService {
    connection: Connection,
}

impl SniService {
    pub async fn new() -> zbus::Result<Self> {
        let connection = Connection::session().await?;
        Ok(Self { connection })
    }
}

#[async_trait]
impl TrayService for SniService {
    type Error = zbus::Error;

    async fn get_all_items(&self) -> Result<Vec<TrayItem>, Self::Error> {
        let watcher = StatusNotifierWatcherProxy::new(&self.connection).await?;
        let items: Vec<String> = watcher.registered_items().await?;

        let mut tray_items = Vec::new();

        for entry in &items {
            let Some(pos) = entry.find('/') else {
                continue; // Skip invalid entries
            };

            let service = &entry[..pos];
            let path = &entry[pos..];

            match self.fetch_item(service, path).await {
                Ok(item) => tray_items.push(item),
                Err(e) => {
                    eprintln!("Failed to fetch item for service '{service}' at path '{path}': {e}")
                }
            }
        }

        Ok(tray_items)
    }

    async fn get_item_by_id(&self, id: &str) -> Result<Option<TrayItem>, Self::Error> {
        self.get_all_items()
            .await
            .map(|items| items.into_iter().find(|item| item.id == id))
    }
}

impl SniService {
    async fn fetch_item(&self, service: &str, path: &str) -> zbus::Result<TrayItem> {
        let proxy = StatusNotifierItemProxy::builder(&self.connection)
            .destination(service)?
            .path(path)?
            .build()
            .await?;

        let id = proxy.id().await?;
        let title = proxy.title().await?;

        let status_str = proxy.status().await?;
        let status = match status_str.as_str() {
            "Active" => TrayItemStatus::Active,
            "NeedsAttention" => TrayItemStatus::NeedsAttention,
            _ => TrayItemStatus::Passive,
        };

        let icon_name = proxy.icon_name().await?;

        // Some items may not provide pixmaps, so we handle errors gracefully and return an empty
        // vector if retrieval fails.
        let icon_pixmaps = match proxy.icon_pixmap().await {
            Ok(pixmaps) => pixmaps
                .into_iter()
                .map(|(w, h, data)| IconPixmap {
                    width: w as u32,
                    height: h as u32,
                    data,
                })
                .collect(),
            Err(e) => {
                eprintln!("Failed to get icon pixmaps for item '{id}': {e}");
                Vec::new()
            }
        };

        let menu_path = proxy.menu().await?;

        Ok(TrayItem {
            id,
            service_name: service.to_string(),
            object_path: path.to_string(),
            title,
            icon_name,
            icon_pixmaps,
            status,
            menu_path: menu_path.to_string(),
        })
    }
}
