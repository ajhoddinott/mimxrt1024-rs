#[doc = "Register `SHIFTBUFBIS[%s]` reader"]
pub struct R(crate::R<SHIFTBUFBIS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFBIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFBIS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFBIS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFBIS[%s]` writer"]
pub struct W(crate::W<SHIFTBUFBIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFBIS_SPEC>;
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
impl From<crate::W<SHIFTBUFBIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFBIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFBIS` reader - Shift Buffer"]
pub type SHIFTBUFBIS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHIFTBUFBIS` writer - Shift Buffer"]
pub type SHIFTBUFBIS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHIFTBUFBIS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbis(&self) -> SHIFTBUFBIS_R {
        SHIFTBUFBIS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn shiftbufbis(&mut self) -> SHIFTBUFBIS_W<0> {
        SHIFTBUFBIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Bit Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbis](index.html) module"]
pub struct SHIFTBUFBIS_SPEC;
impl crate::RegisterSpec for SHIFTBUFBIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufbis::R](R) reader structure"]
impl crate::Readable for SHIFTBUFBIS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufbis::W](W) writer structure"]
impl crate::Writable for SHIFTBUFBIS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTBUFBIS[%s]
to value 0"]
impl crate::Resettable for SHIFTBUFBIS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
