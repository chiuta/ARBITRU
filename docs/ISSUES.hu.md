# ARBITRU — az első tíz feladat

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · **Magyar**

Hozd létre őket a GitHubon a `scripts/create-issues.sh OWNER/REPO` paranccsal. Szólj hozzá egy issue-hoz, hogy lefoglald.

1. **Specifikáld az `.arb` v0 formátumot (EBNF) a `docs/ARB-FORMAT.md`-ben** — `help wanted`, `docs`, `area:language`
   Fejléc (`game`, `board`, `players`), `pieces`, `setup`, `moves`, `end` szakaszok. Indulj abból, amit a `parse_header` elfogad; legyen benne 3 kidolgozott példa (amőba, Connect Four, Gomoku).

2. **Parser a tábladeklarációkhoz: rács, hex, gráf** — `good first issue`, `area:language`
   `board: grid 8x8`, `board: hex 5`, `board: graph file`. Típusos `Board`-ot adj vissza; az értelmetlenséget sorszámmal utasítsd el.

3. **Mozgásprimitívek: leaper, rider, hopper — kombinálhatók** — `help wanted`, `area:rules`
   Valósítsd meg függvényekként a `Board` felett, egységtesztekkel kis táblákon; semmilyen játékspecifikus kód.

4. **Játék adatként: angol dáma a `games/draughts.arb`-ben** — `help wanted`, `area:games`
   Tartalmazza a kötelező ütést és a többszörös ugrást. Megfelelés: perft-értékek egy közzétett forrásból (hivatkozz rá).

5. **Játék adatként: Connect Four és Gomoku** — `good first issue`, `area:games`
   Két `.arb` fájl plusz perft/kimeneti tesztek. Válassz egyet, és mondd meg az issue-ban.

6. **Játék adatként: egy sakkváltozat (Los Alamos, Capablanca vagy Racing Kings)** — `help wanted`, `area:games`
   A bábudefiníciókat, az átváltozást és a sakkszabályokat csak az adatfájlból kell venni. Hivatkozz a szabályok forrására.

7. **Megfelelőségi tesztpad: perft-fájlok a `tests/perft/` alatt** — `good first issue`, `tests`
   Egy apró TSV formátum `position<TAB>depth<TAB>count` és egy tesztfuttató, amely olvasható különbséggel bukik.

8. **Általános MCTS játékos (véletlen lejátszások) a `Game` felett** — `help wanted`, `area:ai`
   Egyelőre seedelt RNG külső crate nélkül; verjen egy véletlen játékost ≥ 90%-ban amőbában és Connect Fourban.

9. **Rejtett információ és véletlen: seedelt keverések, kockák, részleges nézetek** — `help wanted`, `area:rules`
   A traitet óvatosan bővítsd (determinizálás). Először tervezési jegyzet, a kód az átnézés után.

10. **WebAssembly build és játszótér-oldal a játékokhoz** — `help wanted`, `area:wasm`
   `wasm32-unknown-unknown` build, statikus oldal a `docs/demo/` alatt, hogy amőbázhass az MCTS ellen.
