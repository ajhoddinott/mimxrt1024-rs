#[doc = "Register `DCIMVAC` writer"]
pub struct W(crate::W<DCIMVAC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCIMVAC_SPEC>;
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
impl From<crate::W<DCIMVAC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCIMVAC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCIMVAC` writer - D-cache invalidate by MVA to PoC"]
pub type DCIMVAC_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCIMVAC_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - D-cache invalidate by MVA to PoC"]
    #[inline(always)]
    #[must_use]
    pub fn dcimvac(&mut self) -> DCIMVAC_W<0> {
        DCIMVAC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data cache invalidate by address to Point of Coherency (PoC)\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcimvac](index.html) module"]
pub struct DCIMVAC_SPEC;
impl crate::RegisterSpec for DCIMVAC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dcimvac::W](W) writer structure"]
impl crate::Writable for DCIMVAC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCIMVAC to value 0"]
impl crate::Resettable for DCIMVAC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
