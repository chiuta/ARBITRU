# ARBITRU — architecture

`arbitru-core` defines the `Game` trait (moves / apply / outcome) and generic tools over it (`perft`, `count_outcomes`, later MCTS).
Games will be **data**: `games/<name>.arb` files (`key: value` header + rules) loaded by an interpreter that implements `Game`.
The hand-written `TicTacToe` is the oracle that the interpreter must reproduce (255,168 complete games: 131,184 / 77,904 / 46,080).
Each new game = one `.arb` file + conformance numbers, which makes it a self-contained pull request.
