# Esselunga Receipt Reader

This project is written in Rust and has the following requirements:

- Rust and cargo
- Google tesseract (https://github.com/tesseract-ocr/tesseract)
- Python3 and the library PyPDF2, re, argparse
- The script pdf_reader.py should be in the folder `etc` placed in the same folder of the executable file. Don't move the file already correctly placed in the `src-tauri` folder

## Running commands

To run the project locally in dev mode use the command:

```
cargo tauri dev
```

To build the executable file:

```
cargo tauri build
```

After build, the executable file will be in the `/src-tauri/target/release` path.
