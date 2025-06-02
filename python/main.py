import pygame
from dungeon_core import Dungeon, Entity  # Rust module

CELL_SIZE = 40

WALL_THICKNESS = 3
PADDING = 20
FPS = 60

PLAYER_COLOR = (0, 255, 0)
PLAYER_RADIUS = CELL_SIZE // 3

MINIMAP_ROOM_SIZE = 20
MINIMAP_PADDING = 30
MINIMAP_ROOM_MARGIN = 4
ICON_PATH = "icons/"

DIRECTION_KEYS = {
    pygame.K_UP: "up",
    pygame.K_DOWN: "down",
    pygame.K_LEFT: "left",
    pygame.K_RIGHT: "right"
}

class DungeonGUI:
    def __init__(self, dungeon_rows, dungeon_cols, maze_width, maze_height):
        self.player = Entity(int(maze_height/2), int(maze_width/2), 100, 0, 1,0, True)
        self.dungeon = Dungeon(dungeon_rows, dungeon_cols, maze_width, maze_height, self.player)
        self.maze_width = maze_width
        self.maze_height = maze_height
        self.dungeon_rows = dungeon_rows
        self.dungeon_cols = dungeon_cols

        # Add space for minimap on the right and for stats at the top
        self.stats_height = 40  # Height reserved for stats bar
        window_width = (
            PADDING * 2 + maze_width * CELL_SIZE +
            MINIMAP_PADDING + dungeon_cols * (MINIMAP_ROOM_SIZE + MINIMAP_ROOM_MARGIN)
        )
        window_height = max(
            self.stats_height + PADDING * 2 + maze_height * CELL_SIZE,
            MINIMAP_PADDING * 2 + dungeon_rows * (MINIMAP_ROOM_SIZE + MINIMAP_ROOM_MARGIN)
        )

        pygame.init()
        self.screen = pygame.display.set_mode((window_width, window_height))
        pygame.display.set_caption("Dungeon Escape 2D")
        self.clock = pygame.time.Clock()

    def draw_player_stats(self):
        font = pygame.font.SysFont("Arial", 24)
        x = PADDING
        y = (self.stats_height - font.get_height()) // 2

        # Load icons (cache them as attributes to avoid reloading every frame)
        if not hasattr(self, "icon_images"):
            def load_icon(name):
                path = ICON_PATH + name + ".png"
                img = pygame.image.load(path).convert_alpha()
                return pygame.transform.smoothscale(img, (28, 28))
            self.icon_images = {
                "sword": load_icon("sword"),
                "shield": load_icon("shield"),
                "heart": load_icon("heart"),
                "coin": load_icon("coin"),
            }

        # Prepare stat values
        stats = [
            ("sword", str(self.player.attack)),
            ("shield", str(self.player.armor)),
            ("heart", str(self.player.health)),
            ("coin", str(self.player.gold)),
        ]

        # Draw background bar
        stats_bar_rect = pygame.Rect(0, 0, self.screen.get_width(), self.stats_height)
        pygame.draw.rect(self.screen, (30, 30, 30), stats_bar_rect)

        # Draw each icon and value
        for icon_name, value in stats:
            icon = self.icon_images[icon_name]
            self.screen.blit(icon, (x, y))
            x += icon.get_width() + 4
            text_surface = font.render(value, True, (255, 255, 255))
            self.screen.blit(text_surface, (x, y + (icon.get_height() - font.get_height()) // 2))
            x += text_surface.get_width() + 20  # Space between stats

    def draw_maze(self, maze):
        for row in range(self.maze_height):
            for col in range(self.maze_width):
                x = PADDING + col * CELL_SIZE
                y = self.stats_height + PADDING + row * CELL_SIZE  # Offset by stats bar
                walls = maze.get_cell_walls(row, col)
                if walls[0]:
                    pygame.draw.line(self.screen, (255, 255, 255), (x, y), (x + CELL_SIZE, y), WALL_THICKNESS)
                if walls[1]:
                    pygame.draw.line(self.screen, (255, 255, 255), (x + CELL_SIZE, y), (x + CELL_SIZE, y + CELL_SIZE), WALL_THICKNESS)
                if walls[2]:
                    pygame.draw.line(self.screen, (255, 255, 255), (x, y + CELL_SIZE), (x + CELL_SIZE, y + CELL_SIZE), WALL_THICKNESS)
                if walls[3]:
                    pygame.draw.line(self.screen, (255, 255, 255), (x, y), (x, y + CELL_SIZE), WALL_THICKNESS)

    def draw_player(self, player):
        # Load hero image once and cache it
        if not hasattr(self, "hero_image"):
            path = ICON_PATH + "hero.png"
            img = pygame.image.load(path).convert_alpha()
            self.hero_image = pygame.transform.smoothscale(img, (CELL_SIZE, CELL_SIZE))
        x = PADDING + player.x * CELL_SIZE
        y = self.stats_height + PADDING + player.y * CELL_SIZE  # Offset by stats bar
        self.screen.blit(self.hero_image, (x, y))

    def draw_minimap(self):
        # Top-left corner of minimap
        minimap_x = PADDING * 2 + self.maze_width * CELL_SIZE + MINIMAP_PADDING
        minimap_y = self.stats_height + MINIMAP_PADDING  # Offset by stats bar

        # Get current room position
        room_row = self.dungeon.current_room_row
        room_col = self.dungeon.current_room_col

        for row in range(self.dungeon_rows):
            for col in range(self.dungeon_cols):
                rx = minimap_x + col * (MINIMAP_ROOM_SIZE + MINIMAP_ROOM_MARGIN)
                ry = minimap_y + row * (MINIMAP_ROOM_SIZE + MINIMAP_ROOM_MARGIN)
                rect = pygame.Rect(rx, ry, MINIMAP_ROOM_SIZE, MINIMAP_ROOM_SIZE)
                color = (100, 100, 100)
                if row == room_row and col == room_col:
                    color = (0, 255, 0)  # Highlight current room
                pygame.draw.rect(self.screen, color, rect)
                pygame.draw.rect(self.screen, (255, 255, 255), rect, 2)  # Border

    def move_player(self, direction):
        self.dungeon.move_player(direction)
        self.player.x = self.dungeon.player.x
        self.player.y = self.dungeon.player.y

    def run(self):
        running = True
        while running:
            self.screen.fill((0, 0, 0))
            maze = self.dungeon.current_maze()
            self.draw_player_stats()
            self.draw_maze(maze)
            self.draw_player(self.player)
            self.draw_minimap()
            pygame.display.flip()
            self.clock.tick(FPS)

            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
                elif event.type == pygame.KEYDOWN:
                    direction = DIRECTION_KEYS.get(event.key)
                    if direction:
                        self.move_player(direction)

        pygame.quit()

if __name__ == "__main__":
    gui = DungeonGUI(dungeon_rows=5, dungeon_cols=5, maze_width=11, maze_height=11)
    gui.run()
