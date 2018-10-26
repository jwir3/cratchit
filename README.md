cratchit
===========
[![Build Status](https://travis-ci.org/jwir3/cratchit.svg?branch=master)](https://travis-ci.org/jwir3/cratchit) [![Coverage Status](https://coveralls.io/repos/github/jwir3/cratchit/badge.svg?branch=master)](https://coveralls.io/github/jwir3/cratchit?branch=master)

A Rust-based library for handling accounting data.

## Building
There are two methods of building. The first is building natively, which allows
you to run tests locally. To build natively, run:
```
cargo build
```

To build the wasm module, you need to first install the necessary prerequisites:
```
# We use nightly rust for a few features
rustup default nightly

# Install wasm-pack
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

# Install the latest npm (if you don't have npm installed, then you'll need
# to do this first. It comes with nodejs, so however you want to install nodejs
# on your system is probably fine)
npm install npm@latest -g
```

Once the prerequisites are installed, you can run:
```
wasm-pack build
```

## Testing
If you compiled the native code, you can run the unit tests with the command:
```
cargo test
```

To test the wasm module, we have a _very_ small test application in `www`. You
will need to set it up locally on your machine. To do this, run:
```
# First, make sure the wasm package is linked within npm
cd pkg
npm link

# Now, install npm dependencies and make sure we can see the cratchit wasm
# module
cd ../www
npm install
npm link cratchit
```

You should now be able to run:
```
npm start
```

within the `www` subdirectory, which will spawn a webserver at `localhost:8080`
that you can navigate to within a web browser to test the wasm module.
