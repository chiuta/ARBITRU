# ARBITRU — Moteur de règles universel pour jeux de plateau, de cartes, de dés et puzzles, avec un langage de règles déclaratif.

[English](README.md) · [Română](README.ro.md) · **Français** · [Italiano](README.it.md) · [Español](README.es.md) · [Português](README.pt.md) · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Version : 0.1.0-alpha.1 · Dernière mise à jour : 2026-11-22

🌐 https://arbitru.chiuta.ro

## État

Squelette précoce (pré-alpha) : le workspace compile et les tests passent en CI. Chaque fonctionnalité prévue est une tâche ouverte dans `docs/issues/`.

## Périmètre prévu

- Un format de règles déclaratif `.arb` (v0, spécifié en EBNF) et un analyseur pour plateaux en grille, hexagonaux et en graphe.
- Primitives de déplacement composables : sauteur, coureur et hopper.
- Jeux comme données : dames anglaises, Puissance 4, Gomoku et une variante d’échecs (Los Alamos, Capablanca ou Racing Kings).
- Un banc de conformité fondé sur des fichiers perft et un joueur générique par recherche arborescente Monte Carlo.
- Information cachée et hasard (mélanges à graine, dés, vues partielles) et une page de démonstration WebAssembly.

Rien de tout cela n’est encore implémenté : chaque point est une tâche ouverte dans `docs/issues/`.

## Compiler et tester

```bash
cargo test --workspace
cargo run -p arbitru-cli -- perft 5
```

## Contribuer

Lisez `CONTRIBUTING.md`, choisissez une tâche étiquetée `good first issue` dans `docs/issues/` et ouvrez une pull request. Aucune contribution n'est trop petite : une ligne de table, un test ou une traduction comptent.

## Implémentation indépendante

Écrite à partir de spécifications publiques et de manuels ; aucun code source tiers n'a été copié (voir `CLEANROOM.md`).

## Licence

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Soutien

Trade-free : ni pistage ni publicité. Si cela vous est utile, vous pouvez soutenir l'auteur : https://buymeacoffee.com/echo.of.the.strings
