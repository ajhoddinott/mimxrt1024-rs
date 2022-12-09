#[doc = "Register `SMVAL3` reader"]
pub struct R(crate::R<SMVAL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMVAL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMVAL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMVAL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMVAL3` writer"]
pub struct W(crate::W<SMVAL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMVAL3_SPEC>;
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
impl From<crate::W<SMVAL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMVAL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VAL3` reader - Value Register 3"]
pub type VAL3_R = crate::FieldReader<u16, u16>;
#[doc = "Field `VAL3` writer - Value Register 3"]
pub type VAL3_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMVAL3_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Value Register 3"]
    #[inline(always)]
    pub fn val3(&self) -> VAL3_R {
        VAL3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Value Register 3"]
    #[inline(always)]
    #[must_use]
    pub fn val3(&mut self) -> VAL3_W<0> {
        VAL3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smval3](index.html) module"]
pub struct SMVAL3_SPEC;
impl crate::RegisterSpec for SMVAL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smval3::R](R) reader structure"]
impl crate::Readable for SMVAL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smval3::W](W) writer structure"]
impl crate::Writable for SMVAL3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMVAL3 to value 0"]
impl crate::Resettable for SMVAL3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
