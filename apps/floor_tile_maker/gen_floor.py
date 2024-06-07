from PIL import Image
import random

TILE_SIZE = 40

if __name__ == "__main__":
    print("Starting floor generation")

    im = Image.new(mode='RGB', size=(800,800))

    for x in range(0,800,TILE_SIZE):
        for y in range(0,800, TILE_SIZE):
            r = random.randint(0,9)
            fn = f"tile{r}.png"

            nxt_tile = Image.open(fn)

            for c_x in range(0,TILE_SIZE):
                for c_y in range(0, TILE_SIZE):
                    pix = nxt_tile.getpixel((c_x,c_y))
                    im.putpixel((x+c_x, y+c_y), pix)
    
    im.save("floor_800x800.png")