import init, { hello } from '../pkg/piaskownica.js';

async function run() {
    await init();  // ładuje i inicjalizuje moduł WASM
    hello();       // wywołuje naszą funkcję Rust. 
}

run();