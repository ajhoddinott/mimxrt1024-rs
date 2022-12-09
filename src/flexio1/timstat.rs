#[doc = "Register `TIMSTAT` reader"]
pub struct R(crate::R<TIMSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMSTAT` writer"]
pub struct W(crate::W<TIMSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMSTAT_SPEC>;
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
impl From<crate::W<TIMSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TSF` reader - Timer Status Flags"]
pub type TSF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TSF` writer - Timer Status Flags"]
pub type TSF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMSTAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Timer Status Flags"]
    #[inline(always)]
    pub fn tsf(&self) -> TSF_R {
        TSF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Timer Status Flags"]
    #[inline(always)]
    #[must_use]
    pub fn tsf(&mut self) -> TSF_W<0> {
        TSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timstat](index.html) module"]
pub struct TIMSTAT_SPEC;
impl crate::RegisterSpec for TIMSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timstat::R](R) reader structure"]
impl crate::Readable for TIMSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timstat::W](W) writer structure"]
impl crate::Writable for TIMSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
}
#[doc = "`reset()` method sets TIMSTAT to value 0"]
impl crate::Resettable for TIMSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
