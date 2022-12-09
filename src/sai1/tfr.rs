#[doc = "Register `TFR[%s]` reader"]
pub struct R(crate::R<TFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RFP` reader - Read FIFO Pointer"]
pub type RFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WFP` reader - Write FIFO Pointer"]
pub type WFP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WCP` reader - Write Channel Pointer"]
pub type WCP_R = crate::BitReader<WCP_A>;
#[doc = "Write Channel Pointer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WCP_A {
    #[doc = "0: No effect."]
    DISABLE = 0,
    #[doc = "1: FIFO combine is enabled for FIFO writes and this FIFO will be written on the next FIFO write."]
    ENABLE = 1,
}
impl From<WCP_A> for bool {
    #[inline(always)]
    fn from(variant: WCP_A) -> Self {
        variant as u8 != 0
    }
}
impl WCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WCP_A {
        match self.bits {
            false => WCP_A::DISABLE,
            true => WCP_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WCP_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WCP_A::ENABLE
    }
}
impl R {
    #[doc = "Bits 0:5 - Read FIFO Pointer"]
    #[inline(always)]
    pub fn rfp(&self) -> RFP_R {
        RFP_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Write FIFO Pointer"]
    #[inline(always)]
    pub fn wfp(&self) -> WFP_R {
        WFP_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 31 - Write Channel Pointer"]
    #[inline(always)]
    pub fn wcp(&self) -> WCP_R {
        WCP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Transmit FIFO\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfr](index.html) module"]
pub struct TFR_SPEC;
impl crate::RegisterSpec for TFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfr::R](R) reader structure"]
impl crate::Readable for TFR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TFR[%s]
to value 0"]
impl crate::Resettable for TFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
