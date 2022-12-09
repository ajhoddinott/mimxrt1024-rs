#[doc = "Register `LPSMCMR` reader"]
pub struct R(crate::R<LPSMCMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSMCMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSMCMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSMCMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSMCMR` writer"]
pub struct W(crate::W<LPSMCMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSMCMR_SPEC>;
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
impl From<crate::W<LPSMCMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSMCMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MON_COUNTER` reader - Monotonic Counter most-significant 16 Bits Note that writing to this register does not change the value of this field to the value that was written"]
pub type MON_COUNTER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `MON_COUNTER` writer - Monotonic Counter most-significant 16 Bits Note that writing to this register does not change the value of this field to the value that was written"]
pub type MON_COUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPSMCMR_SPEC, u16, u16, 16, O>;
#[doc = "Field `MC_ERA_BITS` reader - Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses"]
pub type MC_ERA_BITS_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - Monotonic Counter most-significant 16 Bits Note that writing to this register does not change the value of this field to the value that was written"]
    #[inline(always)]
    pub fn mon_counter(&self) -> MON_COUNTER_R {
        MON_COUNTER_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Monotonic Counter Era Bits These bits are inputs to the module and typically connect to fuses"]
    #[inline(always)]
    pub fn mc_era_bits(&self) -> MC_ERA_BITS_R {
        MC_ERA_BITS_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Monotonic Counter most-significant 16 Bits Note that writing to this register does not change the value of this field to the value that was written"]
    #[inline(always)]
    #[must_use]
    pub fn mon_counter(&mut self) -> MON_COUNTER_W<0> {
        MON_COUNTER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Secure Monotonic Counter MSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsmcmr](index.html) module"]
pub struct LPSMCMR_SPEC;
impl crate::RegisterSpec for LPSMCMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpsmcmr::R](R) reader structure"]
impl crate::Readable for LPSMCMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpsmcmr::W](W) writer structure"]
impl crate::Writable for LPSMCMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPSMCMR to value 0"]
impl crate::Resettable for LPSMCMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
