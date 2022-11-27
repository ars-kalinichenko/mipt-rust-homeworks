# Game of Trust

In this problem, you'll write the simulator of the game called [The Evolution of Trust](https://ncase.me/trust/).

Before solving this problem, please finish it online by the link above.

## Task

Implement the structure `Game`. `Game::new` accepts two agents that will play against each other. Function `Game::play_round()` simulates one round of the game, and returns the overcome it. Functions `Game::left_score()` and `Game::right_score()` return the score of each player.

The rounds go as follows. Each of the two agents decides whether he will cooperate with the other agent or betray him. If both agents cooperate, the score of each increase by 2. If both agents cheat, then the score of both remains unchanged. If one cooperates and the other cheats, the cheater's score increases by 3 and the cooperating agent's score decreases by 1. Initially, each agent's score is 0.

`Game` accepts two agents as `Box<dyn Trait>`. You have to decide by yourself what methods this common trait will have.

Basically, there are 5 types of agents

- `CheatingAgent` - always cheats.
- `CooperatingAgent` - always cooperates.
- `GrudgerAgent` - always cooperates until first betrayal, then always cheats.
- `CopycatAgent` - cooperates first, then repeats the last turn of opponent.
- `DetectiveAgent` - begins with sequence "cooperate", "cheat", "cooperate", "cooperate". If opponent never cheated, then always cheats. Otherwise, plays as copycat agent.
