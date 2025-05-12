### Rust code

#### General board wiring
One or more of the four blue side rail sections (two per side) can be connected to any ground pins on the Pico (3, 8, 13, 23, 33, or 38)

#### Wiring the DHT20

 Facing the dht20's "grated" side with "ASAIR" right above the pins:
 Starting at the far left:
  - pin 1: power, connect to PICO pin 36(3v) or 40(5v) directly, using red rail
  - pin 2: SDA (serial data), connect to GPIO 18 (Pico pin 24)
  - pin 3: Ground, connect to ground using blue ground side rail
  - pin 4: SCL (clock), connect to GPIO 19 (Pico pin 25)

  ![Image of DHT20 humidity sensor](/docs/dht20_pins.jpg)

_The LED section below can be deleted if not using led bulbs_

#### Wiring LEDs
The external leds have two wires: the long one is positive, the short one is ground.  The led ground gets wired to the ground "rail" (blue) that ties into a Pico ground pin.  The longer led lead is wired to one of the GPIO pins, using a 220 resistor as the wire.  The code in this repo has the following connections:

 - Red to GPIO 15  (Pico pin 20)
 - Yellow to GPIO 14  (Pico pin 19)
 - Green to GPIO 16  (Pico pin 21)
 - Yellow2 to GPIO 13 (Pico pin 17)
 - Red2 to GPIO 12 (Pico pin 16)

 - Ground to a grounded blue side rail, or to one of: 
   - Pico pins: 3, 8, 13, 18, 23, 28, 33, or 38

_The pins should be side by side in the order shown above (red, yellow, green, yellow2, red2)_

### To Run This Code (from the branch)

To check and run this code, first pull the branch 'emb-hal-i2c-dht20' to your machine
 - In your (IDE) terminal, run ```git branch --all```.  You will see ```origin/emb-hal-i2c-dht20```
 - Run ```git checkout emb-hal-i2c-dht20```

 - Make sure you have Rust installed in the directory ```rustc --version```
    - if error appears, initialize Rust with ```curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh``` 
        and then for BASH run ```. “$HOME/.cargo/env"```

- Set the target for the RPP ```rustup target add thumbv6m-none-eabi```
- Install cargo crate package dependencies ```cargo install elf2uf2-rs```

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

### Testing

This project has several testing scripts that can be used to verify the correct functioning of each of the project components both individually and collectively. To run the testing scripts, connect the Raspberry Pi Pico using the connection process described above, and execute `cargo run --bin \[script name without file extension\]`. For example, to run the test script for the led array, execute `cargo run --bin led_test`. The expected functionality for each test script is described in the script file (found in the `bin` directory).

Currently available test scripts:
- `led_test.rs`
- `sensor_test.rs`
