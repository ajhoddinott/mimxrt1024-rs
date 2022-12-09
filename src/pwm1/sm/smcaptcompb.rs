#[doc = "Register `SMCAPTCOMPB` reader"]
pub struct R(crate::R<SMCAPTCOMPB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMCAPTCOMPB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMCAPTCOMPB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMCAPTCOMPB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMCAPTCOMPB` writer"]
pub struct W(crate::W<SMCAPTCOMPB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMCAPTCOMPB_SPEC>;
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
impl From<crate::W<SMCAPTCOMPB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMCAPTCOMPB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EDGCMPB` reader - Edge Compare B"]
pub type EDGCMPB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `EDGCMPB` writer - Edge Compare B"]
pub type EDGCMPB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMCAPTCOMPB_SPEC, u8, u8, 8, O>;
#[doc = "Field `EDGCNTB` reader - Edge Counter B"]
pub type EDGCNTB_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Edge Compare B"]
    #[inline(always)]
    pub fn edgcmpb(&self) -> EDGCMPB_R {
        EDGCMPB_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Edge Counter B"]
    #[inline(always)]
    pub fn edgcntb(&self) -> EDGCNTB_R {
        EDGCNTB_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Edge Compare B"]
    #[inline(always)]
    #[must_use]
    pub fn edgcmpb(&mut self) -> EDGCMPB_W<0> {
        EDGCMPB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Capture Compare B Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smcaptcompb](index.html) module"]
pub struct SMCAPTCOMPB_SPEC;
impl crate::RegisterSpec for SMCAPTCOMPB_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smcaptcompb::R](R) reader structure"]
impl crate::Readable for SMCAPTCOMPB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smcaptcompb::W](W) writer structure"]
impl crate::Writable for SMCAPTCOMPB_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMCAPTCOMPB to value 0"]
impl crate::Resettable for SMCAPTCOMPB_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
