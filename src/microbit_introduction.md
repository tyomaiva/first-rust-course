# What is Micro:bit?

The highlights of Micro:bit V2 [schematics](https://github.com/microbit-foundation/microbit-v2-hardware/blob/main/V2.00/MicroBit_V2.0.0_S_schematic.PDF):
![MCU connections](images/mcu_connections.png)
![LEDs](images/led_matrix.png)

The NRF52833 functional diagram from [the manufacturer](https://infocenter.nordicsemi.com/index.jsp?topic=%2Fstruct_nrf52%2Fstruct%2Fnrf52833.html&cp=3_1):
![MCU](images/NRF52833_diagram.png)
+ The heart of the MCU is an ARM Cortex-M4 CPU core
  + That is where our Rust code executes
  + `thumbv7em-none-eabihf` Rust target basically tells what machine code to emit (e.g., we have 32-bit CPU registers and an FPU for fast floating-point calculations)
