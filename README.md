rust-robot
==========

Robot controller code in Rust for a two wheel robot driven by a _Arduino Mega 2560_.

_This is a Work in Progress_

# Robot Construction
## Bill of Materials
The robot this code is designed to control is built with the following parts (some links are affiliate links):

| Name | Link | Notes |
|:--|:--|:--|
| YIKESHU 2WD Chassis Kit | [Amazon](https://amzn.to/46byvsH) |  |
| ELEGOO MEGA R3 Board | [Amazon](https://amzn.to/3EFRUWZ) | Any Arduino Mega 2560 variant will do |
| MEGA Sensor Shield V1  | [Amazon](https://amzn.to/45UUbtE) | Not strictly necessary, but it does make wiring much easier |
| L298N Motor Drive Controller Board Module Dual | [Amazon](https://amzn.to/44U4wo0) | There are many makers of this fairly standard board |
| Infrared Slotted Optical Optocoupler Module | [Amazon](https://amzn.to/3ZiMxqd) | Many variations on optocoupler models. This form factor works best with chassis. |
| Multicolored Dupont Wires | [Amazon](https://amzn.to/3sOl9US) | You will use the female to female wires mostly in this build. |
| Right angled DC power connector | [Amazon](https://amzn.to/3Pi11SU) | Will be used to connect the Mega 2560 to power. |
| 6 AA Battery Holder | [Amazon](https://amzn.to/3r5ddyk) | The chassis kit comes with a 4 AA battery holder, but th robot works better with a 9V power supply |
| Big Button Module | [Amazon](https://amzn.to/46gTDOx) | Adding a button or two to the robot will allow interactions with the control software. |
| microSD Module | [Pololu](https://www.pololu.com/product/2587) | Used to collect telemetry while the robot is running autnomously |

Furthermore, You might find the following useful:

| Name | Amazon Link | Notes |
|:--|:--|:--|
| Machine Screws, Bolts, Washers, and Nuts Kit | [Amazon](https://amzn.to/3Pi5QLR) | You will need some hardware to connect parts to the chassis. |
| Brass Spacer Standoff Kit | [Amazon](https://amzn.to/3EDg58H) | Using standoffs to connect various items to the chassis makes construction easier. |
| Ball Caster | [Pololu](https://www.pololu.com/product/2692) | The tail caster that comes with the chassis kit creates turning error. A ball caster works better |

Finally, some soldering is required, so material and equipment required for soldering is required.

## Assembly

# Robot Firmware
The goal of this project is to write the robot firmware in Rust. For this, this project is using `avr-hal` as the hardware abstraction layer for the Arduino Mega 2560.

## Build Instructions
1. Install prerequisites as described in the [`avr-hal` README] (`avr-gcc`, `avr-libc`, `avrdude`, [`ravedude`]).

2. Run `cargo build` to build the firmware.

3. Run `cargo run` to flash the firmware to a connected board.  If `ravedude`
   fails to detect your board, check its documentation at
   <https://crates.io/crates/ravedude>.

4. `ravedude` will open a console session after flashing where you can interact
   with the UART console of your board.

[`avr-hal` README]: https://github.com/Rahix/avr-hal#readme
[`ravedude`]: https://crates.io/crates/ravedude

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
