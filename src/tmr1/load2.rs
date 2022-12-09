#[doc = "Register `LOAD2` reader"]
pub struct R(crate::R<LOAD2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOAD2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOAD2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOAD2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LOAD2` writer"]
pub struct W(crate::W<LOAD2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LOAD2_SPEC>;
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
impl From<crate::W<LOAD2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LOAD2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` reader - Timer Load Register"]
pub type LOAD_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LOAD` writer - Timer Load Register"]
pub type LOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, LOAD2_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Timer Load Register"]
    #[inline(always)]
    pub fn load(&self) -> LOAD_R {
        LOAD_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Load Register"]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Load Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [load2](index.html) module"]
pub struct LOAD2_SPEC;
impl crate::RegisterSpec for LOAD2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [load2::R](R) reader structure"]
impl crate::Readable for LOAD2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [load2::W](W) writer structure"]
impl crate::Writable for LOAD2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LOAD2 to value 0"]
impl crate::Resettable for LOAD2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
