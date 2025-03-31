# Auto Chess Backend

This project is a backend for an auto chess game, implemented using the [specs](https://github.com/amethyst/specs) Entity-Component-System (ECS) framework in Rust. It provides the core game logic and state management for an auto chess game.

## Features

- **Entity-Component-System Architecture**: The project uses the `specs` ECS framework to manage game entities and their components efficiently.
- **Chess Entities**: Supports different types of chess pieces, each with unique stats and abilities.
- **Turn-Based System**: Implements a turn-based game loop with phases such as preparation, combat, and resolution.
- **Skills and Effects**: Chess pieces have unique skills and can be affected by various status effects.

## Key Components

### `CombatStats`
The `CombatStats` structure defines the combat-related attributes of a chess piece, including:
- `name`: The name of the chess piece.
- `hp` and `max_hp`: Current and maximum health points.
- `attack`: Attack power.
- `defense`: Defense power.
- `magic_resist`: Resistance to magic damage.
- `attack_speed`: Speed of attacks.
- `attack_range`: Range of attacks.
- `mana` and `max_mana`: Current and maximum mana points.
- `skill`: The unique skill of the chess piece.

### `GameState`
The `GameState` structure manages the overall game state, including:
- `world`: The ECS world containing all entities and components.
- `turn_manager`: Manages the turn-based game loop.

### Chess Types
The game supports multiple chess types, each with unique stats and skills:
- **Warrior**: High health and defense, with a melee skill.
- **Mage**: High magic damage and range, with a fireball skill.
- **Archer**: High attack and range, with a multi-shot skill.
- **Tank**: High health and defense, with a shield bash skill.

## How to Use

1. Clone the repository.
2. Build the project using Cargo:
   ```bash
   cargo build
   ```
3. Run the project:
   ```bash
   cargo run
   ```

## Dependencies

- [specs](https://github.com/amethyst/specs): ECS framework for Rust.
- [uuid](https://crates.io/crates/uuid): For generating unique IDs for chess entities.

## Future Improvements

- Add more chess types and skills.
- Implement AI for automated gameplay.
- Add networking support for multiplayer games.

## License

This project is licensed under the MIT License.