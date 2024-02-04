from PyPDF2 import PdfReader 
import re
import argparse

PAGE_WITH_RECEIPT = 1
START_RECEIPT_PATTERN = "IVA.*?EURO"
END_RECEIPT_PATTERN = "TOTALE.*?EURO"
PROD_PRICE_PATTERN = r"^(.*?)(?: +\*[cad] +)?(\d+,\d{2}(?:-S)? *)$"


def read_receipt_pdf(path):
    reader = PdfReader(path) 
    page = reader.pages[PAGE_WITH_RECEIPT-1] 
    lines = page.extract_text().splitlines() 

    start = 0
    end = 0
    for idx, line in enumerate(lines):
        if bool(re.search(START_RECEIPT_PATTERN, line)):
            start=idx+1
        if start!=0 and bool(re.search(END_RECEIPT_PATTERN, line)):
            end=idx
            break

    prod_prices = lines[start:end]
    prod_prices = list(map(lambda x : re.sub("\\xa0"," ",x) ,prod_prices))

    for prod_price in prod_prices:
        match = re.search(PROD_PRICE_PATTERN, prod_price)
        if match:
            prod, price = match.group(1,2)
            print(prod.strip()+","+price.strip().replace(",","."))



if __name__ == "__main__":
    parser=argparse.ArgumentParser(description="Read receipt from pdf file")
    parser.add_argument("pdf_file_path")
    args=parser.parse_args()
    read_receipt_pdf(args.pdf_file_path)