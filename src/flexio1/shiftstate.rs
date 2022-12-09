#[doc = "Register `SHIFTSTATE` reader"]
pub struct R(crate::R<SHIFTSTATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTSTATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTSTATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTSTATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTSTATE` writer"]
pub struct W(crate::W<SHIFTSTATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTSTATE_SPEC>;
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
impl From<crate::W<SHIFTSTATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTSTATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STATE` reader - Current State Pointer"]
pub type STATE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `STATE` writer - Current State Pointer"]
pub type STATE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFTSTATE_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Current State Pointer"]
    #[inline(always)]
    pub fn state(&self) -> STATE_R {
        STATE_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Current State Pointer"]
    #[inline(always)]
    #[must_use]
    pub fn state(&mut self) -> STATE_W<0> {
        STATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter State Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftstate](index.html) module"]
pub struct SHIFTSTATE_SPEC;
impl crate::RegisterSpec for SHIFTSTATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftstate::R](R) reader structure"]
impl crate::Readable for SHIFTSTATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftstate::W](W) writer structure"]
impl crate::Writable for SHIFTSTATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTSTATE to value 0"]
impl crate::Resettable for SHIFTSTATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
