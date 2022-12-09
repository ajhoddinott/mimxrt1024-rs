#[doc = "Register `MIER` reader"]
pub struct R(crate::R<MIER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIER` writer"]
pub struct W(crate::W<MIER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIER_SPEC>;
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
impl From<crate::W<MIER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDIE` reader - Transmit Data Interrupt Enable"]
pub type TDIE_R = crate::BitReader<TDIE_A>;
#[doc = "Transmit Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<TDIE_A> for bool {
    #[inline(always)]
    fn from(variant: TDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDIE_A {
        match self.bits {
            false => TDIE_A::DISABLED,
            true => TDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDIE_A::ENABLED
    }
}
#[doc = "Field `TDIE` writer - Transmit Data Interrupt Enable"]
pub type TDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIER_SPEC, TDIE_A, O>;
impl<'a, const O: u8> TDIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDIE_A::ENABLED)
    }
}
#[doc = "Field `RDIE` reader - Receive Data Interrupt Enable"]
pub type RDIE_R = crate::BitReader<RDIE_A>;
#[doc = "Receive Data Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<RDIE_A> for bool {
    #[inline(always)]
    fn from(variant: RDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDIE_A {
        match self.bits {
            false => RDIE_A::DISABLED,
            true => RDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDIE_A::ENABLED
    }
}
#[doc = "Field `RDIE` writer - Receive Data Interrupt Enable"]
pub type RDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIER_SPEC, RDIE_A, O>;
impl<'a, const O: u8> RDIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDIE_A::ENABLED)
    }
}
#[doc = "Field `EPIE` reader - End Packet Interrupt Enable"]
pub type EPIE_R = crate::BitReader<EPIE_A>;
#[doc = "End Packet Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EPIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<EPIE_A> for bool {
    #[inline(always)]
    fn from(variant: EPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EPIE_A {
        match self.bits {
            false => EPIE_A::DISABLED,
            true => EPIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EPIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EPIE_A::ENABLED
    }
}
#[doc = "Field `EPIE` writer - End Packet Interrupt Enable"]
pub type EPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIER_SPEC, EPIE_A, O>;
impl<'a, const O: u8> EPIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EPIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EPIE_A::ENABLED)
    }
}
#[doc = "Field `SDIE` reader - STOP Detect Interrupt Enable"]
pub type SDIE_R = crate::BitReader<SDIE_A>;
#[doc = "STOP Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SDIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<SDIE_A> for bool {
    #[inline(always)]
    fn from(variant: SDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl SDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SDIE_A {
        match self.bits {
            false => SDIE_A::DISABLED,
            true => SDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SDIE_A::ENABLED
    }
}
#[doc = "Field `SDIE` writer - STOP Detect Interrupt Enable"]
pub type SDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIER_SPEC, SDIE_A, O>;
impl<'a, const O: u8> SDIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SDIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SDIE_A::ENABLED)
    }
}
#[doc = "Field `NDIE` reader - NACK Detect Interrupt Enable"]
pub type NDIE_R = crate::BitReader<NDIE_A>;
#[doc = "NACK Detect Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<NDIE_A> for bool {
    #[inline(always)]
    fn from(variant: NDIE_A) -> Self {
        variant as u8 != 0
    }
}
impl NDIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDIE_A {
        match self.bits {
            false => NDIE_A::DISABLED,
            true => NDIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NDIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NDIE_A::ENABLED
    }
}
#[doc = "Field `NDIE` writer - NACK Detect Interrupt Enable"]
pub type NDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIER_SPEC, NDIE_A, O>;
impl<'a, const O: u8> NDIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NDIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NDIE_A::ENABLED)
    }
}
#[doc = "Field `ALIE` reader - Arbitration Lost Interrupt Enable"]
pub type ALIE_R = crate::BitReader<ALIE_A>;
#[doc = "Arbitration Lost Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<ALIE_A> for bool {
    #[inline(always)]
    fn from(variant: ALIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ALIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALIE_A {
        match self.bits {
            false => ALIE_A::DISABLED,
            true => ALIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALIE_A::ENABLED
    }
}
#[doc = "Field `ALIE` writer - Arbitration Lost Interrupt Enable"]
pub type ALIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIER_SPEC, ALIE_A, O>;
impl<'a, const O: u8> ALIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALIE_A::ENABLED)
    }
}
#[doc = "Field `FEIE` reader - FIFO Error Interrupt Enable"]
pub type FEIE_R = crate::BitReader<FEIE_A>;
#[doc = "FIFO Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIE_A {
    #[doc = "0: Enabled"]
    ENABLED = 0,
    #[doc = "1: Disabled"]
    DISABLED = 1,
}
impl From<FEIE_A> for bool {
    #[inline(always)]
    fn from(variant: FEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl FEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FEIE_A {
        match self.bits {
            false => FEIE_A::ENABLED,
            true => FEIE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FEIE_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FEIE_A::DISABLED
    }
}
#[doc = "Field `FEIE` writer - FIFO Error Interrupt Enable"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIER_SPEC, FEIE_A, O>;
impl<'a, const O: u8> FEIE_W<'a, O> {
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FEIE_A::ENABLED)
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FEIE_A::DISABLED)
    }
}
#[doc = "Field `PLTIE` reader - Pin Low Timeout Interrupt Enable"]
pub type PLTIE_R = crate::BitReader<PLTIE_A>;
#[doc = "Pin Low Timeout Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLTIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<PLTIE_A> for bool {
    #[inline(always)]
    fn from(variant: PLTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLTIE_A {
        match self.bits {
            false => PLTIE_A::DISABLED,
            true => PLTIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PLTIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PLTIE_A::ENABLED
    }
}
#[doc = "Field `PLTIE` writer - Pin Low Timeout Interrupt Enable"]
pub type PLTIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIER_SPEC, PLTIE_A, O>;
impl<'a, const O: u8> PLTIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PLTIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PLTIE_A::ENABLED)
    }
}
#[doc = "Field `DMIE` reader - Data Match Interrupt Enable"]
pub type DMIE_R = crate::BitReader<DMIE_A>;
#[doc = "Data Match Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMIE_A {
    #[doc = "0: Disabled"]
    DISABLED = 0,
    #[doc = "1: Enabled"]
    ENABLED = 1,
}
impl From<DMIE_A> for bool {
    #[inline(always)]
    fn from(variant: DMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMIE_A {
        match self.bits {
            false => DMIE_A::DISABLED,
            true => DMIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMIE_A::ENABLED
    }
}
#[doc = "Field `DMIE` writer - Data Match Interrupt Enable"]
pub type DMIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIER_SPEC, DMIE_A, O>;
impl<'a, const O: u8> DMIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMIE_A::DISABLED)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    pub fn tdie(&self) -> TDIE_R {
        TDIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    pub fn rdie(&self) -> RDIE_R {
        RDIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - End Packet Interrupt Enable"]
    #[inline(always)]
    pub fn epie(&self) -> EPIE_R {
        EPIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    pub fn sdie(&self) -> SDIE_R {
        SDIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NACK Detect Interrupt Enable"]
    #[inline(always)]
    pub fn ndie(&self) -> NDIE_R {
        NDIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    pub fn alie(&self) -> ALIE_R {
        ALIE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Pin Low Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn pltie(&self) -> PLTIE_R {
        PLTIE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Data Match Interrupt Enable"]
    #[inline(always)]
    pub fn dmie(&self) -> DMIE_R {
        DMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdie(&mut self) -> TDIE_W<0> {
        TDIE_W::new(self)
    }
    #[doc = "Bit 1 - Receive Data Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdie(&mut self) -> RDIE_W<1> {
        RDIE_W::new(self)
    }
    #[doc = "Bit 8 - End Packet Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn epie(&mut self) -> EPIE_W<8> {
        EPIE_W::new(self)
    }
    #[doc = "Bit 9 - STOP Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn sdie(&mut self) -> SDIE_W<9> {
        SDIE_W::new(self)
    }
    #[doc = "Bit 10 - NACK Detect Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ndie(&mut self) -> NDIE_W<10> {
        NDIE_W::new(self)
    }
    #[doc = "Bit 11 - Arbitration Lost Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn alie(&mut self) -> ALIE_W<11> {
        ALIE_W::new(self)
    }
    #[doc = "Bit 12 - FIFO Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<12> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 13 - Pin Low Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pltie(&mut self) -> PLTIE_W<13> {
        PLTIE_W::new(self)
    }
    #[doc = "Bit 14 - Data Match Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmie(&mut self) -> DMIE_W<14> {
        DMIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Interrupt Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mier](index.html) module"]
pub struct MIER_SPEC;
impl crate::RegisterSpec for MIER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mier::R](R) reader structure"]
impl crate::Readable for MIER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mier::W](W) writer structure"]
impl crate::Writable for MIER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIER to value 0"]
impl crate::Resettable for MIER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
