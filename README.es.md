# ARBITRU — Motor de reglas universal para juegos de tablero, cartas, dados y puzles, con un lenguaje de reglas declarativo.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · **Español** · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versión: 0.1.0-alpha.1 · Última actualización: 2026-11-22

🌐 https://arbitru.chiuta.ro

## Estado

Esqueleto inicial (pre-alfa): el workspace compila y las pruebas pasan en CI. Cada función prevista es una tarea abierta en `docs/issues/`.

## Alcance previsto

- Un formato de reglas declarativo `.arb` (v0, especificado en EBNF) y un analizador para tableros de cuadrícula, hexagonales y de grafo.
- Primitivas de movimiento componibles: saltador, corredor y hopper.
- Juegos como datos: damas inglesas, Conecta 4, Gomoku y una variante de ajedrez (Los Álamos, Capablanca o Racing Kings).
- Un banco de conformidad basado en archivos perft y un jugador genérico con búsqueda en árbol Monte Carlo.
- Información oculta y azar (barajados con semilla, dados, vistas parciales) y una página de pruebas WebAssembly.

Nada de esto está implementado todavía: cada punto es una tarea abierta en `docs/issues/`.

## Compilar y probar

```bash
cargo test --workspace
cargo run -p arbitru-cli -- perft 5
```

## Contribuir

Lee `CONTRIBUTING.md`, elige una tarea con la etiqueta `good first issue` en `docs/issues/` y abre un pull request. Ninguna contribución es demasiado pequeña: una fila de tabla, una prueba o una traducción cuentan.

## Implementación independiente

Escrita a partir de especificaciones públicas y manuales; no se ha copiado código fuente de terceros (véase `CLEANROOM.md`).

## Licencia

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoyo

Trade-free: sin rastreo ni publicidad. Si te resulta útil, puedes apoyar al autor: https://buymeacoffee.com/echo.of.the.strings
