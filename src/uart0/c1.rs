#[doc = "Reader of register C1"]
pub type R = crate::R<u8, super::C1>;
#[doc = "Writer for register C1"]
pub type W = crate::W<u8, super::C1>;
#[doc = "Register C1 `reset()`'s with value 0"]
impl crate::ResetValue for super::C1 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PT_A {
    #[doc = "0: Even parity."]
    _0 = 0,
    #[doc = "1: Odd parity."]
    _1 = 1,
}
impl From<PT_A> for bool {
    #[inline(always)]
    fn from(variant: PT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `PT`"]
pub type PT_R = crate::R<bool, PT_A>;
impl PT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT_A {
        match self.bits {
            false => PT_A::_0,
            true => PT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PT_A::_1
    }
}
#[doc = "Write proxy for field `PT`"]
pub struct PT_W<'a> {
    w: &'a mut W,
}
impl<'a> PT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Even parity."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PT_A::_0)
    }
    #[doc = "Odd parity."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PT_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PE_A {
    #[doc = "0: No hardware parity generation or checking."]
    _0 = 0,
    #[doc = "1: Parity enabled."]
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
#[doc = "Write proxy for field `PE`"]
pub struct PE_W<'a> {
    w: &'a mut W,
}
impl<'a> PE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No hardware parity generation or checking."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(PE_A::_0)
    }
    #[doc = "Parity enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(PE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u8) & 0x01) << 1);
        self.w
    }
}
#[doc = "Idle Line Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ILT_A {
    #[doc = "0: Idle character bit count starts after start bit."]
    _0 = 0,
    #[doc = "1: Idle character bit count starts after stop bit."]
    _1 = 1,
}
impl From<ILT_A> for bool {
    #[inline(always)]
    fn from(variant: ILT_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ILT`"]
pub type ILT_R = crate::R<bool, ILT_A>;
impl ILT_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILT_A {
        match self.bits {
            false => ILT_A::_0,
            true => ILT_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ILT_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ILT_A::_1
    }
}
#[doc = "Write proxy for field `ILT`"]
pub struct ILT_W<'a> {
    w: &'a mut W,
}
impl<'a> ILT_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ILT_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Idle character bit count starts after start bit."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(ILT_A::_0)
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(ILT_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u8) & 0x01) << 2);
        self.w
    }
}
#[doc = "Receiver Wakeup Method Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WAKE_A {
    #[doc = "0: Idle-line wakeup."]
    _0 = 0,
    #[doc = "1: Address-mark wakeup."]
    _1 = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `WAKE`"]
pub type WAKE_R = crate::R<bool, WAKE_A>;
impl WAKE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::_0,
            true => WAKE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == WAKE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == WAKE_A::_1
    }
}
#[doc = "Write proxy for field `WAKE`"]
pub struct WAKE_W<'a> {
    w: &'a mut W,
}
impl<'a> WAKE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: WAKE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Idle-line wakeup."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(WAKE_A::_0)
    }
    #[doc = "Address-mark wakeup."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(WAKE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u8) & 0x01) << 3);
        self.w
    }
}
#[doc = "9-Bit or 8-Bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum M_A {
    #[doc = "0: Receiver and transmitter use 8-bit data characters."]
    _0 = 0,
    #[doc = "1: Receiver and transmitter use 9-bit data characters."]
    _1 = 1,
}
impl From<M_A> for bool {
    #[inline(always)]
    fn from(variant: M_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `M`"]
pub type M_R = crate::R<bool, M_A>;
impl M_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_A {
        match self.bits {
            false => M_A::_0,
            true => M_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == M_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == M_A::_1
    }
}
#[doc = "Write proxy for field `M`"]
pub struct M_W<'a> {
    w: &'a mut W,
}
impl<'a> M_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: M_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(M_A::_0)
    }
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(M_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u8) & 0x01) << 4);
        self.w
    }
}
#[doc = "Receiver Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RSRC_A {
    #[doc = "0: Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the UART does not use the UART _RX pins."]
    _0 = 0,
    #[doc = "1: Single-wire UART mode where the UART _TX pin is connected to the transmitter output and receiver input."]
    _1 = 1,
}
impl From<RSRC_A> for bool {
    #[inline(always)]
    fn from(variant: RSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `RSRC`"]
pub type RSRC_R = crate::R<bool, RSRC_A>;
impl RSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSRC_A {
        match self.bits {
            false => RSRC_A::_0,
            true => RSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == RSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == RSRC_A::_1
    }
}
#[doc = "Write proxy for field `RSRC`"]
pub struct RSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> RSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: RSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the UART does not use the UART _RX pins."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(RSRC_A::_0)
    }
    #[doc = "Single-wire UART mode where the UART _TX pin is connected to the transmitter output and receiver input."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(RSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u8) & 0x01) << 5);
        self.w
    }
}
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DOZEEN_A {
    #[doc = "0: UART is enabled in Wait mode."]
    _0 = 0,
    #[doc = "1: UART is disabled in Wait mode."]
    _1 = 1,
}
impl From<DOZEEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEEN_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `DOZEEN`"]
pub type DOZEEN_R = crate::R<bool, DOZEEN_A>;
impl DOZEEN_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEEN_A {
        match self.bits {
            false => DOZEEN_A::_0,
            true => DOZEEN_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DOZEEN_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DOZEEN_A::_1
    }
}
#[doc = "Write proxy for field `DOZEEN`"]
pub struct DOZEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DOZEEN_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOZEEN_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART is enabled in Wait mode."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(DOZEEN_A::_0)
    }
    #[doc = "UART is disabled in Wait mode."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(DOZEEN_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u8) & 0x01) << 6);
        self.w
    }
}
#[doc = "Loop Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LOOPS_A {
    #[doc = "0: Normal operation - UART _RX and UART _TX use separate pins."]
    _0 = 0,
    #[doc = "1: Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input. (See RSRC bit.) UART _RX pin is not used by UART ."]
    _1 = 1,
}
impl From<LOOPS_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPS_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `LOOPS`"]
pub type LOOPS_R = crate::R<bool, LOOPS_A>;
impl LOOPS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPS_A {
        match self.bits {
            false => LOOPS_A::_0,
            true => LOOPS_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == LOOPS_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == LOOPS_A::_1
    }
}
#[doc = "Write proxy for field `LOOPS`"]
pub struct LOOPS_W<'a> {
    w: &'a mut W,
}
impl<'a> LOOPS_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: LOOPS_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Normal operation - UART _RX and UART _TX use separate pins."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(LOOPS_A::_0)
    }
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input. (See RSRC bit.) UART _RX pin is not used by UART ."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(LOOPS_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u8) & 0x01) << 7);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&self) -> ILT_R {
        ILT_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&self) -> RSRC_R {
        RSRC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&self) -> DOZEEN_R {
        DOZEEN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&self) -> LOOPS_R {
        LOOPS_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&mut self) -> PT_W {
        PT_W { w: self }
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&mut self) -> PE_W {
        PE_W { w: self }
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&mut self) -> ILT_W {
        ILT_W { w: self }
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&mut self) -> WAKE_W {
        WAKE_W { w: self }
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    pub fn m(&mut self) -> M_W {
        M_W { w: self }
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&mut self) -> RSRC_W {
        RSRC_W { w: self }
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&mut self) -> DOZEEN_W {
        DOZEEN_W { w: self }
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&mut self) -> LOOPS_W {
        LOOPS_W { w: self }
    }
}
