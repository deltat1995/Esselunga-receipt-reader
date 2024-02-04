use regex::Regex;

use crate::{currency::Money, ExportResult};
use std::{fs::File, io::Write};

const FRAGOLA_POINT_REGEX: &str = r"PUNTI FRAGOLA";
const DISCOUNT_REGEX: &str = r"SCONTO";
const FRUIT_PACK_REGEX: &str = r"(NPSACCHETTO|SACCHETTO COMPOST)";

pub fn apply_discount_and_fruit_pack(
    prodotti: Vec<String>,
    prezzi: Vec<Money>,
) -> (Vec<String>, Vec<Money>) {
    let mut prodotti_result: Vec<String> = Vec::new();
    let mut prezzi_result: Vec<Money> = Vec::new();

    let mut prez_idx = 0;
    for prodotto in prodotti.iter() {
        if Regex::new(FRUIT_PACK_REGEX).unwrap().is_match(prodotto)
            || Regex::new(DISCOUNT_REGEX).unwrap().is_match(prodotto)
        {
            let price_to_update_ref = prezzi_result.last_mut().unwrap();
            *price_to_update_ref += prezzi.get(prez_idx).unwrap().to_owned();
        } else if Regex::new(FRAGOLA_POINT_REGEX).unwrap().is_match(prodotto) {
            continue;
        } else {
            prodotti_result.push(prodotto.clone());
            prezzi_result.push(prezzi.get(prez_idx).unwrap().clone());
        }

        prez_idx += 1;
    }

    return (prodotti_result, prezzi_result);
}

pub fn remove_empty_lines(text: String) -> String {
    let mut result = String::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }

        result.push_str(format!("{}\n", line).as_str());
    }
    return result;
}

pub fn refine_and_split_products(raw: String) -> Vec<String> {
    return remove_empty_lines(raw)
        .lines()
        .map(|s| s.to_string())
        .collect();
}

pub fn refine_and_split_prices(raw: String) -> Vec<Money> {
    return remove_empty_lines(raw)
        .lines()
        .map(|p| {
            let p = &p.replace(&",".to_string(), &".".to_string());
            let re = Regex::new(r"(\d+?\.\d+?)-S").unwrap();
            let p = re.replace_all(p, "-$1");
            p.parse().unwrap_or_default()
        })
        .collect();
}

pub fn build_csv_format_string(export_result: &ExportResult) -> String {
    let mut result: String = String::new();
    for i in 0..export_result.products.len() {
        let product = export_result.products.get(i).unwrap();
        let price = export_result.prices.get(i).unwrap();
        result.push_str(format!("{},{}\n", product, price).as_str());
    }
    return result;
}

pub fn write_string_on_file(content: String, output_path: &str) {
    let mut file = File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(output_path)
        .unwrap();
    let _ = file.write_all(content.as_bytes());
}
