import './style.css';

if (location.pathname === '/' && new URLSearchParams(location.search).get('demo') === '1') {
  location.replace('/demo/?demo=1');
}

const ROUTE_STATE_KEY = 'shotRunnerRouteState';
const ROUTE_TRANSITION_KEY = 'shotRunnerRouteTransition';
const focusableSelector = 'a[href], button:not([disabled]), input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';
const pageHeading = document.querySelector('main h1');
const routeAnnouncement = document.createElement('p');
routeAnnouncement.className = 'screen-reader-text';
routeAnnouncement.dataset.routeAnnouncement = 'true';
routeAnnouncement.setAttribute('aria-live', 'polite');
routeAnnouncement.setAttribute('aria-atomic', 'true');
document.body.append(routeAnnouncement);

function routeEndpoint(url = new URL(location.href)) {
  return `${url.pathname}${url.search}`;
}

function focusableElements() {
  return [...document.querySelectorAll(focusableSelector)].filter(element => element.getClientRects().length > 0);
}

function saveRouteState() {
  const active = document.activeElement;
  const focusables = focusableElements();
  const focus = active === pageHeading
    ? {kind: 'heading'}
    : {kind: 'index', index: focusables.indexOf(active)};
  const current = history.state && typeof history.state === 'object' ? history.state : {};
  history.replaceState({
    ...current,
    [ROUTE_STATE_KEY]: {
      scroll: {x: window.scrollX, y: window.scrollY},
      focus,
    },
  }, '', location.href);
}

function announceRoute() {
  if (pageHeading) routeAnnouncement.textContent = `Page: ${pageHeading.textContent.trim()}`;
}

function focusHeading(announce = false) {
  if (!pageHeading) return;
  pageHeading.setAttribute('tabindex', '-1');
  pageHeading.focus({preventScroll: true});
  if (announce) announceRoute();
}

function restoreRouteState() {
  const state = history.state?.[ROUTE_STATE_KEY];
  if (!state) return;
  requestAnimationFrame(() => {
    window.scrollTo({left: state.scroll?.x || 0, top: state.scroll?.y || 0, behavior: 'instant'});
    requestAnimationFrame(() => {
      window.scrollTo({left: state.scroll?.x || 0, top: state.scroll?.y || 0, behavior: 'instant'});
      if (state.focus?.kind === 'heading') {
        focusHeading(true);
        return;
      }
      const target = focusableElements()[state.focus?.index];
      (target || pageHeading)?.focus({preventScroll: true});
      announceRoute();
    });
  });
}

function pendingRouteTransition() {
  try {
    const pending = JSON.parse(sessionStorage.getItem(ROUTE_TRANSITION_KEY) || 'null');
    if (pending?.to === routeEndpoint()) {
      sessionStorage.removeItem(ROUTE_TRANSITION_KEY);
      return true;
    }
  } catch {}
  return false;
}

if ('scrollRestoration' in history) history.scrollRestoration = 'manual';
const arrivingFromInternalLink = pendingRouteTransition();
if (arrivingFromInternalLink) requestAnimationFrame(() => focusHeading(true));

function internalDocumentLink(event) {
  if (event.defaultPrevented || event.button !== 0 || event.metaKey || event.ctrlKey || event.shiftKey || event.altKey) return null;
  const source = event.target instanceof Element ? event.target : event.target?.parentElement;
  const link = source?.closest('a[href]');
  if (!link || (link.target && link.target !== '_self') || link.hasAttribute('download')) return null;
  const target = new URL(link.href, location.href);
  if (target.origin !== location.origin || target.protocol !== location.protocol || routeEndpoint(target) === routeEndpoint()) return null;
  return target;
}

let pointerPreparedFor = '';
let preservingRouteState = false;
document.addEventListener('pointerdown', event => {
  const target = internalDocumentLink(event);
  if (!target) return;
  saveRouteState();
  pointerPreparedFor = routeEndpoint(target);
}, true);

document.addEventListener('click', event => {
  const target = internalDocumentLink(event);
  if (!target) return;
  const endpoint = routeEndpoint(target);
  if (pointerPreparedFor !== endpoint) saveRouteState();
  pointerPreparedFor = '';
  preservingRouteState = true;
  try { sessionStorage.setItem(ROUTE_TRANSITION_KEY, JSON.stringify({to: endpoint})); } catch {}
}, true);

window.addEventListener('pagehide', () => {
  if (!preservingRouteState) saveRouteState();
});
window.addEventListener('popstate', restoreRouteState);
window.addEventListener('pageshow', event => {
  const navigation = performance.getEntriesByType('navigation')[0];
  if (!arrivingFromInternalLink && (event.persisted || navigation?.type === 'back_forward')) restoreRouteState();
});

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
