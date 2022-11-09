# Putting all together: streaming data from the accelerometer

Micro:bit has a built-in accelerometer that can communicate with the CPU using I\\(^2\\)C protocol, see [this crate](https://crates.io/crates/lsm303agr)

> #### Exercise
> Design an application that reads accelerometer values for X-, Y-, and Z-accelerations every 0.5 second and sends them to your PC using UART.
> + Test it by shaking the board and observing the changes in resulting readings.

Useful code fragments:
```rust,noplayground
// TWIM is the same as I2C master, different name is used for legal reasons
use microbit::hal::twim;
use microbit::twim0::frequency::FREQUENCY_A;

// ...
let board = microbit::Board::take().unwrap();
let mut i2c = twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
let mut sensor = Lsm303agr::new_with_i2c(i2c);
sensor.init().unwrap();
// ...
```
Useful methods:
+ [`set_accel_odr()`](https://docs.rs/lsm303agr/0.2.2/lsm303agr/struct.Lsm303agr.html#method.set_accel_odr)
+ [`accel_status()`](https://docs.rs/lsm303agr/0.2.2/lsm303agr/struct.Lsm303agr.html#method.accel_status)
+ [`accel_data()`](https://docs.rs/lsm303agr/0.2.2/lsm303agr/struct.Lsm303agr.html#method.accel_data)
