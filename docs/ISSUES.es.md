# ARBITRU — primeras diez tareas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · **Español** · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créalas en GitHub con `scripts/create-issues.sh OWNER/REPO`. Comenta en una issue para reclamarla.

1. **Especifica el formato `.arb` v0 (EBNF) en `docs/ARB-FORMAT.md`** — `help wanted`, `docs`, `area:language`
   Cabecera (`game`, `board`, `players`), secciones `pieces`, `setup`, `moves`, `end`. Parte de lo que acepta `parse_header`; incluye 3 ejemplos resueltos (tres en raya, Conecta 4, Gomoku).

2. **Parser para declaraciones de tablero: cuadrícula, hex, grafo** — `good first issue`, `area:language`
   `board: grid 8x8`, `board: hex 5`, `board: graph file`. Devuelve un `Board` tipado; rechaza los disparates con números de línea.

3. **Primitivas de movimiento: leaper, rider, hopper — componibles** — `help wanted`, `area:rules`
   Impleméntalas como funciones sobre `Board` con pruebas unitarias en tableros pequeños; sin código específico de ningún juego.

4. **Juego como datos: damas inglesas en `games/draughts.arb`** — `help wanted`, `area:games`
   Incluye captura obligatoria y saltos múltiples. Conformidad: valores perft de una fuente publicada (cítala).

5. **Juego como datos: Conecta 4 y Gomoku** — `good first issue`, `area:games`
   Dos archivos `.arb` más pruebas perft/resultado. Elige uno y dilo en la issue.

6. **Juego como datos: una variante de ajedrez (Los Álamos, Capablanca o Racing Kings)** — `help wanted`, `area:games`
   Necesita definiciones de piezas, promoción y reglas de jaque solo desde el archivo de datos. Cita la fuente de las reglas.

7. **Banco de conformidad: archivos perft en `tests/perft/`** — `good first issue`, `tests`
   Un pequeño formato TSV `position<TAB>depth<TAB>count` y un ejecutor de pruebas que falla con una diferencia legible.

8. **Jugador MCTS genérico (partidas aleatorias) sobre `Game`** — `help wanted`, `area:ai`
   RNG con semilla sin crates externos por ahora; gana a un jugador aleatorio en ≥ 90 % de las partidas de tres en raya y Conecta 4.

9. **Información oculta y azar: barajados con semilla, dados, vistas parciales** — `help wanted`, `area:rules`
   Amplía el trait con cuidado (determinización). Primero una nota de diseño, el código tras la revisión.

10. **Compilación WebAssembly y una página de pruebas para los juegos** — `help wanted`, `area:wasm`
   Compilación `wasm32-unknown-unknown`, página estática en `docs/demo/` para jugar al tres en raya contra MCTS.
