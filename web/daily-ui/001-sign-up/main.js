(function() {
  // the possible states
  const STATE = {
    NORMAL: 'normal',
    MID: 'mid',
    FULL: 'full',
    COMPLETE: 'complete'
  };

  const face = {
    normal: document.querySelector('#eyes--normal'),
    mid: document.querySelector('#eyes--mid'),
    full: document.querySelector('#eyes--full'),
    complete: document.querySelector('#eyes--complete')
  };

  const svg = document.querySelector('svg');

  function hideAll() {
    Object.keys(face).forEach(e => {
      face[e].setAttribute('hidden', 'true');
    });
  }

  const input = document.querySelector('input[type="email"');
  const form = document.querySelector('form');
  const welcome = document.querySelector('#welcome-message');

  input.addEventListener('input', e => {
    const { value } = e.target;

    if (value.includes('@')) {
      switchAnim(STATE.FULL);
    } else if (value.length > 0) {
      switchAnim(STATE.MID);
    } else {
      switchAnim(STATE.NORMAL);
    }
    
  });

  form.addEventListener('submit', e => {
    e.preventDefault();
    input.setAttribute('disabled', 'true');
    switchAnim(STATE.COMPLETE);
  });

  form.addEventListener('animationend', e => {
    welcome.removeAttribute('hidden');
    form.setAttribute('hidden', true);
  });

  function switchAnim(anim) {
    // do the resets
    hideAll();
    svg.classList.remove('rock');
    
    switch (anim) {
      case STATE.NORMAL:
        // show normal animation
        face.normal.removeAttribute('hidden');
        break;
      case STATE.MID:
        // show mid animation
        face.mid.removeAttribute('hidden');
        break;
      case STATE.FULL:
        // show full animation
        face.full.removeAttribute('hidden');
        break;
      case STATE.COMPLETE:
        // show complete animation
        face.complete.removeAttribute('hidden');
        svg.classList.add('rock');
        form.classList.add('complete');
        break;
    }
  }
})();