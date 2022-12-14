#[doc = "Register `SHIFTBUF[%s]` reader"]
pub struct R(crate::R<SHIFTBUF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUF[%s]` writer"]
pub struct W(crate::W<SHIFTBUF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUF_SPEC>;
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
impl From<crate::W<SHIFTBUF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUF` reader - Shift Buffer"]
pub type SHIFTBUF_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHIFTBUF` writer - Shift Buffer"]
pub type SHIFTBUF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFTBUF_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbuf(&self) -> SHIFTBUF_R {
        SHIFTBUF_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn shiftbuf(&mut self) -> SHIFTBUF_W<0> {
        SHIFTBUF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbuf](index.html) module"]
pub struct SHIFTBUF_SPEC;
impl crate::RegisterSpec for SHIFTBUF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbuf::R](R) reader structure"]
impl crate::Readable for SHIFTBUF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbuf::W](W) writer structure"]
impl crate::Writable for SHIFTBUF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTBUF[%s]
to value 0"]
impl crate::Resettable for SHIFTBUF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
