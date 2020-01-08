# Nimiq WebAssembly Proof of Concept

You'll need to install [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) and [npm](https://www.npmjs.com/).

Then cd into the repository and run

```bash
wasm-pack build
```

This will compile the example to WebAssembly into the `pkg` directory. Now install and run it:

```bash
cd www
npm install
npm run start
```

