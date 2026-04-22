import init, { Universe } from '../pkg/piaskownica.js';

async function run() {
    //załaduj WASM
    const wasm = await init();

    const width = 200;
    const height = 150;
    const scale = 4;  // każdy piksel symulacji = 4x4 piksele ekranu

    const universe = Universe.new(width, height);

    let currentMaterial = 1; 
    document.querySelectorAll('.material-btn').forEach(btn => {
        btn.addEventListener('click', () => {
            currentMaterial = parseInt(btn.dataset.material);
            document.querySelectorAll('.material-btn').forEach(b => b.classList.remove('active'));
            btn.classList.add('active');
        });
    });

    const canvas = document.getElementById('canvas');
    canvas.width = width * scale;
    canvas.height = height * scale;
    const ctx = canvas.getContext('2d');
    ctx.imageSmoothingEnabled = false;

    let mouseDown = false;
    let lastX = -1;
    let lastY = -1;
    let lastDrawX = -1;
    let lastDrawY = -1;

    canvas.addEventListener('mousedown', (event) => {
        mouseDown = true;
        lastX = Math.floor(event.offsetX / scale);
        lastY = Math.floor(event.offsetY / scale);
    });

    canvas.addEventListener('mouseup', () => {
        mouseDown = false;
        lastDrawX = -1;
        lastDrawY = -1;
    });

    canvas.addEventListener('mouseleave', () => {
        mouseDown = false;
        lastDrawX = -1;
        lastDrawY = -1;
    });

    canvas.addEventListener('mousemove', (event) => {

        // pozycja myszy w siatce (nie na canvasie)
        lastX = Math.floor(event.offsetX / scale);
        lastY = Math.floor(event.offsetY / scale);
    });

    function gameLoop() {
    if (mouseDown) {
        if (lastDrawX === -1) {
            universe.draw(lastX, lastY, currentMaterial);
        } else {
            universe.draw_line(lastDrawX, lastDrawY, lastX, lastY, currentMaterial);
        }
        lastDrawX = lastX;
        lastDrawY = lastY;
    }

    universe.tick();

    const pixelsPtr = universe.pixels_ptr();
    const pixels = new Uint8ClampedArray(
        wasm.memory.buffer,
        pixelsPtr,
        width * height * 4
    );
    const imageData = new ImageData(pixels, width, height);

    ctx.putImageData(imageData, 0, 0);
    ctx.drawImage(canvas, 0, 0, width, height, 0, 0, width * scale, height * scale);

    requestAnimationFrame(gameLoop);
}

    gameLoop(); 
}

run();