from PIL import Image
import random

IMAGE_SIZE = 1024 #4096
NUMBER_STARS = 100
WHITE = (255,255,255,255)

if __name__ == "__main__":
    print("Starting skybox generation")

    im = Image.new(mode='RGBA', size=(IMAGE_SIZE,IMAGE_SIZE), color=(0,0,0,255))

    for i in range(NUMBER_STARS):
        x = random.randint(0, IMAGE_SIZE - 1)
        y = random.randint(0, IMAGE_SIZE - 1)
        size = random.randint(3, 5)
        for dx in range(-size, size + 1):
            for dy in range(-size, size + 1):
                if 0 <= x + dx < IMAGE_SIZE and 0 <= y + dy < IMAGE_SIZE:
                    distance = (dx ** 2 + dy ** 2) ** 0.5
                    if distance <= size:
                        brightness = int(255 * (1 - distance / size))
                        im.putpixel((x + dx, y + dy), WHITE)
        # x = int(IMAGE_SIZE * 0.25) + int((IMAGE_SIZE * 0.5) * (i / NUMBER_STARS))
        # y = int(IMAGE_SIZE * 0.25) + int((IMAGE_SIZE * 0.5) * (i / NUMBER_STARS))
        # im.putpixel((x,y), WHITE)

    im.save("starry_sky.png")    