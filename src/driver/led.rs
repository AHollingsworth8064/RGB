use core::time::Duration;
use std::default;
use gpio::GpioValue::{self, High}; 
use smart_leds::{RGB8};

// 1μs ns high, 0.3μs low
pub const HIGH : PulseCode = PulseCode::new(GpioValue::High, 10, GpioValue::Low, 3);

// 1μs ns low, 0.3μs high
pub const LOW : PulseCode = PulseCode::new(GpioValue::Low, 10, GpioValue::High, 3);

// 800μs low
pub const RESET : PulseCode = PulseCode::new(GpioValue::Low, 400, GpioValue::Low, 400);


#[derive(Debug, Clone, Copy)]
pub struct Led{
    pub rgb : RGB8,
}

impl Led{
    pub fn into_pulse(rgb :RGB8) -> [PulseCode; 24]{
        let r_sequence = color_to_pulse_code(rgb.r);
        let g_sequence = color_to_pulse_code(rgb.g);
        let b_sequence = color_to_pulse_code(rgb.b);

        
        let mut pulse_sequence = [LOW; 24];

        pulse_sequence[0..8].copy_from_slice(&g_sequence);
        pulse_sequence[8..16].copy_from_slice(&r_sequence);
        pulse_sequence[16..24].copy_from_slice(&b_sequence);

        pulse_sequence
    } 

    pub fn color_from_rgb8(&mut self, rgb : RGB8){
        self.rgb = rgb
    }

    pub fn color_from_rgb(&mut self, red : u8, blue : u8, green : u8 ){
        self.rgb = RGB8::new(red, green, blue)
    }
}


fn color_to_pulse_code(color : u8) -> [PulseCode; 8]{
    let mut pulse_code_sequence = [LOW; 8];
    let mut color_clone = color.clone() as u8;

    while color_clone != 0 {
        let smallest_bit = &color_clone & (!&color_clone + 1);  
        // Inverts the bits and adds one. The overlapping bit is saved.
        // eg 1001 -> 0110 + 1 == 0111 & 1001 == 0001
        // eg 1010 -> 0101 + 1 == 0110 * 1010 = 0010
        color_clone ^=  smallest_bit;
        //^= is a xor. This means any overlaps are set to 0 
        // eg 1010 ^= 0110 == 1100
        // eg 1111 ^= 1111 == 0000

        let index = (smallest_bit - 1).count_ones();
        // 0010 -> 0001, thus index is one; 
        pulse_code_sequence[7 - index as usize] = HIGH; 
    }
    pulse_code_sequence
}

#[derive(Copy, Clone, Debug)]
pub struct PulseCode{
    pub level1 : GpioValue, 
    pub length1 : Duration,
    pub level2 : GpioValue,
    pub length2 : Duration
}

impl PulseCode{
    pub const fn new(
        state1 : GpioValue,
        length1 : u64,
        state2 : GpioValue,
        length2 : u64
    ) -> Self{
        Self {
            level1: state1,
            length1: Duration::from_nanos(length1 * 100),
            level2: state2,
            length2: Duration::from_nanos(length2 * 100)}
    }
}



// impl 