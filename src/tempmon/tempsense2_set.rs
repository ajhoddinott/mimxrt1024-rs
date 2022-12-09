#[doc = "Register `TEMPSENSE2_SET` reader"]
pub struct R(crate::R<TEMPSENSE2_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPSENSE2_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPSENSE2_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPSENSE2_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPSENSE2_SET` writer"]
pub struct W(crate::W<TEMPSENSE2_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPSENSE2_SET_SPEC>;
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
impl From<crate::W<TEMPSENSE2_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPSENSE2_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOW_ALARM_VALUE` reader - This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
pub type LOW_ALARM_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `LOW_ALARM_VALUE` writer - This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
pub type LOW_ALARM_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TEMPSENSE2_SET_SPEC, u16, u16, 12, O>;
#[doc = "Field `PANIC_ALARM_VALUE` reader - This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
pub type PANIC_ALARM_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PANIC_ALARM_VALUE` writer - This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
pub type PANIC_ALARM_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TEMPSENSE2_SET_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    pub fn low_alarm_value(&self) -> LOW_ALARM_VALUE_R {
        LOW_ALARM_VALUE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub fn panic_alarm_value(&self) -> PANIC_ALARM_VALUE_R {
        PANIC_ALARM_VALUE_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - This bit field contains the temperature count that will generate a low alarm interrupt when the field is exceeded by TEMP_CNT"]
    #[inline(always)]
    #[must_use]
    pub fn low_alarm_value(&mut self) -> LOW_ALARM_VALUE_W<0> {
        LOW_ALARM_VALUE_W::new(self)
    }
    #[doc = "Bits 16:27 - This bit field contains the temperature count that will generate a panic interrupt when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    #[must_use]
    pub fn panic_alarm_value(&mut self) -> PANIC_ALARM_VALUE_W<16> {
        PANIC_ALARM_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tempsensor Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense2_set](index.html) module"]
pub struct TEMPSENSE2_SET_SPEC;
impl crate::RegisterSpec for TEMPSENSE2_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempsense2_set::R](R) reader structure"]
impl crate::Readable for TEMPSENSE2_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempsense2_set::W](W) writer structure"]
impl crate::Writable for TEMPSENSE2_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPSENSE2_SET to value 0"]
impl crate::Resettable for TEMPSENSE2_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
