# DungeonEscape2D
2D dungeon escape game where the player navigates procedurally generated rooms, finds loot, and fights enemies to escape. Written in Python with a Rust backend using PyO3 for dungeon generation and AI concurrency. Game interface built using pygame.

# Installation
- Clone the repository:
```bash
git clone https://github.com/StevePryjmak/DungeonEscape2D
```
- Create and activate venv
```bash
python -m venv venv
.\.venv\Scripts\... # corect script depending on your operating system
```
- Set up Python dependencies:
```bash
pip install -r requirements.txt
```
- Build Rust module
```bash
python -m maturin develop
```
- Run the game:
```bash
python.exe .\python\main.py
```

# Code structure

```
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
```

# Future Ideas

- Add the shop to the game
- Maka some balance changes
- Put exit from dungen in sturting point but it requires 4 keys from each corner
- Refactor code 
- Make diffrent types of enemies (ex. slimes ...)
- Make everytiong more animated
- Add boses and special loot