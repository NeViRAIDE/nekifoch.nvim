#!/usr/bin/env bash

set -e

# get current version from Cargo.toml
get_version() {
	cat Cargo.toml | grep '^version =' | sed -E 's/.*"([^"]+)".*/\1/'
}

# compile from source
build() {
	echo "Building nekifoch.nvim from source..."

	cargo build --release --target-dir ./target

	# Place the compiled library where Neovim can find it.
	mkdir -p lua

	if [ "$(uname)" == "Darwin" ]; then
		mv target/release/libnekifoch.dylib lua/nekifoch.so
	elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
		mv target/release/libnekifoch.so lua/nekifoch.so
	elif [ "$(expr substr $(uname -s) 1 10)" == "MINGW64_NT" ]; then
		mv target/release/nekifoch.dll lua/nekifoch.dll
	fi
}

# download the nekifoch.nvim (of the specified version) from Releases
download() {
	echo "Downloading nekifoch.nvim library: " $1
	if [ "$(uname)" == "Darwin" ]; then
		arch_name="$(uname -m)"
		curl -fsSL https://github.com/NeViRAIDE/nekifoch.nvim/releases/download/$1/nekifoch-mac-${arch_name}.tar.gz | tar -xz
	elif [ "$(expr substr $(uname -s) 1 5)" == "Linux" ]; then
		curl -fsSL https://github.com/NeViRAIDE/nekifoch.nvim/releases/download/$1/nekifoch-linux.tar.gz | tar -xz
	elif [ "$(expr substr $(uname -s) 1 10)" == "MINGW64_NT" ]; then
		# curl -fsSL https://github.com/NeViRAIDE/nekifoch.nvim/releases/download/$1/nekifoch-win.zip --output nekifoch-win.zip
		# unzip nekifoch-win.zip
		echo "Windows build is not available yet."

		build
	fi
}

case "$1" in
build)
	build
	;;
*)
	version="v$(get_version)"
	download $version

	;;
esac
