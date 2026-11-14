# ARBITRU — dix premières tâches

[English](ISSUES.md) · [Română](ISSUES.ro.md) · **Français** · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · [Português](ISSUES.pt.md) · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Créez-les sur GitHub avec `scripts/create-issues.sh OWNER/REPO`. Commentez un ticket pour le réclamer.

1. **Spécifier le format `.arb` v0 (EBNF) dans `docs/ARB-FORMAT.md`** — `help wanted`, `docs`, `area:language`
   En-tête (`game`, `board`, `players`), sections `pieces`, `setup`, `moves`, `end`. Partir de ce qu’accepte `parse_header` ; inclure 3 exemples traités (morpion, Puissance 4, Gomoku).

2. **Analyseur des déclarations de plateau : grille, hex, graphe** — `good first issue`, `area:language`
   `board: grid 8x8`, `board: hex 5`, `board: graph file`. Retourner un `Board` typé ; rejeter les absurdités avec des numéros de ligne.

3. **Primitives de déplacement : leaper, rider, hopper — composables** — `help wanted`, `area:rules`
   Les implémenter comme fonctions sur `Board` avec des tests unitaires sur de petits plateaux ; aucun code propre à un jeu.

4. **Jeu comme données : dames anglaises dans `games/draughts.arb`** — `help wanted`, `area:games`
   Inclut la prise obligatoire et les sauts multiples. Conformité : valeurs perft d’une source publiée (la citer).

5. **Jeu comme données : Puissance 4 et Gomoku** — `good first issue`, `area:games`
   Deux fichiers `.arb` plus des tests perft/résultat. En choisir un et le dire dans le ticket.

6. **Jeu comme données : une variante d’échecs (Los Alamos, Capablanca ou Racing Kings)** — `help wanted`, `area:games`
   Nécessite les définitions des pièces, la promotion et les règles d’échec depuis le seul fichier de données. Citer la source des règles.

7. **Banc de conformité : fichiers perft sous `tests/perft/`** — `good first issue`, `tests`
   Un petit format TSV `position<TAB>depth<TAB>count` et un exécuteur de tests qui échoue avec une différence lisible.

8. **Joueur MCTS générique (parties aléatoires) sur `Game`** — `help wanted`, `area:ai`
   RNG à graine sans crates externes pour l’instant ; battre un joueur aléatoire dans ≥ 90 % des cas au morpion et à Puissance 4.

9. **Information cachée et hasard : mélanges à graine, dés, vues partielles** — `help wanted`, `area:rules`
   Étendre le trait avec précaution (déterminisation). Note de conception d’abord, code après relecture.

10. **Build WebAssembly et une page bac à sable pour les jeux** — `help wanted`, `area:wasm`
   Build `wasm32-unknown-unknown`, page statique sous `docs/demo/` pour jouer au morpion contre MCTS.
