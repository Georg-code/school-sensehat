use sensehat::{Colour, SenseHat};
if let Ok(mut hat) = SenseHat::new() {
    println!("{:?}", hat.get_pressure());
    hat.text("Hi!", Colour::RED, Colour::WHITE).unwrap();
}