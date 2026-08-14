use std::{io, thread};

use crate::driver::led::{self, Led, PulseCode};
use gpio::{self, GpioOut};
use smart_leds::{RGB8, SmartLedsWriteAsync};
use tokio::time::{self, sleep};

#[derive(Debug)]
pub struct WS2812BStrip<const N: usize >{
    pub leds : [led::Led ; N],
    gpio : gpio::sysfs::SysFsGpioOutput,
    code_buff : [PulseCode; N],
    // gpio_chip : Option<gpio::sysfs::SysFsGpioOutput> 
}

impl <const N : usize> WS2812BStrip<N>{
    pub fn new(pin : u16) -> Self{
        let led_strip: [Led; N] = [Led {rgb : RGB8::new(0, 0, 0)}; N] ;
        
        let mut gpio_pin = match gpio::sysfs::SysFsGpioOutput::open(pin){
            Ok(gpio_pin) => gpio_pin,
            Err(err) => panic!("Eror {:?}", err)
        };

        //test 

        gpio_pin.set_low().expect("could not set gpio");


        Self{
            leds : led_strip,
            gpio : gpio_pin,
            code_buff : [led::LOW; N]

        }

    }
   
    async fn transmit(&mut self, code_buff : [PulseCode; N]) -> Result<(), io::Error>{
        for pulse in code_buff{
            self.gpio.set_value(pulse.level1).expect("could not set gpio");
            time::sleep(pulse.length1).await; 
            self.gpio.set_value(pulse.level2).expect("could not set gpio");
            time::sleep(pulse.length2).await; 
        }
        Ok(())
    } 

    pub fn set_color(&mut self, color :RGB8){
        for led in &mut self.leds{
            led.color_from_rgb8(color);
        }

    }

    pub async fn render(&mut self)  ->  Result<(), io::Error>{
        type Error = io::Error;

        let pulses = self.leds
            .into_iter()
            .map(|color| led::Led::into_pulse(color.rgb))
            .flatten()
            .chain([led::RESET])
            .into_iter();

        let mut index = 0; 

        for pulse in pulses{
            if index >= N{
                return Err(io::Error::new(io::ErrorKind::Other, "Invalid Data Length"));
            }
            self.code_buff[index] = pulse;
            index += 1; 
        }
        self.transmit(self.code_buff).await
    }
}
