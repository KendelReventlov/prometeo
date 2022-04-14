from gpiozero import LED
from time import sleep
import cv2

led = LED(26)
led.on()

video_objeto = cv2.VideoCapture("./video.mp4")
count = 0
exito = 1

while exito:
  exito,imagen = video_objeto.read()
  cv2.imwrite("frame%d.jpg" % count,imagen)
  count+=1
  print(imagen)

print("HECHO")
