use std::process::Command;

pub fn extract_receipt_from_pdf(path: &str) -> (String, String) {
    if !cfg!(target_os = "windows") {
        panic!("Feature supported only on Windows")
    }

    let output = Command::new("python3")
        .arg(".\\etc\\pdf_reader.py")
        .arg(path)
        .output()
        .expect("failed to execute process");
    let result = String::from_utf8(output.stdout).unwrap();

    let mut products_raw = String::new();
    let mut prices_raw = String::new();

    for line in result.lines() {
        let result_splitted: Vec<&str> = line.split(",").collect();
        let product = result_splitted[0];
        let price = result_splitted[1];
        products_raw.push_str(product);
        products_raw.push('\n');
        prices_raw.push_str(price);
        prices_raw.push('\n');
    }

    return (products_raw, prices_raw);
}
