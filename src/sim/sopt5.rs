#[doc = "Reader of register SOPT5"]
pub type R = crate::R<u32, super::SOPT5>;
#[doc = "Writer for register SOPT5"]
pub type W = crate::W<u32, super::SOPT5>;
#[doc = "Register SOPT5 `reset()`'s with value 0"]
impl crate::ResetValue for super::SOPT5 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "UART0 transmit data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0TXSRC_A {
    #[doc = "0: UART0_TX pin"]
    _0 = 0,
    #[doc = "1: UART0_TX pin modulated with TPM1 channel 0 output"]
    _1 = 1,
}
impl From<UART0TXSRC_A> for bool {
    #[inline(always)]
    fn from(variant: UART0TXSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0TXSRC`"]
pub type UART0TXSRC_R = crate::R<bool, UART0TXSRC_A>;
impl UART0TXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0TXSRC_A {
        match self.bits {
            false => UART0TXSRC_A::_0,
            true => UART0TXSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0TXSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0TXSRC_A::_1
    }
}
#[doc = "Write proxy for field `UART0TXSRC`"]
pub struct UART0TXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0TXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0TXSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART0_TX pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_0)
    }
    #[doc = "UART0_TX pin modulated with TPM1 channel 0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0TXSRC_A::_1)
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "UART0 receive data source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0RXSRC_A {
    #[doc = "0: UART0_RX pin"]
    _0 = 0,
    #[doc = "1: CMP0 output"]
    _1 = 1,
}
impl From<UART0RXSRC_A> for bool {
    #[inline(always)]
    fn from(variant: UART0RXSRC_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0RXSRC`"]
pub type UART0RXSRC_R = crate::R<bool, UART0RXSRC_A>;
impl UART0RXSRC_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0RXSRC_A {
        match self.bits {
            false => UART0RXSRC_A::_0,
            true => UART0RXSRC_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0RXSRC_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0RXSRC_A::_1
    }
}
#[doc = "Write proxy for field `UART0RXSRC`"]
pub struct UART0RXSRC_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0RXSRC_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0RXSRC_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "UART0_RX pin"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_0)
    }
    #[doc = "CMP0 output"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0RXSRC_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "UART0 Open Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART0ODE_A {
    #[doc = "0: Open drain is disabled on UART0"]
    _0 = 0,
    #[doc = "1: Open drain is enabled on UART0"]
    _1 = 1,
}
impl From<UART0ODE_A> for bool {
    #[inline(always)]
    fn from(variant: UART0ODE_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `UART0ODE`"]
pub type UART0ODE_R = crate::R<bool, UART0ODE_A>;
impl UART0ODE_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UART0ODE_A {
        match self.bits {
            false => UART0ODE_A::_0,
            true => UART0ODE_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == UART0ODE_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == UART0ODE_A::_1
    }
}
#[doc = "Write proxy for field `UART0ODE`"]
pub struct UART0ODE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART0ODE_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART0ODE_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "Open drain is disabled on UART0"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(UART0ODE_A::_0)
    }
    #[doc = "Open drain is enabled on UART0"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(UART0ODE_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - UART0 transmit data source select"]
    #[inline(always)]
    pub fn uart0txsrc(&self) -> UART0TXSRC_R {
        UART0TXSRC_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - UART0 receive data source select"]
    #[inline(always)]
    pub fn uart0rxsrc(&self) -> UART0RXSRC_R {
        UART0RXSRC_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 16 - UART0 Open Drain Enable"]
    #[inline(always)]
    pub fn uart0ode(&self) -> UART0ODE_R {
        UART0ODE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - UART0 transmit data source select"]
    #[inline(always)]
    pub fn uart0txsrc(&mut self) -> UART0TXSRC_W {
        UART0TXSRC_W { w: self }
    }
    #[doc = "Bit 2 - UART0 receive data source select"]
    #[inline(always)]
    pub fn uart0rxsrc(&mut self) -> UART0RXSRC_W {
        UART0RXSRC_W { w: self }
    }
    #[doc = "Bit 16 - UART0 Open Drain Enable"]
    #[inline(always)]
    pub fn uart0ode(&mut self) -> UART0ODE_W {
        UART0ODE_W { w: self }
    }
}
