#[doc = "Register `STCSCH` reader"]
pub struct R(crate::R<STCSCH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STCSCH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STCSCH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STCSCH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STCSCH` writer"]
pub struct W(crate::W<STCSCH_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STCSCH_SPEC>;
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
impl From<crate::W<STCSCH_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STCSCH_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TxCChannelCons_h` reader - SPDIF transmit Cons"]
pub type TX_CCHANNEL_CONS_H_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TxCChannelCons_h` writer - SPDIF transmit Cons"]
pub type TX_CCHANNEL_CONS_H_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STCSCH_SPEC, u32, u32, 24, O>;
impl R {
    #[doc = "Bits 0:23 - SPDIF transmit Cons"]
    #[inline(always)]
    pub fn tx_cchannel_cons_h(&self) -> TX_CCHANNEL_CONS_H_R {
        TX_CCHANNEL_CONS_H_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - SPDIF transmit Cons"]
    #[inline(always)]
    #[must_use]
    pub fn tx_cchannel_cons_h(&mut self) -> TX_CCHANNEL_CONS_H_W<0> {
        TX_CCHANNEL_CONS_H_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPDIFTxCChannelCons_h Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stcsch](index.html) module"]
pub struct STCSCH_SPEC;
impl crate::RegisterSpec for STCSCH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stcsch::R](R) reader structure"]
impl crate::Readable for STCSCH_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stcsch::W](W) writer structure"]
impl crate::Writable for STCSCH_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STCSCH to value 0"]
impl crate::Resettable for STCSCH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
