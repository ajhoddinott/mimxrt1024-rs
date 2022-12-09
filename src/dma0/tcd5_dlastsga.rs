#[doc = "Register `TCD5_DLASTSGA` reader"]
pub struct R(crate::R<TCD5_DLASTSGA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCD5_DLASTSGA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCD5_DLASTSGA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCD5_DLASTSGA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCD5_DLASTSGA` writer"]
pub struct W(crate::W<TCD5_DLASTSGA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCD5_DLASTSGA_SPEC>;
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
impl From<crate::W<TCD5_DLASTSGA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCD5_DLASTSGA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DLASTSGA` reader - Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
pub type DLASTSGA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DLASTSGA` writer - Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
pub type DLASTSGA_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TCD5_DLASTSGA_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    pub fn dlastsga(&self) -> DLASTSGA_R {
        DLASTSGA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination last address adjustment, or next memory address TCD for channel (scatter/gather)"]
    #[inline(always)]
    #[must_use]
    pub fn dlastsga(&mut self) -> DLASTSGA_W<0> {
        DLASTSGA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCD Last Destination Address Adjustment/Scatter Gather Address\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcd5_dlastsga](index.html) module"]
pub struct TCD5_DLASTSGA_SPEC;
impl crate::RegisterSpec for TCD5_DLASTSGA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcd5_dlastsga::R](R) reader structure"]
impl crate::Readable for TCD5_DLASTSGA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcd5_dlastsga::W](W) writer structure"]
impl crate::Writable for TCD5_DLASTSGA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCD5_DLASTSGA to value 0"]
impl crate::Resettable for TCD5_DLASTSGA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
