from picamera import PiCamera
import RPI.GPIO as gpio

gpio.setmode(gpio.BOARD)
gpio.setup(26,gpio.OUT)
gpio.output(26,True)

camera = PiCamera()
camera.resolution = (2592,1944)
camera.framerate = 15
camera.annotate_text="Hola champis"
camera.capture("./imagen.png")
print("HECHO")
