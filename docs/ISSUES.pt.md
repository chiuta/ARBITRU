# ARBITRU — primeiras dez tarefas

[English](ISSUES.md) · [Română](ISSUES.ro.md) · [Français](ISSUES.fr.md) · [Italiano](ISSUES.it.md) · [Español](ISSUES.es.md) · **Português** · [Deutsch](ISSUES.de.md) · [Magyar](ISSUES.hu.md)

Crie-as no GitHub com `scripts/create-issues.sh OWNER/REPO`. Comente numa issue para a reivindicar.

1. **Especifique o formato `.arb` v0 (EBNF) em `docs/ARB-FORMAT.md`** — `help wanted`, `docs`, `area:language`
   Cabeçalho (`game`, `board`, `players`), seções `pieces`, `setup`, `moves`, `end`. Parta do que `parse_header` aceita; inclua 3 exemplos resolvidos (jogo da velha, Connect Four, Gomoku).

2. **Parser para declarações de tabuleiro: grade, hex, grafo** — `good first issue`, `area:language`
   `board: grid 8x8`, `board: hex 5`, `board: graph file`. Devolva um `Board` tipado; rejeite absurdos com números de linha.

3. **Primitivas de movimento: leaper, rider, hopper — componíveis** — `help wanted`, `area:rules`
   Implemente-as como funções sobre `Board` com testes unitários em tabuleiros pequenos; sem código específico de nenhum jogo.

4. **Jogo como dados: damas inglesas em `games/draughts.arb`** — `help wanted`, `area:games`
   Inclui captura obrigatória e saltos múltiplos. Conformidade: valores perft de uma fonte publicada (cite-a).

5. **Jogo como dados: Connect Four e Gomoku** — `good first issue`, `area:games`
   Dois arquivos `.arb` mais testes perft/resultado. Escolha um e diga-o na issue.

6. **Jogo como dados: uma variante de xadrez (Los Alamos, Capablanca ou Racing Kings)** — `help wanted`, `area:games`
   Precisa de definições de peças, promoção e regras de xeque apenas a partir do arquivo de dados. Cite a fonte das regras.

7. **Bancada de conformidade: arquivos perft em `tests/perft/`** — `good first issue`, `tests`
   Um pequeno formato TSV `position<TAB>depth<TAB>count` e um executor de testes que falha com uma diferença legível.

8. **Jogador MCTS genérico (partidas aleatórias) sobre `Game`** — `help wanted`, `area:ai`
   RNG com semente sem crates externos por enquanto; vença um jogador aleatório em ≥ 90% das partidas de jogo da velha e Connect Four.

9. **Informação oculta e acaso: embaralhamentos com semente, dados, vistas parciais** — `help wanted`, `area:rules`
   Estenda o trait com cuidado (determinização). Primeiro uma nota de projeto, o código depois da revisão.

10. **Build WebAssembly e uma página de testes para os jogos** — `help wanted`, `area:wasm`
   Build `wasm32-unknown-unknown`, página estática em `docs/demo/` para jogar jogo da velha contra o MCTS.
