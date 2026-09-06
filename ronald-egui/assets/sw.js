// The cache name is rewritten at build time (see scripts/version-sw.sh) with a
// hash of the built assets. A new build therefore produces a byte-different
// sw.js, which is what makes the browser install this worker again, and a new
// cache name, which is what makes it refetch the app instead of reusing the
// previous release.
var cacheVersion = '__CACHE_VERSION__';
var cacheName = 'ronald-emulator-pwa-' + cacheVersion;
var filesToCache = [
  './',
  './index.html',
  './ronald-egui.js',
  './ronald-egui_bg.wasm',
];

/* Start the service worker and cache all of the app's content */
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches
      .open(cacheName)
      .then(function (cache) {
        return cache.addAll(filesToCache);
      })
      .then(function () {
        // Don't wait for existing tabs to close before taking over.
        return self.skipWaiting();
      })
  );
});

/* Drop the caches of previous releases and take over open tabs */
self.addEventListener('activate', function (e) {
  e.waitUntil(
    caches
      .keys()
      .then(function (keys) {
        return Promise.all(
          keys.map(function (key) {
            if (key !== cacheName) {
              return caches.delete(key);
            }
          })
        );
      })
      .then(function () {
        return self.clients.claim();
      })
  );
});

/* Serve the current release from the cache, falling back to the network */
self.addEventListener('fetch', function (e) {
  if (e.request.method !== 'GET') {
    return;
  }

  var url = new URL(e.request.url);
  if (url.origin !== self.location.origin) {
    return;
  }

  e.respondWith(
    caches.match(e.request).then(function (response) {
      if (response) {
        return response;
      }

      return fetch(e.request).then(function (networkResponse) {
        // Cache assets fetched at runtime (ROMs, key maps, icons) so the app
        // keeps working offline.
        if (networkResponse.ok && networkResponse.type === 'basic') {
          var copy = networkResponse.clone();
          caches.open(cacheName).then(function (cache) {
            cache.put(e.request, copy);
          });
        }

        return networkResponse;
      });
    })
  );
});
