#[doc = "Register `TXFILLTUNING` reader"]
pub struct R(crate::R<TXFILLTUNING_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TXFILLTUNING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TXFILLTUNING_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TXFILLTUNING_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TXFILLTUNING` writer"]
pub struct W(crate::W<TXFILLTUNING_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TXFILLTUNING_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<TXFILLTUNING_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TXFILLTUNING_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXSCHOH` reader - Scheduler Overhead"]
pub type TXSCHOH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSCHOH` writer - Scheduler Overhead"]
pub type TXSCHOH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TXFILLTUNING_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXSCHHEALTH` reader - Scheduler Health Counter"]
pub type TXSCHHEALTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXSCHHEALTH` writer - Scheduler Health Counter"]
pub type TXSCHHEALTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TXFILLTUNING_SPEC, u8, u8, 5, O>;
#[doc = "Field `TXFIFOTHRES` reader - FIFO Burst Threshold"]
pub type TXFIFOTHRES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXFIFOTHRES` writer - FIFO Burst Threshold"]
pub type TXFIFOTHRES_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TXFILLTUNING_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:7 - Scheduler Overhead"]
    #[inline(always)]
    pub fn txschoh(&self) -> TXSCHOH_R {
        TXSCHOH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - Scheduler Health Counter"]
    #[inline(always)]
    pub fn txschhealth(&self) -> TXSCHHEALTH_R {
        TXSCHHEALTH_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:21 - FIFO Burst Threshold"]
    #[inline(always)]
    pub fn txfifothres(&self) -> TXFIFOTHRES_R {
        TXFIFOTHRES_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Scheduler Overhead"]
    #[inline(always)]
    #[must_use]
    pub fn txschoh(&mut self) -> TXSCHOH_W<0> {
        TXSCHOH_W::new(self)
    }
    #[doc = "Bits 8:12 - Scheduler Health Counter"]
    #[inline(always)]
    #[must_use]
    pub fn txschhealth(&mut self) -> TXSCHHEALTH_W<8> {
        TXSCHHEALTH_W::new(self)
    }
    #[doc = "Bits 16:21 - FIFO Burst Threshold"]
    #[inline(always)]
    #[must_use]
    pub fn txfifothres(&mut self) -> TXFIFOTHRES_W<16> {
        TXFIFOTHRES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX FIFO Fill Tuning\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [txfilltuning](index.html) module"]
pub struct TXFILLTUNING_SPEC;
impl crate::RegisterSpec for TXFILLTUNING_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [txfilltuning::R](R) reader structure"]
impl crate::Readable for TXFILLTUNING_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [txfilltuning::W](W) writer structure"]
impl crate::Writable for TXFILLTUNING_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TXFILLTUNING to value 0"]
impl crate::Resettable for TXFILLTUNING_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
