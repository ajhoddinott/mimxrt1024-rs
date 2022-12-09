#[doc = "Register `REGION1_BOT` reader"]
pub struct R(crate::R<REGION1_BOT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION1_BOT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION1_BOT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION1_BOT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION1_BOT` writer"]
pub struct W(crate::W<REGION1_BOT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION1_BOT_SPEC>;
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
impl From<crate::W<REGION1_BOT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION1_BOT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION1_BOT` reader - Address lower limit of region1"]
pub type REGION1_BOT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `REGION1_BOT` writer - Address lower limit of region1"]
pub type REGION1_BOT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REGION1_BOT_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Address lower limit of region1"]
    #[inline(always)]
    pub fn region1_bot(&self) -> REGION1_BOT_R {
        REGION1_BOT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Address lower limit of region1"]
    #[inline(always)]
    #[must_use]
    pub fn region1_bot(&mut self) -> REGION1_BOT_W<0> {
        REGION1_BOT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region1 Bottom Address Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region1_bot](index.html) module"]
pub struct REGION1_BOT_SPEC;
impl crate::RegisterSpec for REGION1_BOT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region1_bot::R](R) reader structure"]
impl crate::Readable for REGION1_BOT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region1_bot::W](W) writer structure"]
impl crate::Writable for REGION1_BOT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION1_BOT to value 0"]
impl crate::Resettable for REGION1_BOT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
