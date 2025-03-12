#!/bin/bash

set -o errexit
set -o nounset
set -o pipefail

PWD="$(dirname "$(readlink -f -- "$0")")"

CHANNEL=""
VERSION=""

function help() {
	local to
	to="$1"

	echo "Usage: $0 <flags>" 1>&"$to"
	echo 1>&"$to"
	echo "flags:" 1>&"$to"
	echo "	--version											release version." 1>&"$to"
	echo "	--dist												path to store artifacts in." 1>&"$to"
	echo "	--channel											the channel to use for the release (release | nightly)." 1>&"$to"
	echo "	--help												display this message." 1>&"$to"
}

function error() {
	echo "error: $*" 1>&2
	echo 1>&2
	help 2
	exit 1
}

function info() {
	echo "$@"
}

function os() {
	local os
	os="$(uname -s)"
	case "$os" in
	Darwin)
		echo "macos"
		;;
	Linux)
		echo "linux"
		;;
	Windows | MSYS* | MINGW*)
		echo "windows"
		;;
	*)
		error "$os: unsupported"
		;;
	esac
}

function arch() {
	local arch
	arch="$(uname -m)"
	case "$arch" in
	arm64 | aarch64)
		echo "aarch64"
		;;
	x86_64)
		echo "x86_64"
		;;
	*)
		error "$arch: unsupported architecture"
		;;
	esac
}

ARCH="$(arch)"
OS="$(os)"
DIST="release"


while [[ $# -gt 0 ]]; do
	case "$1" in
	--help)
		help 1
		exit 1
		;;
	--version)
		VERSION="$2"
		shift
		shift
		;;
	--dist)
		DIST="$2"
		shift
		shift
		;;
	--channel)
		CHANNEL="$2"
		shift
		shift
		;;
	*)
		error "unknown flag $1"
		;;
	esac
done

[ -z "${VERSION-}" ] && error "--version is not set"

[ -z "${TAURI_SIGNING_PRIVATE_KEY-}" ] && error "$TAURI_SIGNING_PRIVATE_KEY is not set"
[ -z "${TAURI_SIGNING_PRIVATE_KEY_PASSWORD-}" ] && error "$TAURI_SIGNING_PRIVATE_KEY_PASSWORD is not set"

if [ "$CHANNEL" != "release" ] && [ "$CHANNEL" != "nightly" ]; then
	error "--channel must be either 'release' or 'nightly'"
fi


info "building:"
info "	channel: $CHANNEL"
info "	version: $VERSION"
info "	os: $OS"
info "	arch: $ARCH"
info "	dist: $DIST"

TMP_DIR="$(mktemp -d)"
trap 'rm -rf "$TMP_DIR"' exit

CONFIG_PATH=$(readlink -f "$PWD/../backend/tauri.conf.json")

# update the version in the tauri release config
jq '.version="'"$VERSION"'"' "$CONFIG_PATH" >"$TMP_DIR/tauri.conf.json"

if [ "$OS" = "windows" ]; then
	FEATURES="windows"
else
	FEATURES=""
fi

# build the app with release config
cd "$PWD/.." && pnpm tauri build \
	--verbose \
	--features "$FEATURES" \
	--config "$TMP_DIR/tauri.conf.json"

BUNDLE_DIR=$(readlink -f "$PWD/../target/release/bundle")
RELEASE_DIR="$DIST/$OS/$ARCH"
mkdir -p "$RELEASE_DIR"

if [ "$OS" = "macos" ]; then
	MACOS_DMG="$(find "$BUNDLE_DIR/dmg" -depth 1 -type f -name "*.dmg")"
	MACOS_UPDATER="$(find "$BUNDLE_DIR/macos" -depth 1 -type f -name "*.tar.gz")"
	MACOS_UPDATER_SIG="$(find "$BUNDLE_DIR/macos" -depth 1 -type f -name "*.tar.gz.sig")"

	cp "$MACOS_DMG" "$RELEASE_DIR"
	cp "$MACOS_UPDATER" "$RELEASE_DIR"
	cp "$MACOS_UPDATER_SIG" "$RELEASE_DIR"

	info "built:"
	info "	- $RELEASE_DIR/$(basename "$MACOS_DMG")"
	info "	- $RELEASE_DIR/$(basename "$MACOS_UPDATER")"
	info "	- $RELEASE_DIR/$(basename "$MACOS_UPDATER_SIG")"
elif [ "$OS" = "windows" ]; then
	WINDOWS_INSTALLER="$(find "$BUNDLE_DIR/msi" -name \*.msi)"
	WINDOWS_UPDATER="$(find "$BUNDLE_DIR/msi" -name \*.msi.zip)"
	WINDOWS_UPDATER_SIG="$(find "$BUNDLE_DIR/msi" -name \*.msi.zip.sig)"

	cp "$WINDOWS_INSTALLER" "$RELEASE_DIR"
	cp "$WINDOWS_UPDATER" "$RELEASE_DIR"
	cp "$WINDOWS_UPDATER_SIG" "$RELEASE_DIR"

	info "built:"
	info "	- $RELEASE_DIR/$(basename "$WINDOWS_INSTALLER")"
	info "	- $RELEASE_DIR/$(basename "$WINDOWS_UPDATER")"
	info "	- $RELEASE_DIR/$(basename "$WINDOWS_UPDATER_SIG")"
else
	error "unsupported os: $OS"
fi

info "done! bye!"
