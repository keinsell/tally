#!/usr/bin/env bash
# Stage installable data files for the cargo-based packagers
# (cargo-deb, cargo-generate-rpm) into target/package-data/.
#
# Renders the @app-id@ templates in data/*.in so `cargo deb` /
# `cargo generate-rpm` can ship the desktop entry, metainfo and icons.
#
# Usage: ./build-aux/stage-package-assets.sh [APP_ID]
set -euo pipefail

cd "$(dirname "$0")/.."

APP_ID="${1:-${APP_ID:-com.keinsell.Tally}}"
OUT="target/package-data"

mkdir -p \
    "$OUT/applications" \
    "$OUT/metainfo" \
    "$OUT/icons/hicolor/scalable/apps" \
    "$OUT/icons/hicolor/symbolic/apps"

sed "s/@app-id@/$APP_ID/g" \
    data/com.keinsell.Tally.desktop.in \
    > "$OUT/applications/$APP_ID.desktop"

sed "s/@app-id@/$APP_ID/g" \
    data/com.keinsell.Tally.metainfo.xml.in \
    > "$OUT/metainfo/$APP_ID.metainfo.xml"

cp data/icons/hicolor/scalable/apps/com.keinsell.Tally.svg \
    "$OUT/icons/hicolor/scalable/apps/$APP_ID.svg"
cp data/icons/hicolor/symbolic/apps/com.keinsell.Tally-symbolic.svg \
    "$OUT/icons/hicolor/symbolic/apps/$APP_ID-symbolic.svg"

if command -v desktop-file-validate >/dev/null; then
    desktop-file-validate "$OUT/applications/$APP_ID.desktop"
fi
if command -v appstreamcli >/dev/null; then
    appstreamcli validate --no-net "$OUT/metainfo/$APP_ID.metainfo.xml"
fi

echo "staged package assets in $OUT"
