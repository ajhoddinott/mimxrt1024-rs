#[doc = "Register `SHIFTSTAT` reader"]
pub struct R(crate::R<SHIFTSTAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTSTAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTSTAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTSTAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTSTAT` writer"]
pub struct W(crate::W<SHIFTSTAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTSTAT_SPEC>;
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
impl From<crate::W<SHIFTSTAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTSTAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSF` reader - Shifter Status Flag"]
pub type SSF_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSF` writer - Shifter Status Flag"]
pub type SSF_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFTSTAT_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Shifter Status Flag"]
    #[inline(always)]
    pub fn ssf(&self) -> SSF_R {
        SSF_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Shifter Status Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ssf(&mut self) -> SSF_W<0> {
        SSF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftstat](index.html) module"]
pub struct SHIFTSTAT_SPEC;
impl crate::RegisterSpec for SHIFTSTAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftstat::R](R) reader structure"]
impl crate::Readable for SHIFTSTAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftstat::W](W) writer structure"]
impl crate::Writable for SHIFTSTAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xff;
}
#[doc = "`reset()` method sets SHIFTSTAT to value 0"]
impl crate::Resettable for SHIFTSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
