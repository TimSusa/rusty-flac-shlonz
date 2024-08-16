#![cfg_attr(
	all(not(debug_assertions), target_os = "windows"),
	windows_subsystem = "windows"
)]
use rusty_flac_shlonz::models::Post;
extern crate walkdir;

use std::env;

use rusty_flac_shlonz::{
	get_all_db_items, scan_by_path_write_db
};

pub mod models;
pub mod schema;

use tauri::api::shell;
use tauri::{
	AboutMetadata, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu, WindowBuilder,
	WindowUrl,
};


use tauri::{command, generate_handler};
use serde::Serialize;

#[derive(Serialize)]
struct Item {
    id: u32,
    name: String,
}

#[command]
fn get_items(page: u32, limit: u32) -> Vec<Item> {
	let list:  Vec<Post> = get_all_db_items();
    let total_items = list.len(); // Example total items
    let start = (page * limit) as usize;
    let end = ((page + 1) * limit) as usize;
    let items: Vec<Item> = (start..end)
        .filter(|&i| i < total_items)
        .map(|i| Item {
            id: i as u32,
            name: format!("Item {}", list.get(i).unwrap().title),
        })
        .collect();

    items
}
fn main() {
	// Get the directory path from the command line arguments
	let args: Vec<String> = env::args().collect();
	// if args.len() != 2 {
	// 	//eprintln!("Usage: {} <directory_path>", args[0]);
	// 	//get_all_db_items();
	// } else 
	
	if args.len() == 2 {	
		let dir_path: &String = &args[1];
		scan_by_path_write_db(dir_path);
	} 

	

	let ctx = tauri::generate_context!();

	tauri::Builder::default()
		.invoke_handler(tauri::generate_handler![greet])
		.invoke_handler(generate_handler![get_items])
       // .run(tauri::generate_context!())
        //.expect("error while running tauri application");
		.setup(|app| {
			let _window = WindowBuilder::new(app, "main", WindowUrl::default())
				.title("Tauri SvelteKit 4 TeamIT")
				.inner_size(800.0, 550.0)
				.min_inner_size(400.0, 200.0)
				.build()
				.expect("Unable to create window");
			Ok(())
		})
		.menu(Menu::with_items([
			#[cfg(target_os = "macos")]
			MenuEntry::Submenu(Submenu::new(
				&ctx.package_info().name,
				Menu::with_items([
					MenuItem::About(ctx.package_info().name.clone(), AboutMetadata::default())
						.into(),
					MenuItem::Separator.into(),
					MenuItem::Services.into(),
					MenuItem::Separator.into(),
					MenuItem::Hide.into(),
					MenuItem::HideOthers.into(),
					MenuItem::ShowAll.into(),
					MenuItem::Separator.into(),
					MenuItem::Quit.into(),
				]),
			)),
			MenuEntry::Submenu(Submenu::new(
				"File",
				Menu::with_items([MenuItem::CloseWindow.into()]),
			)),
			MenuEntry::Submenu(Submenu::new(
				"Edit",
				Menu::with_items([
					MenuItem::Undo.into(),
					MenuItem::Redo.into(),
					MenuItem::Separator.into(),
					MenuItem::Cut.into(),
					MenuItem::Copy.into(),
					MenuItem::Paste.into(),
					#[cfg(not(target_os = "macos"))]
					MenuItem::Separator.into(),
					MenuItem::SelectAll.into(),
				]),
			)),
			MenuEntry::Submenu(Submenu::new(
				"View",
				Menu::with_items([MenuItem::EnterFullScreen.into()]),
			)),
			MenuEntry::Submenu(Submenu::new(
				"Window",
				Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
			)),
			// You should always have a Help menu on macOS because it will automatically
			// show a menu search field
			MenuEntry::Submenu(Submenu::new(
				"Help",
				Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
			)),
		]))
		.on_menu_event(|event| {
			let event_name = event.menu_item_id();
			match event_name {
				"Learn More" => {
					let url =
						"https://github.com/probablykasper/tauri-sveltekit-template".to_string();
					shell::open(&event.window().shell_scope(), url, None).unwrap();
				}
				_ => {}
			}
		})
		.run(ctx)
		.expect("error while running tauri application");
}


#[tauri::command]
fn greet(name: &str) -> String {
	format!("Helllo, {}!", name)
}
