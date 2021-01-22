#!/usr/bin/env node
'use strict';

const fs = require("fs");
const yargs = require("yargs");

const options = yargs
    .option("p", { alias: "path", describe: "prefix applied JavaScript module imports", default: "./", type: "string", demandOption: false })
    .command("* [options] <infile> <outfile>", "create WASI compatible wasm-bindgen javascript", () => {}, (argv) => {
        // console.log(JSON.stringify(argv));

        fs.readFile(argv.infile, 'utf8', function (err,data) {
            if (err) {
                return console.log(err);
            }

            // Replace wasm import with a setter function
            let initWasi = "let wasm;\nexport function setBindingsWasm(w){ wasm = w; }";
            data = data.replace(/import \* as wasm from \'.\/.*\.wasm\'\;/, initWasi);

            // Change value of module_name to path
            data = data.replace(/(import .* from ')(.*)(')/g, '$1' + argv.path + '$2.js$3');

            fs.writeFile(argv.outfile, data, 'utf8', function (err) {
                if (err) return console.log(err);
            });
        });    
    })
    .help()
    .argv;

