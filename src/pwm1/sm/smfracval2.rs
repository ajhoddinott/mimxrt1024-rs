#[doc = "Register `SMFRACVAL2` reader"]
pub struct R(crate::R<SMFRACVAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMFRACVAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMFRACVAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMFRACVAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMFRACVAL2` writer"]
pub struct W(crate::W<SMFRACVAL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMFRACVAL2_SPEC>;
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
impl From<crate::W<SMFRACVAL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMFRACVAL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACVAL2` reader - Fractional Value 2"]
pub type FRACVAL2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRACVAL2` writer - Fractional Value 2"]
pub type FRACVAL2_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMFRACVAL2_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 11:15 - Fractional Value 2"]
    #[inline(always)]
    pub fn fracval2(&self) -> FRACVAL2_R {
        FRACVAL2_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 11:15 - Fractional Value 2"]
    #[inline(always)]
    #[must_use]
    pub fn fracval2(&mut self) -> FRACVAL2_W<11> {
        FRACVAL2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Value Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smfracval2](index.html) module"]
pub struct SMFRACVAL2_SPEC;
impl crate::RegisterSpec for SMFRACVAL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smfracval2::R](R) reader structure"]
impl crate::Readable for SMFRACVAL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smfracval2::W](W) writer structure"]
impl crate::Writable for SMFRACVAL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMFRACVAL2 to value 0"]
impl crate::Resettable for SMFRACVAL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
