#[doc = "Register `SMVAL4` reader"]
pub struct R(crate::R<SMVAL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMVAL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMVAL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMVAL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMVAL4` writer"]
pub struct W(crate::W<SMVAL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMVAL4_SPEC>;
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
impl From<crate::W<SMVAL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMVAL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL4` reader - Value Register 4"]
pub type VAL4_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VAL4` writer - Value Register 4"]
pub type VAL4_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMVAL4_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Value Register 4"]
    #[inline(always)]
    pub fn val4(&self) -> VAL4_R {
        VAL4_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 4"]
    #[inline(always)]
    #[must_use]
    pub fn val4(&mut self) -> VAL4_W<0> {
        VAL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval4](index.html) module"]
pub struct SMVAL4_SPEC;
impl crate::RegisterSpec for SMVAL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smval4::R](R) reader structure"]
impl crate::Readable for SMVAL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smval4::W](W) writer structure"]
impl crate::Writable for SMVAL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMVAL4 to value 0"]
impl crate::Resettable for SMVAL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
