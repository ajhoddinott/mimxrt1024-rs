#[doc = "Register `SHIFTBUFBYS[%s]` reader"]
pub struct R(crate::R<SHIFTBUFBYS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTBUFBYS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTBUFBYS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTBUFBYS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTBUFBYS[%s]` writer"]
pub struct W(crate::W<SHIFTBUFBYS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTBUFBYS_SPEC>;
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
impl From<crate::W<SHIFTBUFBYS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTBUFBYS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SHIFTBUFBYS` reader - Shift Buffer"]
pub type SHIFTBUFBYS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SHIFTBUFBYS` writer - Shift Buffer"]
pub type SHIFTBUFBYS_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SHIFTBUFBYS_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    pub fn shiftbufbys(&self) -> SHIFTBUFBYS_R {
        SHIFTBUFBYS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Shift Buffer"]
    #[inline(always)]
    #[must_use]
    pub fn shiftbufbys(&mut self) -> SHIFTBUFBYS_W<0> {
        SHIFTBUFBYS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Buffer N Byte Swapped Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftbufbys](index.html) module"]
pub struct SHIFTBUFBYS_SPEC;
impl crate::RegisterSpec for SHIFTBUFBYS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftbufbys::R](R) reader structure"]
impl crate::Readable for SHIFTBUFBYS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftbufbys::W](W) writer structure"]
impl crate::Writable for SHIFTBUFBYS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTBUFBYS[%s]
to value 0"]
impl crate::Resettable for SHIFTBUFBYS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
