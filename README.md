## build

```bash
wasm-pack build -t nodejs --release
# or
npm run build
```



## how to use

copy the dist dir into your js project and load:

```js
const wasm = require('./you dir name/index');
babelPlugin.push(wasm.init);
```

