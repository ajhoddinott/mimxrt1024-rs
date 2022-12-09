#[doc = "Register `TIMCTL[%s]` reader"]
pub struct R(crate::R<TIMCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TIMCTL[%s]` writer"]
pub struct W(crate::W<TIMCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TIMCTL_SPEC>;
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
impl From<crate::W<TIMCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TIMCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMOD` reader - Timer Mode"]
pub type TIMOD_R = crate::FieldReader<u8, TIMOD_A>;
#[doc = "Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TIMOD_A {
    #[doc = "0: Timer Disabled."]
    DISABLE = 0,
    #[doc = "1: Dual 8-bit counters baud mode."]
    DUAL8BIT_BAUD = 1,
    #[doc = "2: Dual 8-bit counters PWM high mode."]
    DUAL8BIT_PWM_H = 2,
    #[doc = "3: Single 16-bit counter mode."]
    SINGLE16BIT = 3,
}
impl From<TIMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: TIMOD_A) -> Self {
        variant as _
    }
}
impl TIMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMOD_A {
        match self.bits {
            0 => TIMOD_A::DISABLE,
            1 => TIMOD_A::DUAL8BIT_BAUD,
            2 => TIMOD_A::DUAL8BIT_PWM_H,
            3 => TIMOD_A::SINGLE16BIT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == TIMOD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `DUAL8BIT_BAUD`"]
    #[inline(always)]
    pub fn is_dual8bit_baud(&self) -> bool {
        *self == TIMOD_A::DUAL8BIT_BAUD
    }
    #[doc = "Checks if the value of the field is `DUAL8BIT_PWM_H`"]
    #[inline(always)]
    pub fn is_dual8bit_pwm_h(&self) -> bool {
        *self == TIMOD_A::DUAL8BIT_PWM_H
    }
    #[doc = "Checks if the value of the field is `SINGLE16BIT`"]
    #[inline(always)]
    pub fn is_single16bit(&self) -> bool {
        *self == TIMOD_A::SINGLE16BIT
    }
}
#[doc = "Field `TIMOD` writer - Timer Mode"]
pub type TIMOD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, TIMCTL_SPEC, u8, TIMOD_A, 2, O>;
impl<'a, const O: u8> TIMOD_W<'a, O> {
    #[doc = "Timer Disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(TIMOD_A::DISABLE)
    }
    #[doc = "Dual 8-bit counters baud mode."]
    #[inline(always)]
    pub fn dual8bit_baud(self) -> &'a mut W {
        self.variant(TIMOD_A::DUAL8BIT_BAUD)
    }
    #[doc = "Dual 8-bit counters PWM high mode."]
    #[inline(always)]
    pub fn dual8bit_pwm_h(self) -> &'a mut W {
        self.variant(TIMOD_A::DUAL8BIT_PWM_H)
    }
    #[doc = "Single 16-bit counter mode."]
    #[inline(always)]
    pub fn single16bit(self) -> &'a mut W {
        self.variant(TIMOD_A::SINGLE16BIT)
    }
}
#[doc = "Field `PINPOL` reader - Timer Pin Polarity"]
pub type PINPOL_R = crate::BitReader<PINPOL_A>;
#[doc = "Timer Pin Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINPOL_A {
    #[doc = "0: Pin is active high"]
    ACTIVE_HIGH = 0,
    #[doc = "1: Pin is active low"]
    ACTIVE_LOW = 1,
}
impl From<PINPOL_A> for bool {
    #[inline(always)]
    fn from(variant: PINPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl PINPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINPOL_A {
        match self.bits {
            false => PINPOL_A::ACTIVE_HIGH,
            true => PINPOL_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == PINPOL_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == PINPOL_A::ACTIVE_LOW
    }
}
#[doc = "Field `PINPOL` writer - Timer Pin Polarity"]
pub type PINPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCTL_SPEC, PINPOL_A, O>;
impl<'a, const O: u8> PINPOL_W<'a, O> {
    #[doc = "Pin is active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(PINPOL_A::ACTIVE_HIGH)
    }
    #[doc = "Pin is active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(PINPOL_A::ACTIVE_LOW)
    }
}
#[doc = "Field `PINSEL` reader - Timer Pin Select"]
pub type PINSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINSEL` writer - Timer Pin Select"]
pub type PINSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMCTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PINCFG` reader - Timer Pin Configuration"]
pub type PINCFG_R = crate::FieldReader<u8, PINCFG_A>;
#[doc = "Timer Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: Timer pin output disabled"]
    OUTDISABLE = 0,
    #[doc = "1: Timer pin open drain or bidirectional output enable"]
    OPEND_BIDIROUTEN = 1,
    #[doc = "2: Timer pin bidirectional output data"]
    BIDIR_OUTDATA = 2,
    #[doc = "3: Timer pin output"]
    OUTPUT = 3,
}
impl From<PINCFG_A> for u8 {
    #[inline(always)]
    fn from(variant: PINCFG_A) -> Self {
        variant as _
    }
}
impl PINCFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINCFG_A {
        match self.bits {
            0 => PINCFG_A::OUTDISABLE,
            1 => PINCFG_A::OPEND_BIDIROUTEN,
            2 => PINCFG_A::BIDIR_OUTDATA,
            3 => PINCFG_A::OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OUTDISABLE`"]
    #[inline(always)]
    pub fn is_outdisable(&self) -> bool {
        *self == PINCFG_A::OUTDISABLE
    }
    #[doc = "Checks if the value of the field is `OPEND_BIDIROUTEN`"]
    #[inline(always)]
    pub fn is_opend_bidirouten(&self) -> bool {
        *self == PINCFG_A::OPEND_BIDIROUTEN
    }
    #[doc = "Checks if the value of the field is `BIDIR_OUTDATA`"]
    #[inline(always)]
    pub fn is_bidir_outdata(&self) -> bool {
        *self == PINCFG_A::BIDIR_OUTDATA
    }
    #[doc = "Checks if the value of the field is `OUTPUT`"]
    #[inline(always)]
    pub fn is_output(&self) -> bool {
        *self == PINCFG_A::OUTPUT
    }
}
#[doc = "Field `PINCFG` writer - Timer Pin Configuration"]
pub type PINCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, TIMCTL_SPEC, u8, PINCFG_A, 2, O>;
impl<'a, const O: u8> PINCFG_W<'a, O> {
    #[doc = "Timer pin output disabled"]
    #[inline(always)]
    pub fn outdisable(self) -> &'a mut W {
        self.variant(PINCFG_A::OUTDISABLE)
    }
    #[doc = "Timer pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn opend_bidirouten(self) -> &'a mut W {
        self.variant(PINCFG_A::OPEND_BIDIROUTEN)
    }
    #[doc = "Timer pin bidirectional output data"]
    #[inline(always)]
    pub fn bidir_outdata(self) -> &'a mut W {
        self.variant(PINCFG_A::BIDIR_OUTDATA)
    }
    #[doc = "Timer pin output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PINCFG_A::OUTPUT)
    }
}
#[doc = "Field `TRGSRC` reader - Trigger Source"]
pub type TRGSRC_R = crate::BitReader<TRGSRC_A>;
#[doc = "Trigger Source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGSRC_A {
    #[doc = "0: External trigger selected"]
    EXT_TRIG = 0,
    #[doc = "1: Internal trigger selected"]
    INTERNAL_TRIG = 1,
}
impl From<TRGSRC_A> for bool {
    #[inline(always)]
    fn from(variant: TRGSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGSRC_A {
        match self.bits {
            false => TRGSRC_A::EXT_TRIG,
            true => TRGSRC_A::INTERNAL_TRIG,
        }
    }
    #[doc = "Checks if the value of the field is `EXT_TRIG`"]
    #[inline(always)]
    pub fn is_ext_trig(&self) -> bool {
        *self == TRGSRC_A::EXT_TRIG
    }
    #[doc = "Checks if the value of the field is `INTERNAL_TRIG`"]
    #[inline(always)]
    pub fn is_internal_trig(&self) -> bool {
        *self == TRGSRC_A::INTERNAL_TRIG
    }
}
#[doc = "Field `TRGSRC` writer - Trigger Source"]
pub type TRGSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCTL_SPEC, TRGSRC_A, O>;
impl<'a, const O: u8> TRGSRC_W<'a, O> {
    #[doc = "External trigger selected"]
    #[inline(always)]
    pub fn ext_trig(self) -> &'a mut W {
        self.variant(TRGSRC_A::EXT_TRIG)
    }
    #[doc = "Internal trigger selected"]
    #[inline(always)]
    pub fn internal_trig(self) -> &'a mut W {
        self.variant(TRGSRC_A::INTERNAL_TRIG)
    }
}
#[doc = "Field `TRGPOL` reader - Trigger Polarity"]
pub type TRGPOL_R = crate::BitReader<TRGPOL_A>;
#[doc = "Trigger Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRGPOL_A {
    #[doc = "0: Trigger active high"]
    ACTIVE_HIGH = 0,
    #[doc = "1: Trigger active low"]
    ACTIVE_LOW = 1,
}
impl From<TRGPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TRGPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TRGPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRGPOL_A {
        match self.bits {
            false => TRGPOL_A::ACTIVE_HIGH,
            true => TRGPOL_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == TRGPOL_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == TRGPOL_A::ACTIVE_LOW
    }
}
#[doc = "Field `TRGPOL` writer - Trigger Polarity"]
pub type TRGPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, TIMCTL_SPEC, TRGPOL_A, O>;
impl<'a, const O: u8> TRGPOL_W<'a, O> {
    #[doc = "Trigger active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(TRGPOL_A::ACTIVE_HIGH)
    }
    #[doc = "Trigger active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(TRGPOL_A::ACTIVE_LOW)
    }
}
#[doc = "Field `TRGSEL` reader - Trigger Select"]
pub type TRGSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRGSEL` writer - Trigger Select"]
pub type TRGSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TIMCTL_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    pub fn timod(&self) -> TIMOD_R {
        TIMOD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&self) -> PINPOL_R {
        PINPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Timer Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline(always)]
    pub fn trgsrc(&self) -> TRGSRC_R {
        TRGSRC_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline(always)]
    pub fn trgpol(&self) -> TRGPOL_R {
        TRGPOL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:29 - Trigger Select"]
    #[inline(always)]
    pub fn trgsel(&self) -> TRGSEL_R {
        TRGSEL_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timer Mode"]
    #[inline(always)]
    #[must_use]
    pub fn timod(&mut self) -> TIMOD_W<0> {
        TIMOD_W::new(self)
    }
    #[doc = "Bit 7 - Timer Pin Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pinpol(&mut self) -> PINPOL_W<7> {
        PINPOL_W::new(self)
    }
    #[doc = "Bits 8:12 - Timer Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn pinsel(&mut self) -> PINSEL_W<8> {
        PINSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Timer Pin Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pincfg(&mut self) -> PINCFG_W<16> {
        PINCFG_W::new(self)
    }
    #[doc = "Bit 22 - Trigger Source"]
    #[inline(always)]
    #[must_use]
    pub fn trgsrc(&mut self) -> TRGSRC_W<22> {
        TRGSRC_W::new(self)
    }
    #[doc = "Bit 23 - Trigger Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn trgpol(&mut self) -> TRGPOL_W<23> {
        TRGPOL_W::new(self)
    }
    #[doc = "Bits 24:29 - Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn trgsel(&mut self) -> TRGSEL_W<24> {
        TRGSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timctl](index.html) module"]
pub struct TIMCTL_SPEC;
impl crate::RegisterSpec for TIMCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timctl::R](R) reader structure"]
impl crate::Readable for TIMCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [timctl::W](W) writer structure"]
impl crate::Writable for TIMCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMCTL[%s]
to value 0"]
impl crate::Resettable for TIMCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
