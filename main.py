from gpiozero import LED
import RPi.GPIO as GPIO
from time import sleep

led = LED(26)
leds = LED(16)

GPIO.setmode(GPIO.BCM)
GPIO.setup(7,GPIO.IN)
while True:
  print(GPIO.input(7))
