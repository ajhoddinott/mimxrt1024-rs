#[doc = "Register `LUTCR` reader"]
pub struct R(crate::R<LUTCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LUTCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LUTCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LUTCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LUTCR` writer"]
pub struct W(crate::W<LUTCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LUTCR_SPEC>;
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
impl From<crate::W<LUTCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LUTCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCK` reader - Lock LUT"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `LOCK` writer - Lock LUT"]
pub type LOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUTCR_SPEC, bool, O>;
#[doc = "Field `UNLOCK` reader - Unlock LUT"]
pub type UNLOCK_R = crate::BitReader<bool>;
#[doc = "Field `UNLOCK` writer - Unlock LUT"]
pub type UNLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, LUTCR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Lock LUT"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Unlock LUT"]
    #[inline(always)]
    pub fn unlock(&self) -> UNLOCK_R {
        UNLOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Lock LUT"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<0> {
        LOCK_W::new(self)
    }
    #[doc = "Bit 1 - Unlock LUT"]
    #[inline(always)]
    #[must_use]
    pub fn unlock(&mut self) -> UNLOCK_W<1> {
        UNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LUT Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lutcr](index.html) module"]
pub struct LUTCR_SPEC;
impl crate::RegisterSpec for LUTCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lutcr::R](R) reader structure"]
impl crate::Readable for LUTCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lutcr::W](W) writer structure"]
impl crate::Writable for LUTCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LUTCR to value 0x02"]
impl crate::Resettable for LUTCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
