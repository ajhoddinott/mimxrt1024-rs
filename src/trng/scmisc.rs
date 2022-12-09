#[doc = "Register `SCMISC` reader"]
pub struct R(crate::R<SCMISC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCMISC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCMISC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCMISC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCMISC` writer"]
pub struct W(crate::W<SCMISC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCMISC_SPEC>;
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
impl From<crate::W<SCMISC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCMISC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LRUN_MAX` reader - LONG RUN MAX LIMIT"]
pub type LRUN_MAX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LRUN_MAX` writer - LONG RUN MAX LIMIT"]
pub type LRUN_MAX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCMISC_SPEC, u8, u8, 8, O>;
#[doc = "Field `RTY_CT` reader - RETRY COUNT"]
pub type RTY_CT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTY_CT` writer - RETRY COUNT"]
pub type RTY_CT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCMISC_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:7 - LONG RUN MAX LIMIT"]
    #[inline(always)]
    pub fn lrun_max(&self) -> LRUN_MAX_R {
        LRUN_MAX_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn rty_ct(&self) -> RTY_CT_R {
        RTY_CT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - LONG RUN MAX LIMIT"]
    #[inline(always)]
    #[must_use]
    pub fn lrun_max(&mut self) -> LRUN_MAX_W<0> {
        LRUN_MAX_W::new(self)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    #[must_use]
    pub fn rty_ct(&mut self) -> RTY_CT_W<16> {
        RTY_CT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Statistical Check Miscellaneous Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scmisc](index.html) module"]
pub struct SCMISC_SPEC;
impl crate::RegisterSpec for SCMISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scmisc::R](R) reader structure"]
impl crate::Readable for SCMISC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scmisc::W](W) writer structure"]
impl crate::Writable for SCMISC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCMISC to value 0x0001_0022"]
impl crate::Resettable for SCMISC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0022;
}
