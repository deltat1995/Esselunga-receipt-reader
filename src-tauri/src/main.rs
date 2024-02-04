// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod currency;
mod image_processor;
mod pdf_reader;
mod post_processor;

use std::sync::Mutex;

use currency::Money;
use image_processor::*;
use pdf_reader::extract_receipt_from_pdf;
use post_processor::*;

use image::{self, ImageFormat};
use tauri::State;

#[derive(serde::Serialize)]
struct CustomImage {
    data_src: String,
    extension: String,
    height: u32,
    width: u32,
    x: i32,
    y: i32,
}

#[derive(serde::Deserialize)]
struct CropSize {
    height: u32,
    width: u32,
    x: u32,
    y: u32,
}

#[derive(serde::Serialize)]
struct PreviewImages {
    products_img_src: String,
    prices_img_src: String,
}

#[derive(serde::Serialize)]
struct PreviewResult {
    products: Vec<String>,
    prices: Vec<String>,
}

#[derive(Clone)]
struct ExportResult {
    products: Vec<String>,
    prices: Vec<Money>,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn open_image(state: State<'_, Mutex<Option<String>>>, path: &str) -> Result<CustomImage, String> {
    let receipt_img =
        image::open(path).map_err(|_| "unexpected error to open the receipt image".to_string())?;
    let image_format =
        ImageFormat::from_path(path).map_err(|_| "invalid extension file".to_string())?;
    let image_src: String = generate_img_src_from(&receipt_img, image_format)?;

    *state.lock().unwrap() = Some(path.to_string());

    return Ok(CustomImage {
        data_src: image_src,
        extension: image_format.extensions_str()[0].to_string(),
        height: receipt_img.height(),
        width: receipt_img.width(),
        x: 0,
        y: 0,
    });
}

#[tauri::command]
fn open_pdf(
    result_state: State<'_, Mutex<Option<ExportResult>>>,
    path: &str,
) -> Result<PreviewResult, String> {
    let (products_raw, prices_raw) = extract_receipt_from_pdf(path);
    let mut product_list = refine_and_split_products(products_raw);
    let mut price_list = refine_and_split_prices(prices_raw);
    (product_list, price_list) = apply_discount_and_fruit_pack(product_list, price_list);
    if product_list.len() != price_list.len() {
        return Err("The number of products and prices are different.".to_string());
    }

    *result_state.lock().unwrap() = Some(ExportResult {
        products: product_list.clone(),
        prices: price_list.clone(),
    });

    return Ok(PreviewResult {
        products: product_list,
        prices: price_list.into_iter().map(|x| format!("{}€", x)).collect(),
    });
}

#[tauri::command]
fn crop_image(
    state: State<'_, Mutex<Option<String>>>,
    products_size: CropSize,
    prices_size: CropSize,
) -> Result<PreviewImages, String> {
    let state = state
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("Unexpected error: image to crop not found".to_string())?
        .clone();

    let receipt_img = image::open(&state)
        .map_err(|_| "unexpected error to open the receipt image".to_string())?;
    let image_format =
        ImageFormat::from_path(&state).map_err(|_| "invalid extension file".to_string())?;

    let products_image = receipt_img.crop_imm(
        products_size.x,
        products_size.y,
        products_size.width,
        products_size.height,
    );
    let prices_image = receipt_img.crop_imm(
        prices_size.x,
        prices_size.y,
        prices_size.width,
        prices_size.height,
    );

    return Ok(PreviewImages {
        products_img_src: generate_img_src_from(&products_image, image_format)?,
        prices_img_src: generate_img_src_from(&prices_image, image_format)?,
    });
}

#[tauri::command]
fn load_preview(
    path_state: State<'_, Mutex<Option<String>>>,
    result_state: State<'_, Mutex<Option<ExportResult>>>,
    products_size: CropSize,
    prices_size: CropSize,
) -> Result<PreviewResult, String> {
    let path = path_state
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("Unexpected error: image to crop not found".to_string())?
        .clone();

    let receipt_img =
        image::open(&path).map_err(|_| "unexpected error to open the receipt image".to_string())?;

    let products_image = receipt_img.crop_imm(
        products_size.x,
        products_size.y,
        products_size.width,
        products_size.height,
    );
    let prices_image = receipt_img.crop_imm(
        prices_size.x,
        prices_size.y,
        prices_size.width,
        prices_size.height,
    );

    let products_raw = extract_text_from_image(&products_image, CharList::WORD.value(), 300);
    let prices_raw = extract_text_from_image(&prices_image, CharList::PRICE.value(), 300);
    let mut product_list = refine_and_split_products(products_raw);
    let mut price_list = refine_and_split_prices(prices_raw);
    (product_list, price_list) = apply_discount_and_fruit_pack(product_list, price_list);
    if product_list.len() != price_list.len() {
        return Err("The number of products and prices are different.".to_string());
    }

    *result_state.lock().unwrap() = Some(ExportResult {
        products: product_list.clone(),
        prices: price_list.clone(),
    });

    return Ok(PreviewResult {
        products: product_list,
        prices: price_list.into_iter().map(|x| format!("{}€", x)).collect(),
    });
}

#[tauri::command]
fn export_csv(
    result_state: State<'_, Mutex<Option<ExportResult>>>,
    path: &str,
) -> Result<(), String> {
    let res = result_state
        .lock()
        .unwrap()
        .as_ref()
        .ok_or("Remember to load preview before to export".to_string())?
        .clone();

    let csv_string = build_csv_format_string(&res);
    write_string_on_file(csv_string, path);
    return Ok(());
}

fn main() {
    tauri::Builder::default()
        .manage(Mutex::new(None as Option<String>))
        .manage(Mutex::new(None as Option<ExportResult>))
        .invoke_handler(tauri::generate_handler![
            open_image,
            crop_image,
            load_preview,
            export_csv,
            open_pdf,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
