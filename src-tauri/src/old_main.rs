// mod currency;
// mod image_processor;
// mod pdf_reader;
// mod post_processor;

// use std::borrow::BorrowMut;

// use pdf_reader::extract_receipt_from_pdf;
// use rusty_tesseract::image::{self, imageops, DynamicImage};

// use image_processor::*;
// use post_processor::*;

// // const PRODUCTS_PATH: &str = "./samples/scontrino2/prodotti.jpg";
// // const PRICES_PATH: &str = "./samples/scontrino2/prezzi.jpg";
// const RECEIPT_PATH: &str = "./samples/scontrinoIntero/scontrinoHD.jpg";
// const OUTPUT_PATH: &str = "./samples/scontrinoIntero/risultato.csv";

// const IS_PDF_FILE: bool = true;
// const RECEIPT_PDF_PATH: &str = "./samples/scontrinoIntero/scontrino.pdf";

// fn main() {
//     let products_raw: String;
//     let prices_raw: String;

//     if IS_PDF_FILE {
//         (products_raw, prices_raw) = extract_receipt_from_pdf(RECEIPT_PDF_PATH);
//     } else {
//         let mut receipt_img =
//             image::open(RECEIPT_PATH).expect("Impossibile aprire l'immagine dello scontrino");

//         let products_img_buf =
//             imageops::crop(receipt_img.borrow_mut(), 0, 790, 550, 3450).to_image();
//         let products_img: DynamicImage = products_img_buf.into();
//         products_raw = extract_text_from_image(&products_img, CharList::WORD.value(), 96);

//         let prices_img_buf =
//             imageops::crop(receipt_img.borrow_mut(), 650, 790, 170, 3450).to_image();
//         let prices_img: DynamicImage = prices_img_buf.into();
//         prices_raw = extract_text_from_image(&prices_img, CharList::PRICE.value(), 96);
//     }

//     let mut product_list = refine_and_split_products(products_raw);
//     let mut price_list = refine_and_split_prices(prices_raw);

//     (product_list, price_list) = apply_discount_and_fruit_pack(product_list, price_list);
//     assert!(product_list.len() == price_list.len());

//     let csv_string = build_csv_format_string(product_list, price_list);
//     write_string_on_file(csv_string, OUTPUT_PATH);
// }

use druid::im::{vector, Vector};
use druid::piet::InterpolationMode;
use druid::widget::{
    Button, Container, FillStrat, Flex, Image, Label, LensWrap, List, Split, TextBox,
};
use druid::{
    AppLauncher, Color, Command, Data, FileDialogOptions, ImageBuf, Lens, Widget, WidgetExt,
    WindowDesc,
};
use rusty_tesseract::image::{self, imageops, DynamicImage};
use std::borrow::BorrowMut;

#[derive(Clone, Data, Lens)]
struct TodoList {
    items: Vector<String>,
    next_item: String,
}

// const RECEIPT_PATH: &str = "./samples/scontrinoIntero/scontrinoHD.jpg";
// let mut receipt_img =
//     image::open(RECEIPT_PATH).expect("Impossibile aprire l'immagine dello scontrino");
// let products_img_buf = imageops::crop(receipt_img.borrow_mut(), 0, 790, 550, 500).to_image();
// let products_img: DynamicImage = products_img_buf.into();
// Mettere immagini
// Image::new(ImageBuf::from_dynamic_image(products_img))
// .fill_mode(FillStrat::FitHeight)
// .interpolation_mode(InterpolationMode::Bilinear),

fn build_ui() -> impl Widget<TodoList> {
    Flex::column()
        .with_child(
            Button::new("Import File").on_click(|ctx, data: &mut TodoList, _| {
                ctx.submit_command(Command::new(
                    druid::commands::SHOW_OPEN_PANEL,
                    FileDialogOptions::new(),
                    druid::Target::Auto,
                ));
            }),
        )
        .with_child(Split::columns(
            Container::new(LensWrap::new(
                List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
                TodoList::items,
            ))
            .fix_width(30.0),
            Container::new(LensWrap::new(
                List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
                TodoList::items,
            )),
        ))
    // Split::columns(
    //     Container::new(LensWrap::new(
    //         List::new(|| Label::dynamic(|data, _| format!("List item: {data}"))),
    //         TodoList::items,
    //     ))
    //     .border(Color::grey(0.6), 2.0),
    //     Container::new(
    //         Flex::column()
    //             .with_flex_child(

    //                 1.0,
    //             )
    //             .with_flex_child(LensWrap::new(TextBox::new(), TodoList::next_item), 1.0),
    //     )
    //     .border(Color::grey(0.6), 2.0),
    // )
}

fn main() {
    let main_window = WindowDesc::new(build_ui())
        .window_size((600.0, 400.0))
        .title("My first Druid App");

    let initial_data = TodoList {
        items: vector![
            "first item".into(),
            "second item".into(),
            "third item".into(),
            "foo".into(),
            "bar".into(),
        ],
        next_item: String::new(),
    };

    AppLauncher::with_window(main_window)
        .launch(initial_data)
        .expect("Failed to launch application");
}
