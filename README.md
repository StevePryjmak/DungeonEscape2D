# DungeonEscape2D
2D dungeon escape game where the player navigates procedurally generated rooms, finds loot, and fights enemies to escape. Written in Python with a Rust backend using PyO3 for dungeon generation and AI concurrency. Game interface built using pygame.


DungeonEscape2D/
├── Cargo.toml           # Rust crate config
├── pyproject.toml       # Python packaging config (for maturin)
├── src/
│   ├── lib.rs           # Rust core library code (PyO3 bindings here)
│   ├── maze.rs          # Rust modules
│   └── other.rs
├── python/              # Your Python code (scripts, tests, utils)
│   ├── __init__.py
│   └── main.py
├── README.md
└── ...
