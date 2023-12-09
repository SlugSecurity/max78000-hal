//! Cyclic redundancy check (CRC) peripheral API.

// use core::mem;


use max78000::CRC;
pub struct Crc {
    crc: CRC,
}


// TODO is this right?
pub struct CrcReq<'a> {
    data_buffer: &'a [u32],
    // no need for data len because array length is known at compile timew
    result_crc: u32,
}

enum BitOrder {
    LSB = 0,
    MSB = 1,
}

// TODO : REEVAL DESIGN

pub fn set_field<T>(reg: max78000::generic::Reg<T>, mask: u32, value: u32) {
    reg.modify(|r, w| unsafe { w.bits((r.bits() & !mask) | (value & mask)) })
}

impl Crc {
    // new
    // creates a new crc object
    pub fn new(crc: CRC) -> Self {
        Self { crc }
    }

    // crc_init
    /// initialize control and val
    /// this is unsafe because we're writing to regs
    pub fn crc_init(&self) {
               
        self.crc.ctrl.write(|w| { w.en().bit(true) });
        
    }

    // shutdown crc
    
    pub fn crc_shutdown(&self) {
        self.crc
            .ctrl
            .write(|w| w.en().bit(false))
    }

    pub fn crc_result(&self) -> u32 {
        self.crc.val.read().value()
    }

    pub fn crc_get_poly(&self) -> u32 {
        self.crc.poly.read().value()
    }

    pub fn crc_get_direction(&self) -> u32 {
        self.crc.ctrl.read().msb()
    }

    pub fn crc_set_direction(&self, bitorder: BitOrder) {
        self.crc.ctrl.write(|w| w.msb().bit(bitorder as bool))
    }

    pub fn crc_set_poly(&self, poly: u32) {
        // # SAFETY
        // This requires an unsafe block, to the best of my knowledge because there does not seem to exist a way to write to fields of the
        // register bit-by-bit, unless the HAL poly spec is modified
        self.crc.poly.write(|w| w.poly().variant(poly));
        
    }

    pub fn crc_swap_in(&self, bitorder: BitOrder) {
        self.crc.ctrl.write(|w| w.byte_swap_in().bit(bitorder as bool));
        
    }
    
    pub fn crc_swap_out(&self, bitorder: BitOrder) {
        self.crc.ctrl.write(|w| w.byte_swap_out().bit(bitorder as bool));
 
    }

    pub fn crc_compute(&self, crc_buffer : [u32], result : u32, ) {
       
        // always disable the crc peripheral 
        self.crc_shutdown();
        
        // configure input and output data format fields
        // TODO: follow the instructions in the manual
        

       for i in 0..crc_buffer.len(){
        let j = i as u32;
        self.crc.datain32().write(|w| w.data().variant(crc_buffer[j]));
       }
    }

    // todo: async, confirm with brian
}
