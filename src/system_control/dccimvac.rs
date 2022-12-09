#[doc = "Register `DCCIMVAC` writer"]
pub struct W(crate::W<DCCIMVAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCCIMVAC_SPEC>;
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
impl From<crate::W<DCCIMVAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCCIMVAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCCIMVAC` writer - D-cache clean and invalidate by MVA to PoC"]
pub type DCCIMVAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCCIMVAC_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - D-cache clean and invalidate by MVA to PoC"]
    #[inline(always)]
    #[must_use]
    pub fn dccimvac(&mut self) -> DCCIMVAC_W<0> {
        DCCIMVAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data cache clean and invalidate by address to PoC\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccimvac](index.html) module"]
pub struct DCCIMVAC_SPEC;
impl crate::RegisterSpec for DCCIMVAC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dccimvac::W](W) writer structure"]
impl crate::Writable for DCCIMVAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCCIMVAC to value 0"]
impl crate::Resettable for DCCIMVAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
