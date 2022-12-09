#[doc = "Register `BURSTSIZE` reader"]
pub struct R(crate::R<BURSTSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BURSTSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BURSTSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BURSTSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BURSTSIZE` writer"]
pub struct W(crate::W<BURSTSIZE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BURSTSIZE_SPEC>;
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
impl From<crate::W<BURSTSIZE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BURSTSIZE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXPBURST` reader - Programmable RX Burst Size"]
pub type RXPBURST_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RXPBURST` writer - Programmable RX Burst Size"]
pub type RXPBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BURSTSIZE_SPEC, u8, u8, 8, O>;
#[doc = "Field `TXPBURST` reader - Programmable TX Burst Size"]
pub type TXPBURST_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TXPBURST` writer - Programmable TX Burst Size"]
pub type TXPBURST_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BURSTSIZE_SPEC, u16, u16, 9, O>;
impl R {
    #[doc = "Bits 0:7 - Programmable RX Burst Size"]
    #[inline(always)]
    pub fn rxpburst(&self) -> RXPBURST_R {
        RXPBURST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:16 - Programmable TX Burst Size"]
    #[inline(always)]
    pub fn txpburst(&self) -> TXPBURST_R {
        TXPBURST_R::new(((self.bits >> 8) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - Programmable RX Burst Size"]
    #[inline(always)]
    #[must_use]
    pub fn rxpburst(&mut self) -> RXPBURST_W<0> {
        RXPBURST_W::new(self)
    }
    #[doc = "Bits 8:16 - Programmable TX Burst Size"]
    #[inline(always)]
    #[must_use]
    pub fn txpburst(&mut self) -> TXPBURST_W<8> {
        TXPBURST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Programmable Burst Size\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [burstsize](index.html) module"]
pub struct BURSTSIZE_SPEC;
impl crate::RegisterSpec for BURSTSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [burstsize::R](R) reader structure"]
impl crate::Readable for BURSTSIZE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [burstsize::W](W) writer structure"]
impl crate::Writable for BURSTSIZE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BURSTSIZE to value 0x0808"]
impl crate::Resettable for BURSTSIZE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0808;
}
