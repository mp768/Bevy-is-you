# Bevy is you

A clone of the puzzle game "Baba is you" made in the Bevy game engine for the Rust programming language :)

# Installation

## If you have cargo installed:
clone the repo with 
```
git clone git@github.com:mp768/Bevy-is-you.git
```
and just run
```
cargo run
```

## If you do not have cargo installed:
install cargo using the instructions [here](https://www.rust-lang.org/tools/install)

I'd also recommend reading the [rust book](https://doc.rust-lang.org/book/) if you're unfamiliar with rust :)

## How to build for web

install the wasm32 target and wasm-bindgen
```
rustup target add wasm32-unknown-unknown
cargo install -f wasm-bindgen-cli
```

build the project for wasm32
```
cargo build -r --target wasm32-unknown-unknown
```

Create an `out` directory to place the assets, and generated js and ts files.
Then use `wasm-bindgen` to translate the `.wasm` file to js.
```
wasm-bindgen --out-dir ./out --target web ./target/wasm32-unknown-unknown/release/bevy_is_you.wasm
```

Then create a `.html` with this content inside
```HTML
<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>bevy_is_you</title>
  </head>
  <body>
    <script type="module">
        import init from "./bevy_is_you.js";

        async function run() {
            await init();
        }

        run();
    </script>
  </body>
</html>
```

# Contributors
Special thanks to all these epic folk for helping me out with this project :D
  - [derpyzza](https://linktr.ee/derpyzza) ( Moral support )
