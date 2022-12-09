#[doc = "Register `STR` writer"]
pub struct W(crate::W<STR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STR_SPEC>;
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
impl From<crate::W<STR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TxDataRight` writer - SPDIF transmit right channel data. It is write-only, and always returns zeros when read"]
pub type TX_DATA_RIGHT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STR_SPEC, u32, u32, 24, O>;
impl W {
    #[doc = "Bits 0:23 - SPDIF transmit right channel data. It is write-only, and always returns zeros when read"]
    #[inline(always)]
    #[must_use]
    pub fn tx_data_right(&mut self) -> TX_DATA_RIGHT_W<0> {
        TX_DATA_RIGHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPDIFTxRight Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [str](index.html) module"]
pub struct STR_SPEC;
impl crate::RegisterSpec for STR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [str::W](W) writer structure"]
impl crate::Writable for STR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STR to value 0"]
impl crate::Resettable for STR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
