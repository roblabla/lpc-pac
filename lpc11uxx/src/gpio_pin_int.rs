#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - Pin Interrupt Mode register"]
    pub isel: ISEL,
    #[doc = "0x04 - Pin Interrupt Enable (Rising) register"]
    pub ienr: IENR,
    #[doc = "0x08 - Set Pin Interrupt Enable (Rising) register"]
    pub sienr: SIENR,
    #[doc = "0x0c - Clear Pin Interrupt Enable (Rising) register"]
    pub cienr: CIENR,
    #[doc = "0x10 - Pin Interrupt Enable Falling Edge / Active Level register"]
    pub ienf: IENF,
    #[doc = "0x14 - Set Pin Interrupt Enable Falling Edge / Active Level register"]
    pub sienf: SIENF,
    #[doc = "0x18 - Clear Pin Interrupt Enable Falling Edge / Active Level address"]
    pub cienf: CIENF,
    #[doc = "0x1c - Pin Interrupt Rising Edge register"]
    pub rise: RISE,
    #[doc = "0x20 - Pin Interrupt Falling Edge register"]
    pub fall: FALL,
    #[doc = "0x24 - Pin Interrupt Status register"]
    pub ist: IST,
}
#[doc = "Pin Interrupt Mode register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isel](isel) module"]
pub type ISEL = crate::Reg<u32, _ISEL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _ISEL;
#[doc = "`read()` method returns [isel::R](isel::R) reader structure"]
impl crate::Readable for ISEL {}
#[doc = "`write(|w| ..)` method takes [isel::W](isel::W) writer structure"]
impl crate::Writable for ISEL {}
#[doc = "Pin Interrupt Mode register"]
pub mod isel;
#[doc = "Pin Interrupt Enable (Rising) register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienr](ienr) module"]
pub type IENR = crate::Reg<u32, _IENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IENR;
#[doc = "`read()` method returns [ienr::R](ienr::R) reader structure"]
impl crate::Readable for IENR {}
#[doc = "`write(|w| ..)` method takes [ienr::W](ienr::W) writer structure"]
impl crate::Writable for IENR {}
#[doc = "Pin Interrupt Enable (Rising) register"]
pub mod ienr;
#[doc = "Set Pin Interrupt Enable (Rising) register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sienr](sienr) module"]
pub type SIENR = crate::Reg<u32, _SIENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIENR;
#[doc = "`write(|w| ..)` method takes [sienr::W](sienr::W) writer structure"]
impl crate::Writable for SIENR {}
#[doc = "Set Pin Interrupt Enable (Rising) register"]
pub mod sienr;
#[doc = "Clear Pin Interrupt Enable (Rising) register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cienr](cienr) module"]
pub type CIENR = crate::Reg<u32, _CIENR>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIENR;
#[doc = "`write(|w| ..)` method takes [cienr::W](cienr::W) writer structure"]
impl crate::Writable for CIENR {}
#[doc = "Clear Pin Interrupt Enable (Rising) register"]
pub mod cienr;
#[doc = "Pin Interrupt Enable Falling Edge / Active Level register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ienf](ienf) module"]
pub type IENF = crate::Reg<u32, _IENF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IENF;
#[doc = "`read()` method returns [ienf::R](ienf::R) reader structure"]
impl crate::Readable for IENF {}
#[doc = "`write(|w| ..)` method takes [ienf::W](ienf::W) writer structure"]
impl crate::Writable for IENF {}
#[doc = "Pin Interrupt Enable Falling Edge / Active Level register"]
pub mod ienf;
#[doc = "Set Pin Interrupt Enable Falling Edge / Active Level register\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sienf](sienf) module"]
pub type SIENF = crate::Reg<u32, _SIENF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _SIENF;
#[doc = "`write(|w| ..)` method takes [sienf::W](sienf::W) writer structure"]
impl crate::Writable for SIENF {}
#[doc = "Set Pin Interrupt Enable Falling Edge / Active Level register"]
pub mod sienf;
#[doc = "Clear Pin Interrupt Enable Falling Edge / Active Level address\n\nThis register you can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cienf](cienf) module"]
pub type CIENF = crate::Reg<u32, _CIENF>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _CIENF;
#[doc = "`write(|w| ..)` method takes [cienf::W](cienf::W) writer structure"]
impl crate::Writable for CIENF {}
#[doc = "Clear Pin Interrupt Enable Falling Edge / Active Level address"]
pub mod cienf;
#[doc = "Pin Interrupt Rising Edge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rise](rise) module"]
pub type RISE = crate::Reg<u32, _RISE>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _RISE;
#[doc = "`read()` method returns [rise::R](rise::R) reader structure"]
impl crate::Readable for RISE {}
#[doc = "`write(|w| ..)` method takes [rise::W](rise::W) writer structure"]
impl crate::Writable for RISE {}
#[doc = "Pin Interrupt Rising Edge register"]
pub mod rise;
#[doc = "Pin Interrupt Falling Edge register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [fall](fall) module"]
pub type FALL = crate::Reg<u32, _FALL>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _FALL;
#[doc = "`read()` method returns [fall::R](fall::R) reader structure"]
impl crate::Readable for FALL {}
#[doc = "`write(|w| ..)` method takes [fall::W](fall::W) writer structure"]
impl crate::Writable for FALL {}
#[doc = "Pin Interrupt Falling Edge register"]
pub mod fall;
#[doc = "Pin Interrupt Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ist](ist) module"]
pub type IST = crate::Reg<u32, _IST>;
#[allow(missing_docs)]
#[doc(hidden)]
pub struct _IST;
#[doc = "`read()` method returns [ist::R](ist::R) reader structure"]
impl crate::Readable for IST {}
#[doc = "`write(|w| ..)` method takes [ist::W](ist::W) writer structure"]
impl crate::Writable for IST {}
#[doc = "Pin Interrupt Status register"]
pub mod ist;
