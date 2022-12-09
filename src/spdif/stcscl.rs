#[doc = "Register `STCSCL` reader"]
pub struct R(crate::R<STCSCL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCSCL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCSCL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCSCL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCSCL` writer"]
pub struct W(crate::W<STCSCL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCSCL_SPEC>;
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
impl From<crate::W<STCSCL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCSCL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TxCChannelCons_l` reader - SPDIF transmit Cons"]
pub type TX_CCHANNEL_CONS_L_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TxCChannelCons_l` writer - SPDIF transmit Cons"]
pub type TX_CCHANNEL_CONS_L_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STCSCL_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - SPDIF transmit Cons"]
    #[inline(always)]
    pub fn tx_cchannel_cons_l(&self) -> TX_CCHANNEL_CONS_L_R {
        TX_CCHANNEL_CONS_L_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - SPDIF transmit Cons"]
    #[inline(always)]
    #[must_use]
    pub fn tx_cchannel_cons_l(&mut self) -> TX_CCHANNEL_CONS_L_W<0> {
        TX_CCHANNEL_CONS_L_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPDIFTxCChannelCons_l Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcscl](index.html) module"]
pub struct STCSCL_SPEC;
impl crate::RegisterSpec for STCSCL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcscl::R](R) reader structure"]
impl crate::Readable for STCSCL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcscl::W](W) writer structure"]
impl crate::Writable for STCSCL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCSCL to value 0"]
impl crate::Resettable for STCSCL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
