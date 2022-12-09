#[doc = "Register `SBLIM` reader"]
pub struct R(crate::R<SBLIM_TOTSAM_SBLIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SBLIM_TOTSAM_SBLIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SBLIM_TOTSAM_SBLIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SBLIM_TOTSAM_SBLIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SBLIM` writer"]
pub struct W(crate::W<SBLIM_TOTSAM_SBLIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SBLIM_TOTSAM_SBLIM_SPEC>;
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
impl From<crate::W<SBLIM_TOTSAM_SBLIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SBLIM_TOTSAM_SBLIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SB_LIM` reader - Sparse Bit Limit"]
pub type SB_LIM_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SB_LIM` writer - Sparse Bit Limit"]
pub type SB_LIM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SBLIM_TOTSAM_SBLIM_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 0:9 - Sparse Bit Limit"]
    #[inline(always)]
    pub fn sb_lim(&self) -> SB_LIM_R {
        SB_LIM_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Sparse Bit Limit"]
    #[inline(always)]
    #[must_use]
    pub fn sb_lim(&mut self) -> SB_LIM_W<0> {
        SB_LIM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Sparse Bit Limit Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sblim_totsam_sblim](index.html) module"]
pub struct SBLIM_TOTSAM_SBLIM_SPEC;
impl crate::RegisterSpec for SBLIM_TOTSAM_SBLIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sblim_totsam_sblim::R](R) reader structure"]
impl crate::Readable for SBLIM_TOTSAM_SBLIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sblim_totsam_sblim::W](W) writer structure"]
impl crate::Writable for SBLIM_TOTSAM_SBLIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SBLIM to value 0x3f"]
impl crate::Resettable for SBLIM_TOTSAM_SBLIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x3f;
}
