#[doc = "Register `FFILT0` reader"]
pub struct R(crate::R<FFILT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FFILT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FFILT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FFILT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FFILT0` writer"]
pub struct W(crate::W<FFILT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FFILT0_SPEC>;
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
impl From<crate::W<FFILT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FFILT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FILT_PER` reader - Fault Filter Period"]
pub type FILT_PER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILT_PER` writer - Fault Filter Period"]
pub type FILT_PER_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FFILT0_SPEC, u8, u8, 8, O>;
#[doc = "Field `FILT_CNT` reader - Fault Filter Count"]
pub type FILT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILT_CNT` writer - Fault Filter Count"]
pub type FILT_CNT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, FFILT0_SPEC, u8, u8, 3, O>;
#[doc = "Field `GSTR` reader - Fault Glitch Stretch Enable"]
pub type GSTR_R = crate::BitReader<GSTR_A>;
#[doc = "Fault Glitch Stretch Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GSTR_A {
    #[doc = "0: Fault input glitch stretching is disabled."]
    DISABLED = 0,
    #[doc = "1: Input fault signals will be stretched to at least 2 IPBus clock cycles."]
    ENABLED = 1,
}
impl From<GSTR_A> for bool {
    #[inline(always)]
    fn from(variant: GSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl GSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GSTR_A {
        match self.bits {
            false => GSTR_A::DISABLED,
            true => GSTR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GSTR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GSTR_A::ENABLED
    }
}
#[doc = "Field `GSTR` writer - Fault Glitch Stretch Enable"]
pub type GSTR_W<'a, const O: u8> = crate::BitWriter<'a, u16, FFILT0_SPEC, GSTR_A, O>;
impl<'a, const O: u8> GSTR_W<'a, O> {
    #[doc = "Fault input glitch stretching is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GSTR_A::DISABLED)
    }
    #[doc = "Input fault signals will be stretched to at least 2 IPBus clock cycles."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GSTR_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:7 - Fault Filter Period"]
    #[inline(always)]
    pub fn filt_per(&self) -> FILT_PER_R {
        FILT_PER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - Fault Filter Count"]
    #[inline(always)]
    pub fn filt_cnt(&self) -> FILT_CNT_R {
        FILT_CNT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 15 - Fault Glitch Stretch Enable"]
    #[inline(always)]
    pub fn gstr(&self) -> GSTR_R {
        GSTR_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fault Filter Period"]
    #[inline(always)]
    #[must_use]
    pub fn filt_per(&mut self) -> FILT_PER_W<0> {
        FILT_PER_W::new(self)
    }
    #[doc = "Bits 8:10 - Fault Filter Count"]
    #[inline(always)]
    #[must_use]
    pub fn filt_cnt(&mut self) -> FILT_CNT_W<8> {
        FILT_CNT_W::new(self)
    }
    #[doc = "Bit 15 - Fault Glitch Stretch Enable"]
    #[inline(always)]
    #[must_use]
    pub fn gstr(&mut self) -> GSTR_W<15> {
        GSTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fault Filter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ffilt0](index.html) module"]
pub struct FFILT0_SPEC;
impl crate::RegisterSpec for FFILT0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ffilt0::R](R) reader structure"]
impl crate::Readable for FFILT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ffilt0::W](W) writer structure"]
impl crate::Writable for FFILT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FFILT0 to value 0"]
impl crate::Resettable for FFILT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
