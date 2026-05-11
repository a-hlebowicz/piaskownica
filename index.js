import init, { Universe } from './pkg/piaskownica.js';

async function run() {
    //załaduj WASM
    const wasm = await init();

    const width = 200;
    const height = 150;
    const scale = 4;  // każdy piksel symulacji = 4x4 piksele ekranu

    //let lastTime = performance.now();
    let frames = 0;
    let fps = 0;

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
        const x = Math.floor(event.offsetX / scale);
        const y = Math.floor(event.offsetY / scale);
        const temp = universe.debug_at(x, y);
        document.getElementById('debug').innerText = `(${x}, ${y}): ${temp}°C`;
    });

    const saveBtn = document.getElementById('save-btn');
    const loadBtn = document.getElementById('load-btn');
    const loadInput = document.getElementById('load-input');
    const snapshotMessage = document.getElementById('snapshot-message');

    function showMessage(text, type) {
        snapshotMessage.textContent = text;
        snapshotMessage.className = 'hint ' + type;
        setTimeout(() => {
            snapshotMessage.textContent = '';
            snapshotMessage.className = 'hint';
        }, 3000);
    }

    saveBtn.addEventListener('click', () => {
        const json = universe.export();
        const blob = new Blob([json], { type: 'application/json' });
        const url = URL.createObjectURL(blob);
        
        const a = document.createElement('a');
        a.href = url;
        
        const now = new Date();
        const timestamp = now.toISOString().slice(0, 19).replace(/:/g, '-');
        a.download = `piaskownica-${timestamp}.json`;
        
        a.click();
        URL.revokeObjectURL(url);
        
        showMessage('Zapisano!', 'success');
    });

    loadBtn.addEventListener('click', () => {
        loadInput.click();
    });

    loadInput.addEventListener('change', (event) => {
        const file = event.target.files[0];
        if (!file) return;
        
        const reader = new FileReader();
        reader.onload = (e) => {
            const json = e.target.result;
            try {
                universe.import(json);
                showMessage('Wczytano!', 'success');
            } catch (err) {
                showMessage('Błąd: ' + err, 'error');
            }
        };
        reader.readAsText(file);
        
        loadInput.value = '';
        universe.render();
    });

    let paused = false;
    const pauseBtn = document.getElementById('pause-btn');
    pauseBtn.addEventListener('click', () => {
        paused = !paused;
        pauseBtn.textContent = paused ? '▶ Wznów' : '⏸ Pauza';
    });

    const clearBtn = document.getElementById('clear-btn');
    clearBtn.addEventListener('click', () => {
        universe.clear();
        showMessage('Wyczyszczono', 'success');
    });

    const PHYSICS_HZ = 140;
    const PHYSICS_DT_MS = 1000 / PHYSICS_HZ;  // 16.67ms
    let accumulator = 0;
    let lastTime = performance.now();
    let tickTimes = [];
    let renderTimes = [];

    function gameLoop() {
        const now = performance.now();
        let elapsed = now - lastTime;
        lastTime = now;

        // zabezpieczenie przed "spiral of death":
        // jeśli karta przegrzała się i lecimy 5 sekund, nie próbuj nadrobić 300 tików
        if (elapsed > 250) elapsed = 250;

        accumulator += elapsed;

        // 1. Input
        if (mouseDown) {
            if (lastDrawX === -1) {
                universe.draw(lastX, lastY, currentMaterial);
            } else {
                universe.draw_line(lastDrawX, lastDrawY, lastX, lastY, currentMaterial);
            }
            lastDrawX = lastX;
            lastDrawY = lastY;
        }

        // 2. Fizyka: tyle tików ile trzeba żeby nadrobić, ale stałe DT
        if (!paused) {
            while (accumulator >= PHYSICS_DT_MS) {
                const t1 = performance.now();
                universe.tick();
                const t2 = performance.now();
                tickTimes.push(t2 - t1);
                accumulator -= PHYSICS_DT_MS;
            }
        } else {
            accumulator = 0;  // reset gdy pauza, żeby po wyjściu nie nadrabiać
        }

        // 3. Render (zawsze raz na klatkę monitora)
        const r1 = performance.now();
        universe.render();
        const pixelsPtr = universe.pixels_ptr();
        const pixels = new Uint8ClampedArray(wasm.memory.buffer, pixelsPtr, width * height * 4);
        const imageData = new ImageData(pixels, width, height);
        ctx.putImageData(imageData, 0, 0);
        ctx.drawImage(canvas, 0, 0, width, height, 0, 0, width * scale, height * scale);
        const r2 = performance.now();
        renderTimes.push(r2 - r1);

        // 4. Pomiar
        if (tickTimes.length >= 60) {
            const avgTick = tickTimes.reduce((a, b) => a + b, 0) / tickTimes.length;
            const avgRender = renderTimes.reduce((a, b) => a + b, 0) / renderTimes.length;
            document.getElementById('fps').textContent =
                `tick: ${avgTick.toFixed(2)}ms | render: ${avgRender.toFixed(2)}ms`;
            tickTimes = [];
            renderTimes = [];
        }

        requestAnimationFrame(gameLoop);
    }

    gameLoop();
}

   

run();