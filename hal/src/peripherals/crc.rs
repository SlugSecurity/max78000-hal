//! Cyclic redundancy check (CRC) peripheral API.


use core::mem;
use std::io::SeekFrom;

use max78000::CRC;

pub struct Crc {
    crc: CRC,
}

pub struct CrcReq {
    data_buffer : [u32],
    // no need for data len because array length is known at compile timew
    result_crc :  u32,
}

enum BitOrder {
    LSB, 
    MSB
}

pub unsafe fn set_field(reg : max78000::generic::Reg<T>, mask : u32, value : u32 ){
    reg.modify(|r,w| unsafe {w.bits((r.bits() & !mask) | (value & mask) )})
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
    // use a result, is this mut
    pub unsafe fn crc_shutdown(&self) -> () {
        self.crc.ctrl.modify(|r, w| unsafe {w.bits(r.bits() & 0x1UL << 0)}); 
        
    }

    pub fn crc_get_result(&self) -> R<REG> {
        self.crc.val.read()
    }

    pub fn crc_get_poly(&self) -> R<REG> {
        self.crc.poly.read()
    }


    pub fn crc_get_direction(&self) -> u32{
       self.crc.ctrl.read().bits() & (0x1UL << 2)
        
    }

    pub unsafe fn crc_set_direction(&self, bitorder : BitOrder){

        set_field(self.crc.ctrl, 0x1UL << 2, (BitOrder as u32) << 2);
        
    }
   
    pub unsafe fn crc_set_poly(&self, poly : u32){
        self.crc.poly.write(|w| unsafe {w.bits(poly)});
    }

    pub unsafe fn crc_swap_in(&self, bitorder : BitOrder){
        set_field(self.crc.ctrl, 0x1UL << 3, (BitOrder as u32) << 3);
    }

    pub unsafe fn crc_swap_out(&self, bitorder : BitOrder){
        set_field(self.crc.ctrl, 0x1UL << 4, (BitOrder as u32) << 4);
    }


    pub unsafe fn crc_compute(&self, crc_req: CrcReq){
        let mut i = 0;
        self.crc.ctrl.modify(|r, w| unsafe {r.bits() | 0x1UL } );

        let mut len_counter = crc_req.data_buffer.len();

        while len_counter > 0{
            self.crc.datain32().write(|w| unsafe {crc_req.data_buffer[i]});
            i += 1;
            while self.crc.ctrl.read().bits() & (0x1UL << 16) {};

        }

        crc_req.result_crc = self.crc_get_result();
    }

    
// todo: async, confirm with brian





}


