from picamera import PiCamera
from gpiozero import LED

led = LED(26)
led.on()

camera = PiCamera()
camera.resolution = (2592,1944)
camera.framerate = 15
camera.annotate_text="Hola champis"
camera.capture("./imagen.png")

print("HECHO")
