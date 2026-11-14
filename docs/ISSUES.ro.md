# ARBITRU — primele zece sarcini

[English](ISSUES.md) · **Română** · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creează-le pe GitHub cu `scripts/create-issues.sh OWNER/REPO`. Comentează la un issue ca să-l revendici.

1. **Specifică formatul `.arb` v0 (EBNF) în `docs/ARB-FORMAT.md`** — `help wanted`, `docs`, `area:language`
   Antet (`game`, `board`, `players`), secțiunile `pieces`, `setup`, `moves`, `end`. Pornește de la ce acceptă `parse_header`; include 3 exemple rezolvate (X și 0, Connect Four, Gomoku).

2. **Parser pentru declarațiile de tablă: grilă, hex, graf** — `good first issue`, `area:language`
   `board: grid 8x8`, `board: hex 5`, `board: graph file`. Returnează un `Board` tipat; respinge nonsensul cu numere de linie.

3. **Primitive de mișcare: leaper, rider, hopper — componibile** — `help wanted`, `area:rules`
   Implementează-le ca funcții peste `Board` cu teste unitare pe table mici; fără cod specific unui joc.

4. **Jocul ca date: dame englezești în `games/draughts.arb`** — `help wanted`, `area:games`
   Include capturarea obligatorie și săriturile multiple. Conformitate: valori perft dintr-o sursă publicată (citează-o).

5. **Jocul ca date: Connect Four și Gomoku** — `good first issue`, `area:games`
   Două fișiere `.arb` plus teste perft/rezultat. Alege unul și spune-o în issue.

6. **Jocul ca date: o variantă de șah (Los Alamos, Capablanca sau Racing Kings)** — `help wanted`, `area:games`
   Cere definiții de piese, promovare și reguli de șah doar din fișierul de date. Citează sursa regulilor.

7. **Banc de conformitate: fișiere perft în `tests/perft/`** — `good first issue`, `tests`
   Un format TSV mic `position<TAB>depth<TAB>count` și un executor de teste care eșuează cu o diferență lizibilă.

8. **Jucător MCTS generic (simulări aleatorii) peste `Game`** — `help wanted`, `area:ai`
   RNG cu sămânță, fără crate-uri externe deocamdată; bate un jucător aleatoriu în ≥ 90% din partide la X și 0 și la Connect Four.

9. **Informație ascunsă și șansă: amestecări cu sămânță, zaruri, vederi parțiale** — `help wanted`, `area:rules`
   Extinde trăsătura cu grijă (determinizare). Întâi o notă de proiectare, codul după revizuire.

10. **Build WebAssembly și o pagină-teren de joacă pentru jocuri** — `help wanted`, `area:wasm`
   Build `wasm32-unknown-unknown`, pagină statică în `docs/demo/` ca să joci X și 0 împotriva MCTS.
