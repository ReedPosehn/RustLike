#!/usr/bin/env python3
"""
Generate simple pixel art sprites for RustLike characters and enemies.
Requires: pip install pillow
"""

from PIL import Image
import os


def create_directory(path):
    os.makedirs(path, exist_ok=True)


def save_sprite(filename, pixel_data, size=32):
    """Create a sprite from a 2D array of colors."""
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    pixels = img.load()

    for y, row in enumerate(pixel_data):
        for x, color in enumerate(row):
            if color is not None:
                pixels[x, y] = color

    img.save(filename)
    print(f"✓ Created {filename}")


# Colors
TRANSPARENT = None
RED = (255, 50, 50, 255)
DARK_RED = (139, 0, 0, 255)
BLUE = (100, 150, 255, 255)
DARK_BLUE = (50, 100, 200, 255)
PURPLE = (200, 100, 255, 255)
DARK_PURPLE = (120, 50, 180, 255)
GREEN = (100, 200, 100, 255)
DARK_GREEN = (50, 120, 50, 255)
YELLOW = (255, 255, 100, 255)
GOLD = (200, 180, 50, 255)
SKIN = (255, 200, 170, 255)
DARK_SKIN = (200, 150, 100, 255)
BROWN = (139, 90, 43, 255)
LIGHT_BROWN = (180, 120, 70, 255)
WHITE = (255, 255, 255, 255)
GRAY = (128, 128, 128, 255)
DARK_GRAY = (64, 64, 64, 255)
BLACK = (0, 0, 0, 255)

assets_dir = "assets"
create_directory(assets_dir)

# ============ CHARACTER CLASSES (32x32) ============

# WARRIOR - Red armor, sword
warrior = [
    [TRANSPARENT] * 32,
    [TRANSPARENT] * 5 + [SKIN] + [TRANSPARENT] * 26,
    [TRANSPARENT] * 4 + [SKIN] * 4 + [TRANSPARENT] * 24,
    [TRANSPARENT] * 4 + [SKIN, DARK_SKIN, DARK_SKIN, SKIN] + [TRANSPARENT] * 24,
    [TRANSPARENT] * 3 + [RED] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 3 + [RED, RED, DARK_RED, RED, RED, RED] + [TRANSPARENT] * 23,
    [TRANSPARENT] * 2 + [RED] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2
    + [DARK_RED, RED, RED, RED, RED, RED, RED, DARK_RED]
    + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2 + [RED] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [RED] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 2
    + [BROWN, BROWN]
    + [RED] * 4
    + [BROWN, BROWN]
    + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2 + [BROWN] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [BROWN] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 4 + [BROWN] * 4 + [TRANSPARENT] * 24,
    [TRANSPARENT] * 5 + [BROWN] * 2 + [TRANSPARENT] * 25,
    [TRANSPARENT] * 5 + [BROWN] * 2 + [TRANSPARENT] * 25,
    [TRANSPARENT] * 32,
]
save_sprite(f"{assets_dir}/warrior.png", warrior)

# MAGE - Blue robes, staff
mage = [
    [TRANSPARENT] * 32,
    [TRANSPARENT] * 5 + [SKIN] + [TRANSPARENT] * 26,
    [TRANSPARENT] * 4 + [SKIN] * 4 + [TRANSPARENT] * 24,
    [TRANSPARENT] * 4 + [SKIN, DARK_SKIN, DARK_SKIN, SKIN] + [TRANSPARENT] * 24,
    [TRANSPARENT] * 3 + [BLUE] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 3
    + [BLUE, DARK_BLUE, BLUE, BLUE, DARK_BLUE, BLUE]
    + [TRANSPARENT] * 23,
    [TRANSPARENT] * 2 + [BLUE] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2
    + [DARK_BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, BLUE, DARK_BLUE]
    + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2 + [BLUE] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [BLUE] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 2 + [BLUE] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2 + [BLUE, BLUE] + [PURPLE] * 4 + [BLUE, BLUE] + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [PURPLE] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 4 + [PURPLE] * 4 + [TRANSPARENT] * 24,
    [TRANSPARENT] * 5 + [PURPLE] * 2 + [TRANSPARENT] * 25,
    [TRANSPARENT] * 5 + [PURPLE] * 2 + [TRANSPARENT] * 25,
    [TRANSPARENT] * 32,
]
save_sprite(f"{assets_dir}/mage.png", mage)

# ROGUE - Green armor, dagger
rogue = [
    [TRANSPARENT] * 32,
    [TRANSPARENT] * 5 + [SKIN] + [TRANSPARENT] * 26,
    [TRANSPARENT] * 4 + [SKIN] * 4 + [TRANSPARENT] * 24,
    [TRANSPARENT] * 4 + [SKIN, DARK_SKIN, DARK_SKIN, SKIN] + [TRANSPARENT] * 24,
    [TRANSPARENT] * 3 + [DARK_GREEN] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 3
    + [DARK_GREEN, GREEN, GREEN, GREEN, GREEN, DARK_GREEN]
    + [TRANSPARENT] * 23,
    [TRANSPARENT] * 2 + [DARK_GREEN] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2
    + [GREEN, DARK_GREEN, DARK_GREEN, GREEN, GREEN, DARK_GREEN, DARK_GREEN, GREEN]
    + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2 + [DARK_GREEN] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [DARK_GREEN] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 2 + [BROWN] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2 + [BROWN] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [BROWN] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 4 + [BROWN] * 4 + [TRANSPARENT] * 24,
    [TRANSPARENT] * 5 + [BROWN] * 2 + [TRANSPARENT] * 25,
    [TRANSPARENT] * 5 + [BROWN] * 2 + [TRANSPARENT] * 25,
    [TRANSPARENT] * 32,
]
save_sprite(f"{assets_dir}/rogue.png", rogue)

# PALADIN - Gold and white armor
paladin = [
    [TRANSPARENT] * 32,
    [TRANSPARENT] * 5 + [SKIN] + [TRANSPARENT] * 26,
    [TRANSPARENT] * 4 + [SKIN] * 4 + [TRANSPARENT] * 24,
    [TRANSPARENT] * 4 + [SKIN, DARK_SKIN, DARK_SKIN, SKIN] + [TRANSPARENT] * 24,
    [TRANSPARENT] * 3 + [GOLD] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 3 + [GOLD, WHITE, WHITE, WHITE, WHITE, GOLD] + [TRANSPARENT] * 23,
    [TRANSPARENT] * 2 + [GOLD] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2
    + [WHITE, GOLD, GOLD, GOLD, GOLD, GOLD, GOLD, WHITE]
    + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2 + [GOLD] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [GOLD] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 2 + [WHITE] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 2 + [WHITE] * 8 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [WHITE] * 6 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 4 + [WHITE] * 4 + [TRANSPARENT] * 24,
    [TRANSPARENT] * 5 + [WHITE] * 2 + [TRANSPARENT] * 25,
    [TRANSPARENT] * 5 + [WHITE] * 2 + [TRANSPARENT] * 25,
    [TRANSPARENT] * 32,
]
save_sprite(f"{assets_dir}/paladin.png", paladin)

# ============ ENEMIES (32x32) ============

# GOBLIN - Green skin
goblin = [
    [TRANSPARENT] * 32,
    [TRANSPARENT] * 6 + [DARK_GREEN] * 3 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 5 + [DARK_GREEN] * 5 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 5
    + [DARK_GREEN, GREEN, GREEN, GREEN, DARK_GREEN]
    + [TRANSPARENT] * 22,
    [TRANSPARENT] * 4 + [DARK_GREEN] * 7 + [TRANSPARENT] * 21,
    [TRANSPARENT] * 4
    + [DARK_GREEN, GREEN, GREEN, GREEN, GREEN, GREEN, DARK_GREEN]
    + [TRANSPARENT] * 21,
    [TRANSPARENT] * 3 + [DARK_GREEN] * 9 + [TRANSPARENT] * 20,
    [TRANSPARENT] * 3
    + [
        DARK_GREEN,
        GREEN,
        GREEN,
        DARK_GREEN,
        DARK_GREEN,
        GREEN,
        GREEN,
        DARK_GREEN,
        DARK_GREEN,
    ]
    + [TRANSPARENT] * 20,
    [TRANSPARENT] * 3 + [DARK_GREEN] * 9 + [TRANSPARENT] * 20,
    [TRANSPARENT] * 4 + [DARK_GREEN] * 7 + [TRANSPARENT] * 21,
    [TRANSPARENT] * 4 + [BROWN] * 7 + [TRANSPARENT] * 21,
    [TRANSPARENT] * 5 + [BROWN] * 5 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 6 + [BROWN] * 3 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 32,
]
save_sprite(f"{assets_dir}/goblin.png", goblin)

# ORC - Dark red/brown, large
orc = [
    [TRANSPARENT] * 32,
    [TRANSPARENT] * 5 + [DARK_RED] * 4 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 4 + [DARK_RED] * 6 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 4 + [DARK_RED, RED, RED, RED, RED, DARK_RED] + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [DARK_RED] * 8 + [TRANSPARENT] * 21,
    [TRANSPARENT] * 3
    + [DARK_RED, RED, RED, RED, RED, RED, RED, DARK_RED]
    + [TRANSPARENT] * 21,
    [TRANSPARENT] * 2 + [DARK_RED] * 10 + [TRANSPARENT] * 20,
    [TRANSPARENT] * 2
    + [DARK_RED, RED, RED, RED, RED, RED, RED, RED, RED, DARK_RED]
    + [TRANSPARENT] * 20,
    [TRANSPARENT] * 2 + [DARK_RED] * 10 + [TRANSPARENT] * 20,
    [TRANSPARENT] * 3 + [DARK_RED] * 8 + [TRANSPARENT] * 21,
    [TRANSPARENT] * 2 + [BROWN] * 10 + [TRANSPARENT] * 20,
    [TRANSPARENT] * 3 + [BROWN] * 8 + [TRANSPARENT] * 21,
    [TRANSPARENT] * 4 + [BROWN] * 6 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 5 + [BROWN] * 4 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 32,
]
save_sprite(f"{assets_dir}/orc.png", orc)

# SKELETON - White bones, dark sockets
skeleton = [
    [TRANSPARENT] * 32,
    [TRANSPARENT] * 5 + [WHITE] * 4 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 4 + [WHITE] * 6 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 4 + [WHITE, BLACK, BLACK, BLACK, BLACK, WHITE] + [TRANSPARENT] * 22,
    [TRANSPARENT] * 3 + [WHITE] * 8 + [TRANSPARENT] * 21,
    [TRANSPARENT] * 3
    + [WHITE, WHITE, BLACK, WHITE, WHITE, BLACK, WHITE, WHITE]
    + [TRANSPARENT] * 21,
    [TRANSPARENT] * 2 + [WHITE] * 10 + [TRANSPARENT] * 20,
    [TRANSPARENT] * 2
    + [WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE, WHITE]
    + [TRANSPARENT] * 20,
    [TRANSPARENT] * 2 + [WHITE] * 10 + [TRANSPARENT] * 20,
    [TRANSPARENT] * 3 + [WHITE] * 8 + [TRANSPARENT] * 21,
    [TRANSPARENT] * 2 + [GRAY] * 10 + [TRANSPARENT] * 20,
    [TRANSPARENT] * 3 + [GRAY] * 8 + [TRANSPARENT] * 21,
    [TRANSPARENT] * 4 + [GRAY] * 6 + [TRANSPARENT] * 22,
    [TRANSPARENT] * 5 + [GRAY] * 4 + [TRANSPARENT] * 23,
    [TRANSPARENT] * 32,
]
save_sprite(f"{assets_dir}/skeleton.png", skeleton)

# SPIDER - Black with red eyes
spider = [
    [TRANSPARENT] * 32,
    [TRANSPARENT] * 10 + [BLACK] * 4 + [TRANSPARENT] * 18,
    [TRANSPARENT] * 8 + [BLACK] * 8 + [TRANSPARENT] * 16,
    [TRANSPARENT] * 8
    + [BLACK, RED, RED, BLACK, BLACK, RED, RED, BLACK]
    + [TRANSPARENT] * 16,
    [TRANSPARENT] * 6 + [BLACK] * 12 + [TRANSPARENT] * 14,
    [TRANSPARENT] * 6
    + [
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
    ]
    + [TRANSPARENT] * 14,
    [TRANSPARENT] * 4 + [BLACK] * 16 + [TRANSPARENT] * 12,
    [TRANSPARENT] * 4
    + [
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
        BLACK,
    ]
    + [TRANSPARENT] * 12,
    [TRANSPARENT] * 3 + [BLACK] * 18 + [TRANSPARENT] * 11,
    [TRANSPARENT] * 2 + [BLACK] * 20 + [TRANSPARENT] * 10,
    [TRANSPARENT] * 2 + [BLACK] * 20 + [TRANSPARENT] * 10,
    [TRANSPARENT] * 3 + [BLACK] * 18 + [TRANSPARENT] * 11,
    [TRANSPARENT] * 6 + [BLACK] * 12 + [TRANSPARENT] * 14,
    [TRANSPARENT] * 32,
]
save_sprite(f"{assets_dir}/spider.png", spider)

# SIMPLE TILESET - Grass, dirt, stone, wood, water, sand, gravel, rocks, building tiles
grass_tile = [
    [
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
    ]
    * 4,
    [DARK_GREEN, GREEN, GREEN, DARK_GREEN, DARK_GREEN, GREEN, DARK_GREEN, GREEN] * 4,
    [DARK_GREEN, GREEN, DARK_GREEN, GREEN, GREEN, DARK_GREEN, GREEN, DARK_GREEN] * 4,
    [DARK_GREEN, DARK_GREEN, GREEN, GREEN, DARK_GREEN, DARK_GREEN, DARK_GREEN, GREEN]
    * 4,
    [GREEN, DARK_GREEN, GREEN, DARK_GREEN, GREEN, DARK_GREEN, GREEN, DARK_GREEN] * 4,
    [GREEN, GREEN, DARK_GREEN, GREEN, DARK_GREEN, GREEN, DARK_GREEN, GREEN] * 4,
    [DARK_GREEN, GREEN, GREEN, DARK_GREEN, GREEN, GREEN, DARK_GREEN, DARK_GREEN] * 4,
    [
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
        DARK_GREEN,
    ]
    * 4,
]
save_sprite(f"{assets_dir}/grass.png", grass_tile, size=32)

stone_tile = [
    [GRAY] * 32,
    [GRAY, GRAY, DARK_GRAY, DARK_GRAY] * 8,
    [GRAY, DARK_GRAY, GRAY, DARK_GRAY] * 8,
    [DARK_GRAY, DARK_GRAY, GRAY, GRAY] * 8,
    [GRAY] * 32,
    [DARK_GRAY, DARK_GRAY, GRAY, GRAY] * 8,
    [GRAY, DARK_GRAY, GRAY, DARK_GRAY] * 8,
    [GRAY, GRAY, DARK_GRAY, DARK_GRAY] * 8,
] * 4
save_sprite(f"{assets_dir}/stone.png", stone_tile, size=32)

# DIRT TILE - Brown earthy
dirt_tile = [
    [LIGHT_BROWN] * 32,
    [LIGHT_BROWN, BROWN, BROWN, LIGHT_BROWN] * 8,
    [LIGHT_BROWN, BROWN, LIGHT_BROWN, BROWN] * 8,
    [BROWN, BROWN, LIGHT_BROWN, LIGHT_BROWN] * 8,
    [LIGHT_BROWN] * 32,
    [BROWN, BROWN, LIGHT_BROWN, LIGHT_BROWN] * 8,
    [LIGHT_BROWN, BROWN, LIGHT_BROWN, BROWN] * 8,
    [LIGHT_BROWN, BROWN, BROWN, LIGHT_BROWN] * 8,
] * 4
save_sprite(f"{assets_dir}/dirt.png", dirt_tile, size=32)

# WOOD TILE - Planks
wood_tile = [
    [(139, 69, 19, 255)] * 32,
    [(160, 82, 45, 255), (180, 100, 50, 255)] * 16,
    [(180, 100, 50, 255), (160, 82, 45, 255)] * 16,
    [(160, 82, 45, 255), (180, 100, 50, 255)] * 16,
    [(139, 69, 19, 255)] * 32,
    [(180, 100, 50, 255), (160, 82, 45, 255)] * 16,
    [(160, 82, 45, 255), (180, 100, 50, 255)] * 16,
    [(180, 100, 50, 255), (160, 82, 45, 255)] * 16,
] * 4
save_sprite(f"{assets_dir}/wood.png", wood_tile, size=32)

# WATER TILE - Blue waves
water_tile = [
    [DARK_BLUE] * 32,
    [DARK_BLUE, BLUE, BLUE, DARK_BLUE] * 8,
    [DARK_BLUE, BLUE, DARK_BLUE, BLUE] * 8,
    [BLUE, BLUE, DARK_BLUE, DARK_BLUE] * 8,
    [DARK_BLUE] * 32,
    [BLUE, BLUE, DARK_BLUE, DARK_BLUE] * 8,
    [DARK_BLUE, BLUE, DARK_BLUE, BLUE] * 8,
    [DARK_BLUE, BLUE, BLUE, DARK_BLUE] * 8,
] * 4
save_sprite(f"{assets_dir}/water.png", water_tile, size=32)

# SAND TILE - Golden
sand_tile = [
    [YELLOW] * 32,
    [YELLOW, GOLD, GOLD, YELLOW] * 8,
    [YELLOW, GOLD, YELLOW, GOLD] * 8,
    [GOLD, GOLD, YELLOW, YELLOW] * 8,
    [YELLOW] * 32,
    [GOLD, GOLD, YELLOW, YELLOW] * 8,
    [YELLOW, GOLD, YELLOW, GOLD] * 8,
    [YELLOW, GOLD, GOLD, YELLOW] * 8,
] * 4
save_sprite(f"{assets_dir}/sand.png", sand_tile, size=32)

# GRAVEL TILE - Gray with rocks
gravel_color = (169, 169, 169, 255)
gravel_dark = (105, 105, 105, 255)
gravel_tile = [
    [gravel_color] * 32,
    [gravel_color, gravel_dark, gravel_dark, gravel_color] * 8,
    [gravel_color, gravel_dark, gravel_color, gravel_dark] * 8,
    [gravel_dark, gravel_dark, gravel_color, gravel_color] * 8,
    [gravel_color] * 32,
    [gravel_dark, gravel_dark, gravel_color, gravel_color] * 8,
    [gravel_color, gravel_dark, gravel_color, gravel_dark] * 8,
    [gravel_color, gravel_dark, gravel_dark, gravel_color] * 8,
] * 4
save_sprite(f"{assets_dir}/gravel.png", gravel_tile, size=32)

# ROCK/MOUNTAIN TILE - Large dark stone
rock_tile = [
    [BLACK] * 32,
    [BLACK, DARK_GRAY, DARK_GRAY, BLACK] * 8,
    [BLACK, DARK_GRAY, BLACK, DARK_GRAY] * 8,
    [DARK_GRAY, DARK_GRAY, BLACK, BLACK] * 8,
    [BLACK] * 32,
    [DARK_GRAY, DARK_GRAY, BLACK, BLACK] * 8,
    [BLACK, DARK_GRAY, BLACK, DARK_GRAY] * 8,
    [BLACK, DARK_GRAY, DARK_GRAY, BLACK] * 8,
] * 4
save_sprite(f"{assets_dir}/rock.png", rock_tile, size=32)

# WALL TILE - Stone wall
wall_tile = [
    [GRAY] * 32,
    [GRAY, GRAY, GRAY, GRAY] * 8,
    [DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY] * 8,
    [GRAY, GRAY, GRAY, GRAY] * 8,
    [DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY] * 8,
    [GRAY, GRAY, GRAY, GRAY] * 8,
    [DARK_GRAY, DARK_GRAY, DARK_GRAY, DARK_GRAY] * 8,
    [GRAY, GRAY, GRAY, GRAY] * 8,
] * 4
save_sprite(f"{assets_dir}/wall.png", wall_tile, size=32)

# DOOR TILE - Brown wooden door with window
door_tile = [
    [BROWN] * 32,
    [BROWN] * 32,
    [BROWN, BROWN, BROWN, BROWN, GOLD, GOLD, BROWN, BROWN] * 4,
    [BROWN, BROWN, BROWN, BROWN, GOLD, GOLD, BROWN, BROWN] * 4,
    [BROWN, BROWN, BROWN, BROWN, BROWN, BROWN, BROWN, BROWN] * 4,
    [BROWN, BROWN, BROWN, BROWN, BROWN, BROWN, BROWN, BROWN] * 4,
    [BROWN, BROWN, YELLOW, YELLOW, YELLOW, YELLOW, BROWN, BROWN] * 4,
    [BROWN, BROWN, BROWN, BROWN, BROWN, BROWN, BROWN, BROWN] * 4,
]
save_sprite(f"{assets_dir}/door.png", door_tile, size=32)

print("\n✅ All assets created in ./assets/")
print("\nGenerated sprites:")
print("  Classes: warrior.png, mage.png, rogue.png, paladin.png")
print("  Enemies: goblin.png, orc.png, skeleton.png, spider.png")
print(
    "  Tiles: grass.png, dirt.png, stone.png, wood.png, water.png, sand.png, gravel.png, rock.png, wall.png, door.png"
)
