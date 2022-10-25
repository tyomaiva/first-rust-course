# The LED matrix

Now it's time to control some of the board peripherals, we start with the 5x5 LED matrix. The new things that we need compared to the "Hello World" example are
```rust,noplayground
// Add your code here ...
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::Timer,
};
// Add your code here ...
#[entry]
fn main() -> ! {
// Add your code here ...
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
//  Add your code here ...
}
```
+ `board` is a struct with all the peripherals we have, see [here](https://docs.rs/microbit-v2/latest/microbit/board/struct.Board.html)
  + `TIMER0` is one of the 5 hardware timers available on the board
  + `display_pins` are the 10 GPIO pins that control the LED matrix
    + (*) How can 10 pins control 5 * 5 = 25 LEDs?
+ You can use methods `show()` and `clear()` of the struct [`microbit::display::blocking::Display`](https://docs.rs/microbit-v2/latest/microbit/display/blocking/struct.Display.html) to manipulate the LED state.
+ To introduce time delays ("sleeps"), the method `delay_ms()` is available for [`microbit::hal::Timer`](https://docs.rs/microbit-v2/latest/microbit/hal/struct.Timer.html).

> #### Exercise
> - Make the matrix to light a pattern (e.g., all LEDs are on) for 1 second, then switch to another pattern (e.g., all LEDs are off) for another second, and repeat this switching indefinitely.
> - (*) Make a dice: every second, light up one of 6 dice patterns, either in a deterministi fashion (e.g., first you show 1, then 2, and so on) or randomly (you can use the [`hal::Timer::read()`](https://docs.rs/microbit-v2/latest/microbit/hal/struct.Timer.html#method.read) method for generating random numbers).
