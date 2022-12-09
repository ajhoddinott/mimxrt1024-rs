#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEN` reader - Master Enable"]
pub type MEN_R = crate::BitReader<MEN_A>;
#[doc = "Master Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEN_A {
    #[doc = "0: Master logic is disabled"]
    DISABLED = 0,
    #[doc = "1: Master logic is enabled"]
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
#[doc = "Field `MEN` writer - Master Enable"]
pub type MEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MEN_A, O>;
impl<'a, const O: u8> MEN_W<'a, O> {
    #[doc = "Master logic is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MEN_A::DISABLED)
    }
    #[doc = "Master logic is enabled"]
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
    #[doc = "0: Master logic is not reset"]
    NOT_RESET = 0,
    #[doc = "1: Master logic is reset"]
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
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, RST_A, O>;
impl<'a, const O: u8> RST_W<'a, O> {
    #[doc = "Master logic is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(RST_A::NOT_RESET)
    }
    #[doc = "Master logic is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RST_A::RESET)
    }
}
#[doc = "Field `DOZEN` reader - Doze mode enable"]
pub type DOZEN_R = crate::BitReader<DOZEN_A>;
#[doc = "Doze mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOZEN_A {
    #[doc = "0: Master is enabled in Doze mode"]
    ENABLED = 0,
    #[doc = "1: Master is disabled in Doze mode"]
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
#[doc = "Field `DOZEN` writer - Doze mode enable"]
pub type DOZEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, DOZEN_A, O>;
impl<'a, const O: u8> DOZEN_W<'a, O> {
    #[doc = "Master is enabled in Doze mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOZEN_A::ENABLED)
    }
    #[doc = "Master is disabled in Doze mode"]
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
    #[doc = "0: Master is disabled in debug mode"]
    DISABLED = 0,
    #[doc = "1: Master is enabled in debug mode"]
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
pub type DBGEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, DBGEN_A, O>;
impl<'a, const O: u8> DBGEN_W<'a, O> {
    #[doc = "Master is disabled in debug mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DBGEN_A::DISABLED)
    }
    #[doc = "Master is enabled in debug mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DBGEN_A::ENABLED)
    }
}
#[doc = "Field `RTF` reader - Reset Transmit FIFO"]
pub type RTF_R = crate::BitReader<RTF_A>;
#[doc = "Reset Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTF_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Transmit FIFO is reset"]
    RESET = 1,
}
impl From<RTF_A> for bool {
    #[inline(always)]
    fn from(variant: RTF_A) -> Self {
        variant as u8 != 0
    }
}
impl RTF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTF_A {
        match self.bits {
            false => RTF_A::NO_EFFECT,
            true => RTF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RTF_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RTF_A::RESET
    }
}
#[doc = "Field `RTF` writer - Reset Transmit FIFO"]
pub type RTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, RTF_A, O>;
impl<'a, const O: u8> RTF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RTF_A::NO_EFFECT)
    }
    #[doc = "Transmit FIFO is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RTF_A::RESET)
    }
}
#[doc = "Field `RRF` reader - Reset Receive FIFO"]
pub type RRF_R = crate::BitReader<RRF_A>;
#[doc = "Reset Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRF_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Receive FIFO is reset"]
    RESET = 1,
}
impl From<RRF_A> for bool {
    #[inline(always)]
    fn from(variant: RRF_A) -> Self {
        variant as u8 != 0
    }
}
impl RRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RRF_A {
        match self.bits {
            false => RRF_A::NO_EFFECT,
            true => RRF_A::RESET,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RRF_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RRF_A::RESET
    }
}
#[doc = "Field `RRF` writer - Reset Receive FIFO"]
pub type RRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, RRF_A, O>;
impl<'a, const O: u8> RRF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RRF_A::NO_EFFECT)
    }
    #[doc = "Receive FIFO is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RRF_A::RESET)
    }
}
impl R {
    #[doc = "Bit 0 - Master Enable"]
    #[inline(always)]
    pub fn men(&self) -> MEN_R {
        MEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Doze mode enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DOZEN_R {
        DOZEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Debug Enable"]
    #[inline(always)]
    pub fn dbgen(&self) -> DBGEN_R {
        DBGEN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 8 - Reset Transmit FIFO"]
    #[inline(always)]
    pub fn rtf(&self) -> RTF_R {
        RTF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset Receive FIFO"]
    #[inline(always)]
    pub fn rrf(&self) -> RRF_R {
        RRF_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Master Enable"]
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
    #[doc = "Bit 2 - Doze mode enable"]
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
#[doc = "Master Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
