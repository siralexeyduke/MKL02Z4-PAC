#![doc = "Peripheral access API for MKL02Z4 microcontrollers (generated using svd2rust v0.17.0)\n\nYou can find an overview of the API [here].\n\n[here]: https://docs.rs/svd2rust/0.17.0/svd2rust/#peripheral-api"]
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
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
extern "C" {
    fn FTFA();
    fn LVD_LVW();
    fn I2C0();
    fn I2C1();
    fn SPI0();
    fn UART0();
    fn ADC0();
    fn CMP0();
    fn TPM0();
    fn TPM1();
    fn MCG();
    fn LPTMR0();
    fn PORTA();
    fn PORTB();
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
pub static __INTERRUPTS: [Vector; 32] = [
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: FTFA },
    Vector { _handler: LVD_LVW },
    Vector { _reserved: 0 },
    Vector { _handler: I2C0 },
    Vector { _handler: I2C1 },
    Vector { _handler: SPI0 },
    Vector { _reserved: 0 },
    Vector { _handler: UART0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: ADC0 },
    Vector { _handler: CMP0 },
    Vector { _handler: TPM0 },
    Vector { _handler: TPM1 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector { _handler: MCG },
    Vector { _handler: LPTMR0 },
    Vector { _reserved: 0 },
    Vector { _handler: PORTA },
    Vector { _handler: PORTB },
];
#[doc = r"Enumeration of all the interrupts"]
#[derive(Copy, Clone, Debug)]
#[repr(u8)]
pub enum Interrupt {
    #[doc = "5 - FTFA"]
    FTFA = 5,
    #[doc = "6 - LVD_LVW"]
    LVD_LVW = 6,
    #[doc = "8 - I2C0"]
    I2C0 = 8,
    #[doc = "9 - I2C1"]
    I2C1 = 9,
    #[doc = "10 - SPI0"]
    SPI0 = 10,
    #[doc = "12 - UART0"]
    UART0 = 12,
    #[doc = "15 - ADC0"]
    ADC0 = 15,
    #[doc = "16 - CMP0"]
    CMP0 = 16,
    #[doc = "17 - TPM0"]
    TPM0 = 17,
    #[doc = "18 - TPM1"]
    TPM1 = 18,
    #[doc = "27 - MCG"]
    MCG = 27,
    #[doc = "28 - LPTMR0"]
    LPTMR0 = 28,
    #[doc = "30 - PORTA"]
    PORTA = 30,
    #[doc = "31 - PORTB"]
    PORTB = 31,
}
unsafe impl bare_metal::Nr for Interrupt {
    #[inline(always)]
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
#[doc = "Flash configuration field"]
pub struct FTFA_FLASHCONFIG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA_FLASHCONFIG {}
impl FTFA_FLASHCONFIG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftfa_flash_config::RegisterBlock {
        0x0400 as *const _
    }
}
impl Deref for FTFA_FLASHCONFIG {
    type Target = ftfa_flash_config::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTFA_FLASHCONFIG::ptr() }
    }
}
#[doc = "Flash configuration field"]
pub mod ftfa_flash_config;
#[doc = "Flash Memory Interface"]
pub struct FTFA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FTFA {}
impl FTFA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const ftfa::RegisterBlock {
        0x4002_0000 as *const _
    }
}
impl Deref for FTFA {
    type Target = ftfa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FTFA::ptr() }
    }
}
#[doc = "Flash Memory Interface"]
pub mod ftfa;
#[doc = "Timer/PWM Module"]
pub struct TPM0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM0 {}
impl TPM0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tpm0::RegisterBlock {
        0x4003_8000 as *const _
    }
}
impl Deref for TPM0 {
    type Target = tpm0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TPM0::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm0;
#[doc = "Timer/PWM Module"]
pub struct TPM1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TPM1 {}
impl TPM1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const tpm1::RegisterBlock {
        0x4003_9000 as *const _
    }
}
impl Deref for TPM1 {
    type Target = tpm1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*TPM1::ptr() }
    }
}
#[doc = "Timer/PWM Module"]
pub mod tpm1;
#[doc = "Analog-to-Digital Converter"]
pub struct ADC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ADC0 {}
impl ADC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const adc0::RegisterBlock {
        0x4003_b000 as *const _
    }
}
impl Deref for ADC0 {
    type Target = adc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ADC0::ptr() }
    }
}
#[doc = "Analog-to-Digital Converter"]
pub mod adc0;
#[doc = "Low Power Timer"]
pub struct LPTMR0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for LPTMR0 {}
impl LPTMR0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const lptmr0::RegisterBlock {
        0x4004_0000 as *const _
    }
}
impl Deref for LPTMR0 {
    type Target = lptmr0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*LPTMR0::ptr() }
    }
}
#[doc = "Low Power Timer"]
pub mod lptmr0;
#[doc = "System Integration Module"]
pub struct SIM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SIM {}
impl SIM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sim::RegisterBlock {
        0x4004_7000 as *const _
    }
}
impl Deref for SIM {
    type Target = sim::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SIM::ptr() }
    }
}
#[doc = "System Integration Module"]
pub mod sim;
#[doc = "Pin Control and Interrupts"]
pub struct PORTA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTA {}
impl PORTA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const porta::RegisterBlock {
        0x4004_9000 as *const _
    }
}
impl Deref for PORTA {
    type Target = porta::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTA::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod porta;
#[doc = "Pin Control and Interrupts"]
pub struct PORTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PORTB {}
impl PORTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const portb::RegisterBlock {
        0x4004_a000 as *const _
    }
}
impl Deref for PORTB {
    type Target = portb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PORTB::ptr() }
    }
}
#[doc = "Pin Control and Interrupts"]
pub mod portb;
#[doc = "Multipurpose Clock Generator module"]
pub struct MCG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCG {}
impl MCG {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcg::RegisterBlock {
        0x4006_4000 as *const _
    }
}
impl Deref for MCG {
    type Target = mcg::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCG::ptr() }
    }
}
#[doc = "Multipurpose Clock Generator module"]
pub mod mcg;
#[doc = "Oscillator"]
pub struct OSC0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for OSC0 {}
impl OSC0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const osc0::RegisterBlock {
        0x4006_5000 as *const _
    }
}
impl Deref for OSC0 {
    type Target = osc0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*OSC0::ptr() }
    }
}
#[doc = "Oscillator"]
pub mod osc0;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C0 {}
impl I2C0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c0::RegisterBlock {
        0x4006_6000 as *const _
    }
}
impl Deref for I2C0 {
    type Target = i2c0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C0::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c0;
#[doc = "Inter-Integrated Circuit"]
pub struct I2C1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2C1 {}
impl I2C1 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2c1::RegisterBlock {
        0x4006_7000 as *const _
    }
}
impl Deref for I2C1 {
    type Target = i2c1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*I2C1::ptr() }
    }
}
#[doc = "Inter-Integrated Circuit"]
pub mod i2c1;
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        0x4006_a000 as *const _
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*UART0::ptr() }
    }
}
#[doc = "Universal Asynchronous Receiver/Transmitter"]
pub mod uart0;
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub struct CMP0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for CMP0 {}
impl CMP0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const cmp0::RegisterBlock {
        0x4007_3000 as *const _
    }
}
impl Deref for CMP0 {
    type Target = cmp0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*CMP0::ptr() }
    }
}
#[doc = "High-Speed Comparator (CMP), Voltage Reference (VREF) Digital-to-Analog Converter (DAC), and Analog Mux (ANMUX)"]
pub mod cmp0;
#[doc = "Serial Peripheral Interface"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        0x4007_6000 as *const _
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SPI0::ptr() }
    }
}
#[doc = "Serial Peripheral Interface"]
pub mod spi0;
#[doc = "Power Management Controller"]
pub struct PMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for PMC {}
impl PMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const pmc::RegisterBlock {
        0x4007_d000 as *const _
    }
}
impl Deref for PMC {
    type Target = pmc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*PMC::ptr() }
    }
}
#[doc = "Power Management Controller"]
pub mod pmc;
#[doc = "System Mode Controller"]
pub struct SMC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SMC {}
impl SMC {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const smc::RegisterBlock {
        0x4007_e000 as *const _
    }
}
impl Deref for SMC {
    type Target = smc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SMC::ptr() }
    }
}
#[doc = "System Mode Controller"]
pub mod smc;
#[doc = "Reset Control Module"]
pub struct RCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RCM {}
impl RCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rcm::RegisterBlock {
        0x4007_f000 as *const _
    }
}
impl Deref for RCM {
    type Target = rcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*RCM::ptr() }
    }
}
#[doc = "Reset Control Module"]
pub mod rcm;
#[doc = "General Purpose Input/Output"]
pub struct GPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOA {}
impl GPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpioa::RegisterBlock {
        0x400f_f000 as *const _
    }
}
impl Deref for GPIOA {
    type Target = gpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpioa;
#[doc = "General Purpose Input/Output"]
pub struct GPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIOB {}
impl GPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpiob::RegisterBlock {
        0x400f_f040 as *const _
    }
}
impl Deref for GPIOB {
    type Target = gpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*GPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod gpiob;
#[doc = "System Control Block"]
pub struct SYSTEMCONTROL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTEMCONTROL {}
impl SYSTEMCONTROL {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const system_control::RegisterBlock {
        0xe000_e000 as *const _
    }
}
impl Deref for SYSTEMCONTROL {
    type Target = system_control::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTEMCONTROL::ptr() }
    }
}
#[doc = "System Control Block"]
pub mod system_control;
#[doc = "System timer"]
pub struct SYSTICK {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SYSTICK {}
impl SYSTICK {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const sys_tick::RegisterBlock {
        0xe000_e010 as *const _
    }
}
impl Deref for SYSTICK {
    type Target = sys_tick::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*SYSTICK::ptr() }
    }
}
#[doc = "System timer"]
pub mod sys_tick;
#[doc = "Micro Trace Buffer"]
pub struct MTB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTB {}
impl MTB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mtb::RegisterBlock {
        0xf000_0000 as *const _
    }
}
impl Deref for MTB {
    type Target = mtb::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MTB::ptr() }
    }
}
#[doc = "Micro Trace Buffer"]
pub mod mtb;
#[doc = "MTB data watchpoint and trace"]
pub struct MTBDWT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MTBDWT {}
impl MTBDWT {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mtbdwt::RegisterBlock {
        0xf000_1000 as *const _
    }
}
impl Deref for MTBDWT {
    type Target = mtbdwt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MTBDWT::ptr() }
    }
}
#[doc = "MTB data watchpoint and trace"]
pub mod mtbdwt;
#[doc = "System ROM"]
pub struct ROM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for ROM {}
impl ROM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rom::RegisterBlock {
        0xf000_2000 as *const _
    }
}
impl Deref for ROM {
    type Target = rom::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*ROM::ptr() }
    }
}
#[doc = "System ROM"]
pub mod rom;
#[doc = "Core Platform Miscellaneous Control Module"]
pub struct MCM {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for MCM {}
impl MCM {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const mcm::RegisterBlock {
        0xf000_3000 as *const _
    }
}
impl Deref for MCM {
    type Target = mcm::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*MCM::ptr() }
    }
}
#[doc = "Core Platform Miscellaneous Control Module"]
pub mod mcm;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOA {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOA {}
impl FGPIOA {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpioa::RegisterBlock {
        0xf80f_f000 as *const _
    }
}
impl Deref for FGPIOA {
    type Target = fgpioa::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOA::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpioa;
#[doc = "General Purpose Input/Output"]
pub struct FGPIOB {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for FGPIOB {}
impl FGPIOB {
    #[doc = r"Returns a pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const fgpiob::RegisterBlock {
        0xf80f_f040 as *const _
    }
}
impl Deref for FGPIOB {
    type Target = fgpiob::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*FGPIOB::ptr() }
    }
}
#[doc = "General Purpose Input/Output"]
pub mod fgpiob;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r"All the peripherals"]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "FTFA_FLASHCONFIG"]
    pub FTFA_FLASHCONFIG: FTFA_FLASHCONFIG,
    #[doc = "FTFA"]
    pub FTFA: FTFA,
    #[doc = "TPM0"]
    pub TPM0: TPM0,
    #[doc = "TPM1"]
    pub TPM1: TPM1,
    #[doc = "ADC0"]
    pub ADC0: ADC0,
    #[doc = "LPTMR0"]
    pub LPTMR0: LPTMR0,
    #[doc = "SIM"]
    pub SIM: SIM,
    #[doc = "PORTA"]
    pub PORTA: PORTA,
    #[doc = "PORTB"]
    pub PORTB: PORTB,
    #[doc = "MCG"]
    pub MCG: MCG,
    #[doc = "OSC0"]
    pub OSC0: OSC0,
    #[doc = "I2C0"]
    pub I2C0: I2C0,
    #[doc = "I2C1"]
    pub I2C1: I2C1,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "CMP0"]
    pub CMP0: CMP0,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "PMC"]
    pub PMC: PMC,
    #[doc = "SMC"]
    pub SMC: SMC,
    #[doc = "RCM"]
    pub RCM: RCM,
    #[doc = "GPIOA"]
    pub GPIOA: GPIOA,
    #[doc = "GPIOB"]
    pub GPIOB: GPIOB,
    #[doc = "SYSTEMCONTROL"]
    pub SYSTEMCONTROL: SYSTEMCONTROL,
    #[doc = "SYSTICK"]
    pub SYSTICK: SYSTICK,
    #[doc = "MTB"]
    pub MTB: MTB,
    #[doc = "MTBDWT"]
    pub MTBDWT: MTBDWT,
    #[doc = "ROM"]
    pub ROM: ROM,
    #[doc = "MCM"]
    pub MCM: MCM,
    #[doc = "FGPIOA"]
    pub FGPIOA: FGPIOA,
    #[doc = "FGPIOB"]
    pub FGPIOB: FGPIOB,
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
            FTFA_FLASHCONFIG: FTFA_FLASHCONFIG {
                _marker: PhantomData,
            },
            FTFA: FTFA {
                _marker: PhantomData,
            },
            TPM0: TPM0 {
                _marker: PhantomData,
            },
            TPM1: TPM1 {
                _marker: PhantomData,
            },
            ADC0: ADC0 {
                _marker: PhantomData,
            },
            LPTMR0: LPTMR0 {
                _marker: PhantomData,
            },
            SIM: SIM {
                _marker: PhantomData,
            },
            PORTA: PORTA {
                _marker: PhantomData,
            },
            PORTB: PORTB {
                _marker: PhantomData,
            },
            MCG: MCG {
                _marker: PhantomData,
            },
            OSC0: OSC0 {
                _marker: PhantomData,
            },
            I2C0: I2C0 {
                _marker: PhantomData,
            },
            I2C1: I2C1 {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            CMP0: CMP0 {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            PMC: PMC {
                _marker: PhantomData,
            },
            SMC: SMC {
                _marker: PhantomData,
            },
            RCM: RCM {
                _marker: PhantomData,
            },
            GPIOA: GPIOA {
                _marker: PhantomData,
            },
            GPIOB: GPIOB {
                _marker: PhantomData,
            },
            SYSTEMCONTROL: SYSTEMCONTROL {
                _marker: PhantomData,
            },
            SYSTICK: SYSTICK {
                _marker: PhantomData,
            },
            MTB: MTB {
                _marker: PhantomData,
            },
            MTBDWT: MTBDWT {
                _marker: PhantomData,
            },
            ROM: ROM {
                _marker: PhantomData,
            },
            MCM: MCM {
                _marker: PhantomData,
            },
            FGPIOA: FGPIOA {
                _marker: PhantomData,
            },
            FGPIOB: FGPIOB {
                _marker: PhantomData,
            },
        }
    }
}
