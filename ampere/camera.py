from picamera import PiCamera
from gpiozero import LED
from time import sleep

led = LED(26)
led.on()

camera = PiCamera()
camera.resolution = (2592,1944)
camera.framerate = 15
camera.annotate_text="Hola champis"
sleep(2)
camera.capture("./imagen.png")
sleep(2)

print("HECHO")
