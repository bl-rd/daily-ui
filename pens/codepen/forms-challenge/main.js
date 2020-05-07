(function () {
  const focusClass = 'focused';
  const inputs = Array.from(document.querySelectorAll('input'));
  inputs.map(input => {
    input.addEventListener('focusin', event => {
      input.parentElement.classList.add(focusClass);
    });
    input.addEventListener('focusout', event => {
      input.parentElement.classList.remove(focusClass);
    });
  });

  const button = document.querySelector('button');
  button.addEventListener('click', event => {
    button.parentElement.style.display = 'none';
    const start = document.querySelector('fieldset');
    start.classList.add(focusClass);
    start.querySelector('input').focus();
  });

  const go = document.querySelector('button[type="submit"]');
  go.addEventListener('focusin', event => {
    go.parentElement.classList.add(focusClass);
  });
  go.addEventListener('focusout', event => {
    go.parentElement.classList.remove(focusClass);
  });
})();