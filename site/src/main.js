import './style.css';

if (location.pathname === '/' && new URLSearchParams(location.search).get('demo') === '1') {
  location.replace('/demo/?demo=1');
}

document.querySelectorAll('[data-copy]').forEach(button => button.addEventListener('click', async () => {
  const original = button.textContent;
  try {
    await navigator.clipboard.writeText(button.dataset.copy);
    button.textContent = 'Copied';
  } catch {
    button.textContent = 'Select command';
  }
  window.setTimeout(() => { button.textContent = original; }, 1600);
}));

const DEMO_KEY = 'demo:animation-shot-runner:opened';
const DEMO_PREFIX = 'demo:animation-shot-runner:';
const reset = document.querySelector('[data-reset-demo]');
const startReal = document.querySelector('[data-start-real]');
const demoNotice = document.querySelector('#demo-notice');
if (document.body.dataset.demo === 'true') {
  try { localStorage.setItem(DEMO_KEY, 'true'); } catch {}
  reset?.addEventListener('click', () => {
    try { localStorage.removeItem(DEMO_KEY); localStorage.setItem(DEMO_KEY, 'true'); } catch {}
    if (demoNotice) demoNotice.textContent = 'Sample view reset. Run the command again to create a new temporary folder.';
  });
  startReal?.addEventListener('click', event => {
    event.preventDefault();
    try {
      for (let index = localStorage.length - 1; index >= 0; index -= 1) {
        const key = localStorage.key(index);
        if (key?.startsWith(DEMO_PREFIX)) localStorage.removeItem(key);
      }
    } catch {}
    location.assign(startReal.href);
  });
}

const network = document.querySelector('#network');
function updateNetwork() { if (network) network.hidden = navigator.onLine; }
window.addEventListener('online', updateNetwork);
window.addEventListener('offline', updateNetwork);
updateNetwork();

if ('serviceWorker' in navigator && location.protocol !== 'file:') {
  window.addEventListener('load', () => navigator.serviceWorker.register('/sw.js').catch(() => {}));
}
