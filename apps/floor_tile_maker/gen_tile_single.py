from PIL import Image
import random

PIXEL_WIDTH = 4

def create_tile(fn):
    im = Image.new(mode='RGB', size=(40,40))
    for x in range(0,40,PIXEL_WIDTH):
        for y in range(0,40,PIXEL_WIDTH):
            r = random.randint(0,10)
                    
            for i in range(0,PIXEL_WIDTH):
                for j in range(0,PIXEL_WIDTH):
                    if x == 0 or x == 36 or y == 0 or y == 36:
                        im.putpixel((x+i,y+j),(97, 90, 62))
                    elif r == 1:
                        im.putpixel((x+i,y+j),(117, 107, 66))
                    else:
                        im.putpixel((x+i,y+j), (166, 151, 85))
    
    im.save(f"tile{fn}.png")


if __name__ == "__main__":
    print("Starting to generate tile")

    for i in range(0,10):
        create_tile(i)