from gpiozero import LED
from time import sleep

led = LED(26)
leds = LED(16)
while True:
  leds.on()
  sleep(1)
  leds.off()
  sleep(1)
