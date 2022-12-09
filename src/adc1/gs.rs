#[doc = "Register `GS` reader"]
pub struct R(crate::R<GS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GS` writer"]
pub struct W(crate::W<GS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GS_SPEC>;
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
impl From<crate::W<GS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADACT` reader - Conversion Active"]
pub type ADACT_R = crate::BitReader<ADACT_A>;
#[doc = "Conversion Active\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADACT_A {
    #[doc = "0: Conversion not in progress."]
    ADACT_0 = 0,
    #[doc = "1: Conversion in progress."]
    ADACT_1 = 1,
}
impl From<ADACT_A> for bool {
    #[inline(always)]
    fn from(variant: ADACT_A) -> Self {
        variant as u8 != 0
    }
}
impl ADACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACT_A {
        match self.bits {
            false => ADACT_A::ADACT_0,
            true => ADACT_A::ADACT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADACT_0`"]
    #[inline(always)]
    pub fn is_adact_0(&self) -> bool {
        *self == ADACT_A::ADACT_0
    }
    #[doc = "Checks if the value of the field is `ADACT_1`"]
    #[inline(always)]
    pub fn is_adact_1(&self) -> bool {
        *self == ADACT_A::ADACT_1
    }
}
#[doc = "Field `CALF` reader - Calibration Failed Flag"]
pub type CALF_R = crate::BitReader<CALF_A>;
#[doc = "Calibration Failed Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CALF_A {
    #[doc = "0: Calibration completed normally."]
    CALF_0 = 0,
    #[doc = "1: Calibration failed. ADC accuracy specifications are not guaranteed."]
    CALF_1 = 1,
}
impl From<CALF_A> for bool {
    #[inline(always)]
    fn from(variant: CALF_A) -> Self {
        variant as u8 != 0
    }
}
impl CALF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CALF_A {
        match self.bits {
            false => CALF_A::CALF_0,
            true => CALF_A::CALF_1,
        }
    }
    #[doc = "Checks if the value of the field is `CALF_0`"]
    #[inline(always)]
    pub fn is_calf_0(&self) -> bool {
        *self == CALF_A::CALF_0
    }
    #[doc = "Checks if the value of the field is `CALF_1`"]
    #[inline(always)]
    pub fn is_calf_1(&self) -> bool {
        *self == CALF_A::CALF_1
    }
}
#[doc = "Field `CALF` writer - Calibration Failed Flag"]
pub type CALF_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GS_SPEC, CALF_A, O>;
impl<'a, const O: u8> CALF_W<'a, O> {
    #[doc = "Calibration completed normally."]
    #[inline(always)]
    pub fn calf_0(self) -> &'a mut W {
        self.variant(CALF_A::CALF_0)
    }
    #[doc = "Calibration failed. ADC accuracy specifications are not guaranteed."]
    #[inline(always)]
    pub fn calf_1(self) -> &'a mut W {
        self.variant(CALF_A::CALF_1)
    }
}
#[doc = "Field `AWKST` reader - Asynchronous wakeup interrupt status"]
pub type AWKST_R = crate::BitReader<AWKST_A>;
#[doc = "Asynchronous wakeup interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWKST_A {
    #[doc = "0: No asynchronous interrupt."]
    AWKST_0 = 0,
    #[doc = "1: Asynchronous wake up interrupt occurred in stop mode."]
    AWKST_1 = 1,
}
impl From<AWKST_A> for bool {
    #[inline(always)]
    fn from(variant: AWKST_A) -> Self {
        variant as u8 != 0
    }
}
impl AWKST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWKST_A {
        match self.bits {
            false => AWKST_A::AWKST_0,
            true => AWKST_A::AWKST_1,
        }
    }
    #[doc = "Checks if the value of the field is `AWKST_0`"]
    #[inline(always)]
    pub fn is_awkst_0(&self) -> bool {
        *self == AWKST_A::AWKST_0
    }
    #[doc = "Checks if the value of the field is `AWKST_1`"]
    #[inline(always)]
    pub fn is_awkst_1(&self) -> bool {
        *self == AWKST_A::AWKST_1
    }
}
#[doc = "Field `AWKST` writer - Asynchronous wakeup interrupt status"]
pub type AWKST_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, GS_SPEC, AWKST_A, O>;
impl<'a, const O: u8> AWKST_W<'a, O> {
    #[doc = "No asynchronous interrupt."]
    #[inline(always)]
    pub fn awkst_0(self) -> &'a mut W {
        self.variant(AWKST_A::AWKST_0)
    }
    #[doc = "Asynchronous wake up interrupt occurred in stop mode."]
    #[inline(always)]
    pub fn awkst_1(self) -> &'a mut W {
        self.variant(AWKST_A::AWKST_1)
    }
}
impl R {
    #[doc = "Bit 0 - Conversion Active"]
    #[inline(always)]
    pub fn adact(&self) -> ADACT_R {
        ADACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Calibration Failed Flag"]
    #[inline(always)]
    pub fn calf(&self) -> CALF_R {
        CALF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Asynchronous wakeup interrupt status"]
    #[inline(always)]
    pub fn awkst(&self) -> AWKST_R {
        AWKST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Calibration Failed Flag"]
    #[inline(always)]
    #[must_use]
    pub fn calf(&mut self) -> CALF_W<1> {
        CALF_W::new(self)
    }
    #[doc = "Bit 2 - Asynchronous wakeup interrupt status"]
    #[inline(always)]
    #[must_use]
    pub fn awkst(&mut self) -> AWKST_W<2> {
        AWKST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gs](index.html) module"]
pub struct GS_SPEC;
impl crate::RegisterSpec for GS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gs::R](R) reader structure"]
impl crate::Readable for GS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gs::W](W) writer structure"]
impl crate::Writable for GS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x06;
}
#[doc = "`reset()` method sets GS to value 0"]
impl crate::Resettable for GS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
