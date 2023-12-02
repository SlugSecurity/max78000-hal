//! Cyclic redundancy check (CRC) peripheral API.


use core::mem;

use max78000::CRC;

pub struct Crc {
    crc: CRC,
}


impl Crc {
    
    // new
    // creates a new crc object
    pub fn new(crc: CRC) -> Self {
        Self {crc}
    }


    // crc_init
    /// initialize control and val
    /// this is unsafe because we're writing to regs
    pub unsafe fn crc_init(&self) -> () {

        // TODO : test with clock once its available
        self.crc.ctrl.write_with_zero(f);
        // this is just from the docls
        self.crc.val.write(|w| unsafe {w.bits(0xFFFFFFFF)});

        // 0 ignore?
    }

    // shutdown crc
    // might be unsafe since we're changing register values
    pub unsafe fn crc_shutdown(&self) -> () {
        self.crc.ctrl.write(|w| unsafe {self.crc.ctrl.read(). }); 
        
    }





}

