#[doc = "Register `SMFRACVAL4` reader"]
pub struct R(crate::R<SMFRACVAL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMFRACVAL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMFRACVAL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMFRACVAL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMFRACVAL4` writer"]
pub struct W(crate::W<SMFRACVAL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMFRACVAL4_SPEC>;
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
impl From<crate::W<SMFRACVAL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMFRACVAL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRACVAL4` reader - Fractional Value 4"]
pub type FRACVAL4_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRACVAL4` writer - Fractional Value 4"]
pub type FRACVAL4_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMFRACVAL4_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 11:15 - Fractional Value 4"]
    #[inline(always)]
    pub fn fracval4(&self) -> FRACVAL4_R {
        FRACVAL4_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 11:15 - Fractional Value 4"]
    #[inline(always)]
    #[must_use]
    pub fn fracval4(&mut self) -> FRACVAL4_W<11> {
        FRACVAL4_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Fractional Value Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smfracval4](index.html) module"]
pub struct SMFRACVAL4_SPEC;
impl crate::RegisterSpec for SMFRACVAL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smfracval4::R](R) reader structure"]
impl crate::Readable for SMFRACVAL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smfracval4::W](W) writer structure"]
impl crate::Writable for SMFRACVAL4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMFRACVAL4 to value 0"]
impl crate::Resettable for SMFRACVAL4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
