# Piaskownica
Symulacja cząstek w stylu falling-sand z propagacją temperatury. Rust + WebAssembly, Canvas 2D.

Każda komórka to cząsteczka z typem (piasek, woda, ogień, itd.) i temperaturą. Cząsteczki spadają, woda się rozlewa, ogień się rozprzestrzenia, lód topnieje. Temperatura propaguje się przez sąsiadów zgodnie z przewodnictwem cieplnym danego materiału.
Użytkownik rysuje materiały myszą.

Propagacja ciepła działa też przez powietrze. Powietrze powoli dąży do temperatury pokojowej, żeby ciepło się nie kumulowało bez ograniczeń.

## materiały
 
Planowane:
 
- `Empty` - powietrze, ma temperaturę
- `Sand` - spada, inert
- `Water` - spada i rozlewa się; paruje i zamarza
- `Stone` - nieruchome, niepalne
- `Wood` - nieruchome, zapala się w kontakcie z wysoką temperaturą
- `Fire` - cząsteczka o wysokiej temperaturze, unosi się, gaśnie przez chłodzenie
- `Smoke` - produkt ognia, unosi się
- `Ice` - nieruchome, topi się w wodę
- `Lava` - płynie jak woda, zamarza w kamień
- `Steam` - produkt parowania, unosi się, skrapla w wodę
Każdy materiał ma również `heat_capacity` (jak szybko zmienia temperaturę).

## stack
 
- Rust (core: symulacja, fizyka, rendering do bufora pikseli)
- `wasm-bindgen` + `wasm-pack` (kompilacja do WASM)
- Vanilla JavaScript (ładowanie WASM, game loop, obsługa myszy, rysowanie na canvas)
- HTML5 Canvas 2D

## architektura

Siatka to `Vec<Particle>` o rozmiarze `width * height`, indeks `y * width + x`. Bufor pikseli to osobny `Vec<u8>` (RGBA). JS czyta bufor bezpośrednio z pamięci WASM przez wskaźnik.

## budowanie
 
Wymagane: Rust, `wasm-pack`, jakikolwiek serwer HTTP (np. `python -m http.server`).
 
```
wasm-pack build --target web
python -m http.server 8080
```
