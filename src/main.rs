use rust_gpiozero::*;
use std::thread;
use std::time::Duration;

fn main() {
    let rotary_btn = Button::new(21);
    let rotary_dt = DigitalInputDevice::new(20);
    let rotary_clk = DigitalInputDevice::new(16);



    let mut counter: i64 = 0;
    let mut current_state_clk: bool;
    let mut last_state_clk = rotary_clk.value();

    loop {
        current_state_clk = rotary_clk.value();

        if current_state_clk != last_state_clk && current_state_clk == false {
            if rotary_dt.value() != current_state_clk {
                counter -= 1;
                println!("Counter: {}", counter);
            } else {
                counter += 1;
                println!("Counter: {}", counter);
            }
        }

        last_state_clk = current_state_clk;

        if rotary_btn.is_active() {
            counter = 0;
            println!("RESET!");
            println!("Counter: {}", counter);
            while rotary_btn.is_active() {}
        }

        // Prevent weird things happening
        thread::sleep(Duration::from_millis(1));
    }
}
