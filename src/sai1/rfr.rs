#[doc = "Register `RFR[%s]` reader"]
pub struct R(crate::R<RFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFP` reader - Read FIFO Pointer"]
pub type RFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RCP` reader - Receive Channel Pointer"]
pub type RCP_R = crate::BitReader<RCP_A>;
#[doc = "Receive Channel Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCP_A {
    #[doc = "0: No effect."]
    DISABLE = 0,
    #[doc = "1: FIFO combine is enabled for FIFO reads and this FIFO will be read on the next FIFO read."]
    ENABLE = 1,
}
impl From<RCP_A> for bool {
    #[inline(always)]
    fn from(variant: RCP_A) -> Self {
        variant as u8 != 0
    }
}
impl RCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCP_A {
        match self.bits {
            false => RCP_A::DISABLE,
            true => RCP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == RCP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == RCP_A::ENABLE
    }
}
#[doc = "Field `WFP` reader - Write FIFO Pointer"]
pub type WFP_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:5 - Read FIFO Pointer"]
    #[inline(always)]
    pub fn rfp(&self) -> RFP_R {
        RFP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 15 - Receive Channel Pointer"]
    #[inline(always)]
    pub fn rcp(&self) -> RCP_R {
        RCP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:21 - Write FIFO Pointer"]
    #[inline(always)]
    pub fn wfp(&self) -> WFP_R {
        WFP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
#[doc = "Receive FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfr](index.html) module"]
pub struct RFR_SPEC;
impl crate::RegisterSpec for RFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfr::R](R) reader structure"]
impl crate::Readable for RFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RFR[%s]
to value 0"]
impl crate::Resettable for RFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
