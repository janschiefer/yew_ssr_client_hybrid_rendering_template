# Yew hybrid rendering

**Server side rendering** is necessary when *dealing with search engines ( SEO: Search engine optimization )*.

**Client side rendering** is great for *modern reactive web apps* running on consumer devices.

**Hybrid rendering** combines the best of two worlds and is an absolute killer feature of modern web app frameworks such as [Nuxt](https://nuxtjs.org/).

**Let's have the BEST of both worlds both with Rust, Actix and Yew!**

## Description

A simple template for an web application, utilizing Yew for client side rendering with Webassembly ( WASM ), as well as server side rendering ( SSR ).

The webpage rendered by Yew on the server is served with Actix.
Actix is an awesome web backend written in Rust and according to benchmarks propably the fastest web backend in existence.

## Getting Started

Run ./build-debug.sh for building in debug mode.

Run ./build-production.sh for building in release mode.

### Installing

1. Install the Rust programming language: https://www.rust-lang.org/tools/install

2. Install your toolchain ( I'am using nightly ):
```
rustup toolchain install nightly
```

3. Add the target architecture of your web server ( Linux in this example ):

```
rustup target add x86_64-unknown-linux-gnu 
```

3. Add WASM target:

```
rustup target add wasm32-unknown-unknown
```

4. Install trunk and wasm-bindgen-cli for bundling your WASM code:
```
cargo install trunk
cargo install wasm-bindgen-cli
```

5. Compile and run the application:
```
(go to the code base directory )
./build-debug.sh
```

6. Navigate to http://localhost:8080 to view application

## Authors

- Dr. med. Jan Schiefer [janschiefer](https://github.com/janschiefer)

## Version History

* 0.0.1
    * Initial Release

## License

This project is licensed under the MIT License - see the LICENSE.md file for details
