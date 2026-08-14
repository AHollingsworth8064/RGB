use smart_leds::{RGB, RGB8};
use std::io;
mod driver; 
mod nt;
#[tokio::main]
async fn main() ->  Result<(), io::Error>{
    println!("Hello, world!");

    //the pin offset is 547
    let mut strip = driver::WS2812B_strip::WS2812BStrip::<32>::new(583);
    let color : RGB8 = RGB8::new(255, 255, 255);
    strip.set_color(color);

    loop{
        strip.render();
    }

    Ok(())

}

