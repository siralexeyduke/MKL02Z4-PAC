#[doc = "Reader of register SHCSR"]
pub type R = crate::R<u32, super::SHCSR>;
#[doc = "Writer for register SHCSR"]
pub type W = crate::W<u32, super::SHCSR>;
#[doc = "Register SHCSR `reset()`'s with value 0"]
impl crate::ResetValue for super::SHCSR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "no description available\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SVCALLPENDED_A {
    #[doc = "0: exception is not pending"]
    _0 = 0,
    #[doc = "1: exception is pending"]
    _1 = 1,
}
impl From<SVCALLPENDED_A> for bool {
    #[inline(always)]
    fn from(variant: SVCALLPENDED_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `SVCALLPENDED`"]
pub type SVCALLPENDED_R = crate::R<bool, SVCALLPENDED_A>;
impl SVCALLPENDED_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SVCALLPENDED_A {
        match self.bits {
            false => SVCALLPENDED_A::_0,
            true => SVCALLPENDED_A::_1,
        }
    }
    #[doc = "Checks if the value of the field is `_0`"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == SVCALLPENDED_A::_0
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == SVCALLPENDED_A::_1
    }
}
#[doc = "Write proxy for field `SVCALLPENDED`"]
pub struct SVCALLPENDED_W<'a> {
    w: &'a mut W,
}
impl<'a> SVCALLPENDED_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SVCALLPENDED_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "exception is not pending"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::_0)
    }
    #[doc = "exception is pending"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(SVCALLPENDED_A::_1)
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
impl R {
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&self) -> SVCALLPENDED_R {
        SVCALLPENDED_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 15 - no description available"]
    #[inline(always)]
    pub fn svcallpended(&mut self) -> SVCALLPENDED_W {
        SVCALLPENDED_W { w: self }
    }
}
