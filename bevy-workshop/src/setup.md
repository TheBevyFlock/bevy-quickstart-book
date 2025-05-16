# Setup

## Clone the repository

```sh
git clone https://github.com/vleue/bevy_workshop-rustweek-2025
```

## Environment setup

Option 1 is recommended if your local machine supports it. This workshop won't be GPU heavy so most hardware configurations should support running it.

### Option 1: Local Setup

- Install rust: [https://rustup.rs](https://rustup.rs)

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Linux

If you are using Linux, you can read up about how to install dependencies here:
[https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md)

#### Windows

If you are instead using Windows, you should be good to go.

---

To test out if it's all working, try building the sample project:

```sh
cargo build
```

If you are running into any issues, please check out the Discord and ask in the
beginners-questions channel. The community can probably help you out!

### Option 2: Docker Setup

This option can be interesting if you can't install dependencies on your machine, or the setup fails for some obscure reason. Instead of running natively, the workshop will run in your browser using wasm and WebGL2, delegating most OS/hardware integration to the browser.

#### Run a docker image from scratch

```sh
docker run -it -v `pwd`:/workspace -p 4000:4000 rust:1.82-bullseye /bin/bash
rustup target add wasm32-unknown-unknown
# install cargo binstall
curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
# install a few helpers
cargo binstall --no-confirm wasm-bindgen-cli cargo-watch basic-http-server

cd /workspace
# serve the wasm in the background
basic-http-server wasm 2> /dev/null &
# build for wasm
cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-dir wasm --out-name workshop --target web target/wasm32-unknown-unknown/release/bevy_workshop-rustweek-2025.wasm
```

#### Or use a prebuilt docker image

It will be a bigger initial download but the first build is already done

```sh
docker run -it -v `pwd`:/workspace -p 4000:4000 ghcr.io/vleue/bevy_workshop-rustweek-2025 /bin/bash

# Copy the prebuilt target folder
cp -r bevy_workshop-rustweek-2025/target /workspace/target

cd /workspace
# serve the wasm in the background
basic-http-server wasm 2> /dev/null &
# build for wasm
cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-dir wasm --out-name workshop --target web target/wasm32-unknown-unknown/release/bevy_workshop-rustweek-2025.wasm
```

### Option 3: Use GitHub Codespace

Go to <https://github.com/codespaces/new/vleue/bevy_workshop-rustweek-2025>, it will use a prebuilt image with everything needed to work in wasm. Increase the number of core as much as you're comfortable with. GitHub free tier of codespace is 120 core-hours per month, so with an 8-core machine you'll have 15 hours.

This option uses more bandwidth as you'll download the wasm file from the internet on every rebuild.
