#!/usr/bin/env python3
"""
Generate pixel art sprites for RustLike — all characters centred on 32x32 canvas.
Requires: pip install pillow
"""
from PIL import Image
import os


def save_sprite(filename, pixel_data, size=32):
    img = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    px = img.load()
    for y, row in enumerate(pixel_data):
        for x, color in enumerate(row):
            if color is not None:
                px[x, y] = color
    img.save(filename)
    print(f"✓  {filename}")


T = None  # transparent

# ── palette ──────────────────────────────────────────────────────────────────
RED = (220, 50, 50, 255)
DRED = (140, 0, 0, 255)
BLUE = (80, 130, 230, 255)
DBLUE = (40, 80, 180, 255)
GREEN = (80, 180, 80, 255)
DGREEN = (40, 110, 40, 255)
GOLD = (210, 170, 40, 255)
WHITE = (240, 240, 240, 255)
SKIN = (240, 190, 150, 255)
DSKIN = (190, 140, 90, 255)
BROWN = (130, 80, 30, 255)
LBROWN = (170, 110, 55, 255)
GRAY = (140, 140, 140, 255)
DGRAY = (70, 70, 70, 255)
BLACK = (10, 10, 10, 255)
YELLOW = (240, 220, 60, 255)
PURPLE = (180, 80, 220, 255)
DPURPLE = (110, 40, 160, 255)


# Helper: build a full 32-wide row with `fill` colour between lo and hi (inclusive)
def r(fill, lo, hi, pre=None, post=None):
    row = [T] * 32
    for i in range(lo, hi + 1):
        row[i] = fill
    if pre:
        for pos, col in pre:
            row[pos] = col
    if post:
        for pos, col in post:
            row[pos] = col
    return row


def solid(fill):
    return [fill] * 32


os.makedirs("assets", exist_ok=True)

# ── WARRIOR  (centred, ~20px wide, 26px tall, rows 3-28) ─────────────────────
# Head rows 3-6, torso 7-18, legs 19-28
W = [
    [T] * 32,  # 0
    [T] * 32,  # 1
    [T] * 32,  # 2
    r(T, 0, 31),  # 3  — blank top margin
    r(SKIN, 13, 18),  # 4  — head
    r(SKIN, 12, 19, [(14, DSKIN), (17, DSKIN)]),  # 5
    r(SKIN, 12, 19),  # 6
    r(SKIN, 13, 18),  # 7  — neck
    r(RED, 11, 20),  # 8  — shoulders
    r(RED, 10, 21, [(10, DRED), (21, DRED)]),  # 9  — upper torso
    r(RED, 10, 21),  # 10
    r(RED, 10, 21, [(10, DRED), (21, DRED)]),  # 11
    r(RED, 10, 21),  # 12
    r(RED, 11, 20),  # 13 — lower torso
    r(RED, 11, 20),  # 14
    r(BROWN, 11, 20),  # 15 — belt
    r(BROWN, 11, 14)
    + [T] * 4
    + r(BROWN, 18, 21)[18:22]
    + [T] * 10,  # 16 — hips (gap in middle — overwritten below)
    [T] * 32,  # placeholder
    r(BROWN, 11, 14),  # 18 — left leg
    r(BROWN, 11, 14),  # 19
    r(BROWN, 11, 14),  # 20
    r(BROWN, 12, 14),  # 21
    r(BROWN, 13, 14),  # 22
    [T] * 32,  # 23 right leg (built below)
    [T] * 32,
    [T] * 32,
    [T] * 32,
    [T] * 32,
    [T] * 32,
    [T] * 32,
    [T] * 32,
    [T] * 32,
]


# Fix row 16 and add right leg
def hips():
    row = [T] * 32
    for i in range(11, 15):
        row[i] = BROWN  # left hip
    for i in range(18, 22):
        row[i] = BROWN  # right hip
    return row


W[16] = hips()
W[17] = hips()
# Right leg
for row_i, (lo, hi) in enumerate([(18, 21), (18, 21), (18, 21), (18, 20), (18, 19)]):
    W[18 + row_i + 5] = [T] * 32  # clear
for ri, (lo, hi) in enumerate([(18, 21), (18, 21), (18, 21), (18, 20), (18, 19)]):
    row = list(W[18 + ri]) if 18 + ri < 32 else [T] * 32
    for i in range(lo, hi + 1):
        if 18 + ri < 32:
            W[18 + ri][i] = BROWN

save_sprite("assets/warrior.png", W)

# ── MAGE  (centred, blue robes) ───────────────────────────────────────────────
M = [[T] * 32 for _ in range(32)]
# head
for r2 in range(4, 8):
    M[r2] = r(SKIN, 13, 18)
M[5] = r(SKIN, 12, 19, [(14, DSKIN), (17, DSKIN)])
# robe body (wider at bottom = triangular)
for r2 in range(8, 22):
    width = 5 + (r2 - 8) // 2
    lo = max(10, 16 - width)
    hi = min(21, 16 + width)
    M[r2] = r(BLUE, lo, hi, [(lo, DBLUE), (hi, DBLUE)])
# feet / hem
for r2 in range(22, 27):
    M[r2] = r(DPURPLE, 12, 19)
save_sprite("assets/mage.png", M)

# ── ROGUE  (centred, green/dark) ──────────────────────────────────────────────
RG = [[T] * 32 for _ in range(32)]
for r2 in range(4, 8):
    RG[r2] = r(SKIN, 13, 18)
RG[5] = r(SKIN, 12, 19, [(14, DSKIN), (17, DSKIN)])
for r2 in range(8, 15):
    RG[r2] = r(DGREEN, 10, 21, [(10, GREEN), (21, GREEN)])
for r2 in range(15, 17):
    RG[r2] = r(BROWN, 10, 21)
RG[16] = hips()
RG[17] = hips()
for r2 in range(18, 28):
    RG[r2] = r(BROWN, 11, 14) if r2 < 28 else [T] * 32
for r2 in range(18, 23):
    for i in range(18, 22):
        RG[r2][i] = BROWN
save_sprite("assets/rogue.png", RG)

# ── PALADIN  (centred, gold/white) ────────────────────────────────────────────
P = [[T] * 32 for _ in range(32)]
for r2 in range(4, 8):
    P[r2] = r(SKIN, 13, 18)
P[5] = r(SKIN, 12, 19, [(14, DSKIN), (17, DSKIN)])
for r2 in range(8, 16):
    P[r2] = r(GOLD, 10, 21, [(10, WHITE), (21, WHITE)])
for r2 in range(16, 18):
    P[r2] = r(WHITE, 10, 21)
P[16] = hips()
P[17] = hips()
for c2 in [11, 12, 13, 14, 18, 19, 20, 21]:
    P[16][c2] = WHITE
    P[17][c2] = WHITE
for r2 in range(18, 28):
    for i in range(11, 15):
        P[r2][i] = WHITE
    for i in range(18, 22):
        P[r2][i] = WHITE
save_sprite("assets/paladin.png", P)

# ── GOBLIN ────────────────────────────────────────────────────────────────────
G = [[T] * 32 for _ in range(32)]
for r2 in range(5, 10):
    G[r2] = r(DGREEN, 12, 19)
G[6] = r(DGREEN, 11, 20, [(12, GREEN), (13, BLACK), (16, BLACK), (17, GREEN)])  # eyes
for r2 in range(10, 18):
    G[r2] = r(DGREEN, 11, 20, [(11, GREEN), (20, GREEN)])
for r2 in range(18, 26):
    for i in range(12, 16):
        G[r2][i] = BROWN
    for i in range(16, 20):
        G[r2][i] = BROWN
save_sprite("assets/goblin.png", G)

# ── ORC ───────────────────────────────────────────────────────────────────────
O = [[T] * 32 for _ in range(32)]
for r2 in range(4, 10):
    O[r2] = r(DRED, 11, 20)
O[6] = r(DRED, 10, 21, [(12, RED), (13, BLACK), (17, BLACK), (18, RED)])
for r2 in range(10, 20):
    O[r2] = r(DRED, 9, 22, [(9, RED), (22, RED)])
for r2 in range(20, 28):
    for i in range(10, 15):
        O[r2][i] = BROWN
    for i in range(17, 22):
        O[r2][i] = BROWN
save_sprite("assets/orc.png", O)

# ── SKELETON ──────────────────────────────────────────────────────────────────
SK = [[T] * 32 for _ in range(32)]
for r2 in range(4, 9):
    SK[r2] = r(WHITE, 12, 19)
SK[6] = r(
    WHITE, 11, 20, [(13, BLACK), (14, BLACK), (17, BLACK), (18, BLACK)]
)  # eye sockets
for r2 in range(9, 20):
    SK[r2] = r(WHITE, 12, 19, [(12, GRAY), (19, GRAY)])
for r2 in range(20, 28):
    for i in range(12, 16):
        SK[r2][i] = GRAY
    for i in range(16, 20):
        SK[r2][i] = GRAY
save_sprite("assets/skeleton.png", SK)

# ── SPIDER ────────────────────────────────────────────────────────────────────
SP = [[T] * 32 for _ in range(32)]
for r2 in range(10, 22):
    SP[r2] = r(BLACK, 8, 23)
SP[12] = r(BLACK, 6, 25, [(10, RED), (11, RED), (20, RED), (21, RED)])  # eyes
for r2 in [8, 9, 22, 23]:
    for i in range(6, 10):
        SP[r2][i] = BLACK
    for i in range(22, 26):
        SP[r2][i] = BLACK
for r2 in [6, 7, 24, 25]:
    for i in range(4, 8):
        SP[r2][i] = BLACK
    for i in range(24, 28):
        SP[r2][i] = BLACK
save_sprite("assets/spider.png", SP)

# ── TILES (unchanged — these already fill the canvas) ────────────────────────
DGRAY2 = (64, 64, 64, 255)
GRAY2 = (128, 128, 128, 255)
LBROWN2 = (180, 120, 70, 255)
BROWN2 = (139, 90, 43, 255)
DBLUE2 = (50, 100, 200, 255)
BLUE2 = (100, 150, 255, 255)
DGREEN2 = (50, 120, 50, 255)
GREEN2 = (100, 200, 100, 255)
YELLOW2 = (255, 255, 100, 255)
GOLD2 = (200, 180, 50, 255)


def tile4(a, b):
    row1 = [a, a, b, b] * 8
    row2 = [a, b, a, b] * 8
    row3 = [b, b, a, a] * 8
    base = [[a] * 32, row1, row2, row3, [a] * 32, row3, row2, row1]
    result = []
    for _ in range(4):
        result.extend(base)
    return result[:32]


save_sprite("assets/grass.png", tile4(DGREEN2, GREEN2))
save_sprite("assets/stone.png", tile4(GRAY2, DGRAY2))
save_sprite("assets/dirt.png", tile4(LBROWN2, BROWN2))
save_sprite("assets/wood.png", tile4((160, 82, 45, 255), (180, 100, 50, 255)))
save_sprite("assets/water.png", tile4(DBLUE2, BLUE2))
save_sprite("assets/sand.png", tile4(YELLOW2, GOLD2))
save_sprite("assets/gravel.png", tile4((169, 169, 169, 255), (105, 105, 105, 255)))
save_sprite("assets/rock.png", tile4(BLACK, DGRAY2))
save_sprite("assets/wall.png", tile4(GRAY2, DGRAY2))
save_sprite("assets/door.png", tile4(BROWN2, LBROWN2))

print("\n✅  All assets written to ./assets/")
