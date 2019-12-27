#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Byte pin registers port 0; pins PIO0_0 to PIO0_31"]
    pub b0: [B0; 32],
    #[doc = "0x20 - Byte pin registers port 1"]
    pub b132: B1,
    #[doc = "0x21 - Byte pin registers port 1"]
    pub b133: B1,
    #[doc = "0x22 - Byte pin registers port 1"]
    pub b134: B1,
    #[doc = "0x23 - Byte pin registers port 1"]
    pub b135: B1,
    #[doc = "0x24 - Byte pin registers port 1"]
    pub b136: B1,
    #[doc = "0x25 - Byte pin registers port 1"]
    pub b137: B1,
    #[doc = "0x26 - Byte pin registers port 1"]
    pub b138: B1,
    #[doc = "0x27 - Byte pin registers port 1"]
    pub b139: B1,
    #[doc = "0x28 - Byte pin registers port 1"]
    pub b140: B1,
    #[doc = "0x29 - Byte pin registers port 1"]
    pub b141: B1,
    #[doc = "0x2a - Byte pin registers port 1"]
    pub b142: B1,
    #[doc = "0x2b - Byte pin registers port 1"]
    pub b143: B1,
    #[doc = "0x2c - Byte pin registers port 1"]
    pub b144: B1,
    #[doc = "0x2d - Byte pin registers port 1"]
    pub b145: B1,
    #[doc = "0x2e - Byte pin registers port 1"]
    pub b146: B1,
    #[doc = "0x2f - Byte pin registers port 1"]
    pub b147: B1,
    #[doc = "0x30 - Byte pin registers port 1"]
    pub b148: B1,
    #[doc = "0x31 - Byte pin registers port 1"]
    pub b149: B1,
    #[doc = "0x32 - Byte pin registers port 1"]
    pub b150: B1,
    #[doc = "0x33 - Byte pin registers port 1"]
    pub b151: B1,
    #[doc = "0x34 - Byte pin registers port 1"]
    pub b152: B1,
    #[doc = "0x35 - Byte pin registers port 1"]
    pub b153: B1,
    #[doc = "0x36 - Byte pin registers port 1"]
    pub b154: B1,
    #[doc = "0x37 - Byte pin registers port 1"]
    pub b155: B1,
    #[doc = "0x38 - Byte pin registers port 1"]
    pub b156: B1,
    #[doc = "0x39 - Byte pin registers port 1"]
    pub b157: B1,
    #[doc = "0x3a - Byte pin registers port 1"]
    pub b158: B1,
    #[doc = "0x3b - Byte pin registers port 1"]
    pub b159: B1,
    #[doc = "0x3c - Byte pin registers port 1"]
    pub b160: B1,
    #[doc = "0x3d - Byte pin registers port 1"]
    pub b161: B1,
    #[doc = "0x3e - Byte pin registers port 1"]
    pub b162: B1,
    #[doc = "0x3f - Byte pin registers port 1"]
    pub b163: B1,
    _reserved33: [u8; 4032usize],
    #[doc = "0x1000 - Word pin registers port 0"]
    pub w_0: [W_0; 32],
    #[doc = "0x1080 - Word pin registers port 1"]
    pub w_132: W_1,
    #[doc = "0x1084 - Word pin registers port 1"]
    pub w_133: W_1,
    #[doc = "0x1088 - Word pin registers port 1"]
    pub w_134: W_1,
    #[doc = "0x108c - Word pin registers port 1"]
    pub w_135: W_1,
    #[doc = "0x1090 - Word pin registers port 1"]
    pub w_136: W_1,
    #[doc = "0x1094 - Word pin registers port 1"]
    pub w_137: W_1,
    #[doc = "0x1098 - Word pin registers port 1"]
    pub w_138: W_1,
    #[doc = "0x109c - Word pin registers port 1"]
    pub w_139: W_1,
    #[doc = "0x10a0 - Word pin registers port 1"]
    pub w_140: W_1,
    #[doc = "0x10a4 - Word pin registers port 1"]
    pub w_141: W_1,
    #[doc = "0x10a8 - Word pin registers port 1"]
    pub w_142: W_1,
    #[doc = "0x10ac - Word pin registers port 1"]
    pub w_143: W_1,
    #[doc = "0x10b0 - Word pin registers port 1"]
    pub w_144: W_1,
    #[doc = "0x10b4 - Word pin registers port 1"]
    pub w_145: W_1,
    #[doc = "0x10b8 - Word pin registers port 1"]
    pub w_146: W_1,
    #[doc = "0x10bc - Word pin registers port 1"]
    pub w_147: W_1,
    #[doc = "0x10c0 - Word pin registers port 1"]
    pub w_148: W_1,
    #[doc = "0x10c4 - Word pin registers port 1"]
    pub w_149: W_1,
    #[doc = "0x10c8 - Word pin registers port 1"]
    pub w_150: W_1,
    #[doc = "0x10cc - Word pin registers port 1"]
    pub w_151: W_1,
    #[doc = "0x10d0 - Word pin registers port 1"]
    pub w_152: W_1,
    #[doc = "0x10d4 - Word pin registers port 1"]
    pub w_153: W_1,
    #[doc = "0x10d8 - Word pin registers port 1"]
    pub w_154: W_1,
    #[doc = "0x10dc - Word pin registers port 1"]
    pub w_155: W_1,
    #[doc = "0x10e0 - Word pin registers port 1"]
    pub w_156: W_1,
    #[doc = "0x10e4 - Word pin registers port 1"]
    pub w_157: W_1,
    #[doc = "0x10e8 - Word pin registers port 1"]
    pub w_158: W_1,
    #[doc = "0x10ec - Word pin registers port 1"]
    pub w_159: W_1,
    #[doc = "0x10f0 - Word pin registers port 1"]
    pub w_160: W_1,
    #[doc = "0x10f4 - Word pin registers port 1"]
    pub w_161: W_1,
    #[doc = "0x10f8 - Word pin registers port 1"]
    pub w_162: W_1,
    #[doc = "0x10fc - Word pin registers port 1"]
    pub w_163: W_1,
    _reserved66: [u8; 3840usize],
    #[doc = "0x2000 - Direction registers port 0/1"]
    pub dir: [DIR; 2],
    _reserved67: [u8; 120usize],
    #[doc = "0x2080 - Mask register port 0/1"]
    pub mask: [MASK; 2],
    _reserved68: [u8; 120usize],
    #[doc = "0x2100 - Portpin register port 0"]
    pub pin: [PIN; 2],
    _reserved69: [u8; 120usize],
    #[doc = "0x2180 - Masked port register port 0/1"]
    pub mpin: [MPIN; 2],
    _reserved70: [u8; 120usize],
    #[doc = "0x2200 - Write: Set register for port 0/1 Read: output bits for port 0/1"]
    pub set: [SET; 2],
    _reserved71: [u8; 120usize],
    #[doc = "0x2280 - Clear port 0/1"]
    pub clr: [CLR; 2],
    _reserved72: [u8; 120usize],
    #[doc = "0x2300 - Toggle port 0/1"]
    pub not: [NOT; 2],
}
#[doc = "Byte pin registers port 0; pins PIO0_0 to PIO0_31\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b0](b0) module"]
pub type B0 = crate::Reg<u8, _B0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B0;
#[doc = "`read()` method returns [b0::R](b0::R) reader structure"]
impl crate::Readable for B0 {}
#[doc = "`write(|w| ..)` method takes [b0::W](b0::W) writer structure"]
impl crate::Writable for B0 {}
#[doc = "Byte pin registers port 0; pins PIO0_0 to PIO0_31"]
pub mod b0;
#[doc = "Byte pin registers port 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [b1](b1) module"]
pub type B1 = crate::Reg<u8, _B1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _B1;
#[doc = "`read()` method returns [b1::R](b1::R) reader structure"]
impl crate::Readable for B1 {}
#[doc = "`write(|w| ..)` method takes [b1::W](b1::W) writer structure"]
impl crate::Writable for B1 {}
#[doc = "Byte pin registers port 1"]
pub mod b1;
#[doc = "Word pin registers port 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w_0](w_0) module"]
pub type W_0 = crate::Reg<u32, _W_0>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W_0;
#[doc = "`read()` method returns [w_0::R](w_0::R) reader structure"]
impl crate::Readable for W_0 {}
#[doc = "`write(|w| ..)` method takes [w_0::W](w_0::W) writer structure"]
impl crate::Writable for W_0 {}
#[doc = "Word pin registers port 0"]
pub mod w_0;
#[doc = "Word pin registers port 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w_1](w_1) module"]
pub type W_1 = crate::Reg<u32, _W_1>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _W_1;
#[doc = "`read()` method returns [w_1::R](w_1::R) reader structure"]
impl crate::Readable for W_1 {}
#[doc = "`write(|w| ..)` method takes [w_1::W](w_1::W) writer structure"]
impl crate::Writable for W_1 {}
#[doc = "Word pin registers port 1"]
pub mod w_1;
#[doc = "Direction registers port 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dir](dir) module"]
pub type DIR = crate::Reg<u32, _DIR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _DIR;
#[doc = "`read()` method returns [dir::R](dir::R) reader structure"]
impl crate::Readable for DIR {}
#[doc = "`write(|w| ..)` method takes [dir::W](dir::W) writer structure"]
impl crate::Writable for DIR {}
#[doc = "Direction registers port 0/1"]
pub mod dir;
#[doc = "Mask register port 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](mask) module"]
pub type MASK = crate::Reg<u32, _MASK>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MASK;
#[doc = "`read()` method returns [mask::R](mask::R) reader structure"]
impl crate::Readable for MASK {}
#[doc = "`write(|w| ..)` method takes [mask::W](mask::W) writer structure"]
impl crate::Writable for MASK {}
#[doc = "Mask register port 0/1"]
pub mod mask;
#[doc = "Portpin register port 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pin](pin) module"]
pub type PIN = crate::Reg<u32, _PIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _PIN;
#[doc = "`read()` method returns [pin::R](pin::R) reader structure"]
impl crate::Readable for PIN {}
#[doc = "`write(|w| ..)` method takes [pin::W](pin::W) writer structure"]
impl crate::Writable for PIN {}
#[doc = "Portpin register port 0"]
pub mod pin;
#[doc = "Masked port register port 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mpin](mpin) module"]
pub type MPIN = crate::Reg<u32, _MPIN>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _MPIN;
#[doc = "`read()` method returns [mpin::R](mpin::R) reader structure"]
impl crate::Readable for MPIN {}
#[doc = "`write(|w| ..)` method takes [mpin::W](mpin::W) writer structure"]
impl crate::Writable for MPIN {}
#[doc = "Masked port register port 0/1"]
pub mod mpin;
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [set](set) module"]
pub type SET = crate::Reg<u32, _SET>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SET;
#[doc = "`read()` method returns [set::R](set::R) reader structure"]
impl crate::Readable for SET {}
#[doc = "`write(|w| ..)` method takes [set::W](set::W) writer structure"]
impl crate::Writable for SET {}
#[doc = "Write: Set register for port 0/1 Read: output bits for port 0/1"]
pub mod set;
#[doc = "Clear port 0/1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clr](clr) module"]
pub type CLR = crate::Reg<u32, _CLR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CLR;
#[doc = "`write(|w| ..)` method takes [clr::W](clr::W) writer structure"]
impl crate::Writable for CLR {}
#[doc = "Clear port 0/1"]
pub mod clr;
#[doc = "Toggle port 0/1\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [not](not) module"]
pub type NOT = crate::Reg<u32, _NOT>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _NOT;
#[doc = "`write(|w| ..)` method takes [not::W](not::W) writer structure"]
impl crate::Writable for NOT {}
#[doc = "Toggle port 0/1"]
pub mod not;
