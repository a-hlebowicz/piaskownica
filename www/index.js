import init, { Universe } from '../pkg/piaskownica.js';

async function run() {
    //załaduj WASM
    const wasm = await init();

    const width = 200;
    const height = 150;
    const scale = 4;  // każdy piksel symulacji = 4x4 piksele ekranu

    const universe = Universe.new(width, height);

    const canvas = document.getElementById('canvas');
    canvas.width = width * scale;
    canvas.height = height * scale;
    const ctx = canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;

    let mouseDown = false;

    canvas.addEventListener('mousedown', () => {
        mouseDown = true;
    });

    canvas.addEventListener('mouseup', () => {
        mouseDown = false;
    });

    canvas.addEventListener('mouseleave', () => {
        mouseDown = false;
    });

    canvas.addEventListener('mousemove', (event) => {
        if (!mouseDown) return;

        // pozycja myszy w siatce (nie na canvasie)
        const x = Math.floor(event.offsetX / scale);
        const y = Math.floor(event.offsetY / scale);

        universe.draw(x, y, 1);  // 1 = piasek
    });

    //rysowanie też przy jednym kliknięciu (bez ruchu)
    canvas.addEventListener('mousedown', (event) => {
        const x = Math.floor(event.offsetX / scale);
        const y = Math.floor(event.offsetY / scale);
        universe.draw(x, y, 1);
    });

    //game loop
    function gameLoop() {
        // symulacja + render pikseli
        universe.tick();

        // odczytaj bufor pikseli bezpośrednio z pamięci WASM
        const pixelsPtr = universe.pixels_ptr();
        const pixels = new Uint8ClampedArray(
            wasm.memory.buffer,
            pixelsPtr,
            width * height * 4
        );
        const imageData = new ImageData(pixels, width, height);

        // rysuj na canvasie
        ctx.putImageData(imageData, 0, 0);
        ctx.drawImage(
            canvas,                  
            0, 0, width, height,       
            0, 0, width * scale, height * scale  
        );
        requestAnimationFrame(gameLoop);
    }

    gameLoop(); 
}

run();