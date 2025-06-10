# Daunt

A simple roguelike dungeon crawler built with Entity Component System (ECS) and data-driven programming principles in Rust as a learning project. Started by following Herbert Wolverson's excellent "Hands-On Rust" book, and then continued by adding spells, waves of enemies, and custom sprites.

## Screenshot

![image](https://github.com/user-attachments/assets/63ec6301-ea00-4a48-a9ac-0cfb156ee2fa)

## Screenshot

![image](https://github.com/user-attachments/assets/63ec6301-ea00-4a48-a9ac-0cfb156ee2fa)

## Installation & Running

### Prerequisites
- [Rust](https://rustup.rs/)

### Quick Start
```bash
git clone https://github.com/limeraiin/daunt-rust.git
cd daunt-rust
cargo run --release
```

## Features

### A Simple Spell System
- **Dash** - Teleport through corridors (costs 4 mana)
- **Fireball** - AOE damage with a blast radius (costs 5 mana)

### Waves of Enemies
- 3 waves of enemies
- Different enemy types: Goblins, Orcs, Trolls
- Enemies use flow-field pathfinding

## Controls

| Key | What it does |
|-----|-------------|
| Arrow Keys | Move around, bump onto enemies to attack |
| Space | Skip turn |
| D | Cast 'Dash' |
| F | Cast 'Fireball' |
| Left Click | Cast spell at cursor |
| Right Click | Cancel spell |
