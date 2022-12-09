#[doc = "Register `SMVAL1` reader"]
pub struct R(crate::R<SMVAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMVAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMVAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMVAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMVAL1` writer"]
pub struct W(crate::W<SMVAL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMVAL1_SPEC>;
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
impl From<crate::W<SMVAL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMVAL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL1` reader - Value Register 1"]
pub type VAL1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VAL1` writer - Value Register 1"]
pub type VAL1_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMVAL1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Value Register 1"]
    #[inline(always)]
    pub fn val1(&self) -> VAL1_R {
        VAL1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 1"]
    #[inline(always)]
    #[must_use]
    pub fn val1(&mut self) -> VAL1_W<0> {
        VAL1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval1](index.html) module"]
pub struct SMVAL1_SPEC;
impl crate::RegisterSpec for SMVAL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smval1::R](R) reader structure"]
impl crate::Readable for SMVAL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smval1::W](W) writer structure"]
impl crate::Writable for SMVAL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMVAL1 to value 0"]
impl crate::Resettable for SMVAL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
