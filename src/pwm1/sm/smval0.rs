#[doc = "Register `SMVAL0` reader"]
pub struct R(crate::R<SMVAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMVAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMVAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMVAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMVAL0` writer"]
pub struct W(crate::W<SMVAL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMVAL0_SPEC>;
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
impl From<crate::W<SMVAL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMVAL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL0` reader - Value Register 0"]
pub type VAL0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VAL0` writer - Value Register 0"]
pub type VAL0_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMVAL0_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Value Register 0"]
    #[inline(always)]
    pub fn val0(&self) -> VAL0_R {
        VAL0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 0"]
    #[inline(always)]
    #[must_use]
    pub fn val0(&mut self) -> VAL0_W<0> {
        VAL0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval0](index.html) module"]
pub struct SMVAL0_SPEC;
impl crate::RegisterSpec for SMVAL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smval0::R](R) reader structure"]
impl crate::Readable for SMVAL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smval0::W](W) writer structure"]
impl crate::Writable for SMVAL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMVAL0 to value 0"]
impl crate::Resettable for SMVAL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
