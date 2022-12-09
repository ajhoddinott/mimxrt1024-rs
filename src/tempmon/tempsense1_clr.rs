#[doc = "Register `TEMPSENSE1_CLR` reader"]
pub struct R(crate::R<TEMPSENSE1_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPSENSE1_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPSENSE1_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPSENSE1_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPSENSE1_CLR` writer"]
pub struct W(crate::W<TEMPSENSE1_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPSENSE1_CLR_SPEC>;
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
impl From<crate::W<TEMPSENSE1_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPSENSE1_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEASURE_FREQ` reader - This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
pub type MEASURE_FREQ_R = crate::FieldReader<u16, MEASURE_FREQ_A>;
#[doc = "This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum MEASURE_FREQ_A {
    #[doc = "0: Defines a single measurement with no repeat."]
    MEASURE_FREQ_0 = 0,
    #[doc = "1: Updates the temperature value at a RTC clock rate."]
    MEASURE_FREQ_1 = 1,
    #[doc = "2: Updates the temperature value at a RTC/2 clock rate."]
    MEASURE_FREQ_2 = 2,
    #[doc = "65535: Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    MEASURE_FREQ_65535 = 65535,
}
impl From<MEASURE_FREQ_A> for u16 {
    #[inline(always)]
    fn from(variant: MEASURE_FREQ_A) -> Self {
        variant as _
    }
}
impl MEASURE_FREQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MEASURE_FREQ_A> {
        match self.bits {
            0 => Some(MEASURE_FREQ_A::MEASURE_FREQ_0),
            1 => Some(MEASURE_FREQ_A::MEASURE_FREQ_1),
            2 => Some(MEASURE_FREQ_A::MEASURE_FREQ_2),
            65535 => Some(MEASURE_FREQ_A::MEASURE_FREQ_65535),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `MEASURE_FREQ_0`"]
    #[inline(always)]
    pub fn is_measure_freq_0(&self) -> bool {
        *self == MEASURE_FREQ_A::MEASURE_FREQ_0
    }
    #[doc = "Checks if the value of the field is `MEASURE_FREQ_1`"]
    #[inline(always)]
    pub fn is_measure_freq_1(&self) -> bool {
        *self == MEASURE_FREQ_A::MEASURE_FREQ_1
    }
    #[doc = "Checks if the value of the field is `MEASURE_FREQ_2`"]
    #[inline(always)]
    pub fn is_measure_freq_2(&self) -> bool {
        *self == MEASURE_FREQ_A::MEASURE_FREQ_2
    }
    #[doc = "Checks if the value of the field is `MEASURE_FREQ_65535`"]
    #[inline(always)]
    pub fn is_measure_freq_65535(&self) -> bool {
        *self == MEASURE_FREQ_A::MEASURE_FREQ_65535
    }
}
#[doc = "Field `MEASURE_FREQ` writer - This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
pub type MEASURE_FREQ_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TEMPSENSE1_CLR_SPEC, u16, MEASURE_FREQ_A, 16, O>;
impl<'a, const O: u8> MEASURE_FREQ_W<'a, O> {
    #[doc = "Defines a single measurement with no repeat."]
    #[inline(always)]
    pub fn measure_freq_0(self) -> &'a mut W {
        self.variant(MEASURE_FREQ_A::MEASURE_FREQ_0)
    }
    #[doc = "Updates the temperature value at a RTC clock rate."]
    #[inline(always)]
    pub fn measure_freq_1(self) -> &'a mut W {
        self.variant(MEASURE_FREQ_A::MEASURE_FREQ_1)
    }
    #[doc = "Updates the temperature value at a RTC/2 clock rate."]
    #[inline(always)]
    pub fn measure_freq_2(self) -> &'a mut W {
        self.variant(MEASURE_FREQ_A::MEASURE_FREQ_2)
    }
    #[doc = "Determines a two second sample period with a 32.768KHz RTC clock. Exact timings depend on the accuracy of the RTC clock."]
    #[inline(always)]
    pub fn measure_freq_65535(self) -> &'a mut W {
        self.variant(MEASURE_FREQ_A::MEASURE_FREQ_65535)
    }
}
impl R {
    #[doc = "Bits 0:15 - This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    pub fn measure_freq(&self) -> MEASURE_FREQ_R {
        MEASURE_FREQ_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - This bits determines how many RTC clocks to wait before automatically repeating a temperature measurement"]
    #[inline(always)]
    #[must_use]
    pub fn measure_freq(&mut self) -> MEASURE_FREQ_W<0> {
        MEASURE_FREQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tempsensor Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense1_clr](index.html) module"]
pub struct TEMPSENSE1_CLR_SPEC;
impl crate::RegisterSpec for TEMPSENSE1_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempsense1_clr::R](R) reader structure"]
impl crate::Readable for TEMPSENSE1_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempsense1_clr::W](W) writer structure"]
impl crate::Writable for TEMPSENSE1_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPSENSE1_CLR to value 0x01"]
impl crate::Resettable for TEMPSENSE1_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
