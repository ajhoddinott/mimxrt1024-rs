#[doc = "Register `PINCFG` reader"]
pub struct R(crate::R<PINCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PINCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PINCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PINCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PINCFG` writer"]
pub struct W(crate::W<PINCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PINCFG_SPEC>;
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
impl From<crate::W<PINCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PINCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Select"]
pub type TRGSEL_R = crate::FieldReader<u8, TRGSEL_A>;
#[doc = "Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRGSEL_A {
    #[doc = "0: Input trigger is disabled."]
    DISABLED = 0,
    #[doc = "1: Input trigger is used instead of RXD pin input."]
    TRG_RXD = 1,
    #[doc = "2: Input trigger is used instead of CTS_B pin input."]
    TRG_CTS = 2,
    #[doc = "3: Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is internally ANDed with the input trigger."]
    TRG_TXD = 3,
}
impl From<TRGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: TRGSEL_A) -> Self {
        variant as _
    }
}
impl TRGSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSEL_A {
        match self.bits {
            0 => TRGSEL_A::DISABLED,
            1 => TRGSEL_A::TRG_RXD,
            2 => TRGSEL_A::TRG_CTS,
            3 => TRGSEL_A::TRG_TXD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TRGSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `TRG_RXD`"]
    #[inline(always)]
    pub fn is_trg_rxd(&self) -> bool {
        *self == TRGSEL_A::TRG_RXD
    }
    #[doc = "Checks if the value of the field is `TRG_CTS`"]
    #[inline(always)]
    pub fn is_trg_cts(&self) -> bool {
        *self == TRGSEL_A::TRG_CTS
    }
    #[doc = "Checks if the value of the field is `TRG_TXD`"]
    #[inline(always)]
    pub fn is_trg_txd(&self) -> bool {
        *self == TRGSEL_A::TRG_TXD
    }
}
#[doc = "Field `TRGSEL` writer - Trigger Select"]
pub type TRGSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, PINCFG_SPEC, u8, TRGSEL_A, 2, O>;
impl<'a, const O: u8> TRGSEL_W<'a, O> {
    #[doc = "Input trigger is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TRGSEL_A::DISABLED)
    }
    #[doc = "Input trigger is used instead of RXD pin input."]
    #[inline(always)]
    pub fn trg_rxd(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRG_RXD)
    }
    #[doc = "Input trigger is used instead of CTS_B pin input."]
    #[inline(always)]
    pub fn trg_cts(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRG_CTS)
    }
    #[doc = "Input trigger is used to modulate the TXD pin output. The TXD pin output (after TXINV configuration) is internally ANDed with the input trigger."]
    #[inline(always)]
    pub fn trg_txd(self) -> &'a mut W {
        self.variant(TRGSEL_A::TRG_TXD)
    }
}
impl R {
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<0> {
        TRGSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Pin Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pincfg](index.html) module"]
pub struct PINCFG_SPEC;
impl crate::RegisterSpec for PINCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pincfg::R](R) reader structure"]
impl crate::Readable for PINCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pincfg::W](W) writer structure"]
impl crate::Writable for PINCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PINCFG to value 0"]
impl crate::Resettable for PINCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
