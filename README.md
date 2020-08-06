# WebAssembly and SSVM for the first time

Fork from the [ssvm-nodejs-starter](https://github.com/second-state/ssvm-nodejs-starter), add factorize function in Rust and Javascript, compare execution time of each implementation. By using SSVM tools (and Docker), it's very easy to write Rust functions and call them in Node.js.

The results are very interesting (see below). The first measurement should be ignored because WASM warming up. Thanks to V8 engine, the Javascript code are faster in 3 of the 5 cases, but its performance is unstable. On the contrary, the performance of WASM(Rust) code is more predictable.

The performance comparison is not rigorous, just for fun!

## Steps

1. Pull Docker image
1. Create Docker container and jump in
1. Install NPM package `factorization`
1. Build code
1. Run

```
$ docker pull secondstate/ssvm-nodejs-starter:v1
$ docker run -p 3000:3000 --rm -it -v $(pwd):/app secondstate/ssvm-nodejs-starter:v1
(docker) # cd /app
(docker) # npm i factorization
(docker) # ssvmup build
(docker) # node node/factorize.js
```

## Results

```
root@9edaa75511aa:/app# node node/factorize.js
factorize-wasm 65535 = 3 * 5 * 17 * 257, elapsed 894 µs in WASM
factorize-wasm: 33.143ms
factorize-js   65535 = 5,3,17,257
factorize-js  : 0.631ms

factorize-wasm 101 = 101, elapsed 41 µs in WASM.
factorize-wasm: 0.319ms
factorize-js   101 = 101
factorize-js  : 0.122ms

factorize-wasm 2147483647 = 2147483647, elapsed 322 µs in WASM.
factorize-wasm: 0.513ms
factorize-js   2147483647 = 2147483647
factorize-js  : 2.326ms

factorize-wasm 65536 = 2^16, elapsed 240 µs in WASM
factorize-wasm: 0.474ms
factorize-js   65536 = 2,2,2,2,2,2,2,2,2,2,2,2,2,2,2,2
factorize-js  : 0.117ms

factorize-wasm 20200806 = 2 * 3^3 * 374089, elapsed 603 µs in WASM
factorize-wasm: 0.774ms
factorize-js   20200806 = 2,3,3,3,374089
factorize-js  : 1.611ms

factorize-wasm 2020080522 = 2 * 3 * 17^2 * 131 * 8893, elapsed 4264 µs in WASM
factorize-wasm: 4.464ms
factorize-js   2020080522 = 2,3,17,17,131,8893
factorize-js  : 0.238ms
```
