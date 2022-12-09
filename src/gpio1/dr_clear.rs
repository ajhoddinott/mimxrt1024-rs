#[doc = "Register `DR_CLEAR` writer"]
pub struct W(crate::W<DR_CLEAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DR_CLEAR_SPEC>;
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
impl From<crate::W<DR_CLEAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DR_CLEAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DR_CLEAR` writer - Clear"]
pub type DR_CLEAR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DR_CLEAR_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dr_clear(&mut self) -> DR_CLEAR_W<0> {
        DR_CLEAR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPIO data register CLEAR\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dr_clear](index.html) module"]
pub struct DR_CLEAR_SPEC;
impl crate::RegisterSpec for DR_CLEAR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [dr_clear::W](W) writer structure"]
impl crate::Writable for DR_CLEAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DR_CLEAR to value 0"]
impl crate::Resettable for DR_CLEAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
