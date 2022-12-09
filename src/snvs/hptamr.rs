#[doc = "Register `HPTAMR` reader"]
pub struct R(crate::R<HPTAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPTAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPTAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPTAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPTAMR` writer"]
pub struct W(crate::W<HPTAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPTAMR_SPEC>;
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
impl From<crate::W<HPTAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPTAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPTA_MS` reader - HP Time Alarm, most-significant 15 bits"]
pub type HPTA_MS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `HPTA_MS` writer - HP Time Alarm, most-significant 15 bits"]
pub type HPTA_MS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPTAMR_SPEC, u16, u16, 15, O>;
impl R {
    #[doc = "Bits 0:14 - HP Time Alarm, most-significant 15 bits"]
    #[inline(always)]
    pub fn hpta_ms(&self) -> HPTA_MS_R {
        HPTA_MS_R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - HP Time Alarm, most-significant 15 bits"]
    #[inline(always)]
    #[must_use]
    pub fn hpta_ms(&mut self) -> HPTA_MS_W<0> {
        HPTA_MS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Time Alarm MSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptamr](index.html) module"]
pub struct HPTAMR_SPEC;
impl crate::RegisterSpec for HPTAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hptamr::R](R) reader structure"]
impl crate::Readable for HPTAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hptamr::W](W) writer structure"]
impl crate::Writable for HPTAMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPTAMR to value 0"]
impl crate::Resettable for HPTAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
