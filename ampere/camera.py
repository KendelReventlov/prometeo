from picamera import PiCamera

camera = PiCamera()
camera.resolution = (2592,1944)
camera.framerate = 15
camera.annotate_text="Hola champis"
camara.brightness = 100
camera.capture("./imagen.png")
print("HECHO")
