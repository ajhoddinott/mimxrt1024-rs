#[doc = "Register `UPOS` reader"]
pub struct R(crate::R<UPOS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UPOS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UPOS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UPOS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UPOS` writer"]
pub struct W(crate::W<UPOS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UPOS_SPEC>;
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
impl From<crate::W<UPOS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UPOS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `POS` reader - POS"]
pub type POS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `POS` writer - POS"]
pub type POS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UPOS_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - POS"]
    #[inline(always)]
    pub fn pos(&self) -> POS_R {
        POS_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - POS"]
    #[inline(always)]
    #[must_use]
    pub fn pos(&mut self) -> POS_W<0> {
        POS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper Position Counter Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [upos](index.html) module"]
pub struct UPOS_SPEC;
impl crate::RegisterSpec for UPOS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [upos::R](R) reader structure"]
impl crate::Readable for UPOS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [upos::W](W) writer structure"]
impl crate::Writable for UPOS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UPOS to value 0"]
impl crate::Resettable for UPOS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
