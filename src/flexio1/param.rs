#[doc = "Register `PARAM` reader"]
pub struct R(crate::R<PARAM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PARAM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PARAM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PARAM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SHIFTER` reader - Shifter Number"]
pub type SHIFTER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMER` reader - Timer Number"]
pub type TIMER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIN` reader - Pin Number"]
pub type PIN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRIGGER` reader - Trigger Number"]
pub type TRIGGER_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:7 - Shifter Number"]
    #[inline(always)]
    pub fn shifter(&self) -> SHIFTER_R {
        SHIFTER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Timer Number"]
    #[inline(always)]
    pub fn timer(&self) -> TIMER_R {
        TIMER_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pin Number"]
    #[inline(always)]
    pub fn pin(&self) -> PIN_R {
        PIN_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Trigger Number"]
    #[inline(always)]
    pub fn trigger(&self) -> TRIGGER_R {
        TRIGGER_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Parameter Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [param](index.html) module"]
pub struct PARAM_SPEC;
impl crate::RegisterSpec for PARAM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [param::R](R) reader structure"]
impl crate::Readable for PARAM_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets PARAM to value 0x0220_0808"]
impl crate::Resettable for PARAM_SPEC {
    const RESET_VALUE: Self::Ux = 0x0220_0808;
}
