#[doc = "Register `STL` writer"]
pub struct W(crate::W<STL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STL_SPEC>;
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
impl From<crate::W<STL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TxDataLeft` writer - SPDIF transmit left channel data. It is write-only, and always returns zeros when read"]
pub type TX_DATA_LEFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STL_SPEC, u32, u32, 24, O>;
impl W {
    #[doc = "Bits 0:23 - SPDIF transmit left channel data. It is write-only, and always returns zeros when read"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_left(&mut self) -> TX_DATA_LEFT_W<0> {
        TX_DATA_LEFT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPDIFTxLeft Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stl](index.html) module"]
pub struct STL_SPEC;
impl crate::RegisterSpec for STL_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [stl::W](W) writer structure"]
impl crate::Writable for STL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STL to value 0"]
impl crate::Resettable for STL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
