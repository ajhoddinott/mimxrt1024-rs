#[doc = "Register `DR_TOGGLE` writer"]
pub struct W(crate::W<DR_TOGGLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_TOGGLE_SPEC>;
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
impl From<crate::W<DR_TOGGLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_TOGGLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR_TOGGLE` writer - Toggle"]
pub type DR_TOGGLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DR_TOGGLE_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Toggle"]
    #[inline(always)]
    #[must_use]
    pub fn dr_toggle(&mut self) -> DR_TOGGLE_W<0> {
        DR_TOGGLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO data register TOGGLE\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_toggle](index.html) module"]
pub struct DR_TOGGLE_SPEC;
impl crate::RegisterSpec for DR_TOGGLE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dr_toggle::W](W) writer structure"]
impl crate::Writable for DR_TOGGLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR_TOGGLE to value 0"]
impl crate::Resettable for DR_TOGGLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
