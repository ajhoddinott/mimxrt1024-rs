#[doc = "Register `DMA2` reader"]
pub struct R(crate::R<DMA2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA2` writer"]
pub struct W(crate::W<DMA2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA2_SPEC>;
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
impl From<crate::W<DMA2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IEFDE` reader - Input Edge Flag DMA Enable"]
pub type IEFDE_R = crate::BitReader<bool>;
#[doc = "Field `IEFDE` writer - Input Edge Flag DMA Enable"]
pub type IEFDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2_SPEC, bool, O>;
#[doc = "Field `CMPLD1DE` reader - Comparator Preload Register 1 DMA Enable"]
pub type CMPLD1DE_R = crate::BitReader<bool>;
#[doc = "Field `CMPLD1DE` writer - Comparator Preload Register 1 DMA Enable"]
pub type CMPLD1DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2_SPEC, bool, O>;
#[doc = "Field `CMPLD2DE` reader - Comparator Preload Register 2 DMA Enable"]
pub type CMPLD2DE_R = crate::BitReader<bool>;
#[doc = "Field `CMPLD2DE` writer - Comparator Preload Register 2 DMA Enable"]
pub type CMPLD2DE_W<'a, const O: u8> = crate::BitWriter<'a, u16, DMA2_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Input Edge Flag DMA Enable"]
    #[inline(always)]
    pub fn iefde(&self) -> IEFDE_R {
        IEFDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    pub fn cmpld1de(&self) -> CMPLD1DE_R {
        CMPLD1DE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    pub fn cmpld2de(&self) -> CMPLD2DE_R {
        CMPLD2DE_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Input Edge Flag DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn iefde(&mut self) -> IEFDE_W<0> {
        IEFDE_W::new(self)
    }
    #[doc = "Bit 1 - Comparator Preload Register 1 DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpld1de(&mut self) -> CMPLD1DE_W<1> {
        CMPLD1DE_W::new(self)
    }
    #[doc = "Bit 2 - Comparator Preload Register 2 DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpld2de(&mut self) -> CMPLD2DE_W<2> {
        CMPLD2DE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Channel DMA Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma2](index.html) module"]
pub struct DMA2_SPEC;
impl crate::RegisterSpec for DMA2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [dma2::R](R) reader structure"]
impl crate::Readable for DMA2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma2::W](W) writer structure"]
impl crate::Writable for DMA2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA2 to value 0"]
impl crate::Resettable for DMA2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
