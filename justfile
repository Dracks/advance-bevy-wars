# https://just.systems
# Wasm build extracted from https://blog.erikhorton.com/2024/03/31/deploy-bevy-to-android-and-wasm.html

_build_wasm:
    cargo build --release --target wasm32-unknown-unknown

_prepare_webbuild:
    mkdir -p webbuild
    wasm-bindgen --out-dir ./webbuild/out/ --target web ./target/wasm32-unknown-unknown/release/advance_bevy_wars.wasm
    cp -r resources/index.html assets ./webbuild/

web_archive: _build_wasm _prepare_webbuild
    cd webbuild && zip -r "../abw-$(date +%Y-%m-%d).zip" .
    rm -rf webbuild

web_server: _build_wasm _prepare_webbuild
    cd webbuild && python3 -m http.server
    rm -rf webbuild

default:
    @echo "Build and run the bevy project"

short-cut:
    cargo run --features short-cut

debug:
    cargo run --features debug
