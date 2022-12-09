#[doc = "Register `SMDISMAP0` reader"]
pub struct R(crate::R<SMDISMAP0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMDISMAP0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMDISMAP0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMDISMAP0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMDISMAP0` writer"]
pub struct W(crate::W<SMDISMAP0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMDISMAP0_SPEC>;
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
impl From<crate::W<SMDISMAP0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMDISMAP0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIS0A` reader - PWM_A Fault Disable Mask 0"]
pub type DIS0A_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIS0A` writer - PWM_A Fault Disable Mask 0"]
pub type DIS0A_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMDISMAP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIS0B` reader - PWM_B Fault Disable Mask 0"]
pub type DIS0B_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIS0B` writer - PWM_B Fault Disable Mask 0"]
pub type DIS0B_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMDISMAP0_SPEC, u8, u8, 4, O>;
#[doc = "Field `DIS0X` reader - PWM_X Fault Disable Mask 0"]
pub type DIS0X_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIS0X` writer - PWM_X Fault Disable Mask 0"]
pub type DIS0X_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMDISMAP0_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    pub fn dis0a(&self) -> DIS0A_R {
        DIS0A_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    pub fn dis0b(&self) -> DIS0B_R {
        DIS0B_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    pub fn dis0x(&self) -> DIS0X_R {
        DIS0X_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_A Fault Disable Mask 0"]
    #[inline(always)]
    #[must_use]
    pub fn dis0a(&mut self) -> DIS0A_W<0> {
        DIS0A_W::new(self)
    }
    #[doc = "Bits 4:7 - PWM_B Fault Disable Mask 0"]
    #[inline(always)]
    #[must_use]
    pub fn dis0b(&mut self) -> DIS0B_W<4> {
        DIS0B_W::new(self)
    }
    #[doc = "Bits 8:11 - PWM_X Fault Disable Mask 0"]
    #[inline(always)]
    #[must_use]
    pub fn dis0x(&mut self) -> DIS0X_W<8> {
        DIS0X_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Disable Mapping Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smdismap0](index.html) module"]
pub struct SMDISMAP0_SPEC;
impl crate::RegisterSpec for SMDISMAP0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smdismap0::R](R) reader structure"]
impl crate::Readable for SMDISMAP0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smdismap0::W](W) writer structure"]
impl crate::Writable for SMDISMAP0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMDISMAP0 to value 0xffff"]
impl crate::Resettable for SMDISMAP0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
