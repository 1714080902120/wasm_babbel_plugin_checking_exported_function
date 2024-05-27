## build

```bash
wasm-pack build -t nodejs --release
# or
npm run build
```



## how to use

```bash
npm install babel-plugin-check-exported-function-comments -D
```

```js
const wasm = require('./you dir name/index');
babelPlugin.push(wasm.init);
```

