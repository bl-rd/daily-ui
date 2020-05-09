(function () {
  const template = document.querySelector('#noodles');
  const delay = 0.45 * 1000;
  const count = 3;

  for (let i = 0; i < count; i++) {
    let item = document.importNode(template.content, true);
    setTimeout(() => document.querySelector('ul').appendChild(item), delay * (i + 1));
  }
})();