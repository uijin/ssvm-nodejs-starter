const { factorize } = require("../pkg/ssvm_nodejs_starter_lib.js")

const number = [65535, 101, 2147483647, 65536, 20200806, 2020080522]
for (const n of number) {
    console.time("factorize-wasm")
    console.log("factorize-wasm " + n + " = " + factorize(n))
    console.timeEnd("factorize-wasm")

    console.time("factorize-js  ")
    const Factorization = require("factorization")
    console.log("factorize-js   " + n + " = " + Factorization(n))
    console.timeEnd("factorize-js  ")
    console.log("")
}