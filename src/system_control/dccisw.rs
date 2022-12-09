#[doc = "Register `DCCISW` writer"]
pub struct W(crate::W<DCCISW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCCISW_SPEC>;
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
impl From<crate::W<DCCISW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCCISW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCCISW` writer - D-cache clean and invalidate by set-way"]
pub type DCCISW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DCCISW_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - D-cache clean and invalidate by set-way"]
    #[inline(always)]
    #[must_use]
    pub fn dccisw(&mut self) -> DCCISW_W<0> {
        DCCISW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data cache clean and invalidate by set/way\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dccisw](index.html) module"]
pub struct DCCISW_SPEC;
impl crate::RegisterSpec for DCCISW_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dccisw::W](W) writer structure"]
impl crate::Writable for DCCISW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCCISW to value 0"]
impl crate::Resettable for DCCISW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
