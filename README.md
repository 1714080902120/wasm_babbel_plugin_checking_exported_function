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
const wasm = require('babel-plugin-check-exported-function-comments/index');
babelPlugin.push(wasm.init);
```

