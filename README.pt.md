# ARBITRU — Motor de regras universal para jogos de tabuleiro, cartas, dados e quebra-cabeças, com linguagem de regras declarativa.

[English](README.md) · [Română](README.ro.md) · [Français](README.fr.md) · [Italiano](README.it.md) · [Español](README.es.md) · **Português** · [Deutsch](README.de.md) · [Magyar](README.hu.md)

Versão: 0.1.0-alpha.1 · Última atualização: 2026-11-14

🌐 https://arbitru.chiuta.ro

## Estado

Esqueleto inicial (pré-alfa): o workspace compila e os testes passam no CI. Cada funcionalidade prevista é uma tarefa aberta em `docs/issues/`.

## Âmbito previsto

- Um formato de regras declarativo `.arb` (v0, especificado em EBNF) e um analisador para tabuleiros de grelha, hexagonais e de grafo.
- Primitivas de movimento combináveis: saltador, corredor e hopper.
- Jogos como dados: damas inglesas, Quatro em Linha, Gomoku e uma variante de xadrez (Los Alamos, Capablanca ou Racing Kings).
- Um banco de conformidade baseado em ficheiros perft e um jogador genérico com pesquisa em árvore Monte Carlo.
- Informação oculta e acaso (baralhamentos com semente, dados, vistas parciais) e uma página de demonstração WebAssembly.

Nada disto está ainda implementado: cada ponto é uma tarefa aberta em `docs/issues/`.

## Compilar e testar

```bash
cargo test --workspace
cargo run -p arbitru-cli -- perft 5
```

## Contribuir

Leia `CONTRIBUTING.md`, escolha uma tarefa com a etiqueta `good first issue` em `docs/issues/` e abra um pull request. Nenhuma contribuição é demasiado pequena: uma linha de tabela, um teste ou uma tradução contam.

## Implementação independente

Escrita a partir de especificações públicas e manuais; não foi copiado código-fonte de terceiros (ver `CLEANROOM.md`).

## Licença

MIT — Copyright (c) 2026 Alexandru-Ionuț Chiuță (Alexio).

## Apoio

Trade-free: sem rastreamento nem publicidade. Se lhe for útil, pode apoiar o autor: https://buymeacoffee.com/echo.of.the.strings
