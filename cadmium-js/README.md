# To compile

```
wasm-pack build --target web
cd pkg
npm link
```

Over in cadmium-spa, after any npm changes:
```
npm link cadmium-js
```