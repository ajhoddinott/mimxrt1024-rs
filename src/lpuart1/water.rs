#[doc = "Register `WATER` reader"]
pub struct R(crate::R<WATER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WATER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WATER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WATER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WATER` writer"]
pub struct W(crate::W<WATER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WATER_SPEC>;
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
impl From<crate::W<WATER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WATER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXWATER` reader - Transmit Watermark"]
pub type TXWATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXWATER` writer - Transmit Watermark"]
pub type TXWATER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WATER_SPEC, u8, u8, 2, O>;
#[doc = "Field `TXCOUNT` reader - Transmit Counter"]
pub type TXCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXWATER` reader - Receive Watermark"]
pub type RXWATER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXWATER` writer - Receive Watermark"]
pub type RXWATER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, WATER_SPEC, u8, u8, 2, O>;
#[doc = "Field `RXCOUNT` reader - Receive Counter"]
pub type RXCOUNT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - Transmit Watermark"]
    #[inline(always)]
    pub fn txwater(&self) -> TXWATER_R {
        TXWATER_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:10 - Transmit Counter"]
    #[inline(always)]
    pub fn txcount(&self) -> TXCOUNT_R {
        TXCOUNT_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 16:17 - Receive Watermark"]
    #[inline(always)]
    pub fn rxwater(&self) -> RXWATER_R {
        RXWATER_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 24:26 - Receive Counter"]
    #[inline(always)]
    pub fn rxcount(&self) -> RXCOUNT_R {
        RXCOUNT_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Transmit Watermark"]
    #[inline(always)]
    #[must_use]
    pub fn txwater(&mut self) -> TXWATER_W<0> {
        TXWATER_W::new(self)
    }
    #[doc = "Bits 16:17 - Receive Watermark"]
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
#[doc = "LPUART Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [water](index.html) module"]
pub struct WATER_SPEC;
impl crate::RegisterSpec for WATER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [water::R](R) reader structure"]
impl crate::Readable for WATER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [water::W](W) writer structure"]
impl crate::Writable for WATER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WATER to value 0"]
impl crate::Resettable for WATER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
