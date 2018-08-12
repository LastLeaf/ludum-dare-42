#!/bin/sh

SCRIPT=$(readlink -f "$0")
SCRIPTPATH=$(dirname "$SCRIPT")

export EMMAKEN_CFLAGS="-s WASM=0 -s SINGLE_FILE=1 -s EXTRA_EXPORTED_RUNTIME_METHODS=[\'UTF8ToString\',\'stringToUTF8\'] --js-library $SCRIPTPATH/../glayout/lib/bin/interfaces-release.js --js-library $SCRIPTPATH/../js_interfaces.js --pre-js $SCRIPTPATH/pre.js --post-js $SCRIPTPATH/post.js --llvm-lto 3 -O3 -Os --closure 0"

cargo build --target=asmjs-unknown-emscripten --release
