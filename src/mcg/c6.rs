#[doc = "Reader of register C6"]
pub type R = crate::R<u8, super::C6>;
#[doc = "Writer for register C6"]
pub type W = crate::W<u8, super::C6>;
#[doc = "Register C6 `reset()`'s with value 0"]
impl crate::ResetValue for super::C6 {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Clock Monitor Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CME_A {
    #[doc = "0: External clock monitor is disabled."]
    _0 = 0,
    #[doc = "1: Generate a reset request on loss of external clock."]
    _1 = 1,
}
impl From<CME_A> for bool {
    #[inline(always)]
    fn from(variant: CME_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `CME`"]
pub type CME_R = crate::R<bool, CME_A>;
impl CME_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CME_A {
        match self.bits {
            false => CME_A::_0,
            true => CME_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CME_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CME_A::_1
    }
}
#[doc = "Write proxy for field `CME`"]
pub struct CME_W<'a> {
    w: &'a mut W,
}
impl<'a> CME_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CME_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "External clock monitor is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(CME_A::_0)
    }
    #[doc = "Generate a reset request on loss of external clock."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(CME_A::_1)
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
impl R {
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme(&self) -> CME_R {
        CME_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clock Monitor Enable"]
    #[inline(always)]
    pub fn cme(&mut self) -> CME_W {
        CME_W { w: self }
    }
}
