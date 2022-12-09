#[doc = "Register `TEMPSENSE0` reader"]
pub struct R(crate::R<TEMPSENSE0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEMPSENSE0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEMPSENSE0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEMPSENSE0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEMPSENSE0` writer"]
pub struct W(crate::W<TEMPSENSE0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEMPSENSE0_SPEC>;
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
impl From<crate::W<TEMPSENSE0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEMPSENSE0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POWER_DOWN` reader - This bit powers down the temperature sensor."]
pub type POWER_DOWN_R = crate::BitReader<POWER_DOWN_A>;
#[doc = "This bit powers down the temperature sensor.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POWER_DOWN_A {
    #[doc = "0: Enable power to the temperature sensor."]
    POWER_UP = 0,
    #[doc = "1: Power down the temperature sensor."]
    POWER_DOWN = 1,
}
impl From<POWER_DOWN_A> for bool {
    #[inline(always)]
    fn from(variant: POWER_DOWN_A) -> Self {
        variant as u8 != 0
    }
}
impl POWER_DOWN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POWER_DOWN_A {
        match self.bits {
            false => POWER_DOWN_A::POWER_UP,
            true => POWER_DOWN_A::POWER_DOWN,
        }
    }
    #[doc = "Checks if the value of the field is `POWER_UP`"]
    #[inline(always)]
    pub fn is_power_up(&self) -> bool {
        *self == POWER_DOWN_A::POWER_UP
    }
    #[doc = "Checks if the value of the field is `POWER_DOWN`"]
    #[inline(always)]
    pub fn is_power_down(&self) -> bool {
        *self == POWER_DOWN_A::POWER_DOWN
    }
}
#[doc = "Field `POWER_DOWN` writer - This bit powers down the temperature sensor."]
pub type POWER_DOWN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TEMPSENSE0_SPEC, POWER_DOWN_A, O>;
impl<'a, const O: u8> POWER_DOWN_W<'a, O> {
    #[doc = "Enable power to the temperature sensor."]
    #[inline(always)]
    pub fn power_up(self) -> &'a mut W {
        self.variant(POWER_DOWN_A::POWER_UP)
    }
    #[doc = "Power down the temperature sensor."]
    #[inline(always)]
    pub fn power_down(self) -> &'a mut W {
        self.variant(POWER_DOWN_A::POWER_DOWN)
    }
}
#[doc = "Field `MEASURE_TEMP` reader - Starts the measurement process"]
pub type MEASURE_TEMP_R = crate::BitReader<MEASURE_TEMP_A>;
#[doc = "Starts the measurement process\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEASURE_TEMP_A {
    #[doc = "0: Do not start the measurement process."]
    STOP = 0,
    #[doc = "1: Start the measurement process."]
    START = 1,
}
impl From<MEASURE_TEMP_A> for bool {
    #[inline(always)]
    fn from(variant: MEASURE_TEMP_A) -> Self {
        variant as u8 != 0
    }
}
impl MEASURE_TEMP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEASURE_TEMP_A {
        match self.bits {
            false => MEASURE_TEMP_A::STOP,
            true => MEASURE_TEMP_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == MEASURE_TEMP_A::STOP
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == MEASURE_TEMP_A::START
    }
}
#[doc = "Field `MEASURE_TEMP` writer - Starts the measurement process"]
pub type MEASURE_TEMP_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TEMPSENSE0_SPEC, MEASURE_TEMP_A, O>;
impl<'a, const O: u8> MEASURE_TEMP_W<'a, O> {
    #[doc = "Do not start the measurement process."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(MEASURE_TEMP_A::STOP)
    }
    #[doc = "Start the measurement process."]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(MEASURE_TEMP_A::START)
    }
}
#[doc = "Field `FINISHED` reader - Indicates that the latest temp is valid"]
pub type FINISHED_R = crate::BitReader<FINISHED_A>;
#[doc = "Indicates that the latest temp is valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FINISHED_A {
    #[doc = "0: Last measurement is not ready yet."]
    INVALID = 0,
    #[doc = "1: Last measurement is valid."]
    VALID = 1,
}
impl From<FINISHED_A> for bool {
    #[inline(always)]
    fn from(variant: FINISHED_A) -> Self {
        variant as u8 != 0
    }
}
impl FINISHED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FINISHED_A {
        match self.bits {
            false => FINISHED_A::INVALID,
            true => FINISHED_A::VALID,
        }
    }
    #[doc = "Checks if the value of the field is `INVALID`"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == FINISHED_A::INVALID
    }
    #[doc = "Checks if the value of the field is `VALID`"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == FINISHED_A::VALID
    }
}
#[doc = "Field `TEMP_CNT` reader - This bit field contains the last measured temperature count."]
pub type TEMP_CNT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ALARM_VALUE` reader - This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
pub type ALARM_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ALARM_VALUE` writer - This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
pub type ALARM_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TEMPSENSE0_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bit 0 - This bit powers down the temperature sensor."]
    #[inline(always)]
    pub fn power_down(&self) -> POWER_DOWN_R {
        POWER_DOWN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Starts the measurement process"]
    #[inline(always)]
    pub fn measure_temp(&self) -> MEASURE_TEMP_R {
        MEASURE_TEMP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that the latest temp is valid"]
    #[inline(always)]
    pub fn finished(&self) -> FINISHED_R {
        FINISHED_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:19 - This bit field contains the last measured temperature count."]
    #[inline(always)]
    pub fn temp_cnt(&self) -> TEMP_CNT_R {
        TEMP_CNT_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    pub fn alarm_value(&self) -> ALARM_VALUE_R {
        ALARM_VALUE_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - This bit powers down the temperature sensor."]
    #[inline(always)]
    #[must_use]
    pub fn power_down(&mut self) -> POWER_DOWN_W<0> {
        POWER_DOWN_W::new(self)
    }
    #[doc = "Bit 1 - Starts the measurement process"]
    #[inline(always)]
    #[must_use]
    pub fn measure_temp(&mut self) -> MEASURE_TEMP_W<1> {
        MEASURE_TEMP_W::new(self)
    }
    #[doc = "Bits 20:31 - This bit field contains the temperature count (raw sensor output) that will generate a high alarm when TEMP_CNT is smaller than this field"]
    #[inline(always)]
    #[must_use]
    pub fn alarm_value(&mut self) -> ALARM_VALUE_W<20> {
        ALARM_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Tempsensor Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tempsense0](index.html) module"]
pub struct TEMPSENSE0_SPEC;
impl crate::RegisterSpec for TEMPSENSE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tempsense0::R](R) reader structure"]
impl crate::Readable for TEMPSENSE0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tempsense0::W](W) writer structure"]
impl crate::Writable for TEMPSENSE0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEMPSENSE0 to value 0x01"]
impl crate::Resettable for TEMPSENSE0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
