#[doc = "Register `LPSMCLR` reader"]
pub struct R(crate::R<LPSMCLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPSMCLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPSMCLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPSMCLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPSMCLR` writer"]
pub struct W(crate::W<LPSMCLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPSMCLR_SPEC>;
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
impl From<crate::W<LPSMCLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPSMCLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MON_COUNTER` reader - Monotonic Counter bits Note that writing to this register does not change the value of this field to the value that was written"]
pub type MON_COUNTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `MON_COUNTER` writer - Monotonic Counter bits Note that writing to this register does not change the value of this field to the value that was written"]
pub type MON_COUNTER_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, LPSMCLR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Monotonic Counter bits Note that writing to this register does not change the value of this field to the value that was written"]
    #[inline(always)]
    pub fn mon_counter(&self) -> MON_COUNTER_R {
        MON_COUNTER_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Monotonic Counter bits Note that writing to this register does not change the value of this field to the value that was written"]
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
#[doc = "SNVS_LP Secure Monotonic Counter LSB Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lpsmclr](index.html) module"]
pub struct LPSMCLR_SPEC;
impl crate::RegisterSpec for LPSMCLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lpsmclr::R](R) reader structure"]
impl crate::Readable for LPSMCLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lpsmclr::W](W) writer structure"]
impl crate::Writable for LPSMCLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPSMCLR to value 0"]
impl crate::Resettable for LPSMCLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
