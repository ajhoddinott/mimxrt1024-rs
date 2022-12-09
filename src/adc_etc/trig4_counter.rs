#[doc = "Register `TRIG4_COUNTER` reader"]
pub struct R(crate::R<TRIG4_COUNTER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TRIG4_COUNTER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TRIG4_COUNTER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TRIG4_COUNTER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TRIG4_COUNTER` writer"]
pub struct W(crate::W<TRIG4_COUNTER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TRIG4_COUNTER_SPEC>;
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
impl From<crate::W<TRIG4_COUNTER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TRIG4_COUNTER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT_DELAY` reader - TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
pub type INIT_DELAY_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INIT_DELAY` writer - TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
pub type INIT_DELAY_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG4_COUNTER_SPEC, u16, u16, 16, O>;
#[doc = "Field `SAMPLE_INTERVAL` reader - TRIGGER sampling interval counter"]
pub type SAMPLE_INTERVAL_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAMPLE_INTERVAL` writer - TRIGGER sampling interval counter"]
pub type SAMPLE_INTERVAL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TRIG4_COUNTER_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    pub fn init_delay(&self) -> INIT_DELAY_R {
        INIT_DELAY_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - TRIGGER sampling interval counter"]
    #[inline(always)]
    pub fn sample_interval(&self) -> SAMPLE_INTERVAL_R {
        SAMPLE_INTERVAL_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TRIGGER initial delay counter. Initial_delay = (INIT_DELAY+1)*(PRE_DIVIDER+1)*ipg_clk"]
    #[inline(always)]
    #[must_use]
    pub fn init_delay(&mut self) -> INIT_DELAY_W<0> {
        INIT_DELAY_W::new(self)
    }
    #[doc = "Bits 16:31 - TRIGGER sampling interval counter"]
    #[inline(always)]
    #[must_use]
    pub fn sample_interval(&mut self) -> SAMPLE_INTERVAL_W<16> {
        SAMPLE_INTERVAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ETC_TRIG Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [trig4_counter](index.html) module"]
pub struct TRIG4_COUNTER_SPEC;
impl crate::RegisterSpec for TRIG4_COUNTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [trig4_counter::R](R) reader structure"]
impl crate::Readable for TRIG4_COUNTER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [trig4_counter::W](W) writer structure"]
impl crate::Writable for TRIG4_COUNTER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIG4_COUNTER to value 0"]
impl crate::Resettable for TRIG4_COUNTER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
