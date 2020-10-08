#[doc = "Reader of register CCR"]
pub type R = crate::R<u32, super::CCR>;
#[doc = "Reader of field `UNALIGN_TRP`"]
pub type UNALIGN_TRP_R = crate::R<bool, bool>;
#[doc = "Reader of field `STKALIGN`"]
pub type STKALIGN_R = crate::R<bool, bool>;
impl R {
    #[doc = "Bit 3 - Always reads as one, indicates that all unaligned accesses generate a HardFault"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
