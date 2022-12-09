#[doc = "Register `SHIFTBUFHWS[%s]` reader"]
pub struct R(crate::R<SHIFTBUFHWS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFHWS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFHWS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFHWS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFHWS[%s]` writer"]
pub struct W(crate::W<SHIFTBUFHWS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFHWS_SPEC>;
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
impl From<crate::W<SHIFTBUFHWS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFHWS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFHWS` reader - Shift Buffer"]
pub type SHIFTBUFHWS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHIFTBUFHWS` writer - Shift Buffer"]
pub type SHIFTBUFHWS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHIFTBUFHWS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufhws(&self) -> SHIFTBUFHWS_R {
        SHIFTBUFHWS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn shiftbufhws(&mut self) -> SHIFTBUFHWS_W<0> {
        SHIFTBUFHWS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Half Word Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufhws](index.html) module"]
pub struct SHIFTBUFHWS_SPEC;
impl crate::RegisterSpec for SHIFTBUFHWS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufhws::R](R) reader structure"]
impl crate::Readable for SHIFTBUFHWS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufhws::W](W) writer structure"]
impl crate::Writable for SHIFTBUFHWS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTBUFHWS[%s]
to value 0"]
impl crate::Resettable for SHIFTBUFHWS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
