import pygame
from dungeon_core import Dungeon, Entity  # Rust module

CELL_SIZE = 40

WALL_THICKNESS = 3
PADDING = 20
FPS = 60

PLAYER_COLOR = (0, 255, 0)

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
        self.player = Entity(int(maze_height/2), int(maze_width/2), 1, 0, 1,0, True)
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
            ("sword", str(self.dungeon.player.attack)),
            ("shield", str(self.dungeon.player.armor)),
            ("heart", str(self.dungeon.player.health)),
            ("coin", str(self.dungeon.player.gold)),
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
    
    def draw_enemy_info(self, enemy, mouse_pos):
        font = pygame.font.SysFont("Arial", 18)
        # Prepare stat icons and values
        if not hasattr(self, "icon_images"):
            def load_icon(name):
                path = ICON_PATH + name + ".png"
                img = pygame.image.load(path).convert_alpha()
                return pygame.transform.smoothscale(img, (22, 22))
            self.icon_images = {
                "sword": load_icon("sword"),
                "heart": load_icon("heart"),
            }
        stats = [
            ("sword", str(enemy.attack)),
            ("heart", str(enemy.health)),
        ]
        # Calculate width and height for the info box
        padding = 8
        spacing = 8
        icon_w = 22
        text_surfaces = [font.render(val, True, (30, 30, 30)) for _, val in stats]
        width = sum(icon_w + spacing + ts.get_width() + spacing for ts in text_surfaces) - spacing
        height = max(icon_w, font.get_height()) + padding * 2

        # Position box near mouse, but keep inside window
        x, y = mouse_pos[0] + 16, mouse_pos[1] + 16
        if x + width > self.screen.get_width():
            x = self.screen.get_width() - width - 4
        if y + height > self.screen.get_height():
            y = self.screen.get_height() - height - 4

        # Draw rounded background
        rect = pygame.Rect(x, y, width, height)
        pygame.draw.rect(self.screen, (245, 245, 245), rect, border_radius=8)
        pygame.draw.rect(self.screen, (80, 80, 80), rect, 2, border_radius=8)

        # Draw icons and values
        draw_x = x + padding
        draw_y = y + (height - icon_w) // 2
        for icon_name, val in stats:
            icon = self.icon_images[icon_name]
            self.screen.blit(icon, (draw_x, draw_y))
            draw_x += icon_w + 4
            text_surface = font.render(val, True, (30, 30, 30))
            self.screen.blit(text_surface, (draw_x, y + (height - text_surface.get_height()) // 2))
            draw_x += text_surface.get_width() + spacing

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

    def draw_chests(self):
    # Load chest image once and cache it
        if not hasattr(self, "chest_image"):
            path = ICON_PATH + "chest_closed.png"
            img = pygame.image.load(path).convert_alpha()
            self.chest_image = pygame.transform.smoothscale(img, (CELL_SIZE, CELL_SIZE))
        for chest in self.dungeon.current_maze().chests:
            # Only draw closed chests
            if not chest.is_open:
                x = PADDING + chest.col * CELL_SIZE
                y = self.stats_height + PADDING + chest.row * CELL_SIZE
                self.screen.blit(self.chest_image, (x, y))
    
    def draw_player(self, player):
        # Load hero image once and cache it
        if not hasattr(self, "hero_image"):
            path = ICON_PATH + "hero.png"
            img = pygame.image.load(path).convert_alpha()
            self.hero_image = pygame.transform.smoothscale(img, (CELL_SIZE, CELL_SIZE))
        x = PADDING + player.x * CELL_SIZE
        y = self.stats_height + PADDING + player.y * CELL_SIZE  # Offset by stats bar
        self.screen.blit(self.hero_image, (x, y))
    
    def draw_enemies(self, enemies):
        # Load enemy image once and cache it
        if not hasattr(self, "enemy_image"):
            path = ICON_PATH + "enemy_bat.png"
            img = pygame.image.load(path).convert_alpha()
            self.enemy_image = pygame.transform.smoothscale(img, (CELL_SIZE, CELL_SIZE))
        for enemy in enemies:
            # Only draw enemies in the current room
            x = PADDING + enemy.x * CELL_SIZE
            y = self.stats_height + PADDING + enemy.y * CELL_SIZE
            self.screen.blit(self.enemy_image, (x, y))

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
        # self.dungeon.spawn_enemies(5)  # Spawn enemies in the dungeon
        hovered_enemy = None
        self.refresh()
        while running:

            mouse_pos = pygame.mouse.get_pos()
            hovered_enemy = None
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    running = False
                elif event.type == pygame.KEYDOWN:
                    direction = DIRECTION_KEYS.get(event.key)
                    if direction:
                        secsess = self.move_player(direction)
            self.refresh()
            # After handling events, check if mouse is over any enemy
            for enemy in self.dungeon.enemies:
                enemy_rect = pygame.Rect(
                    PADDING + enemy.x * CELL_SIZE,
                    self.stats_height + PADDING + enemy.y * CELL_SIZE,
                    CELL_SIZE, CELL_SIZE
                )
                if enemy_rect.collidepoint(mouse_pos):
                    hovered_enemy = enemy
                    break
            if hovered_enemy:
                self.draw_enemy_info(hovered_enemy, mouse_pos)
            self.win_check()
            self.eng_check()
            self.clock.tick(FPS)

        pygame.quit()
    
    def refresh(self):
        self.screen.fill((0, 0, 0))
        maze = self.dungeon.current_maze()
        self.draw_player_stats()
        self.draw_maze(maze)
        self.draw_player(self.player)
        self.draw_minimap()
        self.draw_chests()
        self.draw_enemies(self.dungeon.enemies)
        pygame.display.flip()
    
    def win_check(self):
        font = pygame.font.SysFont("Arial", 48)
        # Check if player is at an exit position in a corner room
        # Exits are always in the middle cell of each corner room
        exits = [
            (0, 0, int(self.maze_height // 2), 0),  # top-left room, middle of left edge
            (0, self.dungeon_cols - 1, int(self.maze_height // 2), self.maze_width - 1),  # top-right room, middle of right edge
            (self.dungeon_rows - 1, 0, int(self.maze_height // 2), 0),  # bottom-left room, middle of left edge
            (self.dungeon_rows - 1, self.dungeon_cols - 1, int(self.maze_height // 2), self.maze_width - 1),  # bottom-right room, middle of right edge
        ]
        player_room_row = self.dungeon.current_room_row
        player_room_col = self.dungeon.current_room_col
        player_y = self.dungeon.player.y
        player_x = self.dungeon.player.x
        at_exit = any(
            (player_room_row == er and player_room_col == ec and player_y == ey and player_x == ex)
            for er, ec, ey, ex in exits
        )
        if not at_exit:
            return
        text = font.render("You Win!", True, (0, 255, 0))
        rect = text.get_rect(center=(self.screen.get_width() // 2, self.screen.get_height() // 2))
        self.screen.blit(text, rect)

        # Draw Restart Button
        button_font = pygame.font.SysFont("Arial", 36)
        button_text = button_font.render("Restart", True, (255, 255, 255))
        button_rect = pygame.Rect(0, 0, 200, 60)
        button_rect.center = (self.screen.get_width() // 2, self.screen.get_height() // 2 + 100)
        pygame.draw.rect(self.screen, (0, 128, 0), button_rect)
        self.screen.blit(button_text, button_text.get_rect(center=button_rect.center))

        pygame.display.flip()

        waiting = True
        while waiting:
            for event in pygame.event.get():
                if event.type == pygame.QUIT:
                    pygame.quit()
                    exit()
                if event.type == pygame.MOUSEBUTTONDOWN:
                    if button_rect.collidepoint(event.pos):
                        # Reset other game variables
                        self = DungeonGUI(
                            dungeon_rows=self.dungeon_rows,
                            dungeon_cols=self.dungeon_cols,
                            maze_width=self.maze_width,
                            maze_height=self.maze_height
                        )
                        return self.run()
            pygame.time.wait(10)

    def eng_check(self):
        if self.dungeon.player.health <= 0:
            font = pygame.font.SysFont("Arial", 48)
            text = font.render("Game Over!", True, (255, 0, 0))
            rect = text.get_rect(center=(self.screen.get_width() // 2, self.screen.get_height() // 2))
            self.screen.blit(text, rect)

            # Draw Restart Button
            button_font = pygame.font.SysFont("Arial", 36)
            button_text = button_font.render("Restart", True, (255, 255, 255))
            button_rect = pygame.Rect(0, 0, 200, 60)
            button_rect.center = (self.screen.get_width() // 2, self.screen.get_height() // 2 + 100)
            pygame.draw.rect(self.screen, (0, 128, 0), button_rect)
            self.screen.blit(button_text, button_text.get_rect(center=button_rect.center))

            pygame.display.flip()

            waiting = True
            while waiting:
                for event in pygame.event.get():
                    if event.type == pygame.QUIT:
                        pygame.quit()
                        exit()
                    if event.type == pygame.MOUSEBUTTONDOWN:
                        if button_rect.collidepoint(event.pos):
                            # Reset other game variables
                            self = DungeonGUI(
                                dungeon_rows=self.dungeon_rows,
                                dungeon_cols=self.dungeon_cols,
                                maze_width=self.maze_width,
                                maze_height=self.maze_height
                            )
                            return self.run()
                pygame.time.wait(10)


if __name__ == "__main__":
    gui = DungeonGUI(dungeon_rows=5, dungeon_cols=5, maze_width=11, maze_height=11)
    gui.run()
