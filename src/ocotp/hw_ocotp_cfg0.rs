#[doc = "Register `HW_OCOTP_CFG0` reader"]
pub struct R(crate::R<HW_OCOTP_CFG0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_CFG0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_CFG0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_CFG0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_CFG0` writer"]
pub struct W(crate::W<HW_OCOTP_CFG0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_CFG0_SPEC>;
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
impl From<crate::W<HW_OCOTP_CFG0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_CFG0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BITS` reader - BITS"]
pub type BITS_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BITS` writer - BITS"]
pub type BITS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HW_OCOTP_CFG0_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - BITS"]
    #[inline(always)]
    pub fn bits_(&self) -> BITS_R {
        BITS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - BITS"]
    #[inline(always)]
    #[must_use]
    pub fn bits_(&mut self) -> BITS_W<0> {
        BITS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Value of OTP Bank0 Word1 (Configuration and Manufacturing Info.)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_cfg0](index.html) module"]
pub struct HW_OCOTP_CFG0_SPEC;
impl crate::RegisterSpec for HW_OCOTP_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_cfg0::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_CFG0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_cfg0::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_CFG0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_OCOTP_CFG0 to value 0"]
impl crate::Resettable for HW_OCOTP_CFG0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
