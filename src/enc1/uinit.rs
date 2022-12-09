#[doc = "Register `UINIT` reader"]
pub struct R(crate::R<UINIT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UINIT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UINIT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UINIT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UINIT` writer"]
pub struct W(crate::W<UINIT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UINIT_SPEC>;
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
impl From<crate::W<UINIT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UINIT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INIT` reader - INIT"]
pub type INIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `INIT` writer - INIT"]
pub type INIT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, UINIT_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - INIT"]
    #[inline(always)]
    pub fn init(&self) -> INIT_R {
        INIT_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - INIT"]
    #[inline(always)]
    #[must_use]
    pub fn init(&mut self) -> INIT_W<0> {
        INIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Upper Initialization Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [uinit](index.html) module"]
pub struct UINIT_SPEC;
impl crate::RegisterSpec for UINIT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [uinit::R](R) reader structure"]
impl crate::Readable for UINIT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [uinit::W](W) writer structure"]
impl crate::Writable for UINIT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets UINIT to value 0"]
impl crate::Resettable for UINIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
