#[doc = "Register `HPTALR` reader"]
pub struct R(crate::R<HPTALR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPTALR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPTALR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPTALR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPTALR` writer"]
pub struct W(crate::W<HPTALR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPTALR_SPEC>;
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
impl From<crate::W<HPTALR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPTALR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPTA_LS` reader - HP Time Alarm, 32 least-significant bits"]
pub type HPTA_LS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HPTA_LS` writer - HP Time Alarm, 32 least-significant bits"]
pub type HPTA_LS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HPTALR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - HP Time Alarm, 32 least-significant bits"]
    #[inline(always)]
    pub fn hpta_ls(&self) -> HPTA_LS_R {
        HPTA_LS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - HP Time Alarm, 32 least-significant bits"]
    #[inline(always)]
    #[must_use]
    pub fn hpta_ls(&mut self) -> HPTA_LS_W<0> {
        HPTA_LS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP Time Alarm LSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hptalr](index.html) module"]
pub struct HPTALR_SPEC;
impl crate::RegisterSpec for HPTALR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hptalr::R](R) reader structure"]
impl crate::Readable for HPTALR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hptalr::W](W) writer structure"]
impl crate::Writable for HPTALR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPTALR to value 0"]
impl crate::Resettable for HPTALR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
