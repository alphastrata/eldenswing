import cv2
from PIL import Image

import pytesseract

# files at ../ocr/assets/*.png

img_cv = cv2.imread(r"../ocr/assets/1336.png")

# By default OpenCV stores images in BGR format and since pytesseract assumes RGB format,
# we need to convert from BGR to RGB format/mode:
img_rgb = cv2.cvtColor(img_cv, cv2.COLOR_BGR2RGB)
print(pytesseract.image_to_string(img_rgb))
# OR
print(pytesseract.get_languages(config=""))

img_rgb = Image.frombytes("RGB", img_cv.shape[:2], img_cv, "raw", "BGR", 0, 0)
print(pytesseract.image_to_string(img_rgb))
print(pytesseract.image_to_data(Image.open("test.png")))
