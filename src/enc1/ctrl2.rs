#[doc = "Register `CTRL2` reader"]
pub struct R(crate::R<CTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL2` writer"]
pub struct W(crate::W<CTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL2_SPEC>;
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
impl From<crate::W<CTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDHLD` reader - Update Hold Registers"]
pub type UPDHLD_R = crate::BitReader<UPDHLD_A>;
#[doc = "Update Hold Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDHLD_A {
    #[doc = "0: Disable updates of hold registers on the rising edge of TRIGGER input signal"]
    UPDHLD_0 = 0,
    #[doc = "1: Enable updates of hold registers on the rising edge of TRIGGER input signal"]
    UPDHLD_1 = 1,
}
impl From<UPDHLD_A> for bool {
    #[inline(always)]
    fn from(variant: UPDHLD_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDHLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDHLD_A {
        match self.bits {
            false => UPDHLD_A::UPDHLD_0,
            true => UPDHLD_A::UPDHLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `UPDHLD_0`"]
    #[inline(always)]
    pub fn is_updhld_0(&self) -> bool {
        *self == UPDHLD_A::UPDHLD_0
    }
    #[doc = "Checks if the value of the field is `UPDHLD_1`"]
    #[inline(always)]
    pub fn is_updhld_1(&self) -> bool {
        *self == UPDHLD_A::UPDHLD_1
    }
}
#[doc = "Field `UPDHLD` writer - Update Hold Registers"]
pub type UPDHLD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL2_SPEC, UPDHLD_A, O>;
impl<'a, const O: u8> UPDHLD_W<'a, O> {
    #[doc = "Disable updates of hold registers on the rising edge of TRIGGER input signal"]
    #[inline(always)]
    pub fn updhld_0(self) -> &'a mut W {
        self.variant(UPDHLD_A::UPDHLD_0)
    }
    #[doc = "Enable updates of hold registers on the rising edge of TRIGGER input signal"]
    #[inline(always)]
    pub fn updhld_1(self) -> &'a mut W {
        self.variant(UPDHLD_A::UPDHLD_1)
    }
}
#[doc = "Field `UPDPOS` reader - Update Position Registers"]
pub type UPDPOS_R = crate::BitReader<UPDPOS_A>;
#[doc = "Update Position Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDPOS_A {
    #[doc = "0: No action for POSD, REV, UPOS and LPOS registers on rising edge of TRIGGER"]
    UPDPOS_0 = 0,
    #[doc = "1: Clear POSD, REV, UPOS and LPOS registers on rising edge of TRIGGER"]
    UPDPOS_1 = 1,
}
impl From<UPDPOS_A> for bool {
    #[inline(always)]
    fn from(variant: UPDPOS_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDPOS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDPOS_A {
        match self.bits {
            false => UPDPOS_A::UPDPOS_0,
            true => UPDPOS_A::UPDPOS_1,
        }
    }
    #[doc = "Checks if the value of the field is `UPDPOS_0`"]
    #[inline(always)]
    pub fn is_updpos_0(&self) -> bool {
        *self == UPDPOS_A::UPDPOS_0
    }
    #[doc = "Checks if the value of the field is `UPDPOS_1`"]
    #[inline(always)]
    pub fn is_updpos_1(&self) -> bool {
        *self == UPDPOS_A::UPDPOS_1
    }
}
#[doc = "Field `UPDPOS` writer - Update Position Registers"]
pub type UPDPOS_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL2_SPEC, UPDPOS_A, O>;
impl<'a, const O: u8> UPDPOS_W<'a, O> {
    #[doc = "No action for POSD, REV, UPOS and LPOS registers on rising edge of TRIGGER"]
    #[inline(always)]
    pub fn updpos_0(self) -> &'a mut W {
        self.variant(UPDPOS_A::UPDPOS_0)
    }
    #[doc = "Clear POSD, REV, UPOS and LPOS registers on rising edge of TRIGGER"]
    #[inline(always)]
    pub fn updpos_1(self) -> &'a mut W {
        self.variant(UPDPOS_A::UPDPOS_1)
    }
}
#[doc = "Field `MOD` reader - Enable Modulo Counting"]
pub type MOD_R = crate::BitReader<MOD_A>;
#[doc = "Enable Modulo Counting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOD_A {
    #[doc = "0: Disable modulo counting"]
    MOD_0 = 0,
    #[doc = "1: Enable modulo counting"]
    MOD_1 = 1,
}
impl From<MOD_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_A) -> Self {
        variant as u8 != 0
    }
}
impl MOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_A {
        match self.bits {
            false => MOD_A::MOD_0,
            true => MOD_A::MOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_0`"]
    #[inline(always)]
    pub fn is_mod_0(&self) -> bool {
        *self == MOD_A::MOD_0
    }
    #[doc = "Checks if the value of the field is `MOD_1`"]
    #[inline(always)]
    pub fn is_mod_1(&self) -> bool {
        *self == MOD_A::MOD_1
    }
}
#[doc = "Field `MOD` writer - Enable Modulo Counting"]
pub type MOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL2_SPEC, MOD_A, O>;
impl<'a, const O: u8> MOD_W<'a, O> {
    #[doc = "Disable modulo counting"]
    #[inline(always)]
    pub fn mod_0(self) -> &'a mut W {
        self.variant(MOD_A::MOD_0)
    }
    #[doc = "Enable modulo counting"]
    #[inline(always)]
    pub fn mod_1(self) -> &'a mut W {
        self.variant(MOD_A::MOD_1)
    }
}
#[doc = "Field `DIR` reader - Count Direction Flag"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Count Direction Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Last count was in the down direction"]
    DIR_0 = 0,
    #[doc = "1: Last count was in the up direction"]
    DIR_1 = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::DIR_0,
            true => DIR_A::DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIR_0`"]
    #[inline(always)]
    pub fn is_dir_0(&self) -> bool {
        *self == DIR_A::DIR_0
    }
    #[doc = "Checks if the value of the field is `DIR_1`"]
    #[inline(always)]
    pub fn is_dir_1(&self) -> bool {
        *self == DIR_A::DIR_1
    }
}
#[doc = "Field `RUIE` reader - Roll-under Interrupt Enable"]
pub type RUIE_R = crate::BitReader<RUIE_A>;
#[doc = "Roll-under Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUIE_A {
    #[doc = "0: Disabled"]
    RUIE_0 = 0,
    #[doc = "1: Enabled"]
    RUIE_1 = 1,
}
impl From<RUIE_A> for bool {
    #[inline(always)]
    fn from(variant: RUIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RUIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUIE_A {
        match self.bits {
            false => RUIE_A::RUIE_0,
            true => RUIE_A::RUIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `RUIE_0`"]
    #[inline(always)]
    pub fn is_ruie_0(&self) -> bool {
        *self == RUIE_A::RUIE_0
    }
    #[doc = "Checks if the value of the field is `RUIE_1`"]
    #[inline(always)]
    pub fn is_ruie_1(&self) -> bool {
        *self == RUIE_A::RUIE_1
    }
}
#[doc = "Field `RUIE` writer - Roll-under Interrupt Enable"]
pub type RUIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL2_SPEC, RUIE_A, O>;
impl<'a, const O: u8> RUIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn ruie_0(self) -> &'a mut W {
        self.variant(RUIE_A::RUIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ruie_1(self) -> &'a mut W {
        self.variant(RUIE_A::RUIE_1)
    }
}
#[doc = "Field `RUIRQ` reader - Roll-under Interrupt Request"]
pub type RUIRQ_R = crate::BitReader<RUIRQ_A>;
#[doc = "Roll-under Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUIRQ_A {
    #[doc = "0: No roll-under has occurred"]
    RUIRQ_0 = 0,
    #[doc = "1: Roll-under has occurred"]
    RUIRQ_1 = 1,
}
impl From<RUIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: RUIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl RUIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUIRQ_A {
        match self.bits {
            false => RUIRQ_A::RUIRQ_0,
            true => RUIRQ_A::RUIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `RUIRQ_0`"]
    #[inline(always)]
    pub fn is_ruirq_0(&self) -> bool {
        *self == RUIRQ_A::RUIRQ_0
    }
    #[doc = "Checks if the value of the field is `RUIRQ_1`"]
    #[inline(always)]
    pub fn is_ruirq_1(&self) -> bool {
        *self == RUIRQ_A::RUIRQ_1
    }
}
#[doc = "Field `RUIRQ` writer - Roll-under Interrupt Request"]
pub type RUIRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL2_SPEC, RUIRQ_A, O>;
impl<'a, const O: u8> RUIRQ_W<'a, O> {
    #[doc = "No roll-under has occurred"]
    #[inline(always)]
    pub fn ruirq_0(self) -> &'a mut W {
        self.variant(RUIRQ_A::RUIRQ_0)
    }
    #[doc = "Roll-under has occurred"]
    #[inline(always)]
    pub fn ruirq_1(self) -> &'a mut W {
        self.variant(RUIRQ_A::RUIRQ_1)
    }
}
#[doc = "Field `ROIE` reader - Roll-over Interrupt Enable"]
pub type ROIE_R = crate::BitReader<ROIE_A>;
#[doc = "Roll-over Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROIE_A {
    #[doc = "0: Disabled"]
    ROIE_0 = 0,
    #[doc = "1: Enabled"]
    ROIE_1 = 1,
}
impl From<ROIE_A> for bool {
    #[inline(always)]
    fn from(variant: ROIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ROIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROIE_A {
        match self.bits {
            false => ROIE_A::ROIE_0,
            true => ROIE_A::ROIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROIE_0`"]
    #[inline(always)]
    pub fn is_roie_0(&self) -> bool {
        *self == ROIE_A::ROIE_0
    }
    #[doc = "Checks if the value of the field is `ROIE_1`"]
    #[inline(always)]
    pub fn is_roie_1(&self) -> bool {
        *self == ROIE_A::ROIE_1
    }
}
#[doc = "Field `ROIE` writer - Roll-over Interrupt Enable"]
pub type ROIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL2_SPEC, ROIE_A, O>;
impl<'a, const O: u8> ROIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn roie_0(self) -> &'a mut W {
        self.variant(ROIE_A::ROIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn roie_1(self) -> &'a mut W {
        self.variant(ROIE_A::ROIE_1)
    }
}
#[doc = "Field `ROIRQ` reader - Roll-over Interrupt Request"]
pub type ROIRQ_R = crate::BitReader<ROIRQ_A>;
#[doc = "Roll-over Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROIRQ_A {
    #[doc = "0: No roll-over has occurred"]
    ROIRQ_0 = 0,
    #[doc = "1: Roll-over has occurred"]
    ROIRQ_1 = 1,
}
impl From<ROIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: ROIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl ROIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROIRQ_A {
        match self.bits {
            false => ROIRQ_A::ROIRQ_0,
            true => ROIRQ_A::ROIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROIRQ_0`"]
    #[inline(always)]
    pub fn is_roirq_0(&self) -> bool {
        *self == ROIRQ_A::ROIRQ_0
    }
    #[doc = "Checks if the value of the field is `ROIRQ_1`"]
    #[inline(always)]
    pub fn is_roirq_1(&self) -> bool {
        *self == ROIRQ_A::ROIRQ_1
    }
}
#[doc = "Field `ROIRQ` writer - Roll-over Interrupt Request"]
pub type ROIRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL2_SPEC, ROIRQ_A, O>;
impl<'a, const O: u8> ROIRQ_W<'a, O> {
    #[doc = "No roll-over has occurred"]
    #[inline(always)]
    pub fn roirq_0(self) -> &'a mut W {
        self.variant(ROIRQ_A::ROIRQ_0)
    }
    #[doc = "Roll-over has occurred"]
    #[inline(always)]
    pub fn roirq_1(self) -> &'a mut W {
        self.variant(ROIRQ_A::ROIRQ_1)
    }
}
#[doc = "Field `REVMOD` reader - Revolution Counter Modulus Enable"]
pub type REVMOD_R = crate::BitReader<REVMOD_A>;
#[doc = "Revolution Counter Modulus Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REVMOD_A {
    #[doc = "0: Use INDEX pulse to increment/decrement revolution counter (REV)"]
    REVMOD_0 = 0,
    #[doc = "1: Use modulus counting roll-over/under to increment/decrement revolution counter (REV)"]
    REVMOD_1 = 1,
}
impl From<REVMOD_A> for bool {
    #[inline(always)]
    fn from(variant: REVMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl REVMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REVMOD_A {
        match self.bits {
            false => REVMOD_A::REVMOD_0,
            true => REVMOD_A::REVMOD_1,
        }
    }
    #[doc = "Checks if the value of the field is `REVMOD_0`"]
    #[inline(always)]
    pub fn is_revmod_0(&self) -> bool {
        *self == REVMOD_A::REVMOD_0
    }
    #[doc = "Checks if the value of the field is `REVMOD_1`"]
    #[inline(always)]
    pub fn is_revmod_1(&self) -> bool {
        *self == REVMOD_A::REVMOD_1
    }
}
#[doc = "Field `REVMOD` writer - Revolution Counter Modulus Enable"]
pub type REVMOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL2_SPEC, REVMOD_A, O>;
impl<'a, const O: u8> REVMOD_W<'a, O> {
    #[doc = "Use INDEX pulse to increment/decrement revolution counter (REV)"]
    #[inline(always)]
    pub fn revmod_0(self) -> &'a mut W {
        self.variant(REVMOD_A::REVMOD_0)
    }
    #[doc = "Use modulus counting roll-over/under to increment/decrement revolution counter (REV)"]
    #[inline(always)]
    pub fn revmod_1(self) -> &'a mut W {
        self.variant(REVMOD_A::REVMOD_1)
    }
}
#[doc = "Field `OUTCTL` reader - Output Control"]
pub type OUTCTL_R = crate::BitReader<OUTCTL_A>;
#[doc = "Output Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTCTL_A {
    #[doc = "0: POSMATCH pulses when a match occurs between the position counters (POS) and the corresponding compare value (COMP )"]
    OUTCTL_0 = 0,
    #[doc = "1: POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read"]
    OUTCTL_1 = 1,
}
impl From<OUTCTL_A> for bool {
    #[inline(always)]
    fn from(variant: OUTCTL_A) -> Self {
        variant as u8 != 0
    }
}
impl OUTCTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OUTCTL_A {
        match self.bits {
            false => OUTCTL_A::OUTCTL_0,
            true => OUTCTL_A::OUTCTL_1,
        }
    }
    #[doc = "Checks if the value of the field is `OUTCTL_0`"]
    #[inline(always)]
    pub fn is_outctl_0(&self) -> bool {
        *self == OUTCTL_A::OUTCTL_0
    }
    #[doc = "Checks if the value of the field is `OUTCTL_1`"]
    #[inline(always)]
    pub fn is_outctl_1(&self) -> bool {
        *self == OUTCTL_A::OUTCTL_1
    }
}
#[doc = "Field `OUTCTL` writer - Output Control"]
pub type OUTCTL_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL2_SPEC, OUTCTL_A, O>;
impl<'a, const O: u8> OUTCTL_W<'a, O> {
    #[doc = "POSMATCH pulses when a match occurs between the position counters (POS) and the corresponding compare value (COMP )"]
    #[inline(always)]
    pub fn outctl_0(self) -> &'a mut W {
        self.variant(OUTCTL_A::OUTCTL_0)
    }
    #[doc = "POSMATCH pulses when the UPOS, LPOS, REV, or POSD registers are read"]
    #[inline(always)]
    pub fn outctl_1(self) -> &'a mut W {
        self.variant(OUTCTL_A::OUTCTL_1)
    }
}
impl R {
    #[doc = "Bit 0 - Update Hold Registers"]
    #[inline(always)]
    pub fn updhld(&self) -> UPDHLD_R {
        UPDHLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update Position Registers"]
    #[inline(always)]
    pub fn updpos(&self) -> UPDPOS_R {
        UPDPOS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Modulo Counting"]
    #[inline(always)]
    pub fn mod_(&self) -> MOD_R {
        MOD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Count Direction Flag"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Roll-under Interrupt Enable"]
    #[inline(always)]
    pub fn ruie(&self) -> RUIE_R {
        RUIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Roll-under Interrupt Request"]
    #[inline(always)]
    pub fn ruirq(&self) -> RUIRQ_R {
        RUIRQ_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Roll-over Interrupt Enable"]
    #[inline(always)]
    pub fn roie(&self) -> ROIE_R {
        ROIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Roll-over Interrupt Request"]
    #[inline(always)]
    pub fn roirq(&self) -> ROIRQ_R {
        ROIRQ_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Revolution Counter Modulus Enable"]
    #[inline(always)]
    pub fn revmod(&self) -> REVMOD_R {
        REVMOD_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Output Control"]
    #[inline(always)]
    pub fn outctl(&self) -> OUTCTL_R {
        OUTCTL_R::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Update Hold Registers"]
    #[inline(always)]
    #[must_use]
    pub fn updhld(&mut self) -> UPDHLD_W<0> {
        UPDHLD_W::new(self)
    }
    #[doc = "Bit 1 - Update Position Registers"]
    #[inline(always)]
    #[must_use]
    pub fn updpos(&mut self) -> UPDPOS_W<1> {
        UPDPOS_W::new(self)
    }
    #[doc = "Bit 2 - Enable Modulo Counting"]
    #[inline(always)]
    #[must_use]
    pub fn mod_(&mut self) -> MOD_W<2> {
        MOD_W::new(self)
    }
    #[doc = "Bit 4 - Roll-under Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ruie(&mut self) -> RUIE_W<4> {
        RUIE_W::new(self)
    }
    #[doc = "Bit 5 - Roll-under Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn ruirq(&mut self) -> RUIRQ_W<5> {
        RUIRQ_W::new(self)
    }
    #[doc = "Bit 6 - Roll-over Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn roie(&mut self) -> ROIE_W<6> {
        ROIE_W::new(self)
    }
    #[doc = "Bit 7 - Roll-over Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn roirq(&mut self) -> ROIRQ_W<7> {
        ROIRQ_W::new(self)
    }
    #[doc = "Bit 8 - Revolution Counter Modulus Enable"]
    #[inline(always)]
    #[must_use]
    pub fn revmod(&mut self) -> REVMOD_W<8> {
        REVMOD_W::new(self)
    }
    #[doc = "Bit 9 - Output Control"]
    #[inline(always)]
    #[must_use]
    pub fn outctl(&mut self) -> OUTCTL_W<9> {
        OUTCTL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl2](index.html) module"]
pub struct CTRL2_SPEC;
impl crate::RegisterSpec for CTRL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl2::R](R) reader structure"]
impl crate::Readable for CTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl2::W](W) writer structure"]
impl crate::Writable for CTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0xa0;
}
#[doc = "`reset()` method sets CTRL2 to value 0"]
impl crate::Resettable for CTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
