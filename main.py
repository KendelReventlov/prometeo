from gpiozero import LED
import RPi.GPIO as GPIO
import time
from urllib import request
from urllib import parse

mensaje = request.urlopen("https://prometeoia.herokuapp.com/"+parse.urlencode({"nombre":"RASPBERRY"}))
print("Se envio: ")
print("https://prometeoia.herokuapp.com/?"+parse.urlencode({"nombre":"RASPBERRY"}))
print(mensaje.read().decode('utf-8'))

led = LED(26)
leds = LED(16)

#define the pin that goes to the circuit
pin_to_circuit = 7

def rc_time (pin_to_circuit):
    count = 0
  
    #Output on the pin for 
    GPIO.setup(pin_to_circuit, GPIO.OUT)
    GPIO.output(pin_to_circuit, GPIO.LOW)
    time.sleep(0.1)

    #Change the pin back to input
    GPIO.setup(pin_to_circuit, GPIO.IN)
  
    #Count until the pin goes high
    while (GPIO.input(pin_to_circuit) == GPIO.LOW):
        count += 1
        if count <= 20:
            led.on()
        elif count > 20:
            led.off()

    return count

#Catch when script is interupted, cleanup correctly
try:
    # Main loop
    while True:
        rc_time(pin_to_circuit)
except KeyboardInterrupt:
    pass
finally:
    GPIO.cleanup()
