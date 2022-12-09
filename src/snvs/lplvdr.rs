#[doc = "Register `LPLVDR` reader"]
pub struct R(crate::R<LPLVDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPLVDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPLVDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPLVDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPLVDR` writer"]
pub struct W(crate::W<LPLVDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPLVDR_SPEC>;
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
impl From<crate::W<LPLVDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPLVDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LVD` reader - Low-Voltage Detector Value"]
pub type LVD_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LVD` writer - Low-Voltage Detector Value"]
pub type LVD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPLVDR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Low-Voltage Detector Value"]
    #[inline(always)]
    pub fn lvd(&self) -> LVD_R {
        LVD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low-Voltage Detector Value"]
    #[inline(always)]
    #[must_use]
    pub fn lvd(&mut self) -> LVD_W<0> {
        LVD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Digital Low-Voltage Detector Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lplvdr](index.html) module"]
pub struct LPLVDR_SPEC;
impl crate::RegisterSpec for LPLVDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lplvdr::R](R) reader structure"]
impl crate::Readable for LPLVDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lplvdr::W](W) writer structure"]
impl crate::Writable for LPLVDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPLVDR to value 0"]
impl crate::Resettable for LPLVDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
