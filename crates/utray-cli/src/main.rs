mod cli;

use crate::cli::{Cli, Commands, GetArgs, IconDisplayMode, ListArgs};
use clap::Parser;
use utray::{SniService, TrayItem, TrayService};

macro_rules! exit {
    ($expr:expr) => {{
        eprintln!($expr);
        std::process::exit(1);
    }};
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::List(args) => list_tray_items(args).await,
        Commands::Get(args) => get_tray_item(args).await,
    }
}

async fn list_tray_items(args: ListArgs) {
    let tray_service = SniService::new().await.unwrap();

    let tray_items = tray_service.get_all_items().await.unwrap();

    if args.json {
        let json = serde_json::to_string_pretty(&tray_items).unwrap();
        println!("{json}");
        return;
    }

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

    match &item.icon_name {
        Some(name) => println!("  ├─ Icon Name:   {name}",),
        None => println!("  ├─ Icon Name:   (None)"),
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

    let item = match tray_service.get_item_by_id(&args.id).await {
        Ok(Some(item)) => item,
        Ok(None) => exit!("No tray item found"),
        Err(e) => exit!("Error retrieving tray item: {e}"),
    };

    if let Some(icon_mode) = args.icon {
        if item.icon_pixmaps.is_empty() {
            exit!("No pixmaps available for this item");
        }

        match icon_mode {
            IconDisplayMode::Rgba => {
                let pixmap = &item.icon_pixmaps[0];
                let rgba_data = pixmap.to_rgba_data();
                for chunk in rgba_data.chunks(16) {
                    println!(
                        "{}",
                        chunk
                            .iter()
                            .map(|b| format!("{b:02X}"))
                            .collect::<Vec<_>>()
                            .join(" ")
                    );
                }
            }

            IconDisplayMode::Kitty => {
                let pixmap = &item.icon_pixmaps[0];
                println!("{}", pixmap.display_kitty());
            }
        }

        return;
    }

    if args.json {
        let json = serde_json::to_string_pretty(&item).unwrap();
        println!("{json}");
        return;
    }

    pretty_print_tray_item(&item);
}
