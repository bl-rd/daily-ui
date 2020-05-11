(function () {
  let darkMode = false;
  const button = document.querySelector('button');
  const partyRoom = document.querySelector('.party-room');
  const header = document.querySelector('header');
  const main = document.querySelector('main');

  const opacityCss = '--party-room-opacity';
  const transitionDuration = 1750;

  button.addEventListener('click', toggleDarkMode);

  function toggleDarkMode() {
    darkMode = !darkMode;
    switch (darkMode) {
      case true:
        handleButton(true);
        header.classList.add('party-time');
        partyRoom.removeAttribute('hidden');
        setTimeout(() => partyRoom.style.setProperty(opacityCss, '1'), 10);
        break;
      case false:
        handleButton(false);
        header.classList.remove('party-time');
        partyRoom.style.setProperty(opacityCss, '0');
        setTimeout(() => {
          partyRoom.setAttribute('hidden', '');
        }, transitionDuration);
        break;
      default:
        return;
    }
  }

  /**
   * 
   * @param {Boolean} goingDark 
   */
  function handleButton(goingDark) {
    button.setAttribute('disabled', 'true');
    setTimeout(() => button.removeAttribute('disabled'), transitionDuration);
    button.innerHTML = goingDark ? 'Turn the lights on!' : 'Dark mode';
  }


})();