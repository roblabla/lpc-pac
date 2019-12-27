#![doc = "Peripheral access API for LPC176X5X microcontrollers (generated using svd2rust v0.16.1)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.16.1/svd2rust/#peripheral-api"]
#![deny(const_err)]
#![deny(dead_code)]
#![deny(improper_ctypes)]
#![deny(legacy_directory_ownership)]
#![deny(missing_docs)]
#![deny(no_mangle_generic_items)]
#![deny(non_shorthand_field_patterns)]
#![deny(overflowing_literals)]
#![deny(path_statements)]
#![deny(patterns_in_fns_without_body)]
#![deny(plugin_as_library)]
#![deny(private_in_public)]
#![deny(safe_extern_statics)]
#![deny(unconditional_recursion)]
#![deny(unions_with_drop_fields)]
#![deny(unused_allocation)]
#![deny(unused_comparisons)]
#![deny(unused_parens)]
#![deny(while_true)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]
extern crate bare_metal;
extern crate cortex_m;
#[cfg(feature = "rt")]
extern crate cortex_m_rt;
extern crate vcell;
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 5;
#[cfg(feature = "rt")]
extern "C" {
    fn WDT();
    fn TIMER0();
    fn TIMER1();
    fn TIMER2();
    fn TIMER3();
    fn UART0();
    fn UART1();
    fn UART2();
    fn UART3();
    fn PWM1();
    fn I2C0();
    fn I2C1();
    fn I2C2();
    fn SPI();
    fn SSP0();
    fn SSP1();
    fn PLL0();
    fn RTC();
    fn EINT0();
    fn EINT1();
    fn EINT2();
    fn EINT3();
    fn ADC();
    fn BOD();
    fn USB();
    fn CAN();
    fn DMA();
    fn I2S();
    fn ENET();
    fn RIT();
    fn MCPWM();
    fn QEI();
    fn PLL1();
    fn USBACTIVITY();
    fn CANACTIVITY();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 35] = [
    Vector { _handler: WDT },
    Vector { _handler: TIMER0 },
    Vector { _handler: TIMER1 },
    Vector { _handler: TIMER2 },
    Vector { _handler: TIMER3 },
    Vector { _handler: UART0 },
    Vector { _handler: UART1 },
    Vector { _handler: UART2 },
    Vector { _handler: UART3 },
    Vector { _handler: PWM1 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: I2C2 },
    Vector { _handler: SPI },
    Vector { _handler: SSP0 },
    Vector { _handler: SSP1 },
    Vector { _handler: PLL0 },
    Vector { _handler: RTC },
    Vector { _handler: EINT0 },
    Vector { _handler: EINT1 },
    Vector { _handler: EINT2 },
    Vector { _handler: EINT3 },
    Vector { _handler: ADC },
    Vector { _handler: BOD },
    Vector { _handler: USB },
    Vector { _handler: CAN },
    Vector { _handler: DMA },
    Vector { _handler: I2S },
    Vector { _handler: ENET },
    Vector { _handler: RIT },
    Vector { _handler: MCPWM },
    Vector { _handler: QEI },
    Vector { _handler: PLL1 },
    Vector {
        _handler: USBACTIVITY,
    },
    Vector {
        _handler: CANACTIVITY,
    },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
pub enum Interrupt {
    #[doc = "0 - WDT"]
    WDT = 0,
    #[doc = "1 - TIMER0"]
    TIMER0 = 1,
    #[doc = "2 - TIMER1"]
    TIMER1 = 2,
    #[doc = "3 - TIMER2"]
    TIMER2 = 3,
    #[doc = "4 - TIMER3"]
    TIMER3 = 4,
    #[doc = "5 - UART0"]
    UART0 = 5,
    #[doc = "6 - UART1"]
    UART1 = 6,
    #[doc = "7 - UART2"]
    UART2 = 7,
    #[doc = "8 - UART3"]
    UART3 = 8,
    #[doc = "9 - PWM1"]
    PWM1 = 9,
    #[doc = "10 - I2C0"]
    I2C0 = 10,
    #[doc = "11 - I2C1"]
    I2C1 = 11,
    #[doc = "12 - I2C2"]
    I2C2 = 12,
    #[doc = "13 - SPI"]
    SPI = 13,
    #[doc = "14 - SSP0"]
    SSP0 = 14,
    #[doc = "15 - SSP1"]
    SSP1 = 15,
    #[doc = "16 - PLL0"]
    PLL0 = 16,
    #[doc = "17 - RTC"]
    RTC = 17,
    #[doc = "18 - EINT0"]
    EINT0 = 18,
    #[doc = "19 - EINT1"]
    EINT1 = 19,
    #[doc = "20 - EINT2"]
    EINT2 = 20,
    #[doc = "21 - EINT3"]
    EINT3 = 21,
    #[doc = "22 - ADC"]
    ADC = 22,
    #[doc = "23 - BOD"]
    BOD = 23,
    #[doc = "24 - USB"]
    USB = 24,
    #[doc = "25 - CAN"]
    CAN = 25,
    #[doc = "26 - DMA"]
    DMA = 26,
    #[doc = "27 - I2S"]
    I2S = 27,
    #[doc = "28 - ENET"]
    ENET = 28,
    #[doc = "29 - RIT"]
    RIT = 29,
    #[doc = "30 - MCPWM"]
    MCPWM = 30,
    #[doc = "31 - QEI"]
    QEI = 31,
    #[doc = "32 - PLL1"]
    PLL1 = 32,
    #[doc = "33 - USBActivity"]
    USBACTIVITY = 33,
    #[doc = "34 - CANActivity"]
    CANACTIVITY = 34,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline]
    fn nr(&self) -> u8 {
        *self as u8
    }
}
#[cfg(feature = "rt")]
pub use self::Interrupt as interrupt;
pub use cortex_m::peripheral::Peripherals as CorePeripherals;
pub use cortex_m::peripheral::{CBP, CPUID, DCB, DWT, FPB, ITM, MPU, NVIC, SCB, SYST, TPIU};
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[doc = "Watchdog Timer (WDT)"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        0x4000_0000 as *const _
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*WDT::ptr() }
    }
}
#[doc = "Watchdog Timer (WDT)"]
pub mod wdt;
#[doc = "Timer0/1/2/3"]
pub struct TIMER0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER0 {}
impl TIMER0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4000_4000 as *const _
    }
}
impl Deref for TIMER0 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER0::ptr() }
    }
}
#[doc = "Timer0/1/2/3"]
pub mod timer0;
#[doc = "Timer0/1/2/3"]
pub struct TIMER1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER1 {}
impl TIMER1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4000_8000 as *const _
    }
}
impl Deref for TIMER1 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER1::ptr() }
    }
}
#[doc = "UART0/2/3"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4000_c000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "UART0/2/3"]
pub mod uart0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        0x4001_0000 as *const _
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART1::ptr() }
    }
}
#[doc = "UART1"]
pub mod uart1;
#[doc = "Pulse Width Modulators (PWM1)"]
pub struct PWM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PWM1 {}
impl PWM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pwm1::RegisterBlock {
        0x4001_8000 as *const _
    }
}
impl Deref for PWM1 {
    type Target = pwm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PWM1::ptr() }
    }
}
#[doc = "Pulse Width Modulators (PWM1)"]
pub mod pwm1;
#[doc = "I2C bus interface"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4001_c000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "I2C bus interface"]
pub mod i2c0;
#[doc = "SPI"]
pub struct SPI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI {}
impl SPI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for SPI {
    type Target = spi::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI::ptr() }
    }
}
#[doc = "SPI"]
pub mod spi;
#[doc = "Real Time Clock (RTC)"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        0x4002_4000 as *const _
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RTC::ptr() }
    }
}
#[doc = "Real Time Clock (RTC)"]
pub mod rtc;
#[doc = "GPIO"]
pub struct GPIOINT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOINT {}
impl GPIOINT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioint::RegisterBlock {
        0x4002_8080 as *const _
    }
}
impl Deref for GPIOINT {
    type Target = gpioint::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOINT::ptr() }
    }
}
#[doc = "GPIO"]
pub mod gpioint;
#[doc = "Pin connect block"]
pub struct PINCONNECT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PINCONNECT {}
impl PINCONNECT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pinconnect::RegisterBlock {
        0x4002_c000 as *const _
    }
}
impl Deref for PINCONNECT {
    type Target = pinconnect::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PINCONNECT::ptr() }
    }
}
#[doc = "Pin connect block"]
pub mod pinconnect;
#[doc = "SSP1 controller"]
pub struct SSP1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP1 {}
impl SSP1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssp1::RegisterBlock {
        0x4003_0000 as *const _
    }
}
impl Deref for SSP1 {
    type Target = ssp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSP1::ptr() }
    }
}
#[doc = "SSP1 controller"]
pub mod ssp1;
#[doc = "Analog-to-Digital Converter (ADC)"]
pub struct ADC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC {}
impl ADC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc::RegisterBlock {
        0x4003_4000 as *const _
    }
}
impl Deref for ADC {
    type Target = adc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter (ADC)"]
pub mod adc;
#[doc = "CAN acceptance filter RAM"]
pub struct CANAFRAM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CANAFRAM {}
impl CANAFRAM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const canafram::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for CANAFRAM {
    type Target = canafram::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CANAFRAM::ptr() }
    }
}
#[doc = "CAN acceptance filter RAM"]
pub mod canafram;
#[doc = "CAN controller acceptance filter"]
pub struct CANAF {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CANAF {}
impl CANAF {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const canaf::RegisterBlock {
        0x4003_c000 as *const _
    }
}
impl Deref for CANAF {
    type Target = canaf::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CANAF::ptr() }
    }
}
#[doc = "CAN controller acceptance filter"]
pub mod canaf;
#[doc = "Central CAN controller"]
pub struct CCAN {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CCAN {}
impl CCAN {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ccan::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for CCAN {
    type Target = ccan::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CCAN::ptr() }
    }
}
#[doc = "Central CAN controller"]
pub mod ccan;
#[doc = "CAN1 controller"]
pub struct CAN1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN1 {}
impl CAN1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x4004_4000 as *const _
    }
}
impl Deref for CAN1 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN1::ptr() }
    }
}
#[doc = "CAN1 controller"]
pub mod can1;
#[doc = "CAN1 controller"]
pub struct CAN2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CAN2 {}
impl CAN2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const can1::RegisterBlock {
        0x4004_8000 as *const _
    }
}
impl Deref for CAN2 {
    type Target = can1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CAN2::ptr() }
    }
}
#[doc = "I2C bus interface"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4005_c000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "SSP controller"]
pub struct SSP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SSP0 {}
impl SSP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ssp1::RegisterBlock {
        0x4008_8000 as *const _
    }
}
impl Deref for SSP0 {
    type Target = ssp1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SSP0::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter (DAC)"]
pub struct DAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DAC {}
impl DAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dac::RegisterBlock {
        0x4008_c000 as *const _
    }
}
impl Deref for DAC {
    type Target = dac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*DAC::ptr() }
    }
}
#[doc = "Digital-to-Analog Converter (DAC)"]
pub mod dac;
#[doc = "Timer0/1/2/3"]
pub struct TIMER2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER2 {}
impl TIMER2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4009_0000 as *const _
    }
}
impl Deref for TIMER2 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER2::ptr() }
    }
}
#[doc = "Timer0/1/2/3"]
pub struct TIMER3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER3 {}
impl TIMER3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer0::RegisterBlock {
        0x4009_4000 as *const _
    }
}
impl Deref for TIMER3 {
    type Target = timer0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TIMER3::ptr() }
    }
}
#[doc = "UART0/2/3"]
pub struct UART2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART2 {}
impl UART2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4009_8000 as *const _
    }
}
impl Deref for UART2 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART2::ptr() }
    }
}
#[doc = "UART0/2/3"]
pub struct UART3 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART3 {}
impl UART3 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4009_c000 as *const _
    }
}
impl Deref for UART3 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART3::ptr() }
    }
}
#[doc = "I2C bus interface"]
pub struct I2C2 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C2 {}
impl I2C2 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x400a_0000 as *const _
    }
}
impl Deref for I2C2 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C2::ptr() }
    }
}
#[doc = "I2S interface"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        0x400a_8000 as *const _
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2S::ptr() }
    }
}
#[doc = "I2S interface"]
pub mod i2s;
#[doc = "Repetitive Interrupt Timer (RIT)"]
pub struct RITIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RITIMER {}
impl RITIMER {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ritimer::RegisterBlock {
        0x400b_0000 as *const _
    }
}
impl Deref for RITIMER {
    type Target = ritimer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RITIMER::ptr() }
    }
}
#[doc = "Repetitive Interrupt Timer (RIT)"]
pub mod ritimer;
#[doc = "Motor Control PWM"]
pub struct MCPWM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCPWM {}
impl MCPWM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcpwm::RegisterBlock {
        0x400b_8000 as *const _
    }
}
impl Deref for MCPWM {
    type Target = mcpwm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCPWM::ptr() }
    }
}
#[doc = "Motor Control PWM"]
pub mod mcpwm;
#[doc = "Quadrature Encoder Interface (QEI)"]
pub struct QEI {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for QEI {}
impl QEI {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const qei::RegisterBlock {
        0x400b_c000 as *const _
    }
}
impl Deref for QEI {
    type Target = qei::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*QEI::ptr() }
    }
}
#[doc = "Quadrature Encoder Interface (QEI)"]
pub mod qei;
#[doc = "System and clock control"]
pub struct SYSCON {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSCON {}
impl SYSCON {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const syscon::RegisterBlock {
        0x400f_c000 as *const _
    }
}
impl Deref for SYSCON {
    type Target = syscon::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSCON::ptr() }
    }
}
#[doc = "System and clock control"]
pub mod syscon;
#[doc = "Ethernet"]
pub struct EMAC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EMAC {}
impl EMAC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const emac::RegisterBlock {
        0x5000_0000 as *const _
    }
}
impl Deref for EMAC {
    type Target = emac::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*EMAC::ptr() }
    }
}
#[doc = "Ethernet"]
pub mod emac;
#[doc = "General purpose DMA controller"]
pub struct GPDMA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPDMA {}
impl GPDMA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpdma::RegisterBlock {
        0x5000_4000 as *const _
    }
}
impl Deref for GPDMA {
    type Target = gpdma::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPDMA::ptr() }
    }
}
#[doc = "General purpose DMA controller"]
pub mod gpdma;
#[doc = "USB device/host/OTG controller"]
pub struct USB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for USB {}
impl USB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const usb::RegisterBlock {
        0x5000_c000 as *const _
    }
}
impl Deref for USB {
    type Target = usb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*USB::ptr() }
    }
}
#[doc = "USB device/host/OTG controller"]
pub mod usb;
#[doc = "General Purpose I/O"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        0x2009_c000 as *const _
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIO::ptr() }
    }
}
#[doc = "General Purpose I/O"]
pub mod gpio;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "TIMER0"]
    pub TIMER0: TIMER0,
    #[doc = "TIMER1"]
    pub TIMER1: TIMER1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "PWM1"]
    pub PWM1: PWM1,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "SPI"]
    pub SPI: SPI,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "GPIOINT"]
    pub GPIOINT: GPIOINT,
    #[doc = "PINCONNECT"]
    pub PINCONNECT: PINCONNECT,
    #[doc = "SSP1"]
    pub SSP1: SSP1,
    #[doc = "ADC"]
    pub ADC: ADC,
    #[doc = "CANAFRAM"]
    pub CANAFRAM: CANAFRAM,
    #[doc = "CANAF"]
    pub CANAF: CANAF,
    #[doc = "CCAN"]
    pub CCAN: CCAN,
    #[doc = "CAN1"]
    pub CAN1: CAN1,
    #[doc = "CAN2"]
    pub CAN2: CAN2,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "SSP0"]
    pub SSP0: SSP0,
    #[doc = "DAC"]
    pub DAC: DAC,
    #[doc = "TIMER2"]
    pub TIMER2: TIMER2,
    #[doc = "TIMER3"]
    pub TIMER3: TIMER3,
    #[doc = "UART2"]
    pub UART2: UART2,
    #[doc = "UART3"]
    pub UART3: UART3,
    #[doc = "I2C2"]
    pub I2C2: I2C2,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "RITIMER"]
    pub RITIMER: RITIMER,
    #[doc = "MCPWM"]
    pub MCPWM: MCPWM,
    #[doc = "QEI"]
    pub QEI: QEI,
    #[doc = "SYSCON"]
    pub SYSCON: SYSCON,
    #[doc = "EMAC"]
    pub EMAC: EMAC,
    #[doc = "GPDMA"]
    pub GPDMA: GPDMA,
    #[doc = "USB"]
    pub USB: USB,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
}
impl Peripherals {
    #[doc = r"Returns all the peripherals *once*"]
    #[inline]
    pub fn take() -> Option<Self> {
        cortex_m::interrupt::free(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                None
            } else {
                Some(unsafe { Peripherals::steal() })
            }
        })
    }
    #[doc = r"Unchecked version of `Peripherals::take`"]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            WDT: WDT {
                _marker: PhantomData,
            },
            TIMER0: TIMER0 {
                _marker: PhantomData,
            },
            TIMER1: TIMER1 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            PWM1: PWM1 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            SPI: SPI {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            GPIOINT: GPIOINT {
                _marker: PhantomData,
            },
            PINCONNECT: PINCONNECT {
                _marker: PhantomData,
            },
            SSP1: SSP1 {
                _marker: PhantomData,
            },
            ADC: ADC {
                _marker: PhantomData,
            },
            CANAFRAM: CANAFRAM {
                _marker: PhantomData,
            },
            CANAF: CANAF {
                _marker: PhantomData,
            },
            CCAN: CCAN {
                _marker: PhantomData,
            },
            CAN1: CAN1 {
                _marker: PhantomData,
            },
            CAN2: CAN2 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            SSP0: SSP0 {
                _marker: PhantomData,
            },
            DAC: DAC {
                _marker: PhantomData,
            },
            TIMER2: TIMER2 {
                _marker: PhantomData,
            },
            TIMER3: TIMER3 {
                _marker: PhantomData,
            },
            UART2: UART2 {
                _marker: PhantomData,
            },
            UART3: UART3 {
                _marker: PhantomData,
            },
            I2C2: I2C2 {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            RITIMER: RITIMER {
                _marker: PhantomData,
            },
            MCPWM: MCPWM {
                _marker: PhantomData,
            },
            QEI: QEI {
                _marker: PhantomData,
            },
            SYSCON: SYSCON {
                _marker: PhantomData,
            },
            EMAC: EMAC {
                _marker: PhantomData,
            },
            GPDMA: GPDMA {
                _marker: PhantomData,
            },
            USB: USB {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
        }
    }
}
