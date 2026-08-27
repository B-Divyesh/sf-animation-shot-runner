const CACHE = 'shot-runner-v3';
const SHELL = [
  '/',
  '/index.html',
  '/privacy/',
  '/terms/',
  '/favicon.svg',
  '/shot-proof-e5786fb0.webp',
  '/shot-proof-768-97c150b8.webp',
  '/fonts/instrument-serif-5eb09b5a.woff2',
  '/fonts/ibm-plex-mono-08949f72.woff2',
];
const SHELL_PATHS = new Set(SHELL);
const SENSITIVE_URL_PART = /license|token|entitlement/i;

function isSensitiveUrl(url) {
  return url.pathname.includes('/verify') || [...url.searchParams.keys()].some(key => SENSITIVE_URL_PART.test(key));
}

function isCacheableStaticRequest(request) {
  if (request.method !== 'GET') return false;
  const url = new URL(request.url);
  // Sociobot verification is cross-origin today. Keep this guard even if the
  // endpoint changes: only public, same-origin documentation assets may enter
  // Cache Storage.
  if (url.origin !== self.location.origin || isSensitiveUrl(url)) return false;
  if (request.mode === 'navigate') return !url.search && SHELL_PATHS.has(url.pathname);
  return !url.search && (
    SHELL_PATHS.has(url.pathname)
    || url.pathname.startsWith('/assets/')
  );
}

self.addEventListener('install', event => event.waitUntil(
  caches.open(CACHE).then(cache => cache.addAll(SHELL)).then(() => self.skipWaiting()),
));

self.addEventListener('activate', event => event.waitUntil(
  caches.keys()
    .then(keys => Promise.all(keys.filter(key => key !== CACHE).map(key => caches.delete(key))))
    .then(() => self.clients.claim()),
));

self.addEventListener('fetch', event => {
  if (!isCacheableStaticRequest(event.request)) return;
  event.respondWith(caches.match(event.request).then(cached => {
    if (cached) return cached;
    return fetch(event.request).then(response => {
      if (!response.ok || response.type !== 'basic') return response;
      const copy = response.clone();
      event.waitUntil(caches.open(CACHE).then(cache => cache.put(event.request, copy)));
      return response;
    });
  }));
});
