# Auto Chess Backend / 自走棋后端

This project is a backend for an auto chess game, implemented using the [specs](https://github.com/amethyst/specs) Entity-Component-System (ECS) framework in Rust. It provides the core game logic and state management for an auto chess game.

本项目是一个自走棋游戏的后端实现，使用 Rust 的 [specs](https://github.com/amethyst/specs) 实体-组件-系统（ECS）框架开发。它提供了自走棋游戏的核心逻辑和状态管理。

## Features / 功能特性

- **Entity-Component-System Architecture**: The project uses the `specs` ECS framework to manage game entities and their components efficiently.
  **实体-组件-系统架构**：项目使用 `specs` ECS 框架高效地管理游戏实体及其组件。

- **Chess Entities**: Supports different types of chess pieces, each with unique stats and abilities.
  **棋子实体**：支持不同类型的棋子，每个棋子都有独特的属性和能力。

- **Turn-Based System**: Implements a turn-based game loop with phases such as preparation, combat, and resolution.
  **回合制系统**：实现了包含准备、战斗和结算等阶段的回合制游戏循环。

- **Skills and Effects**: Chess pieces have unique skills and can be affected by various status effects.
  **技能和效果**：棋子拥有独特的技能，并可能受到各种状态效果的影响。

## Key Components / 核心组件

### `CombatStats` / 战斗属性
The `CombatStats` structure defines the combat-related attributes of a chess piece, including:
`CombatStats` 结构定义了棋子的战斗相关属性，包括：

- `name`: The name of the chess piece.
  `name`：棋子名称。

- `hp` and `max_hp`: Current and maximum health points.
  `hp` 和 `max_hp`：当前和最大生命值。

- `attack`: Attack power.
  `attack`：攻击力。

- `defense`: Defense power.
  `defense`：防御力。

- `magic_resist`: Resistance to magic damage.
  `magic_resist`：魔法抗性。

- `attack_speed`: Speed of attacks.
  `attack_speed`：攻击速度。

- `attack_range`: Range of attacks.
  `attack_range`：攻击范围。

- `mana` and `max_mana`: Current and maximum mana points.
  `mana` 和 `max_mana`：当前和最大法力值。

- `skill`: The unique skill of the chess piece.
  `skill`：棋子的独特技能。

### `GameState` / 游戏状态
The `GameState` structure manages the overall game state, including:
`GameState` 结构管理整体游戏状态，包括：

- `world`: The ECS world containing all entities and components.
  `world`：包含所有实体和组件的 ECS 世界。

- `turn_manager`: Manages the turn-based game loop.
  `turn_manager`：管理回合制游戏循环。

### Chess Types / 棋子类型
The game supports multiple chess types, each with unique stats and skills:
游戏支持多种棋子类型，每种都有独特的属性和技能：

- **Warrior**: High health and defense, with a melee skill.
  **战士**：高生命值和防御力，拥有近战技能。

- **Mage**: High magic damage and range, with a fireball skill.
  **法师**：高魔法伤害和攻击范围，拥有火球技能。

- **Archer**: High attack and range, with a multi-shot skill.
  **弓箭手**：高攻击力和攻击范围，拥有多重射击技能。

- **Tank**: High health and defense, with a shield bash skill.
  **坦克**：高生命值和防御力，拥有盾击技能。

## How to Use / 使用方法

1. Clone the repository.
   克隆仓库。

2. Build the project using Cargo:
   使用 Cargo 构建项目：
   ```bash
   cargo build
   ```

3. Run the project:
   运行项目：
   ```bash
   cargo run
   ```

## Dependencies / 依赖项

- [specs](https://github.com/amethyst/specs): ECS framework for Rust.
  [specs](https://github.com/amethyst/specs)：Rust 的 ECS 框架。

- [uuid](https://crates.io/crates/uuid): For generating unique IDs for chess entities.
  [uuid](https://crates.io/crates/uuid)：用于为棋子实体生成唯一 ID。

## Future Improvements / 未来改进

- Add more chess types and skills.
  添加更多棋子类型和技能。

- Implement AI for automated gameplay.
  实现 AI 用于自动游戏。

- Add networking support for multiplayer games.
  添加网络支持以实现多人游戏。

## License / 许可证

This project is licensed under the MIT License.
本项目采用 MIT 许可证。