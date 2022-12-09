#[doc = "Register `CMPLD23` reader"]
pub struct R(crate::R<CMPLD23_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPLD23_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPLD23_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPLD23_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPLD23` writer"]
pub struct W(crate::W<CMPLD23_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPLD23_SPEC>;
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
impl From<crate::W<CMPLD23_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPLD23_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMPARATOR_LOAD_2` reader - COMPARATOR_LOAD_2"]
pub type COMPARATOR_LOAD_2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COMPARATOR_LOAD_2` writer - COMPARATOR_LOAD_2"]
pub type COMPARATOR_LOAD_2_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, CMPLD23_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - COMPARATOR_LOAD_2"]
    #[inline(always)]
    pub fn comparator_load_2(&self) -> COMPARATOR_LOAD_2_R {
        COMPARATOR_LOAD_2_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - COMPARATOR_LOAD_2"]
    #[inline(always)]
    #[must_use]
    pub fn comparator_load_2(&mut self) -> COMPARATOR_LOAD_2_W<0> {
        COMPARATOR_LOAD_2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel Comparator Load Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpld23](index.html) module"]
pub struct CMPLD23_SPEC;
impl crate::RegisterSpec for CMPLD23_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [cmpld23::R](R) reader structure"]
impl crate::Readable for CMPLD23_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpld23::W](W) writer structure"]
impl crate::Writable for CMPLD23_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPLD23 to value 0"]
impl crate::Resettable for CMPLD23_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
