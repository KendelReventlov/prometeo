from picamera import PiCamera
from gpiozero import LED
from time import sleep

led = LED(26)
led.on()

camera = PiCamera()
camera.resolution = (2592,1944)
camera.framerate = 15
sleep(2)
while True:
  camera.annotate_text="Hola champis"
  camera.capture("./imagen.png")

print("HECHO")
