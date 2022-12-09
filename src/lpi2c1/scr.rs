#[doc = "Register `SCR` reader"]
pub struct R(crate::R<SCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCR` writer"]
pub struct W(crate::W<SCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCR_SPEC>;
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
impl From<crate::W<SCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SEN` reader - Slave Enable"]
pub type SEN_R = crate::BitReader<SEN_A>;
#[doc = "Slave Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SEN_A {
    #[doc = "0: I2C Slave mode is disabled"]
    DISABLED = 0,
    #[doc = "1: I2C Slave mode is enabled"]
    ENABLED = 1,
}
impl From<SEN_A> for bool {
    #[inline(always)]
    fn from(variant: SEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SEN_A {
        match self.bits {
            false => SEN_A::DISABLED,
            true => SEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SEN_A::ENABLED
    }
}
#[doc = "Field `SEN` writer - Slave Enable"]
pub type SEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, SEN_A, O>;
impl<'a, const O: u8> SEN_W<'a, O> {
    #[doc = "I2C Slave mode is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SEN_A::DISABLED)
    }
    #[doc = "I2C Slave mode is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SEN_A::ENABLED)
    }
}
#[doc = "Field `RST` reader - Software Reset"]
pub type RST_R = crate::BitReader<RST_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RST_A {
    #[doc = "0: Slave mode logic is not reset"]
    NOT_RESET = 0,
    #[doc = "1: Slave mode logic is reset"]
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
pub type RST_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, RST_A, O>;
impl<'a, const O: u8> RST_W<'a, O> {
    #[doc = "Slave mode logic is not reset"]
    #[inline(always)]
    pub fn not_reset(self) -> &'a mut W {
        self.variant(RST_A::NOT_RESET)
    }
    #[doc = "Slave mode logic is reset"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RST_A::RESET)
    }
}
#[doc = "Field `FILTEN` reader - Filter Enable"]
pub type FILTEN_R = crate::BitReader<FILTEN_A>;
#[doc = "Filter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILTEN_A {
    #[doc = "0: Disable digital filter and output delay counter for slave mode"]
    DISABLE = 0,
    #[doc = "1: Enable digital filter and output delay counter for slave mode"]
    ENABLE = 1,
}
impl From<FILTEN_A> for bool {
    #[inline(always)]
    fn from(variant: FILTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FILTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTEN_A {
        match self.bits {
            false => FILTEN_A::DISABLE,
            true => FILTEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FILTEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FILTEN_A::ENABLE
    }
}
#[doc = "Field `FILTEN` writer - Filter Enable"]
pub type FILTEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, FILTEN_A, O>;
impl<'a, const O: u8> FILTEN_W<'a, O> {
    #[doc = "Disable digital filter and output delay counter for slave mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FILTEN_A::DISABLE)
    }
    #[doc = "Enable digital filter and output delay counter for slave mode"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FILTEN_A::ENABLE)
    }
}
#[doc = "Field `FILTDZ` reader - Filter Doze Enable"]
pub type FILTDZ_R = crate::BitReader<FILTDZ_A>;
#[doc = "Filter Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FILTDZ_A {
    #[doc = "0: Filter remains enabled in Doze mode"]
    FILTER_ENABLED = 0,
    #[doc = "1: Filter is disabled in Doze mode"]
    FILTER_DISABLED = 1,
}
impl From<FILTDZ_A> for bool {
    #[inline(always)]
    fn from(variant: FILTDZ_A) -> Self {
        variant as u8 != 0
    }
}
impl FILTDZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTDZ_A {
        match self.bits {
            false => FILTDZ_A::FILTER_ENABLED,
            true => FILTDZ_A::FILTER_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_ENABLED`"]
    #[inline(always)]
    pub fn is_filter_enabled(&self) -> bool {
        *self == FILTDZ_A::FILTER_ENABLED
    }
    #[doc = "Checks if the value of the field is `FILTER_DISABLED`"]
    #[inline(always)]
    pub fn is_filter_disabled(&self) -> bool {
        *self == FILTDZ_A::FILTER_DISABLED
    }
}
#[doc = "Field `FILTDZ` writer - Filter Doze Enable"]
pub type FILTDZ_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, FILTDZ_A, O>;
impl<'a, const O: u8> FILTDZ_W<'a, O> {
    #[doc = "Filter remains enabled in Doze mode"]
    #[inline(always)]
    pub fn filter_enabled(self) -> &'a mut W {
        self.variant(FILTDZ_A::FILTER_ENABLED)
    }
    #[doc = "Filter is disabled in Doze mode"]
    #[inline(always)]
    pub fn filter_disabled(self) -> &'a mut W {
        self.variant(FILTDZ_A::FILTER_DISABLED)
    }
}
#[doc = "Field `RTF` reader - Reset Transmit FIFO"]
pub type RTF_R = crate::BitReader<RTF_A>;
#[doc = "Reset Transmit FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTF_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Transmit Data Register is now empty"]
    NOW_EMPTY = 1,
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
            true => RTF_A::NOW_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RTF_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `NOW_EMPTY`"]
    #[inline(always)]
    pub fn is_now_empty(&self) -> bool {
        *self == RTF_A::NOW_EMPTY
    }
}
#[doc = "Field `RTF` writer - Reset Transmit FIFO"]
pub type RTF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, RTF_A, O>;
impl<'a, const O: u8> RTF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RTF_A::NO_EFFECT)
    }
    #[doc = "Transmit Data Register is now empty"]
    #[inline(always)]
    pub fn now_empty(self) -> &'a mut W {
        self.variant(RTF_A::NOW_EMPTY)
    }
}
#[doc = "Field `RRF` reader - Reset Receive FIFO"]
pub type RRF_R = crate::BitReader<RRF_A>;
#[doc = "Reset Receive FIFO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RRF_A {
    #[doc = "0: No effect"]
    NO_EFFECT = 0,
    #[doc = "1: Receive Data Register is now empty"]
    NOW_EMPTY = 1,
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
            true => RRF_A::NOW_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RRF_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `NOW_EMPTY`"]
    #[inline(always)]
    pub fn is_now_empty(&self) -> bool {
        *self == RRF_A::NOW_EMPTY
    }
}
#[doc = "Field `RRF` writer - Reset Receive FIFO"]
pub type RRF_W<'a, const O: u8> = crate::BitWriter<'a, u32, SCR_SPEC, RRF_A, O>;
impl<'a, const O: u8> RRF_W<'a, O> {
    #[doc = "No effect"]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RRF_A::NO_EFFECT)
    }
    #[doc = "Receive Data Register is now empty"]
    #[inline(always)]
    pub fn now_empty(self) -> &'a mut W {
        self.variant(RRF_A::NOW_EMPTY)
    }
}
impl R {
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    pub fn sen(&self) -> SEN_R {
        SEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn rst(&self) -> RST_R {
        RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline(always)]
    pub fn filten(&self) -> FILTEN_R {
        FILTEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline(always)]
    pub fn filtdz(&self) -> FILTDZ_R {
        FILTDZ_R::new(((self.bits >> 5) & 1) != 0)
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
    #[doc = "Bit 0 - Slave Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sen(&mut self) -> SEN_W<0> {
        SEN_W::new(self)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn rst(&mut self) -> RST_W<1> {
        RST_W::new(self)
    }
    #[doc = "Bit 4 - Filter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filten(&mut self) -> FILTEN_W<4> {
        FILTEN_W::new(self)
    }
    #[doc = "Bit 5 - Filter Doze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn filtdz(&mut self) -> FILTDZ_W<5> {
        FILTDZ_W::new(self)
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
#[doc = "Slave Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scr](index.html) module"]
pub struct SCR_SPEC;
impl crate::RegisterSpec for SCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scr::R](R) reader structure"]
impl crate::Readable for SCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scr::W](W) writer structure"]
impl crate::Writable for SCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCR to value 0"]
impl crate::Resettable for SCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
