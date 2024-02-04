const { open, save } = window.__TAURI__.dialog
const { invoke } = window.__TAURI__.tauri;

const IMPORT_IMAGE_BUTTON = 'import-image-button'
const IMPORT_PDF_BUTTON = 'import-pdf-button'
const PROD_PRICE_SPLITTER_FORM = 'prod-price-splitter'
const PRODS_IMG = "products-img"
const PRICES_IMG = "prices-img"
const LOAD_PREVIEW_BUTTON = "loadPreviewButton"
const TABLE_EXPORTER = 'table-exporter'
const PREVIEW_TABLE_BODY = 'preview-table-body'
const EXPORT_BUTTON = 'export-button'


window.addEventListener("DOMContentLoaded", () => {
  const prodPriceForm = document.getElementById(PROD_PRICE_SPLITTER_FORM)
  prodPriceForm.className = 'not_visible'
  prodPriceForm.addEventListener('submit', onSubmitProdPriceSplitter)
  document.getElementById(LOAD_PREVIEW_BUTTON).addEventListener('click', onLoadPreviewProdPriceSplitter)
  document.getElementById(TABLE_EXPORTER).className = 'not_visible'
  document.getElementById(IMPORT_IMAGE_BUTTON).addEventListener('click', importImage);
  document.getElementById(IMPORT_PDF_BUTTON).addEventListener('click', importPDF);
  document.getElementById(EXPORT_BUTTON).addEventListener('click', exportCsvFile);
});

async function importImage(e) {
  e.preventDefault()
  // Open a selection dialog for image files
  const filePath = await open({
    title: "Select an receipt as image",
    filters: [{ name: 'Receipt', extensions: ['png', 'jpeg', 'jpg'] }]
  });

  invoke("open_image", { path: filePath })
    .then(result => {
      document.getElementById(PRODS_IMG).src = result.data_src
      document.getElementById(PRICES_IMG).src = result.data_src

      document.getElementById('productsHeight').value = result.height
      document.getElementById('productsWidth').value = result.width
      document.getElementById('productsX').value = result.x
      document.getElementById('productsY').value = result.y
      document.getElementById('pricesHeight').value = result.height
      document.getElementById('pricesWidth').value = result.width
      document.getElementById('pricesX').value = result.x
      document.getElementById('pricesY').value = result.y


      document.getElementById(PROD_PRICE_SPLITTER_FORM).className = 'visible'
      document.getElementById(TABLE_EXPORTER).className = 'not_visible'
    })
    .catch(err => console.error(`There is an error: ${err}`));
}

async function importPDF(e) {
  e.preventDefault()
  const filePath = await open({
    title: "Select an receipt as pdf",
    filters: [{ name: 'Receipt', extensions: ['pdf'] }]
  });
  invoke("open_pdf", { path: filePath })
    .then(result => {
      const { products, prices } = result
      const tableBody = document.getElementById(PREVIEW_TABLE_BODY)
      tableBody.innerHTML = ""
      for (let i = 0; i < products.length; i++) {
        const tr = document.createElement('tr')
        const prod = document.createElement('td')
        prod.textContent = products[i]
        const price = document.createElement('td')
        price.textContent = prices[i]
        tr.appendChild(prod)
        tr.appendChild(price)
        tableBody.appendChild(tr)
      }
      document.getElementById(PROD_PRICE_SPLITTER_FORM).className = 'not_visible'
      document.getElementById(TABLE_EXPORTER).className = 'visible'
    })
    .catch(err => console.error(`There is an error: ${err}`));
}

function onLoadPreviewProdPriceSplitter(e) {
  e.preventDefault()
  const form = e.target.form
  const productsCropBox = {
    height: parseInt(form.productsHeight.value),
    width: parseInt(form.productsWidth.value),
    x: parseInt(form.productsX.value),
    y: parseInt(form.productsY.value),
  }
  const pricesCropBox = {
    height: parseInt(form.pricesHeight.value),
    width: parseInt(form.pricesWidth.value),
    x: parseInt(form.pricesX.value),
    y: parseInt(form.pricesY.value),
  }

  invoke("load_preview", { productsSize: productsCropBox, pricesSize: pricesCropBox })
    .then(result => {
      const { products, prices } = result
      const tableBody = document.getElementById(PREVIEW_TABLE_BODY)
      tableBody.innerHTML = ""
      for (let i = 0; i < products.length; i++) {
        const tr = document.createElement('tr')
        const prod = document.createElement('td')
        prod.textContent = products[i]
        const price = document.createElement('td')
        price.textContent = prices[i]
        tr.appendChild(prod)
        tr.appendChild(price)
        tableBody.appendChild(tr)
      }
      document.getElementById(TABLE_EXPORTER).className = 'visible'
    })
    .catch(err => console.error(`There is an error: ${err}`));

}

function onSubmitProdPriceSplitter(e) {
  e.preventDefault()
  const form = e.target
  const productsCropBox = {
    height: parseInt(form.productsHeight.value),
    width: parseInt(form.productsWidth.value),
    x: parseInt(form.productsX.value),
    y: parseInt(form.productsY.value),
  }
  const pricesCropBox = {
    height: parseInt(form.pricesHeight.value),
    width: parseInt(form.pricesWidth.value),
    x: parseInt(form.pricesX.value),
    y: parseInt(form.pricesY.value),
  }

  invoke("crop_image", { productsSize: productsCropBox, pricesSize: pricesCropBox })
    .then(result => {
      document.getElementById(PRODS_IMG).src = result.products_img_src
      document.getElementById(PRICES_IMG).src = result.prices_img_src
    })
    .catch(err => console.error(`There is an error: ${err}`));
}

async function exportCsvFile(e) {
  e.preventDefault()
  const filePath = await save({
    title: "Save your receipt as csv",
    filters: [{
      name: 'Receipt',
      extensions: ['csv']
    }]
  });

  invoke("export_csv", { path: filePath })
    .then(result => {
      document.querySelector('.success-message').className = 'success-message visible'
      setTimeout(() => {
        document.querySelector('.success-message').className = 'success-message not_visible'
      }, 3000)
    })
    .catch(err => console.error(`There is an error: ${err}`));
}
