from PIL import Image
IMAGE_SIZE = 4096

if __name__ == "__main__":
    print("Starting floor plane generation")
    im = Image.new(mode='RGBA', size=(IMAGE_SIZE,IMAGE_SIZE), color=(0,0,0,0))
    start_x = int(0)
    end_x = int(IMAGE_SIZE)
    start_y = int(0)
    end_y = int(IMAGE_SIZE)
    for x in range(start_x, end_x):
        for y in range(start_y, end_y):
            out_x = (x % 64) / 64
            out_y = (y % 64) / 64
            if out_x < 0.5:
                if out_y < 0.5:
                    im.putpixel((x,y), (255,255,255,255))
                else:
                    im.putpixel((x,y), (0,0,0,255))
            else:
                if out_y < 0.5:
                    im.putpixel((x,y), (0,0,0,255))
                else:
                    im.putpixel((x,y), (255,255,255,255))
    im.save("./skybox_no_walls.png")
    im.save("../assets/texture/skybox_no_walls.png")