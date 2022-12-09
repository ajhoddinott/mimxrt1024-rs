#[doc = "Register `SHIFTCTL[%s]` reader"]
pub struct R(crate::R<SHIFTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SHIFTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SHIFTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SHIFTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SHIFTCTL[%s]` writer"]
pub struct W(crate::W<SHIFTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SHIFTCTL_SPEC>;
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
impl From<crate::W<SHIFTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SHIFTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMOD` reader - Shifter Mode"]
pub type SMOD_R = crate::FieldReader<u8, SMOD_A>;
#[doc = "Shifter Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMOD_A {
    #[doc = "0: Disabled."]
    DISABLE = 0,
    #[doc = "1: Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    RECEIVE = 1,
    #[doc = "2: Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    TRANSMIT = 2,
    #[doc = "4: Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    MATCHSTORE = 4,
    #[doc = "5: Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    MATCHCONT = 5,
    #[doc = "6: State mode. SHIFTBUF contents are used for storing programmable state attributes."]
    STATE = 6,
    #[doc = "7: Logic mode. SHIFTBUF contents are used for implementing programmable logic look up table."]
    LOGIC = 7,
}
impl From<SMOD_A> for u8 {
    #[inline(always)]
    fn from(variant: SMOD_A) -> Self {
        variant as _
    }
}
impl SMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SMOD_A> {
        match self.bits {
            0 => Some(SMOD_A::DISABLE),
            1 => Some(SMOD_A::RECEIVE),
            2 => Some(SMOD_A::TRANSMIT),
            4 => Some(SMOD_A::MATCHSTORE),
            5 => Some(SMOD_A::MATCHCONT),
            6 => Some(SMOD_A::STATE),
            7 => Some(SMOD_A::LOGIC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SMOD_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `RECEIVE`"]
    #[inline(always)]
    pub fn is_receive(&self) -> bool {
        *self == SMOD_A::RECEIVE
    }
    #[doc = "Checks if the value of the field is `TRANSMIT`"]
    #[inline(always)]
    pub fn is_transmit(&self) -> bool {
        *self == SMOD_A::TRANSMIT
    }
    #[doc = "Checks if the value of the field is `MATCHSTORE`"]
    #[inline(always)]
    pub fn is_matchstore(&self) -> bool {
        *self == SMOD_A::MATCHSTORE
    }
    #[doc = "Checks if the value of the field is `MATCHCONT`"]
    #[inline(always)]
    pub fn is_matchcont(&self) -> bool {
        *self == SMOD_A::MATCHCONT
    }
    #[doc = "Checks if the value of the field is `STATE`"]
    #[inline(always)]
    pub fn is_state(&self) -> bool {
        *self == SMOD_A::STATE
    }
    #[doc = "Checks if the value of the field is `LOGIC`"]
    #[inline(always)]
    pub fn is_logic(&self) -> bool {
        *self == SMOD_A::LOGIC
    }
}
#[doc = "Field `SMOD` writer - Shifter Mode"]
pub type SMOD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFTCTL_SPEC, u8, SMOD_A, 3, O>;
impl<'a, const O: u8> SMOD_W<'a, O> {
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SMOD_A::DISABLE)
    }
    #[doc = "Receive mode. Captures the current Shifter content into the SHIFTBUF on expiration of the Timer."]
    #[inline(always)]
    pub fn receive(self) -> &'a mut W {
        self.variant(SMOD_A::RECEIVE)
    }
    #[doc = "Transmit mode. Load SHIFTBUF contents into the Shifter on expiration of the Timer."]
    #[inline(always)]
    pub fn transmit(self) -> &'a mut W {
        self.variant(SMOD_A::TRANSMIT)
    }
    #[doc = "Match Store mode. Shifter data is compared to SHIFTBUF content on expiration of the Timer."]
    #[inline(always)]
    pub fn matchstore(self) -> &'a mut W {
        self.variant(SMOD_A::MATCHSTORE)
    }
    #[doc = "Match Continuous mode. Shifter data is continuously compared to SHIFTBUF contents."]
    #[inline(always)]
    pub fn matchcont(self) -> &'a mut W {
        self.variant(SMOD_A::MATCHCONT)
    }
    #[doc = "State mode. SHIFTBUF contents are used for storing programmable state attributes."]
    #[inline(always)]
    pub fn state(self) -> &'a mut W {
        self.variant(SMOD_A::STATE)
    }
    #[doc = "Logic mode. SHIFTBUF contents are used for implementing programmable logic look up table."]
    #[inline(always)]
    pub fn logic(self) -> &'a mut W {
        self.variant(SMOD_A::LOGIC)
    }
}
#[doc = "Field `PINPOL` reader - Shifter Pin Polarity"]
pub type PINPOL_R = crate::BitReader<PINPOL_A>;
#[doc = "Shifter Pin Polarity\n\nValue on reset: 0"]
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
#[doc = "Field `PINPOL` writer - Shifter Pin Polarity"]
pub type PINPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHIFTCTL_SPEC, PINPOL_A, O>;
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
#[doc = "Field `PINSEL` reader - Shifter Pin Select"]
pub type PINSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PINSEL` writer - Shifter Pin Select"]
pub type PINSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFTCTL_SPEC, u8, u8, 5, O>;
#[doc = "Field `PINCFG` reader - Shifter Pin Configuration"]
pub type PINCFG_R = crate::FieldReader<u8, PINCFG_A>;
#[doc = "Shifter Pin Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINCFG_A {
    #[doc = "0: Shifter pin output disabled"]
    DISABLE = 0,
    #[doc = "1: Shifter pin open drain or bidirectional output enable"]
    OPEND_BIDIROUTEN = 1,
    #[doc = "2: Shifter pin bidirectional output data"]
    BIDIR_OUTDATA = 2,
    #[doc = "3: Shifter pin output"]
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
            0 => PINCFG_A::DISABLE,
            1 => PINCFG_A::OPEND_BIDIROUTEN,
            2 => PINCFG_A::BIDIR_OUTDATA,
            3 => PINCFG_A::OUTPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == PINCFG_A::DISABLE
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
#[doc = "Field `PINCFG` writer - Shifter Pin Configuration"]
pub type PINCFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, SHIFTCTL_SPEC, u8, PINCFG_A, 2, O>;
impl<'a, const O: u8> PINCFG_W<'a, O> {
    #[doc = "Shifter pin output disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(PINCFG_A::DISABLE)
    }
    #[doc = "Shifter pin open drain or bidirectional output enable"]
    #[inline(always)]
    pub fn opend_bidirouten(self) -> &'a mut W {
        self.variant(PINCFG_A::OPEND_BIDIROUTEN)
    }
    #[doc = "Shifter pin bidirectional output data"]
    #[inline(always)]
    pub fn bidir_outdata(self) -> &'a mut W {
        self.variant(PINCFG_A::BIDIR_OUTDATA)
    }
    #[doc = "Shifter pin output"]
    #[inline(always)]
    pub fn output(self) -> &'a mut W {
        self.variant(PINCFG_A::OUTPUT)
    }
}
#[doc = "Field `TIMPOL` reader - Timer Polarity"]
pub type TIMPOL_R = crate::BitReader<TIMPOL_A>;
#[doc = "Timer Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMPOL_A {
    #[doc = "0: Shift on posedge of Shift clock"]
    POSEDGE = 0,
    #[doc = "1: Shift on negedge of Shift clock"]
    NEGEDGE = 1,
}
impl From<TIMPOL_A> for bool {
    #[inline(always)]
    fn from(variant: TIMPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMPOL_A {
        match self.bits {
            false => TIMPOL_A::POSEDGE,
            true => TIMPOL_A::NEGEDGE,
        }
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        *self == TIMPOL_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        *self == TIMPOL_A::NEGEDGE
    }
}
#[doc = "Field `TIMPOL` writer - Timer Polarity"]
pub type TIMPOL_W<'a, const O: u8> = crate::BitWriter<'a, u32, SHIFTCTL_SPEC, TIMPOL_A, O>;
impl<'a, const O: u8> TIMPOL_W<'a, O> {
    #[doc = "Shift on posedge of Shift clock"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(TIMPOL_A::POSEDGE)
    }
    #[doc = "Shift on negedge of Shift clock"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(TIMPOL_A::NEGEDGE)
    }
}
#[doc = "Field `TIMSEL` reader - Timer Select"]
pub type TIMSEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMSEL` writer - Timer Select"]
pub type TIMSEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SHIFTCTL_SPEC, u8, u8, 3, O>;
impl R {
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline(always)]
    pub fn smod(&self) -> SMOD_R {
        SMOD_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline(always)]
    pub fn pinpol(&self) -> PINPOL_R {
        PINPOL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Shifter Pin Select"]
    #[inline(always)]
    pub fn pinsel(&self) -> PINSEL_R {
        PINSEL_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline(always)]
    pub fn pincfg(&self) -> PINCFG_R {
        PINCFG_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline(always)]
    pub fn timpol(&self) -> TIMPOL_R {
        TIMPOL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Timer Select"]
    #[inline(always)]
    pub fn timsel(&self) -> TIMSEL_R {
        TIMSEL_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Shifter Mode"]
    #[inline(always)]
    #[must_use]
    pub fn smod(&mut self) -> SMOD_W<0> {
        SMOD_W::new(self)
    }
    #[doc = "Bit 7 - Shifter Pin Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn pinpol(&mut self) -> PINPOL_W<7> {
        PINPOL_W::new(self)
    }
    #[doc = "Bits 8:12 - Shifter Pin Select"]
    #[inline(always)]
    #[must_use]
    pub fn pinsel(&mut self) -> PINSEL_W<8> {
        PINSEL_W::new(self)
    }
    #[doc = "Bits 16:17 - Shifter Pin Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn pincfg(&mut self) -> PINCFG_W<16> {
        PINCFG_W::new(self)
    }
    #[doc = "Bit 23 - Timer Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn timpol(&mut self) -> TIMPOL_W<23> {
        TIMPOL_W::new(self)
    }
    #[doc = "Bits 24:26 - Timer Select"]
    #[inline(always)]
    #[must_use]
    pub fn timsel(&mut self) -> TIMSEL_W<24> {
        TIMSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Shifter Control N Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [shiftctl](index.html) module"]
pub struct SHIFTCTL_SPEC;
impl crate::RegisterSpec for SHIFTCTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [shiftctl::R](R) reader structure"]
impl crate::Readable for SHIFTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [shiftctl::W](W) writer structure"]
impl crate::Writable for SHIFTCTL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SHIFTCTL[%s]
to value 0"]
impl crate::Resettable for SHIFTCTL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
