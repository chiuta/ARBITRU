# ARBITRU — Univerzális szabálymotor társas-, kártya-, kocka- és logikai játékokhoz, deklaratív szabálynyelvvel.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · **Magyar**

Verzió: 0.1.0-alpha.1 · Utolsó frissítés: 2026-11-14

🌐 https://arbitru.chiuta.ro

## Állapot

Korai váz (pre-alfa): a workspace lefordul, a tesztek a CI-ben lefutnak. Minden tervezett funkció nyitott feladat a `docs/issues/` mappában.

## Tervezett terjedelem

- Deklaratív `.arb` szabályformátum (v0, EBNF-ben leírva) és értelmező rács-, hexa- és gráftáblákhoz.
- Összeállítható lépésprimitívek: ugró, futó és hopper.
- Játékok adatként: angol dáma, Négy egy sorban, Gomoku és egy sakkváltozat (Los Alamos, Capablanca vagy Racing Kings).
- Perft-fájlokra épülő megfelelőségi vizsgálat és általános Monte Carlo fakeresésű játékos.
- Rejtett információ és véletlen (magolt keverés, kocka, részleges nézet) és egy WebAssembly kipróbálóoldal.

Ezek közül még semmi sincs megvalósítva: minden pont nyitott feladat a `docs/issues/` mappában.

## Fordítás és tesztelés

```bash
cargo test --workspace
cargo run -p arbitru-cli -- perft 5
```

## Közreműködés

Olvasd el a `CONTRIBUTING.md` fájlt, válassz egy `good first issue` címkéjű feladatot a `docs/issues/` mappából, és nyiss pull requestet. Nincs túl kicsi hozzájárulás: egy táblázatsor, egy teszt vagy egy fordítás is számít.

## Független megvalósítás

Nyilvános specifikációk és tankönyvek alapján íródott; harmadik fél forráskódját nem másoltuk (lásd `CLEANROOM.md`).

## Licenc

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Támogatás

Trade-free: nincs követés, nincs reklám. Ha hasznos számodra, támogathatod a szerzőt: https://buymeacoffee.com/echo.of.the.strings
