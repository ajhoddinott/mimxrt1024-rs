#[doc = "Register `REGION1_TOP` reader"]
pub struct R(crate::R<REGION1_TOP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION1_TOP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION1_TOP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION1_TOP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION1_TOP` writer"]
pub struct W(crate::W<REGION1_TOP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION1_TOP_SPEC>;
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
impl From<crate::W<REGION1_TOP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION1_TOP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION1_TOP` reader - Address upper limit of region1"]
pub type REGION1_TOP_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REGION1_TOP` writer - Address upper limit of region1"]
pub type REGION1_TOP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REGION1_TOP_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address upper limit of region1"]
    #[inline(always)]
    pub fn region1_top(&self) -> REGION1_TOP_R {
        REGION1_TOP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address upper limit of region1"]
    #[inline(always)]
    #[must_use]
    pub fn region1_top(&mut self) -> REGION1_TOP_W<0> {
        REGION1_TOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region1 Top Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region1_top](index.html) module"]
pub struct REGION1_TOP_SPEC;
impl crate::RegisterSpec for REGION1_TOP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region1_top::R](R) reader structure"]
impl crate::Readable for REGION1_TOP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region1_top::W](W) writer structure"]
impl crate::Writable for REGION1_TOP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION1_TOP to value 0"]
impl crate::Resettable for REGION1_TOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
