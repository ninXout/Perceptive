// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use ahash::AHashMap;
use image::{codecs::png::PngDecoder, DynamicImage, EncodableLayout, ImageFormat};
use itertools::Itertools;

enum Texture {
    Simple(image::DynamicImage),
    Spritesheet(AHashMap<String, image::DynamicImage>),
}

struct Project {
    textures: AHashMap<String, Texture>,
}

struct AppState {
    loaded_project: Option<Project>,
}

fn split_image(img: &DynamicImage, plist: &plist::Value) -> AHashMap<String, image::DynamicImage> {
    // let Some(frames) = v
    //     .as_dictionary()
    //     .and_then(|v| v.get("frames"))
    //     .and_then(|v| v.as_dictionary())
    // else {
    //     continue;
    // };

    // TODO
    AHashMap::new()
}

#[tauri::command]
fn read_resources_folder(
    // state: tauri::State<Mutex<AppState>>,
    path: &str,
) -> Option<String> {
    let mut pngs = AHashMap::new();
    let mut plists = AHashMap::new();

    for file in std::fs::read_dir(path).ok()? {
        // go crazy !!!!
        let path = file.ok()?.path();

        let Some(name) = path.file_stem().and_then(|v| v.to_str()) else {
            continue;
        };
        let Some(ext) = path.extension() else {
            continue;
        };

        if ext == "png" {
            let mut reader = image::io::Reader::open(&path).ok()?;
            reader.set_format(ImageFormat::Png);
            let img = reader.decode().ok()?;
            pngs.insert(name.to_string(), img.to_rgba8());
        } else if ext == "plist" {
            let p = plist::Value::from_file(&path).ok()?;
            plists.insert(name.to_string(), p);
        }

        // println!("{:?}", file.file_name().to_str())
    }

    let h = pngs.values().next().unwrap();
    Some(h.as_bytes().iter().map(|s| s.to_string()).join(","))

    // state.

    // state.lock().unwrap().loaded_project = Some(Project {
    //     textures: pngs
    //         .into_iter()
    //         .map(|(name, img)| {
    //             let tex = if let Some(v) = plists.get(&name) {
    //                 Texture::Spritesheet(split_image(&img, v))
    //             } else {
    //                 Texture::Simple(img)
    //             };
    //             (name, tex)
    //         })
    //         .collect(),
    // });
    // Some(())
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(AppState {
            loaded_project: None,
        }))
        .invoke_handler(tauri::generate_handler![read_resources_folder])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
