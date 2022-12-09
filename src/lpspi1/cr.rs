#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEN` reader - Module Enable"]
pub type MEN_R = crate::BitReader<MEN_A>;
#[doc = "Module Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEN_A {
    #[doc = "0: Module is disabled"]
    DISABLED = 0,
    #[doc = "1: Module is enabled"]
    ENABLED = 1,
}
impl From<MEN_A> for bool {
    #[inline(always)]
    fn from(variant: MEN_A) -> Self {
        variant as u8 != 0
    }
}
impl MEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEN_A {
        match self.bits {
            false => MEN_A::DISABLED,
            true => MEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MEN_A::ENABLED
    }
}
#[doc = "Field `MEN` writer - Module Enable"]
pub type MEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, MEN_A, O>;
impl<'a, const O: u8> MEN_W<'a, O> {
    #[doc = "Module is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MEN_A::DISABLED)
    }
    #[doc = "Module is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MEN_A::ENABLED)
    }
}
#[doc = "Field `RST` reader - Software Reset"]
pub type RST_R = crate::BitReader<RST_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    #[doc = "0: Module is not reset"]
    NOT_RESET = 0,
    #[doc = "1: Module is reset"]
    RESET = 1,
}
impl From<RST_A> for bool {
    #[inline(always)]
    fn from(variant: RST_A) -> Self {
        variant as u8 != 0
    }
}
impl RST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RST_A {
        match self.bits {
            false => RST_A::NOT_RESET,
            true => RST_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_RESET`"]
    #[inline(always)]
    pub fn is_not_reset(&self) -> bool {
        *self == RST_A::NOT_RESET
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RST_A::RESET
    }
}
#[doc = "Field `RST` writer - Software Reset"]
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RST_A, O>;
impl<'a, const O: u8> RST_W<'a, O> {
    #[doc = "Module is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(RST_A::NOT_RESET)
    }
    #[doc = "Module is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RST_A::RESET)
    }
}
#[doc = "Field `DOZEN` reader - Doze Mode Enable"]
pub type DOZEN_R = crate::BitReader<DOZEN_A>;
#[doc = "Doze Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOZEN_A {
    #[doc = "0: LPSPI module is enabled in Doze mode"]
    ENABLED = 0,
    #[doc = "1: LPSPI module is disabled in Doze mode"]
    DISABLED = 1,
}
impl From<DOZEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DOZEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEN_A {
        match self.bits {
            false => DOZEN_A::ENABLED,
            true => DOZEN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOZEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOZEN_A::DISABLED
    }
}
#[doc = "Field `DOZEN` writer - Doze Mode Enable"]
pub type DOZEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DOZEN_A, O>;
impl<'a, const O: u8> DOZEN_W<'a, O> {
    #[doc = "LPSPI module is enabled in Doze mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOZEN_A::ENABLED)
    }
    #[doc = "LPSPI module is disabled in Doze mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOZEN_A::DISABLED)
    }
}
#[doc = "Field `DBGEN` reader - Debug Enable"]
pub type DBGEN_R = crate::BitReader<DBGEN_A>;
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGEN_A {
    #[doc = "0: LPSPI module is disabled in debug mode"]
    DISABLED = 0,
    #[doc = "1: LPSPI module is enabled in debug mode"]
    ENABLED = 1,
}
impl From<DBGEN_A> for bool {
    #[inline(always)]
    fn from(variant: DBGEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGEN_A {
        match self.bits {
            false => DBGEN_A::DISABLED,
            true => DBGEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DBGEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DBGEN_A::ENABLED
    }
}
#[doc = "Field `DBGEN` writer - Debug Enable"]
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, DBGEN_A, O>;
impl<'a, const O: u8> DBGEN_W<'a, O> {
    #[doc = "LPSPI module is disabled in debug mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBGEN_A::DISABLED)
    }
    #[doc = "LPSPI module is enabled in debug mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBGEN_A::ENABLED)
    }
}
#[doc = "Reset Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTF_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Reset the Transmit FIFO. The register bit always reads zero."]
    TXFIFO_RST = 1,
}
impl From<RTF_AW> for bool {
    #[inline(always)]
    fn from(variant: RTF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RTF` writer - Reset Transmit FIFO"]
pub type RTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RTF_AW, O>;
impl<'a, const O: u8> RTF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RTF_AW::NO_EFFECT)
    }
    #[doc = "Reset the Transmit FIFO. The register bit always reads zero."]
    #[inline(always)]
    pub fn txfifo_rst(self) -> &'a mut W {
        self.variant(RTF_AW::TXFIFO_RST)
    }
}
#[doc = "Reset Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRF_AW {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Reset the Receive FIFO. The register bit always reads zero."]
    RXFIFO_RST = 1,
}
impl From<RRF_AW> for bool {
    #[inline(always)]
    fn from(variant: RRF_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRF` writer - Reset Receive FIFO"]
pub type RRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, CR_SPEC, RRF_AW, O>;
impl<'a, const O: u8> RRF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RRF_AW::NO_EFFECT)
    }
    #[doc = "Reset the Receive FIFO. The register bit always reads zero."]
    #[inline(always)]
    pub fn rxfifo_rst(self) -> &'a mut W {
        self.variant(RRF_AW::RXFIFO_RST)
    }
}
impl R {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    pub fn men(&self) -> MEN_R {
        MEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doze Mode Enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DOZEN_R {
        DOZEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Module Enable"]
    #[inline(always)]
    #[must_use]
    pub fn men(&mut self) -> MEN_W<0> {
        MEN_W::new(self)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<1> {
        RST_W::new(self)
    }
    #[doc = "Bit 2 - Doze Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dozen(&mut self) -> DOZEN_W<2> {
        DOZEN_W::new(self)
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbgen(&mut self) -> DBGEN_W<3> {
        DBGEN_W::new(self)
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rtf(&mut self) -> RTF_W<8> {
        RTF_W::new(self)
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline(always)]
    #[must_use]
    pub fn rrf(&mut self) -> RRF_W<9> {
        RRF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
