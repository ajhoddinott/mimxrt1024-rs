#[doc = "Register `LCOMP` reader"]
pub struct R(crate::R<LCOMP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LCOMP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LCOMP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LCOMP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LCOMP` writer"]
pub struct W(crate::W<LCOMP_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LCOMP_SPEC>;
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
impl From<crate::W<LCOMP_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LCOMP_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP` reader - COMP"]
pub type COMP_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMP` writer - COMP"]
pub type COMP_W<'a, const O: u8> = crate::FieldWriter<'a, u16, LCOMP_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - COMP"]
    #[inline(always)]
    pub fn comp(&self) -> COMP_R {
        COMP_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - COMP"]
    #[inline(always)]
    #[must_use]
    pub fn comp(&mut self) -> COMP_W<0> {
        COMP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Lower Position Compare Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lcomp](index.html) module"]
pub struct LCOMP_SPEC;
impl crate::RegisterSpec for LCOMP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [lcomp::R](R) reader structure"]
impl crate::Readable for LCOMP_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lcomp::W](W) writer structure"]
impl crate::Writable for LCOMP_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LCOMP to value 0xffff"]
impl crate::Resettable for LCOMP_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff;
}
