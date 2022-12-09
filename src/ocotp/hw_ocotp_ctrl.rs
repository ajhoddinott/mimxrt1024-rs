#[doc = "Register `HW_OCOTP_CTRL` reader"]
pub struct R(crate::R<HW_OCOTP_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HW_OCOTP_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HW_OCOTP_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HW_OCOTP_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HW_OCOTP_CTRL` writer"]
pub struct W(crate::W<HW_OCOTP_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HW_OCOTP_CTRL_SPEC>;
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
impl From<crate::W<HW_OCOTP_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HW_OCOTP_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR` reader - OTP write and read access address register"]
pub type ADDR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ADDR` writer - OTP write and read access address register"]
pub type ADDR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HW_OCOTP_CTRL_SPEC, u8, u8, 6, O>;
#[doc = "Field `BUSY` reader - OTP controller status bit"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "OTP controller status bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: No write or read access to OTP started."]
    NOT_BUSY = 0,
    #[doc = "1: Write or read access to OTP started."]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NOT_BUSY,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `ERROR` reader - Locked Region Access Error"]
pub type ERROR_R = crate::BitReader<ERROR_A>;
#[doc = "Locked Region Access Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERROR_A {
    #[doc = "0: No error."]
    NO_ERROR = 0,
    #[doc = "1: Error - access to a locked region requested."]
    ERROR = 1,
}
impl From<ERROR_A> for bool {
    #[inline(always)]
    fn from(variant: ERROR_A) -> Self {
        variant as u8 != 0
    }
}
impl ERROR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERROR_A {
        match self.bits {
            false => ERROR_A::NO_ERROR,
            true => ERROR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == ERROR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == ERROR_A::ERROR
    }
}
#[doc = "Field `ERROR` writer - Locked Region Access Error"]
pub type ERROR_W<'a, const O: u8> = crate::BitWriter<'a, u32, HW_OCOTP_CTRL_SPEC, ERROR_A, O>;
impl<'a, const O: u8> ERROR_W<'a, O> {
    #[doc = "No error."]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(ERROR_A::NO_ERROR)
    }
    #[doc = "Error - access to a locked region requested."]
    #[inline(always)]
    pub fn error(self) -> &'a mut W {
        self.variant(ERROR_A::ERROR)
    }
}
#[doc = "Field `RELOAD_SHADOWS` reader - Reload Shadow Registers"]
pub type RELOAD_SHADOWS_R = crate::BitReader<RELOAD_SHADOWS_A>;
#[doc = "Reload Shadow Registers\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD_SHADOWS_A {
    #[doc = "0: Do not force shadow register re-load."]
    SHADOW_NOFORCE_RELOAD = 0,
    #[doc = "1: Force shadow register re-load. This bit is cleared automatically after shadow registers are re-loaded."]
    SHADOW_FORCE_RELOAD = 1,
}
impl From<RELOAD_SHADOWS_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_SHADOWS_A) -> Self {
        variant as u8 != 0
    }
}
impl RELOAD_SHADOWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_SHADOWS_A {
        match self.bits {
            false => RELOAD_SHADOWS_A::SHADOW_NOFORCE_RELOAD,
            true => RELOAD_SHADOWS_A::SHADOW_FORCE_RELOAD,
        }
    }
    #[doc = "Checks if the value of the field is `SHADOW_NOFORCE_RELOAD`"]
    #[inline(always)]
    pub fn is_shadow_noforce_reload(&self) -> bool {
        *self == RELOAD_SHADOWS_A::SHADOW_NOFORCE_RELOAD
    }
    #[doc = "Checks if the value of the field is `SHADOW_FORCE_RELOAD`"]
    #[inline(always)]
    pub fn is_shadow_force_reload(&self) -> bool {
        *self == RELOAD_SHADOWS_A::SHADOW_FORCE_RELOAD
    }
}
#[doc = "Field `RELOAD_SHADOWS` writer - Reload Shadow Registers"]
pub type RELOAD_SHADOWS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, HW_OCOTP_CTRL_SPEC, RELOAD_SHADOWS_A, O>;
impl<'a, const O: u8> RELOAD_SHADOWS_W<'a, O> {
    #[doc = "Do not force shadow register re-load."]
    #[inline(always)]
    pub fn shadow_noforce_reload(self) -> &'a mut W {
        self.variant(RELOAD_SHADOWS_A::SHADOW_NOFORCE_RELOAD)
    }
    #[doc = "Force shadow register re-load. This bit is cleared automatically after shadow registers are re-loaded."]
    #[inline(always)]
    pub fn shadow_force_reload(self) -> &'a mut W {
        self.variant(RELOAD_SHADOWS_A::SHADOW_FORCE_RELOAD)
    }
}
#[doc = "Field `WR_UNLOCK` reader - Write Unlock"]
pub type WR_UNLOCK_R = crate::FieldReader<u16, WR_UNLOCK_A>;
#[doc = "Write Unlock\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum WR_UNLOCK_A {
    #[doc = "0: OTP write access is locked."]
    OTP_W_LOCKED = 0,
    #[doc = "15991: OTP write access is unlocked."]
    OTP_W_UNLOCKED = 15991,
}
impl From<WR_UNLOCK_A> for u16 {
    #[inline(always)]
    fn from(variant: WR_UNLOCK_A) -> Self {
        variant as _
    }
}
impl WR_UNLOCK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WR_UNLOCK_A> {
        match self.bits {
            0 => Some(WR_UNLOCK_A::OTP_W_LOCKED),
            15991 => Some(WR_UNLOCK_A::OTP_W_UNLOCKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OTP_W_LOCKED`"]
    #[inline(always)]
    pub fn is_otp_w_locked(&self) -> bool {
        *self == WR_UNLOCK_A::OTP_W_LOCKED
    }
    #[doc = "Checks if the value of the field is `OTP_W_UNLOCKED`"]
    #[inline(always)]
    pub fn is_otp_w_unlocked(&self) -> bool {
        *self == WR_UNLOCK_A::OTP_W_UNLOCKED
    }
}
#[doc = "Field `WR_UNLOCK` writer - Write Unlock"]
pub type WR_UNLOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HW_OCOTP_CTRL_SPEC, u16, WR_UNLOCK_A, 16, O>;
impl<'a, const O: u8> WR_UNLOCK_W<'a, O> {
    #[doc = "OTP write access is locked."]
    #[inline(always)]
    pub fn otp_w_locked(self) -> &'a mut W {
        self.variant(WR_UNLOCK_A::OTP_W_LOCKED)
    }
    #[doc = "OTP write access is unlocked."]
    #[inline(always)]
    pub fn otp_w_unlocked(self) -> &'a mut W {
        self.variant(WR_UNLOCK_A::OTP_W_UNLOCKED)
    }
}
impl R {
    #[doc = "Bits 0:5 - OTP write and read access address register"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - OTP controller status bit"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Locked Region Access Error"]
    #[inline(always)]
    pub fn error(&self) -> ERROR_R {
        ERROR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reload Shadow Registers"]
    #[inline(always)]
    pub fn reload_shadows(&self) -> RELOAD_SHADOWS_R {
        RELOAD_SHADOWS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Write Unlock"]
    #[inline(always)]
    pub fn wr_unlock(&self) -> WR_UNLOCK_R {
        WR_UNLOCK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - OTP write and read access address register"]
    #[inline(always)]
    #[must_use]
    pub fn addr(&mut self) -> ADDR_W<0> {
        ADDR_W::new(self)
    }
    #[doc = "Bit 9 - Locked Region Access Error"]
    #[inline(always)]
    #[must_use]
    pub fn error(&mut self) -> ERROR_W<9> {
        ERROR_W::new(self)
    }
    #[doc = "Bit 10 - Reload Shadow Registers"]
    #[inline(always)]
    #[must_use]
    pub fn reload_shadows(&mut self) -> RELOAD_SHADOWS_W<10> {
        RELOAD_SHADOWS_W::new(self)
    }
    #[doc = "Bits 16:31 - Write Unlock"]
    #[inline(always)]
    #[must_use]
    pub fn wr_unlock(&mut self) -> WR_UNLOCK_W<16> {
        WR_UNLOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "OTP Controller Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hw_ocotp_ctrl](index.html) module"]
pub struct HW_OCOTP_CTRL_SPEC;
impl crate::RegisterSpec for HW_OCOTP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hw_ocotp_ctrl::R](R) reader structure"]
impl crate::Readable for HW_OCOTP_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hw_ocotp_ctrl::W](W) writer structure"]
impl crate::Writable for HW_OCOTP_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HW_OCOTP_CTRL to value 0"]
impl crate::Resettable for HW_OCOTP_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
