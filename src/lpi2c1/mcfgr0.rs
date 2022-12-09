#[doc = "Register `MCFGR0` reader"]
pub struct R(crate::R<MCFGR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFGR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFGR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFGR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFGR0` writer"]
pub struct W(crate::W<MCFGR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFGR0_SPEC>;
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
impl From<crate::W<MCFGR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFGR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HREN` reader - Host Request Enable"]
pub type HREN_R = crate::BitReader<HREN_A>;
#[doc = "Host Request Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HREN_A {
    #[doc = "0: Host request input is disabled"]
    DISABLED = 0,
    #[doc = "1: Host request input is enabled"]
    ENABLED = 1,
}
impl From<HREN_A> for bool {
    #[inline(always)]
    fn from(variant: HREN_A) -> Self {
        variant as u8 != 0
    }
}
impl HREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HREN_A {
        match self.bits {
            false => HREN_A::DISABLED,
            true => HREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HREN_A::ENABLED
    }
}
#[doc = "Field `HREN` writer - Host Request Enable"]
pub type HREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCFGR0_SPEC, HREN_A, O>;
impl<'a, const O: u8> HREN_W<'a, O> {
    #[doc = "Host request input is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HREN_A::DISABLED)
    }
    #[doc = "Host request input is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HREN_A::ENABLED)
    }
}
#[doc = "Field `HRPOL` reader - Host Request Polarity"]
pub type HRPOL_R = crate::BitReader<HRPOL_A>;
#[doc = "Host Request Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRPOL_A {
    #[doc = "0: Active low"]
    ACTIVE_LOW = 0,
    #[doc = "1: Active high"]
    ACTIVE_HIGH = 1,
}
impl From<HRPOL_A> for bool {
    #[inline(always)]
    fn from(variant: HRPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl HRPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRPOL_A {
        match self.bits {
            false => HRPOL_A::ACTIVE_LOW,
            true => HRPOL_A::ACTIVE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == HRPOL_A::ACTIVE_LOW
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == HRPOL_A::ACTIVE_HIGH
    }
}
#[doc = "Field `HRPOL` writer - Host Request Polarity"]
pub type HRPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCFGR0_SPEC, HRPOL_A, O>;
impl<'a, const O: u8> HRPOL_W<'a, O> {
    #[doc = "Active low"]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(HRPOL_A::ACTIVE_LOW)
    }
    #[doc = "Active high"]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(HRPOL_A::ACTIVE_HIGH)
    }
}
#[doc = "Field `HRSEL` reader - Host Request Select"]
pub type HRSEL_R = crate::BitReader<HRSEL_A>;
#[doc = "Host Request Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HRSEL_A {
    #[doc = "0: Host request input is pin HREQ"]
    DISABLED = 0,
    #[doc = "1: Host request input is input trigger"]
    ENABLED = 1,
}
impl From<HRSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HRSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl HRSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HRSEL_A {
        match self.bits {
            false => HRSEL_A::DISABLED,
            true => HRSEL_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HRSEL_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HRSEL_A::ENABLED
    }
}
#[doc = "Field `HRSEL` writer - Host Request Select"]
pub type HRSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCFGR0_SPEC, HRSEL_A, O>;
impl<'a, const O: u8> HRSEL_W<'a, O> {
    #[doc = "Host request input is pin HREQ"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HRSEL_A::DISABLED)
    }
    #[doc = "Host request input is input trigger"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HRSEL_A::ENABLED)
    }
}
#[doc = "Field `CIRFIFO` reader - Circular FIFO Enable"]
pub type CIRFIFO_R = crate::BitReader<CIRFIFO_A>;
#[doc = "Circular FIFO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRFIFO_A {
    #[doc = "0: Circular FIFO is disabled"]
    DISABLED = 0,
    #[doc = "1: Circular FIFO is enabled"]
    ENABLED = 1,
}
impl From<CIRFIFO_A> for bool {
    #[inline(always)]
    fn from(variant: CIRFIFO_A) -> Self {
        variant as u8 != 0
    }
}
impl CIRFIFO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIRFIFO_A {
        match self.bits {
            false => CIRFIFO_A::DISABLED,
            true => CIRFIFO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRFIFO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRFIFO_A::ENABLED
    }
}
#[doc = "Field `CIRFIFO` writer - Circular FIFO Enable"]
pub type CIRFIFO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCFGR0_SPEC, CIRFIFO_A, O>;
impl<'a, const O: u8> CIRFIFO_W<'a, O> {
    #[doc = "Circular FIFO is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRFIFO_A::DISABLED)
    }
    #[doc = "Circular FIFO is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRFIFO_A::ENABLED)
    }
}
#[doc = "Field `RDMO` reader - Receive Data Match Only"]
pub type RDMO_R = crate::BitReader<RDMO_A>;
#[doc = "Receive Data Match Only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDMO_A {
    #[doc = "0: Received data is stored in the receive FIFO"]
    DISABLED = 0,
    #[doc = "1: Received data is discarded unless the the Data Match Flag (MSR\\[DMF\\]) is set"]
    ENABLED = 1,
}
impl From<RDMO_A> for bool {
    #[inline(always)]
    fn from(variant: RDMO_A) -> Self {
        variant as u8 != 0
    }
}
impl RDMO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDMO_A {
        match self.bits {
            false => RDMO_A::DISABLED,
            true => RDMO_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDMO_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDMO_A::ENABLED
    }
}
#[doc = "Field `RDMO` writer - Receive Data Match Only"]
pub type RDMO_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCFGR0_SPEC, RDMO_A, O>;
impl<'a, const O: u8> RDMO_W<'a, O> {
    #[doc = "Received data is stored in the receive FIFO"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDMO_A::DISABLED)
    }
    #[doc = "Received data is discarded unless the the Data Match Flag (MSR\\[DMF\\]) is set"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDMO_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline(always)]
    pub fn hren(&self) -> HREN_R {
        HREN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline(always)]
    pub fn hrpol(&self) -> HRPOL_R {
        HRPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline(always)]
    pub fn hrsel(&self) -> HRSEL_R {
        HRSEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline(always)]
    pub fn cirfifo(&self) -> CIRFIFO_R {
        CIRFIFO_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline(always)]
    pub fn rdmo(&self) -> RDMO_R {
        RDMO_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host Request Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hren(&mut self) -> HREN_W<0> {
        HREN_W::new(self)
    }
    #[doc = "Bit 1 - Host Request Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn hrpol(&mut self) -> HRPOL_W<1> {
        HRPOL_W::new(self)
    }
    #[doc = "Bit 2 - Host Request Select"]
    #[inline(always)]
    #[must_use]
    pub fn hrsel(&mut self) -> HRSEL_W<2> {
        HRSEL_W::new(self)
    }
    #[doc = "Bit 8 - Circular FIFO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cirfifo(&mut self) -> CIRFIFO_W<8> {
        CIRFIFO_W::new(self)
    }
    #[doc = "Bit 9 - Receive Data Match Only"]
    #[inline(always)]
    #[must_use]
    pub fn rdmo(&mut self) -> RDMO_W<9> {
        RDMO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr0](index.html) module"]
pub struct MCFGR0_SPEC;
impl crate::RegisterSpec for MCFGR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfgr0::R](R) reader structure"]
impl crate::Readable for MCFGR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfgr0::W](W) writer structure"]
impl crate::Writable for MCFGR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCFGR0 to value 0"]
impl crate::Resettable for MCFGR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
