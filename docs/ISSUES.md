# ARBITRU — first ten tasks

**English** · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Create them on GitHub with `scripts/create-issues.sh OWNER/REPO`. Comment on an issue to claim it.

1. **Specify the `.arb` v0 format (EBNF) in `docs/ARB-FORMAT.md`** — `help wanted`, `docs`, `area:language`
   Header (`game`, `board`, `players`), `pieces`, `setup`, `moves`, `end` sections. Start from what `parse_header` accepts; include 3 worked examples (tic-tac-toe, Connect Four, Gomoku).

2. **Parser for board declarations: grid, hex, graph** — `good first issue`, `area:language`
   `board: grid 8x8`, `board: hex 5`, `board: graph file`. Return a typed `Board`; reject nonsense with line numbers.

3. **Movement primitives: leaper, rider, hopper — composable** — `help wanted`, `area:rules`
   Implement them as functions over `Board` with unit tests on small boards; no game-specific code.

4. **Game as data: English draughts in `games/draughts.arb`** — `help wanted`, `area:games`
   Includes mandatory capture and multi-jump. Conformance: perft values from a published source (cite it).

5. **Game as data: Connect Four and Gomoku** — `good first issue`, `area:games`
   Two `.arb` files plus perft/outcome tests. Pick one, say so in the issue.

6. **Game as data: a chess variant (Los Alamos, Capablanca or Racing Kings)** — `help wanted`, `area:games`
   Needs piece definitions, promotion and check rules from the data file only. Cite the rules source.

7. **Conformance harness: perft files under `tests/perft/`** — `good first issue`, `tests`
   A tiny TSV format `position<TAB>depth<TAB>count` and a test runner that fails with a readable diff.

8. **Generic MCTS player (random playouts) over `Game`** — `help wanted`, `area:ai`
   Seeded RNG without external crates for now; beat a random player ≥ 90% on tic-tac-toe and Connect Four.

9. **Hidden information and chance: seeded shuffles, dice, partial views** — `help wanted`, `area:rules`
   Extend the trait carefully (determinisation). Design note first, code after review.

10. **WebAssembly build and a playground page for the games** — `help wanted`, `area:wasm`
   `wasm32-unknown-unknown` build, static page under `docs/demo/` to play tic-tac-toe against MCTS.
