import("../pkg/index.js")
  .then(rust => {
    console.log({rust})
    rust.main_js();
  })
  .catch(console.error);
