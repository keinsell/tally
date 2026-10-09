#!/usr/bin/env bash
# Build the Flatpak and export a single-file bundle for distribution.
#
# This is the single place where the flatpak-builder invocation lives.
# Usage:
#   ./build-aux/flatpak.sh            # builds ./tally-<arch>.flatpak
#   BUNDLE=my-app.flatpak ./build-aux/flatpak.sh
#   APP_ID=com.keinsell.Tally ./build-aux/flatpak.sh
set -euo pipefail

cd "$(dirname "$0")/.."

APP_ID="${APP_ID:-com.keinsell.Tally}"
MANIFEST="${MANIFEST:-build-aux/$APP_ID.json}"
BUILD_DIR="${BUILD_DIR:-_flatpak}"
REPO_DIR="${REPO_DIR:-repo}"
BRANCH="${BRANCH:-stable}"
ARCH="${ARCH:-$(flatpak --default-arch 2>/dev/null || echo x86_64)}"
BUNDLE="${BUNDLE:-tally-$ARCH.flatpak}"

flatpak-builder --force-clean --user --ccache --repo="$REPO_DIR" \
	--default-branch="$BRANCH" \
	--install-deps-from=flathub \
	"$BUILD_DIR" "$MANIFEST"

flatpak build-bundle --arch="$ARCH" \
	--runtime-repo=https://flathub.org/repo/flathub.flatpakrepo \
	"$REPO_DIR" "$BUNDLE" "$APP_ID" "$BRANCH"

echo "bundle ready: $BUNDLE"
ls -lh "$BUNDLE"
