### Rust code hello-leds

Wiring the board:  The external leds have two wires: the long one is positive, the short one is ground.  The led ground gets wired to the ground "rail" (blue) that ties into a Pico ground pin.  The longer led lead is wired to one of the GPIO pins, using a 220 resistor as the wire.  The code in this repo has the following connections:

 - Red to GPIO 15  (Pico pin 20)
 - Yellow to GPIO 14  (Pico pin 19)
 - Green to GPIO 16  (Pico pin 21)
 

To check and run this code, first pull the branch 'hello-leds' to your machine

 - In your (IDE) terminal, run ```git branch``` --all.  You will see ```origin/hello-leds```
 - Run git ```checkout hello-leds```

To check this code to make sure it compiles:  navigate to the top level of the workdir after cloning the repo (OSU-RPMH) and run the following command:

```cargo build```   (or ```cargo check```)

To run it, you will plug your Pi Pico to the USB connection with your computer, holding the button on the Pi Pico down while plugging it in.  You will then be able to see the Pico as a USB device on your machine.  In your terminal, run:

```bash
cargo run
```

You may find your computer objects to your "unplugging" the usb device without ejecting.  This appears to be normal and unavoidable.  However, on an apple device, close that warning before attempting to download another build.  I think apple prevents a usb from being visible while that warning is on the screen.

![Image of Raspberry Pi Pico board with pin connections](/docs/pico_pinout.jpg)