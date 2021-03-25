(function() {
    const slider = document.querySelector('input');
    const button = document.querySelector('button');

    const baseValue = '345';
    const varName = '--base-hue';
    reset();

    slider.addEventListener('input', event => setHue(event.target.value));

    button.addEventListener('click', reset);

    function setHue(value) {
        document.documentElement.style.setProperty(varName, value);
    }

    function reset() {
        setHue(baseValue);
        slider.value = baseValue;
    }
})();