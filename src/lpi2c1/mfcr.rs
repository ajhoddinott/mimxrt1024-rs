#[doc = "Register `MFCR` reader"]
pub struct R(crate::R<MFCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MFCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MFCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MFCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MFCR` writer"]
pub struct W(crate::W<MFCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MFCR_SPEC>;
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
impl From<crate::W<MFCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MFCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXWATER` reader - Transmit FIFO Watermark"]
pub type TXWATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXWATER` writer - Transmit FIFO Watermark"]
pub type TXWATER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MFCR_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXWATER` reader - Receive FIFO Watermark"]
pub type RXWATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXWATER` writer - Receive FIFO Watermark"]
pub type RXWATER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MFCR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Transmit FIFO Watermark"]
    #[inline(always)]
    pub fn txwater(&self) -> TXWATER_R {
        TXWATER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:17 - Receive FIFO Watermark"]
    #[inline(always)]
    pub fn rxwater(&self) -> RXWATER_R {
        RXWATER_R::new(((self.bits >> 16) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit FIFO Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn txwater(&mut self) -> TXWATER_W<0> {
        TXWATER_W::new(self)
    }
    #[doc = "Bits 16:17 - Receive FIFO Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn rxwater(&mut self) -> RXWATER_W<16> {
        RXWATER_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master FIFO Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mfcr](index.html) module"]
pub struct MFCR_SPEC;
impl crate::RegisterSpec for MFCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mfcr::R](R) reader structure"]
impl crate::Readable for MFCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mfcr::W](W) writer structure"]
impl crate::Writable for MFCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MFCR to value 0"]
impl crate::Resettable for MFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
