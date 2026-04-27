# Piaskownica
 
Symulacja cząstek w stylu falling-sand z propagacją temperatury. Rust + WebAssembly, Canvas 2D.
 
Każda komórka to cząsteczka z typem (piasek, woda, ogień, itd.) i temperaturą. Cząsteczki spadają, woda się rozlewa, ogień się rozprzestrzenia, lód topnieje. Temperatura propaguje się przez sąsiadów zgodnie z przewodnictwem cieplnym danego materiału. Użytkownik rysuje materiały myszą.
 
## materiały
 
- `Empty` - powietrze, izoluje termicznie (nie wymienia ciepła)
- `Piasek` - spada, tonie w wodzie i lawie
- `Woda` - spada i rozlewa się; paruje powyżej 110°C, zamarza poniżej -10°C
- `Kamień` - nieruchomy, niepalny
- `Drewno` - nieruchome, zapala się powyżej 50°C
- `Ogień` - unosi się chaotycznie, stygnie pasywnie, gaśnie poniżej 300°C
- `Lód` - nieruchomy, topnieje powyżej 10°C
- `Lawa` - płynie jak woda, zamarza w kamień poniżej 800°C
- `Para` - unosi się, skrapla w wodę poniżej 90°C
- `Metal` - nieruchomy, świetnie przewodzi ciepło; świeci na czerwono gdy gorący
Każdy materiał ma temperaturę (`i16`, °C) i `conductivity` (jak chętnie wymienia ciepło).
 
## fizyka temperatury
 
Powietrze traktowane jest jako próżnia termiczna. Ciepło przenosi się tylko między dotykającymi się materiałami. Lawa wylana w powietrzu nie ostygnie sama; trzeba ją zalać wodą albo zasypać piaskiem.
 
Propagacja przez średnią ważoną z 8 sąsiadów (4 proste + 4 ukosami z osłabionym wpływem). Wymiana proporcjonalna do różnicy temperatur i iloczynu conductivity obu komórek.
 
Transformacje (woda↔lód, woda↔para, lawa→kamień, drewno→ogień) są osobnym krokiem, wykonywanym po propagacji. Para wodna ma histerezę (110/90°C) żeby uniknąć oscylacji.
 
Ogień ma wbudowane pasywne chłodzenie - traci 1-5°C/tick niezależnie od sąsiadów. Bez paliwa gaśnie, z paliwem propaguje się łańcuchowo.
 
## stack
 
- Rust (core: symulacja, fizyka, rendering do bufora pikseli)
- `wasm-bindgen` + `wasm-pack` (kompilacja do WASM)
- Vanilla JavaScript (ładowanie WASM, game loop, obsługa myszy, rysowanie na canvas)
- HTML5 Canvas 2D
## architektura
 
Siatka to `Vec<Particle>` o rozmiarze `width * height`, indeks `y * width + x`. Bufor pikseli to osobny `Vec<u8>` (RGBA). JS czyta bufor bezpośrednio z pamięci WASM przez wskaźnik.
 
Każdy tick wykonuje trzy fazy:
1. **Ruch cząsteczek** - piasek spada, woda płynie, gazy unoszą się
2. **Propagacja ciepła** - double-buffered, czytamy stary stan, zapisujemy do drugiego bufora, na końcu kopiujemy
3. **Transformacje termiczne** - cząsteczki przekraczające progi zmieniają typ
## budowanie
 
Wymagane: Rust, `wasm-pack`, jakikolwiek serwer HTTP (np. `python -m http.server`).
 
```
wasm-pack build --target web
python -m http.server 8080
```
