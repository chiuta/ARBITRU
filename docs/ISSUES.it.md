# ARBITRU — primi dieci compiti

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · **Italiano** · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Creale su GitHub con `scripts/create-issues.sh OWNER/REPO`. Commenta una issue per prenderla in carico.

1. **Specifica il formato `.arb` v0 (EBNF) in `docs/ARB-FORMAT.md`** — `help wanted`, `docs`, `area:language`
   Intestazione (`game`, `board`, `players`), sezioni `pieces`, `setup`, `moves`, `end`. Parti da ciò che accetta `parse_header`; includi 3 esempi svolti (tris, Forza 4, Gomoku).

2. **Parser per le dichiarazioni della scacchiera: griglia, esagonale, grafo** — `good first issue`, `area:language`
   `board: grid 8x8`, `board: hex 5`, `board: graph file`. Restituisci un `Board` tipizzato; rifiuta le assurdità con numeri di riga.

3. **Primitive di movimento: leaper, rider, hopper — componibili** — `help wanted`, `area:rules`
   Implementale come funzioni su `Board` con test unitari su scacchiere piccole; nessun codice specifico di un gioco.

4. **Gioco come dati: dama inglese in `games/draughts.arb`** — `help wanted`, `area:games`
   Include la cattura obbligatoria e i salti multipli. Conformità: valori perft da una fonte pubblicata (citala).

5. **Gioco come dati: Forza 4 e Gomoku** — `good first issue`, `area:games`
   Due file `.arb` più test perft/esito. Scegline uno e dillo nella issue.

6. **Gioco come dati: una variante degli scacchi (Los Alamos, Capablanca o Racing Kings)** — `help wanted`, `area:games`
   Richiede definizioni dei pezzi, promozione e regole di scacco solo dal file di dati. Cita la fonte delle regole.

7. **Banco di conformità: file perft in `tests/perft/`** — `good first issue`, `tests`
   Un piccolo formato TSV `position<TAB>depth<TAB>count` e un esecutore di test che fallisce con una differenza leggibile.

8. **Giocatore MCTS generico (partite casuali) su `Game`** — `help wanted`, `area:ai`
   RNG con seme senza crate esterni per ora; batti un giocatore casuale in ≥ 90% dei casi a tris e Forza 4.

9. **Informazione nascosta e caso: mescolamenti con seme, dadi, viste parziali** — `help wanted`, `area:rules`
   Estendi il trait con cautela (determinizzazione). Prima una nota di progetto, il codice dopo la revisione.

10. **Build WebAssembly e una pagina playground per i giochi** — `help wanted`, `area:wasm`
   Build `wasm32-unknown-unknown`, pagina statica in `docs/demo/` per giocare a tris contro MCTS.
