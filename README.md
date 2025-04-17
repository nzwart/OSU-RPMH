### Rust code hello-leds

Wiring the board:  The external leds have two wires: the long one is positive, the short one is ground.  The led ground gets wired to the ground "rail" (blue) that ties into a Pico ground pin.  The longer led lead is wired to one of the GPIO pins, using a 220 resistor as the wire.  The code in this repo has the following connections:

 - Red to GPIO 15  (Pico pin 20)
 - Yellow to GPIO 14  (Pico pin 19)
 - Green to GPIO 16  (Pico pin 21)
 
 - Ground (negative, blue) (Pico pins: 3, 8, 13, 18, 23, 28, 33, or 38)

To check and run this code, first pull the branch 'hello-leds' to your machine
 - In your (IDE) terminal, run ```git branch --all```.  You will see ```origin/hello-leds```
  - Run ```git checkout hello-leds```
 
  - Make sure you have Rust installed in the directory ```rustc --version```
     - if error appears, initialize Rust with ```curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh``` 
         and then for BASH run ```. “$HOME/.cargo/env"```

To check this code to make sure it compiles:  navigate to the top level of the workdir after cloning the repo (OSU-RPMH) and run the following command:

```cargo build```   (or ```cargo check```)

To run it, you will plug your Pi Pico to the USB connection with your computer, __holding the "boot selector" button on the Pi Pico down while plugging it in__.  You will then be able to see the Pico as a USB device on your machine ```RPI-RP2```.  In your IDE terminal, run:

```bash
cargo run
```

You may find your computer generates an error popup, complaining that you "unplugged" the usb device without ejecting.  This appears to be normal and unavoidable. Once the executable transfers, the Pico ejects itself, reboots and runs the program. Nothing to worry about.  However, on an apple device, do make sure to close that warning before attempting to download another build.  I think apple prevents a usb from being visible while that warning is on the screen.

**One additional note:** 
Once you have loaded a program executable onto the Pico, you can re-run the program any time by plugging it into the computer USB or any USB power source without holding down the "boot selector" button.  The program will run automatically.

![Image of Raspberry Pi Pico board with pin connections](/docs/pico_pinout.jpg)