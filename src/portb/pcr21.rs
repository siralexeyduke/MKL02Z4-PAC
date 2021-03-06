#[doc = "Reader of register PCR21"]
pub type R = crate::R<u32, super::PCR21>;
#[doc = "Writer for register PCR21"]
pub type W = crate::W<u32, super::PCR21>;
#[doc = "Register PCR21 `reset()`'s with value 0"]
impl crate::ResetValue for super::PCR21 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Pull Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: Internal pullup or pulldown resistor is not enabled on the corresponding pin."]
    _0 = 0,
    #[doc = "1: Internal pullup or pulldown resistor is enabled on the corresponding pin, if the pin is configured as a digital input."]
    _1 = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PE`"]
pub type PE_R = crate::R<bool, PE_A>;
impl PE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::_0,
            true => PE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PE_A::_1
    }
}
#[doc = "Passive Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PFE_A {
    #[doc = "0: Passive input filter is disabled on the corresponding pin."]
    _0 = 0,
    #[doc = "1: Passive input filter is enabled on the corresponding pin, if the pin is configured as a digital input. Refer to the device data sheet for filter characteristics."]
    _1 = 1,
}
impl From<PFE_A> for bool {
    #[inline(always)]
    fn from(variant: PFE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PFE`"]
pub type PFE_R = crate::R<bool, PFE_A>;
impl PFE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PFE_A {
        match self.bits {
            false => PFE_A::_0,
            true => PFE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PFE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PFE_A::_1
    }
}
#[doc = "Drive Strength Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DSE_A {
    #[doc = "0: Low drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    _0 = 0,
    #[doc = "1: High drive strength is configured on the corresponding pin, if pin is configured as a digital output."]
    _1 = 1,
}
impl From<DSE_A> for bool {
    #[inline(always)]
    fn from(variant: DSE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DSE`"]
pub type DSE_R = crate::R<bool, DSE_A>;
impl DSE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DSE_A {
        match self.bits {
            false => DSE_A::_0,
            true => DSE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DSE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DSE_A::_1
    }
}
#[doc = "Pin Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum MUX_A {
    #[doc = "0: Pin disabled (analog)."]
    _000 = 0,
    #[doc = "1: Alternative 1 (GPIO)."]
    _001 = 1,
    #[doc = "2: Alternative 2 (chip-specific)."]
    _010 = 2,
    #[doc = "3: Alternative 3 (chip-specific)."]
    _011 = 3,
    #[doc = "4: Alternative 4 (chip-specific)."]
    _100 = 4,
    #[doc = "5: Alternative 5 (chip-specific)."]
    _101 = 5,
    #[doc = "6: Alternative 6 (chip-specific)."]
    _110 = 6,
    #[doc = "7: Alternative 7 (chip-specific)."]
    _111 = 7,
}
impl From<MUX_A> for u8 {
    #[inline(always)]
    fn from(variant: MUX_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `MUX`"]
pub type MUX_R = crate::R<u8, MUX_A>;
impl MUX_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MUX_A {
        match self.bits {
            0 => MUX_A::_000,
            1 => MUX_A::_001,
            2 => MUX_A::_010,
            3 => MUX_A::_011,
            4 => MUX_A::_100,
            5 => MUX_A::_101,
            6 => MUX_A::_110,
            7 => MUX_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_000`"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == MUX_A::_000
    }
    #[doc = "Checks if the value of the field is `_001`"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == MUX_A::_001
    }
    #[doc = "Checks if the value of the field is `_010`"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == MUX_A::_010
    }
    #[doc = "Checks if the value of the field is `_011`"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == MUX_A::_011
    }
    #[doc = "Checks if the value of the field is `_100`"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == MUX_A::_100
    }
    #[doc = "Checks if the value of the field is `_101`"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == MUX_A::_101
    }
    #[doc = "Checks if the value of the field is `_110`"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == MUX_A::_110
    }
    #[doc = "Checks if the value of the field is `_111`"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == MUX_A::_111
    }
}
#[doc = "Write proxy for field `MUX`"]
pub struct MUX_W<'a> {
    w: &'a mut W,
}
impl<'a> MUX_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: MUX_A) -> &'a mut W {
        {
            self.bits(variant.into())
        }
    }
    #[doc = "Pin disabled (analog)."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut W {
        self.variant(MUX_A::_000)
    }
    #[doc = "Alternative 1 (GPIO)."]
    #[inline(always)]
    pub fn _001(self) -> &'a mut W {
        self.variant(MUX_A::_001)
    }
    #[doc = "Alternative 2 (chip-specific)."]
    #[inline(always)]
    pub fn _010(self) -> &'a mut W {
        self.variant(MUX_A::_010)
    }
    #[doc = "Alternative 3 (chip-specific)."]
    #[inline(always)]
    pub fn _011(self) -> &'a mut W {
        self.variant(MUX_A::_011)
    }
    #[doc = "Alternative 4 (chip-specific)."]
    #[inline(always)]
    pub fn _100(self) -> &'a mut W {
        self.variant(MUX_A::_100)
    }
    #[doc = "Alternative 5 (chip-specific)."]
    #[inline(always)]
    pub fn _101(self) -> &'a mut W {
        self.variant(MUX_A::_101)
    }
    #[doc = "Alternative 6 (chip-specific)."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut W {
        self.variant(MUX_A::_110)
    }
    #[doc = "Alternative 7 (chip-specific)."]
    #[inline(always)]
    pub fn _111(self) -> &'a mut W {
        self.variant(MUX_A::_111)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | (((value as u32) & 0x07) << 8);
        self.w
    }
}
#[doc = "Interrupt Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IRQC_A {
    #[doc = "0: Interrupt request disabled."]
    _0000 = 0,
    #[doc = "8: Interrupt when logic zero."]
    _1000 = 8,
    #[doc = "9: Interrupt on rising edge."]
    _1001 = 9,
    #[doc = "10: Interrupt on falling edge."]
    _1010 = 10,
    #[doc = "11: Interrupt on either edge."]
    _1011 = 11,
    #[doc = "12: Interrupt when logic one."]
    _1100 = 12,
}
impl From<IRQC_A> for u8 {
    #[inline(always)]
    fn from(variant: IRQC_A) -> Self {
        variant as _
    }
}
#[doc = "Reader of field `IRQC`"]
pub type IRQC_R = crate::R<u8, IRQC_A>;
impl IRQC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, IRQC_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(IRQC_A::_0000),
            8 => Val(IRQC_A::_1000),
            9 => Val(IRQC_A::_1001),
            10 => Val(IRQC_A::_1010),
            11 => Val(IRQC_A::_1011),
            12 => Val(IRQC_A::_1100),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `_0000`"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == IRQC_A::_0000
    }
    #[doc = "Checks if the value of the field is `_1000`"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == IRQC_A::_1000
    }
    #[doc = "Checks if the value of the field is `_1001`"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == IRQC_A::_1001
    }
    #[doc = "Checks if the value of the field is `_1010`"]
    #[inline(always)]
    pub fn is_1010(&self) -> bool {
        *self == IRQC_A::_1010
    }
    #[doc = "Checks if the value of the field is `_1011`"]
    #[inline(always)]
    pub fn is_1011(&self) -> bool {
        *self == IRQC_A::_1011
    }
    #[doc = "Checks if the value of the field is `_1100`"]
    #[inline(always)]
    pub fn is_1100(&self) -> bool {
        *self == IRQC_A::_1100
    }
}
#[doc = "Write proxy for field `IRQC`"]
pub struct IRQC_W<'a> {
    w: &'a mut W,
}
impl<'a> IRQC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRQC_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Interrupt request disabled."]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut W {
        self.variant(IRQC_A::_0000)
    }
    #[doc = "Interrupt when logic zero."]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut W {
        self.variant(IRQC_A::_1000)
    }
    #[doc = "Interrupt on rising edge."]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut W {
        self.variant(IRQC_A::_1001)
    }
    #[doc = "Interrupt on falling edge."]
    #[inline(always)]
    pub fn _1010(self) -> &'a mut W {
        self.variant(IRQC_A::_1010)
    }
    #[doc = "Interrupt on either edge."]
    #[inline(always)]
    pub fn _1011(self) -> &'a mut W {
        self.variant(IRQC_A::_1011)
    }
    #[doc = "Interrupt when logic one."]
    #[inline(always)]
    pub fn _1100(self) -> &'a mut W {
        self.variant(IRQC_A::_1100)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
#[doc = "Interrupt Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ISF_A {
    #[doc = "0: Configured interrupt is not detected."]
    _0 = 0,
    #[doc = "1: Configured interrupt is detected. The flag remains set until a logic one is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    _1 = 1,
}
impl From<ISF_A> for bool {
    #[inline(always)]
    fn from(variant: ISF_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ISF`"]
pub type ISF_R = crate::R<bool, ISF_A>;
impl ISF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ISF_A {
        match self.bits {
            false => ISF_A::_0,
            true => ISF_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ISF_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ISF_A::_1
    }
}
#[doc = "Write proxy for field `ISF`"]
pub struct ISF_W<'a> {
    w: &'a mut W,
}
impl<'a> ISF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ISF_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Configured interrupt is not detected."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ISF_A::_0)
    }
    #[doc = "Configured interrupt is detected. The flag remains set until a logic one is written to the flag. If the pin is configured for a level sensitive interrupt and the pin remains asserted, then the flag is set again immediately after it is cleared."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ISF_A::_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 24)) | (((value as u32) & 0x01) << 24);
        self.w
    }
}
impl R {
    #[doc = "Bit 1 - Pull Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Passive Filter Enable"]
    #[inline(always)]
    pub fn pfe(&self) -> PFE_R {
        PFE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Drive Strength Enable"]
    #[inline(always)]
    pub fn dse(&self) -> DSE_R {
        DSE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    pub fn mux(&self) -> MUX_R {
        MUX_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    pub fn irqc(&self) -> IRQC_R {
        IRQC_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&self) -> ISF_R {
        ISF_R::new(((self.bits >> 24) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - Pin Mux Control"]
    #[inline(always)]
    pub fn mux(&mut self) -> MUX_W {
        MUX_W { w: self }
    }
    #[doc = "Bits 16:19 - Interrupt Configuration"]
    #[inline(always)]
    pub fn irqc(&mut self) -> IRQC_W {
        IRQC_W { w: self }
    }
    #[doc = "Bit 24 - Interrupt Status Flag"]
    #[inline(always)]
    pub fn isf(&mut self) -> ISF_W {
        ISF_W { w: self }
    }
}
