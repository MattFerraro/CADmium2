# hello-wasm

Just a test of how to write a rust function and compile it to wasm as an npm package that the cadmium-spa repo can use

# local test:

```
# wasm-pack build --target bundler
wasm-pack build --target web
python3 -m http.server
open localhost://
```