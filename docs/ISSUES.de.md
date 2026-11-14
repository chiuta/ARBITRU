# ARBITRU — die ersten zehn Aufgaben

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · **Deutsch** · [Magyar](ISSUES.hu.md)

Lege sie auf GitHub mit `scripts/create-issues.sh OWNER/REPO` an. Kommentiere ein Issue, um es zu übernehmen.

1. **Das Format `.arb` v0 (EBNF) in `docs/ARB-FORMAT.md` spezifizieren** — `help wanted`, `docs`, `area:language`
   Kopf (`game`, `board`, `players`), Abschnitte `pieces`, `setup`, `moves`, `end`. Von dem ausgehen, was `parse_header` akzeptiert; 3 durchgerechnete Beispiele einschließen (Tic-Tac-Toe, Vier gewinnt, Gomoku).

2. **Parser für Brettdeklarationen: Gitter, Hex, Graph** — `good first issue`, `area:language`
   `board: grid 8x8`, `board: hex 5`, `board: graph file`. Ein typisiertes `Board` zurückgeben; Unsinn mit Zeilennummern ablehnen.

3. **Bewegungsprimitive: Leaper, Rider, Hopper — kombinierbar** — `help wanted`, `area:rules`
   Als Funktionen über `Board` mit Unit-Tests auf kleinen Brettern implementieren; kein spielspezifischer Code.

4. **Spiel als Daten: englisches Dame in `games/draughts.arb`** — `help wanted`, `area:games`
   Mit Schlagzwang und Mehrfachsprüngen. Konformität: Perft-Werte aus einer veröffentlichten Quelle (zitieren).

5. **Spiel als Daten: Vier gewinnt und Gomoku** — `good first issue`, `area:games`
   Zwei `.arb`-Dateien plus Perft-/Ergebnistests. Eines wählen und das im Issue sagen.

6. **Spiel als Daten: eine Schachvariante (Los Alamos, Capablanca oder Racing Kings)** — `help wanted`, `area:games`
   Braucht Figurendefinitionen, Umwandlung und Schachregeln allein aus der Datendatei. Die Quelle der Regeln nennen.

7. **Konformitätsprüfstand: Perft-Dateien unter `tests/perft/`** — `good first issue`, `tests`
   Ein winziges TSV-Format `position<TAB>depth<TAB>count` und ein Testläufer, der mit lesbarem Diff fehlschlägt.

8. **Generischer MCTS-Spieler (zufällige Playouts) über `Game`** — `help wanted`, `area:ai`
   Zufallsgenerator mit Seed ohne externe Crates vorerst; einen Zufallsspieler in ≥ 90 % der Partien bei Tic-Tac-Toe und Vier gewinnt schlagen.

9. **Verborgene Information und Zufall: Shuffles mit Seed, Würfel, Teilsichten** — `help wanted`, `area:rules`
   Den Trait vorsichtig erweitern (Determinisierung). Zuerst eine Designnotiz, Code nach dem Review.

10. **WebAssembly-Build und eine Playground-Seite für die Spiele** — `help wanted`, `area:wasm`
   Build `wasm32-unknown-unknown`, statische Seite unter `docs/demo/`, um Tic-Tac-Toe gegen MCTS zu spielen.
