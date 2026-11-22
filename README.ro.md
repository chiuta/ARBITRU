# ARBITRU — Motor universal de reguli pentru jocuri de tablă, cărți, zaruri și puzzle, cu limbaj declarativ de reguli.

[English](README.md) · **Română** · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versiune: 0.1.0-alpha.1 · Ultima actualizare: 2026-11-22

🌐 https://arbitru.chiuta.ro

## Stare

Schelet timpuriu (pre-alfa): workspace-ul se compilează, iar testele trec în CI. Fiecare funcție planificată este o sarcină deschisă în `docs/issues/`.

## Domeniu planificat

- Un format declarativ de reguli `.arb` (v0, specificat în EBNF) și un parser pentru table de tip grilă, hexagonal și graf.
- Primitive de mișcare combinabile: săritor, alergător și hopper.
- Jocuri ca date: dame englezești, Connect Four, Gomoku și o variantă de șah (Los Alamos, Capablanca sau Racing Kings).
- Un banc de conformitate bazat pe fișiere perft și un jucător generic cu căutare Monte Carlo în arbore.
- Informație ascunsă și șansă (amestecări cu sămânță, zaruri, vederi parțiale) și o pagină WebAssembly de joacă.

Nimic din asta nu este implementat încă: fiecare punct este o sarcină deschisă în `docs/issues/`.

## Compilare și testare

```bash
cargo test --workspace
cargo run -p arbitru-cli -- perft 5
```

## Contribuie

Citește `CONTRIBUTING.md`, alege o sarcină etichetată `good first issue` din `docs/issues/` și deschide un pull request. Nicio contribuție nu e prea mică: un rând de tabel, un test sau o traducere contează.

## Implementare independentă

Scrisă pornind de la specificații publice și manuale; nu s-a copiat cod sursă de la terți (vezi `CLEANROOM.md`).

## Licență

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Sprijin

Trade-free: fără urmărire, fără reclame. Dacă ți-e de folos, poți sprijini autorul: https://buymeacoffee.com/echo.of.the.strings
