#[doc = "Register `IPTXDAT` reader"]
pub struct R(crate::R<IPTXDAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPTXDAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPTXDAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPTXDAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPTXDAT` writer"]
pub struct W(crate::W<IPTXDAT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPTXDAT_SPEC>;
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
impl From<crate::W<IPTXDAT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPTXDAT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DAT` reader - Data value to use for an IP write command"]
pub type DAT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DAT` writer - Data value to use for an IP write command"]
pub type DAT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IPTXDAT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Data value to use for an IP write command"]
    #[inline(always)]
    pub fn dat(&self) -> DAT_R {
        DAT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Data value to use for an IP write command"]
    #[inline(always)]
    #[must_use]
    pub fn dat(&mut self) -> DAT_W<0> {
        DAT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TX DATA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iptxdat](index.html) module"]
pub struct IPTXDAT_SPEC;
impl crate::RegisterSpec for IPTXDAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iptxdat::R](R) reader structure"]
impl crate::Readable for IPTXDAT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iptxdat::W](W) writer structure"]
impl crate::Writable for IPTXDAT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPTXDAT to value 0"]
impl crate::Resettable for IPTXDAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
