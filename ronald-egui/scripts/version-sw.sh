#!/bin/sh
# Trunk post_build hook: stamp sw.js with a hash of the built app.
#
# Trunk is configured with `filehash = false`, so the JS and WASM bundles keep
# the same URL across releases. Cache busting therefore has to come from the
# service worker: hashing the bundles into the cache name makes sw.js differ
# whenever the app differs, which is the signal the browser uses to install the
# new worker and drop the previous release's cache.
set -eu

staging="${TRUNK_STAGING_DIR:?TRUNK_STAGING_DIR is not set}"
sw="${staging}/sw.js"

version=$(cat "${staging}/index.html" "${staging}/ronald-egui.js" \
  "${staging}/ronald-egui_bg.wasm" | sha256sum | cut -c1-16)

sed -i "s/__CACHE_VERSION__/${version}/" "${sw}"

echo "stamped sw.js with cache version ${version}"
