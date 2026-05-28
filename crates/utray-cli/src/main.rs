mod cli;

use crate::cli::{Cli, Commands, GetArgs};
use clap::Parser;
use utray::{SniService, TrayItem, TrayService};

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::List => list_tray_items().await,
        Commands::Get(args) => get_tray_item(args).await,
    }
}

async fn list_tray_items() {
    let tray_service = SniService::new().await.unwrap();

    let tray_items = tray_service.get_all_items().await.unwrap();

    for item in tray_items {
        pretty_print_tray_item(&item);
    }
}

fn pretty_print_tray_item(item: &TrayItem) {
    println!("╔════════════════════════════════════════════════════════════════════╗");
    println!("  {}", item.title);
    println!("╚════════════════════════════════════════════════════════════════════╝");
    println!("  ├─ ID:          {}", item.id);
    println!("  ├─ Service:     {}", item.service_name);
    println!("  ├─ Object Path: {}", item.object_path);
    println!("  ├─ Menu Path:   {}", item.menu_path);
    println!("  ├─ Status:      {:?}", item.status);

    if !item.icon_name.is_empty() {
        println!("  ├─ Icon Name:   {}", item.icon_name);
    } else {
        println!("  ├─ Icon Name:   (None)");
    }

    if item.icon_pixmaps.is_empty() {
        println!("  └─ Pixmaps:     None");
    } else {
        println!("  └─ Pixmaps:     {} item(s)", item.icon_pixmaps.len());
        for (i, pixmap) in item.icon_pixmaps.iter().enumerate() {
            println!("     └─ [{}] {}x{} px", i, pixmap.width, pixmap.height);
        }
    }

    println!();
}

async fn get_tray_item(args: GetArgs) {
    let tray_service = SniService::new().await.unwrap();

    match tray_service.get_item_by_id(&args.id).await {
        Ok(Some(item)) => pretty_print_tray_item(&item),
        Ok(None) => eprintln!("No tray item found"),
        Err(e) => eprintln!("Error retrieving tray item: {e}"),
    }
}
