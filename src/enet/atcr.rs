#[doc = "Register `ATCR` reader"]
pub struct R(crate::R<ATCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ATCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ATCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ATCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ATCR` writer"]
pub struct W(crate::W<ATCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ATCR_SPEC>;
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
impl From<crate::W<ATCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ATCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Enable Timer"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Enable Timer\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: The timer stops at the current value."]
    ZERO = 0,
    #[doc = "1: The timer starts incrementing."]
    ONE = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::ZERO,
            true => EN_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == EN_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == EN_A::ONE
    }
}
#[doc = "Field `EN` writer - Enable Timer"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "The timer stops at the current value."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(EN_A::ZERO)
    }
    #[doc = "The timer starts incrementing."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(EN_A::ONE)
    }
}
#[doc = "Field `OFFEN` reader - Enable One-Shot Offset Event"]
pub type OFFEN_R = crate::BitReader<OFFEN_A>;
#[doc = "Enable One-Shot Offset Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFEN_A {
    #[doc = "0: Disable."]
    ZERO = 0,
    #[doc = "1: The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    ONE = 1,
}
impl From<OFFEN_A> for bool {
    #[inline(always)]
    fn from(variant: OFFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl OFFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFEN_A {
        match self.bits {
            false => OFFEN_A::ZERO,
            true => OFFEN_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == OFFEN_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == OFFEN_A::ONE
    }
}
#[doc = "Field `OFFEN` writer - Enable One-Shot Offset Event"]
pub type OFFEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR_SPEC, OFFEN_A, O>;
impl<'a, const O: u8> OFFEN_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(OFFEN_A::ZERO)
    }
    #[doc = "The timer can be reset to zero when the given offset time is reached (offset event). The field is cleared when the offset event is reached, so no further event occurs until the field is set again. The timer offset value must be set before setting this field."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(OFFEN_A::ONE)
    }
}
#[doc = "Field `OFFRST` reader - Reset Timer On Offset Event"]
pub type OFFRST_R = crate::BitReader<OFFRST_A>;
#[doc = "Reset Timer On Offset Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OFFRST_A {
    #[doc = "0: The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    ZERO = 0,
    #[doc = "1: If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    ONE = 1,
}
impl From<OFFRST_A> for bool {
    #[inline(always)]
    fn from(variant: OFFRST_A) -> Self {
        variant as u8 != 0
    }
}
impl OFFRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OFFRST_A {
        match self.bits {
            false => OFFRST_A::ZERO,
            true => OFFRST_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == OFFRST_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == OFFRST_A::ONE
    }
}
#[doc = "Field `OFFRST` writer - Reset Timer On Offset Event"]
pub type OFFRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR_SPEC, OFFRST_A, O>;
impl<'a, const O: u8> OFFRST_W<'a, O> {
    #[doc = "The timer is not affected and no action occurs, besides clearing OFFEN, when the offset is reached."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(OFFRST_A::ZERO)
    }
    #[doc = "If OFFEN is set, the timer resets to zero when the offset setting is reached. The offset event does not cause a timer interrupt."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(OFFRST_A::ONE)
    }
}
#[doc = "Field `PEREN` reader - Enable Periodical Event"]
pub type PEREN_R = crate::BitReader<PEREN_A>;
#[doc = "Enable Periodical Event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEREN_A {
    #[doc = "0: Disable."]
    ZERO = 0,
    #[doc = "1: A period event interrupt can be generated (EIR\\[TS_TIMER\\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    ONE = 1,
}
impl From<PEREN_A> for bool {
    #[inline(always)]
    fn from(variant: PEREN_A) -> Self {
        variant as u8 != 0
    }
}
impl PEREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEREN_A {
        match self.bits {
            false => PEREN_A::ZERO,
            true => PEREN_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == PEREN_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == PEREN_A::ONE
    }
}
#[doc = "Field `PEREN` writer - Enable Periodical Event"]
pub type PEREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR_SPEC, PEREN_A, O>;
impl<'a, const O: u8> PEREN_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(PEREN_A::ZERO)
    }
    #[doc = "A period event interrupt can be generated (EIR\\[TS_TIMER\\]) and the event signal output is asserted when the timer wraps around according to the periodic setting ATPER. The timer period value must be set before setting this bit. Not all devices contain the event signal output. See the chip configuration details."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(PEREN_A::ONE)
    }
}
#[doc = "Field `PINPER` reader - Enables event signal output external pin frc_evt_period assertion on period event"]
pub type PINPER_R = crate::BitReader<PINPER_A>;
#[doc = "Enables event signal output external pin frc_evt_period assertion on period event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINPER_A {
    #[doc = "0: Disable."]
    ZERO = 0,
    #[doc = "1: Enable."]
    ONE = 1,
}
impl From<PINPER_A> for bool {
    #[inline(always)]
    fn from(variant: PINPER_A) -> Self {
        variant as u8 != 0
    }
}
impl PINPER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINPER_A {
        match self.bits {
            false => PINPER_A::ZERO,
            true => PINPER_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == PINPER_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == PINPER_A::ONE
    }
}
#[doc = "Field `PINPER` writer - Enables event signal output external pin frc_evt_period assertion on period event"]
pub type PINPER_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR_SPEC, PINPER_A, O>;
impl<'a, const O: u8> PINPER_W<'a, O> {
    #[doc = "Disable."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(PINPER_A::ZERO)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(PINPER_A::ONE)
    }
}
#[doc = "Field `RESTART` reader - Reset Timer"]
pub type RESTART_R = crate::BitReader<bool>;
#[doc = "Field `RESTART` writer - Reset Timer"]
pub type RESTART_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR_SPEC, bool, O>;
#[doc = "Field `CAPTURE` reader - Capture Timer Value"]
pub type CAPTURE_R = crate::BitReader<CAPTURE_A>;
#[doc = "Capture Timer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAPTURE_A {
    #[doc = "0: No effect."]
    ZERO = 0,
    #[doc = "1: The current time is captured and can be read from the ATVR register."]
    ONE = 1,
}
impl From<CAPTURE_A> for bool {
    #[inline(always)]
    fn from(variant: CAPTURE_A) -> Self {
        variant as u8 != 0
    }
}
impl CAPTURE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAPTURE_A {
        match self.bits {
            false => CAPTURE_A::ZERO,
            true => CAPTURE_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == CAPTURE_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == CAPTURE_A::ONE
    }
}
#[doc = "Field `CAPTURE` writer - Capture Timer Value"]
pub type CAPTURE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR_SPEC, CAPTURE_A, O>;
impl<'a, const O: u8> CAPTURE_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(CAPTURE_A::ZERO)
    }
    #[doc = "The current time is captured and can be read from the ATVR register."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(CAPTURE_A::ONE)
    }
}
#[doc = "Field `SLAVE` reader - Enable Timer Slave Mode"]
pub type SLAVE_R = crate::BitReader<SLAVE_A>;
#[doc = "Enable Timer Slave Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLAVE_A {
    #[doc = "0: The timer is active and all configuration fields in this register are relevant."]
    ZERO = 0,
    #[doc = "1: The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    ONE = 1,
}
impl From<SLAVE_A> for bool {
    #[inline(always)]
    fn from(variant: SLAVE_A) -> Self {
        variant as u8 != 0
    }
}
impl SLAVE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLAVE_A {
        match self.bits {
            false => SLAVE_A::ZERO,
            true => SLAVE_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == SLAVE_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == SLAVE_A::ONE
    }
}
#[doc = "Field `SLAVE` writer - Enable Timer Slave Mode"]
pub type SLAVE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ATCR_SPEC, SLAVE_A, O>;
impl<'a, const O: u8> SLAVE_W<'a, O> {
    #[doc = "The timer is active and all configuration fields in this register are relevant."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(SLAVE_A::ZERO)
    }
    #[doc = "The internal timer is disabled and the externally provided timer value is used. All other fields, except CAPTURE, in this register have no effect. CAPTURE can still be used to capture the current timer value."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(SLAVE_A::ONE)
    }
}
impl R {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - Enable One-Shot Offset Event"]
    #[inline(always)]
    pub fn offen(&self) -> OFFEN_R {
        OFFEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reset Timer On Offset Event"]
    #[inline(always)]
    pub fn offrst(&self) -> OFFRST_R {
        OFFRST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Periodical Event"]
    #[inline(always)]
    pub fn peren(&self) -> PEREN_R {
        PEREN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables event signal output external pin frc_evt_period assertion on period event"]
    #[inline(always)]
    pub fn pinper(&self) -> PINPER_R {
        PINPER_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Reset Timer"]
    #[inline(always)]
    pub fn restart(&self) -> RESTART_R {
        RESTART_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture Timer Value"]
    #[inline(always)]
    pub fn capture(&self) -> CAPTURE_R {
        CAPTURE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable Timer Slave Mode"]
    #[inline(always)]
    pub fn slave(&self) -> SLAVE_R {
        SLAVE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Timer"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 2 - Enable One-Shot Offset Event"]
    #[inline(always)]
    #[must_use]
    pub fn offen(&mut self) -> OFFEN_W<2> {
        OFFEN_W::new(self)
    }
    #[doc = "Bit 3 - Reset Timer On Offset Event"]
    #[inline(always)]
    #[must_use]
    pub fn offrst(&mut self) -> OFFRST_W<3> {
        OFFRST_W::new(self)
    }
    #[doc = "Bit 4 - Enable Periodical Event"]
    #[inline(always)]
    #[must_use]
    pub fn peren(&mut self) -> PEREN_W<4> {
        PEREN_W::new(self)
    }
    #[doc = "Bit 7 - Enables event signal output external pin frc_evt_period assertion on period event"]
    #[inline(always)]
    #[must_use]
    pub fn pinper(&mut self) -> PINPER_W<7> {
        PINPER_W::new(self)
    }
    #[doc = "Bit 9 - Reset Timer"]
    #[inline(always)]
    #[must_use]
    pub fn restart(&mut self) -> RESTART_W<9> {
        RESTART_W::new(self)
    }
    #[doc = "Bit 11 - Capture Timer Value"]
    #[inline(always)]
    #[must_use]
    pub fn capture(&mut self) -> CAPTURE_W<11> {
        CAPTURE_W::new(self)
    }
    #[doc = "Bit 13 - Enable Timer Slave Mode"]
    #[inline(always)]
    #[must_use]
    pub fn slave(&mut self) -> SLAVE_W<13> {
        SLAVE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Adjustable Timer Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [atcr](index.html) module"]
pub struct ATCR_SPEC;
impl crate::RegisterSpec for ATCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [atcr::R](R) reader structure"]
impl crate::Readable for ATCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [atcr::W](W) writer structure"]
impl crate::Writable for ATCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ATCR to value 0"]
impl crate::Resettable for ATCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
