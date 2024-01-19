//! Cyclic redundancy check (CRC) peripheral API.

// use core::mem;

use core::marker::PhantomData;

use az::OverflowingCastFrom; // as suggested by brian

use max78000::CRC;

#[derive(Debug)]
/// CRC struct for CRC regs
struct Crc {
    /// the crc registers.
    _crc: CRC,
}

/// CRC - 8 structure (stores to crc8 register)
#[derive(Debug)]
pub struct CrcDataU8<'a> {
    data: &'a [u8],
}

impl<'a> CrcDataU8<'a> {
    /// New CRC-U8 array
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

/// CRC 16 bit
#[derive(Debug)]
pub struct CrcDataU16<'a> {
    data: &'a [u8],
}

impl<'a> CrcDataU16<'a> {
    /// New CRC-U16 array
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

/// CRC 32 bit
#[derive(Debug)]
pub struct CrcDataU32<'a> {
    data: &'a [u8],
}

impl<'a> CrcDataU32<'a> {
    /// New CRC-U32 array
    pub fn new(data: &'a [u8]) -> Self {
        Self { data }
    }
}

/// CRC Width trait for builder
pub trait CrcWidth {
    /// Output width
    type OutWidth: Sized + OverflowingCastFrom<u32>;

    /// get data from u8?
    fn get_data(&self) -> &[u8];
}

impl<'a> CrcWidth for CrcDataU8<'a> {
    type OutWidth = u8;
    fn get_data(&self) -> &[u8] {
        self.data
    }
}

impl<'a> CrcWidth for CrcDataU16<'a> {
    type OutWidth = u16;
    fn get_data(&self) -> &[u8] {
        self.data
    }
}

impl<'a> CrcWidth for CrcDataU32<'a> {
    type OutWidth = u32;

    fn get_data(&self) -> &[u8] {
        self.data
    }
}
#[derive(Debug)]
/// Crc Calculator struct. Uses a builder-style pattern
pub struct CrcCalculator<Width: CrcWidth> {
    msb: bool,
    byte_swap_in: bool,
    byte_swap_out: bool,
    value: u32,
    poly: u32,
    xorout: u32,
    crc: Crc,

    phantom: PhantomData<Width::OutWidth>, // data: Option<Width::OutWidth>,
}

impl<Width: CrcWidth> CrcCalculator<Width> {
    /// Instantiate a new instance of the CRC Calculator.
    pub fn new(crc_regs: CRC) -> Self {
        Self {
            crc: Crc::new(crc_regs),
            msb: false,
            byte_swap_in: false,
            byte_swap_out: false,
            poly: 0xEDB8_8320,
            value: 0x0,
            xorout: 0xFFFF_FFFF,
            phantom: PhantomData,
        }
    }

    /// Set msb
    pub fn msb(&mut self, msb: bool) -> &mut Self {
        self.msb = msb;
        self
    }

    /// Set input byteorder (inverse of refin)
    pub fn byte_swap_in(&mut self, byte_swap_in: bool) -> &mut Self {
        self.byte_swap_in = byte_swap_in;
        self
    }

    /// Set output byteorder (inverse of refout)
    pub fn byte_swap_out(&mut self, byte_swap_out: bool) -> &mut Self {
        self.byte_swap_out = byte_swap_out;
        self
    }

    /// Set poly byteorder (bitwise inverse of polynomial in standard representation)
    pub fn poly(&mut self, poly: u32) -> &mut Self {
        self.poly = poly;
        self
    }

    /// Set value  register
    pub fn value(&mut self, value: u32) -> &mut Self {
        self.value = value;
        self
    }

    /// Set xorout. Value is xor'd with xorout.
    pub fn xorout(&mut self, xorout: u32) -> &mut Self {
        self.xorout = xorout;
        self
    }

    /// Calculate CRC result for CRC-8, CRC-16 and CRC-32.
    pub fn calc(&self, data: Width) -> Width::OutWidth {
        // shutdown crc
        self.crc._crc.ctrl().write(|w| w.en().bit(false));

        // set crc reg params
        // set msb
        self.crc._crc.ctrl().write(|w| w.msb().bit(self.msb));
        // set swap in
        self.crc
            ._crc
            .ctrl()
            .write(|w| w.byte_swap_in().bit(self.byte_swap_in));
        // set swap   out
        self.crc
            ._crc
            .ctrl()
            .write(|w| w.byte_swap_out().bit(self.byte_swap_out));

        // set poly
        self.crc._crc.poly().write(|w| w.poly().variant(self.poly));
        // set value
        self.crc._crc.val().write(|w| w.value().variant(self.value));

        // re initialize
        self.crc._crc.ctrl().write(|w| w.en().bit(true));

        let crc_data = data.get_data();
        /// get data from here
        // pad to use
        const CHUNK_SIZE: usize = core::mem::size_of::<u32>();
        let chunk_width: usize = core::mem::size_of::<Width::OutWidth>();
        for chunk in crc_data.chunks(chunk_width) {
            // make an array of 4 bytes
            let mut padded_bytes = [0u8; CHUNK_SIZE];

            // input slice length is always smaller than or equal to u32 size
            // so
            let len = chunk.len();

            padded_bytes[..len].copy_from_slice(chunk);

            // step 2: place padded chunk in CRC register
            if self.crc._crc.ctrl().read().busy().bit() == false {
            self.crc
                ._crc
                .datain32()
                .write(|w| w.data().variant(u32::from_ne_bytes(padded_bytes)));
            }

            // do nothing while computation is ongoing
            while !(self.crc._crc.ctrl().read().busy().bit() == false) {}
        }

        // convert to generic type from u32, need num-traits for this
        let (a, _) = Width::OutWidth::overflowing_cast_from(self.crc._crc.val().read().bits() ^ self.xorout);

        a
    }

    pub fn val_reg_bits(&self) -> u32{
        self.crc._crc.val().read().bits()
    }
}

impl Crc {
    /// creates a new crc object
    fn new(_crc: CRC) -> Self {
        Self { _crc }
    }
}
