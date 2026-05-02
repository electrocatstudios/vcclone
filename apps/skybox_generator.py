from PIL import Image


IMAGE_SIZE = 4096
LIGHT_SANDSTONE = (212, 167, 74)
DARK_SANDSTONE = (84, 67, 34)
DOOR_WIDTH = 256
DOOR_HEIGHT = 256

if __name__ == "__main__":
    print("Starting skybox generation")

    im = Image.new(mode='RGB', size=(IMAGE_SIZE,IMAGE_SIZE))
    # square 1
    start_x = 0
    end_x = int(IMAGE_SIZE / 4)
    start_y = int(IMAGE_SIZE / 4)
    end_y = int(IMAGE_SIZE / 2)
    # flipped = False
    for x in range(start_x, end_x):
        for y in range(start_y, end_y):
            out_x = (x % 64) / 64
            out_y = (y % 64) / 64
            flipped = (x % 128) >= 64
            if not flipped and out_y < 0.1:
                im.putpixel((x,y), DARK_SANDSTONE)
            elif flipped and out_y > 0.4 and out_y < 0.5:
                im.putpixel((x,y), DARK_SANDSTONE)
            else:
                if out_x < 0.1:
                    im.putpixel((x,y), DARK_SANDSTONE)
                else:
                    im.putpixel((x,y), LIGHT_SANDSTONE)

    # square 2
    start_x = int(IMAGE_SIZE / 4)
    end_x = int(IMAGE_SIZE / 2)
    start_y = int(IMAGE_SIZE / 4)
    end_y = int(IMAGE_SIZE / 2)
    for x in range(start_x, end_x):
        for y in range(start_y, end_y):
            im.putpixel((x,y), (0,255,0))

    # square 3 - Backwall
    start_x = int(IMAGE_SIZE / 4)
    end_x = int(IMAGE_SIZE / 2)
    start_y = 0
    end_y = int(IMAGE_SIZE / 4)
    section_width = IMAGE_SIZE / 4
    door_x_start = (section_width - DOOR_WIDTH) / 2
    door_x_end = door_x_start + DOOR_WIDTH
    door_y_start = 0
    door_y_end = DOOR_HEIGHT
    print(f"Door coordinates: x: {door_x_start} to {door_x_end}, y: {door_y_start} to {door_y_end}")

    for x in range(start_x, end_x):
        for y in range(start_y, end_y):
            if x - (IMAGE_SIZE / 4) >= door_x_start and x - (IMAGE_SIZE / 4) < door_x_end and y > door_y_start and y < door_y_end:
                im.putpixel((x,y), (25,25,25))
                continue
            out_x = (x % 64) / 64
            out_y = (y % 64) / 64
            flipped = (y % 128) >= 64
            if not flipped and out_x < 0.1:
                im.putpixel((x,y), DARK_SANDSTONE)
            elif flipped and out_x > 0.4 and out_x < 0.5:
                im.putpixel((x,y), DARK_SANDSTONE)
            else:
                if out_y < 0.1:
                    im.putpixel((x,y), DARK_SANDSTONE)
                else:
                    im.putpixel((x,y), LIGHT_SANDSTONE)

    # square 4
    start_x = int(IMAGE_SIZE / 4)
    end_x = int(IMAGE_SIZE / 2)
    start_y = int(IMAGE_SIZE / 2)
    end_y = int(IMAGE_SIZE * 0.75)
    for x in range(start_x, end_x):
        for y in range(start_y, end_y):
            im.putpixel((x,y), (255,255,0))

    # square 5
    start_x = int(IMAGE_SIZE * 0.75)
    end_x = int(IMAGE_SIZE)
    start_y = int(IMAGE_SIZE / 4)
    end_y = int(IMAGE_SIZE / 2)
    for x in range(start_x, end_x):
        for y in range(start_y, end_y):
            out_x = (x % 64) / 64
            out_y = (y % 64) / 64
            if out_x < 0.5:
                if out_y < 0.5:
                    im.putpixel((x,y), (255,255,255))
                else:
                    im.putpixel((x,y), (0,0,0))
            else:
                if out_y < 0.5:
                    im.putpixel((x,y), (0,0,0))
                else:
                    im.putpixel((x,y), (255,255,255))
                    
                
            # im.putpixel((x,y), (0,255,255))


    # square 6
    start_x = int(IMAGE_SIZE / 2)
    end_x = int(IMAGE_SIZE * 0.75)
    start_y = int(IMAGE_SIZE / 4)
    end_y = int(IMAGE_SIZE / 2)
    for x in range(start_x, end_x):
        for y in range(start_y, end_y):
            out_x = (x % 64) / 64
            out_y = (y % 64) / 64
            flipped = (x % 128) >= 64
            if not flipped and out_y < 0.1:
                im.putpixel((x,y), DARK_SANDSTONE)
            elif flipped and out_y > 0.4 and out_y < 0.5:
                im.putpixel((x,y), DARK_SANDSTONE)
            else:
                if out_x < 0.1:
                    im.putpixel((x,y), DARK_SANDSTONE)
                else:
                    im.putpixel((x,y), LIGHT_SANDSTONE)

    im.save("skybox_4096x4096.png")
    im.save("../assets/texture/skybox.png")