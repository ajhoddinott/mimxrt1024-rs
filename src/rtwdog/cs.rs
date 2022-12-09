#[doc = "Register `CS` reader"]
pub struct R(crate::R<CS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CS` writer"]
pub struct W(crate::W<CS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CS_SPEC>;
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
impl From<crate::W<CS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `STOP` reader - Stop Enable"]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "Stop Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: Watchdog disabled in chip stop mode."]
    STOP_0 = 0,
    #[doc = "1: Watchdog enabled in chip stop mode."]
    STOP_1 = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::STOP_0,
            true => STOP_A::STOP_1,
        }
    }
    #[doc = "Checks if the value of the field is `STOP_0`"]
    #[inline(always)]
    pub fn is_stop_0(&self) -> bool {
        *self == STOP_A::STOP_0
    }
    #[doc = "Checks if the value of the field is `STOP_1`"]
    #[inline(always)]
    pub fn is_stop_1(&self) -> bool {
        *self == STOP_A::STOP_1
    }
}
#[doc = "Field `STOP` writer - Stop Enable"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, STOP_A, O>;
impl<'a, const O: u8> STOP_W<'a, O> {
    #[doc = "Watchdog disabled in chip stop mode."]
    #[inline(always)]
    pub fn stop_0(self) -> &'a mut W {
        self.variant(STOP_A::STOP_0)
    }
    #[doc = "Watchdog enabled in chip stop mode."]
    #[inline(always)]
    pub fn stop_1(self) -> &'a mut W {
        self.variant(STOP_A::STOP_1)
    }
}
#[doc = "Field `WAIT` reader - Wait Enable"]
pub type WAIT_R = crate::BitReader<WAIT_A>;
#[doc = "Wait Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAIT_A {
    #[doc = "0: Watchdog disabled in chip wait mode."]
    WAIT_0 = 0,
    #[doc = "1: Watchdog enabled in chip wait mode."]
    WAIT_1 = 1,
}
impl From<WAIT_A> for bool {
    #[inline(always)]
    fn from(variant: WAIT_A) -> Self {
        variant as u8 != 0
    }
}
impl WAIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAIT_A {
        match self.bits {
            false => WAIT_A::WAIT_0,
            true => WAIT_A::WAIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WAIT_0`"]
    #[inline(always)]
    pub fn is_wait_0(&self) -> bool {
        *self == WAIT_A::WAIT_0
    }
    #[doc = "Checks if the value of the field is `WAIT_1`"]
    #[inline(always)]
    pub fn is_wait_1(&self) -> bool {
        *self == WAIT_A::WAIT_1
    }
}
#[doc = "Field `WAIT` writer - Wait Enable"]
pub type WAIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, WAIT_A, O>;
impl<'a, const O: u8> WAIT_W<'a, O> {
    #[doc = "Watchdog disabled in chip wait mode."]
    #[inline(always)]
    pub fn wait_0(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_0)
    }
    #[doc = "Watchdog enabled in chip wait mode."]
    #[inline(always)]
    pub fn wait_1(self) -> &'a mut W {
        self.variant(WAIT_A::WAIT_1)
    }
}
#[doc = "Field `DBG` reader - Debug Enable"]
pub type DBG_R = crate::BitReader<DBG_A>;
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBG_A {
    #[doc = "0: Watchdog disabled in chip debug mode."]
    DBG_0 = 0,
    #[doc = "1: Watchdog enabled in chip debug mode."]
    DBG_1 = 1,
}
impl From<DBG_A> for bool {
    #[inline(always)]
    fn from(variant: DBG_A) -> Self {
        variant as u8 != 0
    }
}
impl DBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBG_A {
        match self.bits {
            false => DBG_A::DBG_0,
            true => DBG_A::DBG_1,
        }
    }
    #[doc = "Checks if the value of the field is `DBG_0`"]
    #[inline(always)]
    pub fn is_dbg_0(&self) -> bool {
        *self == DBG_A::DBG_0
    }
    #[doc = "Checks if the value of the field is `DBG_1`"]
    #[inline(always)]
    pub fn is_dbg_1(&self) -> bool {
        *self == DBG_A::DBG_1
    }
}
#[doc = "Field `DBG` writer - Debug Enable"]
pub type DBG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, DBG_A, O>;
impl<'a, const O: u8> DBG_W<'a, O> {
    #[doc = "Watchdog disabled in chip debug mode."]
    #[inline(always)]
    pub fn dbg_0(self) -> &'a mut W {
        self.variant(DBG_A::DBG_0)
    }
    #[doc = "Watchdog enabled in chip debug mode."]
    #[inline(always)]
    pub fn dbg_1(self) -> &'a mut W {
        self.variant(DBG_A::DBG_1)
    }
}
#[doc = "Field `TST` reader - Watchdog Test"]
pub type TST_R = crate::FieldReader<u8, TST_A>;
#[doc = "Watchdog Test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TST_A {
    #[doc = "0: Watchdog test mode disabled."]
    TST_0 = 0,
    #[doc = "1: Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    TST_1 = 1,
    #[doc = "2: Watchdog test mode enabled, only the low byte is used. CNT\\[CNTLOW\\]
is compared with TOVAL\\[TOVALLOW\\]."]
    TST_2 = 2,
    #[doc = "3: Watchdog test mode enabled, only the high byte is used. CNT\\[CNTHIGH\\]
is compared with TOVAL\\[TOVALHIGH\\]."]
    TST_3 = 3,
}
impl From<TST_A> for u8 {
    #[inline(always)]
    fn from(variant: TST_A) -> Self {
        variant as _
    }
}
impl TST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TST_A {
        match self.bits {
            0 => TST_A::TST_0,
            1 => TST_A::TST_1,
            2 => TST_A::TST_2,
            3 => TST_A::TST_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `TST_0`"]
    #[inline(always)]
    pub fn is_tst_0(&self) -> bool {
        *self == TST_A::TST_0
    }
    #[doc = "Checks if the value of the field is `TST_1`"]
    #[inline(always)]
    pub fn is_tst_1(&self) -> bool {
        *self == TST_A::TST_1
    }
    #[doc = "Checks if the value of the field is `TST_2`"]
    #[inline(always)]
    pub fn is_tst_2(&self) -> bool {
        *self == TST_A::TST_2
    }
    #[doc = "Checks if the value of the field is `TST_3`"]
    #[inline(always)]
    pub fn is_tst_3(&self) -> bool {
        *self == TST_A::TST_3
    }
}
#[doc = "Field `TST` writer - Watchdog Test"]
pub type TST_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CS_SPEC, u8, TST_A, 2, O>;
impl<'a, const O: u8> TST_W<'a, O> {
    #[doc = "Watchdog test mode disabled."]
    #[inline(always)]
    pub fn tst_0(self) -> &'a mut W {
        self.variant(TST_A::TST_0)
    }
    #[doc = "Watchdog user mode enabled. (Watchdog test mode disabled.) After testing the watchdog, software should use this setting to indicate that the watchdog is functioning normally in user mode."]
    #[inline(always)]
    pub fn tst_1(self) -> &'a mut W {
        self.variant(TST_A::TST_1)
    }
    #[doc = "Watchdog test mode enabled, only the low byte is used. CNT\\[CNTLOW\\]
is compared with TOVAL\\[TOVALLOW\\]."]
    #[inline(always)]
    pub fn tst_2(self) -> &'a mut W {
        self.variant(TST_A::TST_2)
    }
    #[doc = "Watchdog test mode enabled, only the high byte is used. CNT\\[CNTHIGH\\]
is compared with TOVAL\\[TOVALHIGH\\]."]
    #[inline(always)]
    pub fn tst_3(self) -> &'a mut W {
        self.variant(TST_A::TST_3)
    }
}
#[doc = "Field `UPDATE` reader - Allow updates"]
pub type UPDATE_R = crate::BitReader<UPDATE_A>;
#[doc = "Allow updates\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UPDATE_A {
    #[doc = "0: Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    UPDATE_0 = 0,
    #[doc = "1: Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence."]
    UPDATE_1 = 1,
}
impl From<UPDATE_A> for bool {
    #[inline(always)]
    fn from(variant: UPDATE_A) -> Self {
        variant as u8 != 0
    }
}
impl UPDATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UPDATE_A {
        match self.bits {
            false => UPDATE_A::UPDATE_0,
            true => UPDATE_A::UPDATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `UPDATE_0`"]
    #[inline(always)]
    pub fn is_update_0(&self) -> bool {
        *self == UPDATE_A::UPDATE_0
    }
    #[doc = "Checks if the value of the field is `UPDATE_1`"]
    #[inline(always)]
    pub fn is_update_1(&self) -> bool {
        *self == UPDATE_A::UPDATE_1
    }
}
#[doc = "Field `UPDATE` writer - Allow updates"]
pub type UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, UPDATE_A, O>;
impl<'a, const O: u8> UPDATE_W<'a, O> {
    #[doc = "Updates not allowed. After the initial configuration, the watchdog cannot be later modified without forcing a reset."]
    #[inline(always)]
    pub fn update_0(self) -> &'a mut W {
        self.variant(UPDATE_A::UPDATE_0)
    }
    #[doc = "Updates allowed. Software can modify the watchdog configuration registers within 128 bus clocks after performing the unlock write sequence."]
    #[inline(always)]
    pub fn update_1(self) -> &'a mut W {
        self.variant(UPDATE_A::UPDATE_1)
    }
}
#[doc = "Field `INT` reader - Watchdog Interrupt"]
pub type INT_R = crate::BitReader<INT_A>;
#[doc = "Watchdog Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_A {
    #[doc = "0: Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    INT_0 = 0,
    #[doc = "1: Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks from the interrupt vector fetch."]
    INT_1 = 1,
}
impl From<INT_A> for bool {
    #[inline(always)]
    fn from(variant: INT_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_A {
        match self.bits {
            false => INT_A::INT_0,
            true => INT_A::INT_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_0`"]
    #[inline(always)]
    pub fn is_int_0(&self) -> bool {
        *self == INT_A::INT_0
    }
    #[doc = "Checks if the value of the field is `INT_1`"]
    #[inline(always)]
    pub fn is_int_1(&self) -> bool {
        *self == INT_A::INT_1
    }
}
#[doc = "Field `INT` writer - Watchdog Interrupt"]
pub type INT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, INT_A, O>;
impl<'a, const O: u8> INT_W<'a, O> {
    #[doc = "Watchdog interrupts are disabled. Watchdog resets are not delayed."]
    #[inline(always)]
    pub fn int_0(self) -> &'a mut W {
        self.variant(INT_A::INT_0)
    }
    #[doc = "Watchdog interrupts are enabled. Watchdog resets are delayed by 128 bus clocks from the interrupt vector fetch."]
    #[inline(always)]
    pub fn int_1(self) -> &'a mut W {
        self.variant(INT_A::INT_1)
    }
}
#[doc = "Field `EN` reader - Watchdog Enable"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Watchdog Enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Watchdog disabled."]
    EN_0 = 0,
    #[doc = "1: Watchdog enabled."]
    EN_1 = 1,
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
            false => EN_A::EN_0,
            true => EN_A::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        *self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        *self == EN_A::EN_1
    }
}
#[doc = "Field `EN` writer - Watchdog Enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Watchdog disabled."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "Watchdog enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
    }
}
#[doc = "Field `CLK` reader - Watchdog Clock"]
pub type CLK_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLK` writer - Watchdog Clock"]
pub type CLK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CS_SPEC, u8, u8, 2, O>;
#[doc = "Field `RCS` reader - Reconfiguration Success"]
pub type RCS_R = crate::BitReader<RCS_A>;
#[doc = "Reconfiguration Success\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RCS_A {
    #[doc = "0: Reconfiguring WDOG."]
    RCS_0 = 0,
    #[doc = "1: Reconfiguration is successful."]
    RCS_1 = 1,
}
impl From<RCS_A> for bool {
    #[inline(always)]
    fn from(variant: RCS_A) -> Self {
        variant as u8 != 0
    }
}
impl RCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RCS_A {
        match self.bits {
            false => RCS_A::RCS_0,
            true => RCS_A::RCS_1,
        }
    }
    #[doc = "Checks if the value of the field is `RCS_0`"]
    #[inline(always)]
    pub fn is_rcs_0(&self) -> bool {
        *self == RCS_A::RCS_0
    }
    #[doc = "Checks if the value of the field is `RCS_1`"]
    #[inline(always)]
    pub fn is_rcs_1(&self) -> bool {
        *self == RCS_A::RCS_1
    }
}
#[doc = "Field `ULK` reader - Unlock status"]
pub type ULK_R = crate::BitReader<ULK_A>;
#[doc = "Unlock status\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ULK_A {
    #[doc = "0: WDOG is locked."]
    ULK_0 = 0,
    #[doc = "1: WDOG is unlocked."]
    ULK_1 = 1,
}
impl From<ULK_A> for bool {
    #[inline(always)]
    fn from(variant: ULK_A) -> Self {
        variant as u8 != 0
    }
}
impl ULK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ULK_A {
        match self.bits {
            false => ULK_A::ULK_0,
            true => ULK_A::ULK_1,
        }
    }
    #[doc = "Checks if the value of the field is `ULK_0`"]
    #[inline(always)]
    pub fn is_ulk_0(&self) -> bool {
        *self == ULK_A::ULK_0
    }
    #[doc = "Checks if the value of the field is `ULK_1`"]
    #[inline(always)]
    pub fn is_ulk_1(&self) -> bool {
        *self == ULK_A::ULK_1
    }
}
#[doc = "Field `PRES` reader - Watchdog prescaler"]
pub type PRES_R = crate::BitReader<PRES_A>;
#[doc = "Watchdog prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PRES_A {
    #[doc = "0: 256 prescaler disabled."]
    PRES_0 = 0,
    #[doc = "1: 256 prescaler enabled."]
    PRES_1 = 1,
}
impl From<PRES_A> for bool {
    #[inline(always)]
    fn from(variant: PRES_A) -> Self {
        variant as u8 != 0
    }
}
impl PRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PRES_A {
        match self.bits {
            false => PRES_A::PRES_0,
            true => PRES_A::PRES_1,
        }
    }
    #[doc = "Checks if the value of the field is `PRES_0`"]
    #[inline(always)]
    pub fn is_pres_0(&self) -> bool {
        *self == PRES_A::PRES_0
    }
    #[doc = "Checks if the value of the field is `PRES_1`"]
    #[inline(always)]
    pub fn is_pres_1(&self) -> bool {
        *self == PRES_A::PRES_1
    }
}
#[doc = "Field `PRES` writer - Watchdog prescaler"]
pub type PRES_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, PRES_A, O>;
impl<'a, const O: u8> PRES_W<'a, O> {
    #[doc = "256 prescaler disabled."]
    #[inline(always)]
    pub fn pres_0(self) -> &'a mut W {
        self.variant(PRES_A::PRES_0)
    }
    #[doc = "256 prescaler enabled."]
    #[inline(always)]
    pub fn pres_1(self) -> &'a mut W {
        self.variant(PRES_A::PRES_1)
    }
}
#[doc = "Field `CMD32EN` reader - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
pub type CMD32EN_R = crate::BitReader<CMD32EN_A>;
#[doc = "Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMD32EN_A {
    #[doc = "0: Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    CMD32EN_0 = 0,
    #[doc = "1: Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    CMD32EN_1 = 1,
}
impl From<CMD32EN_A> for bool {
    #[inline(always)]
    fn from(variant: CMD32EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CMD32EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMD32EN_A {
        match self.bits {
            false => CMD32EN_A::CMD32EN_0,
            true => CMD32EN_A::CMD32EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMD32EN_0`"]
    #[inline(always)]
    pub fn is_cmd32en_0(&self) -> bool {
        *self == CMD32EN_A::CMD32EN_0
    }
    #[doc = "Checks if the value of the field is `CMD32EN_1`"]
    #[inline(always)]
    pub fn is_cmd32en_1(&self) -> bool {
        *self == CMD32EN_A::CMD32EN_1
    }
}
#[doc = "Field `CMD32EN` writer - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
pub type CMD32EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, CMD32EN_A, O>;
impl<'a, const O: u8> CMD32EN_W<'a, O> {
    #[doc = "Disables support for 32-bit refresh/unlock command write words. Only 16-bit or 8-bit is supported."]
    #[inline(always)]
    pub fn cmd32en_0(self) -> &'a mut W {
        self.variant(CMD32EN_A::CMD32EN_0)
    }
    #[doc = "Enables support for 32-bit refresh/unlock command write words. 16-bit or 8-bit is NOT supported."]
    #[inline(always)]
    pub fn cmd32en_1(self) -> &'a mut W {
        self.variant(CMD32EN_A::CMD32EN_1)
    }
}
#[doc = "Field `FLG` reader - Watchdog Interrupt Flag"]
pub type FLG_R = crate::BitReader<FLG_A>;
#[doc = "Watchdog Interrupt Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLG_A {
    #[doc = "0: No interrupt occurred."]
    FLG_0 = 0,
    #[doc = "1: An interrupt occurred."]
    FLG_1 = 1,
}
impl From<FLG_A> for bool {
    #[inline(always)]
    fn from(variant: FLG_A) -> Self {
        variant as u8 != 0
    }
}
impl FLG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLG_A {
        match self.bits {
            false => FLG_A::FLG_0,
            true => FLG_A::FLG_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLG_0`"]
    #[inline(always)]
    pub fn is_flg_0(&self) -> bool {
        *self == FLG_A::FLG_0
    }
    #[doc = "Checks if the value of the field is `FLG_1`"]
    #[inline(always)]
    pub fn is_flg_1(&self) -> bool {
        *self == FLG_A::FLG_1
    }
}
#[doc = "Field `FLG` writer - Watchdog Interrupt Flag"]
pub type FLG_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, CS_SPEC, FLG_A, O>;
impl<'a, const O: u8> FLG_W<'a, O> {
    #[doc = "No interrupt occurred."]
    #[inline(always)]
    pub fn flg_0(self) -> &'a mut W {
        self.variant(FLG_A::FLG_0)
    }
    #[doc = "An interrupt occurred."]
    #[inline(always)]
    pub fn flg_1(self) -> &'a mut W {
        self.variant(FLG_A::FLG_1)
    }
}
#[doc = "Field `WIN` reader - Watchdog Window"]
pub type WIN_R = crate::BitReader<WIN_A>;
#[doc = "Watchdog Window\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WIN_A {
    #[doc = "0: Window mode disabled."]
    WIN_0 = 0,
    #[doc = "1: Window mode enabled."]
    WIN_1 = 1,
}
impl From<WIN_A> for bool {
    #[inline(always)]
    fn from(variant: WIN_A) -> Self {
        variant as u8 != 0
    }
}
impl WIN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIN_A {
        match self.bits {
            false => WIN_A::WIN_0,
            true => WIN_A::WIN_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIN_0`"]
    #[inline(always)]
    pub fn is_win_0(&self) -> bool {
        *self == WIN_A::WIN_0
    }
    #[doc = "Checks if the value of the field is `WIN_1`"]
    #[inline(always)]
    pub fn is_win_1(&self) -> bool {
        *self == WIN_A::WIN_1
    }
}
#[doc = "Field `WIN` writer - Watchdog Window"]
pub type WIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CS_SPEC, WIN_A, O>;
impl<'a, const O: u8> WIN_W<'a, O> {
    #[doc = "Window mode disabled."]
    #[inline(always)]
    pub fn win_0(self) -> &'a mut W {
        self.variant(WIN_A::WIN_0)
    }
    #[doc = "Window mode enabled."]
    #[inline(always)]
    pub fn win_1(self) -> &'a mut W {
        self.variant(WIN_A::WIN_1)
    }
}
impl R {
    #[doc = "Bit 0 - Stop Enable"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline(always)]
    pub fn wait(&self) -> WAIT_R {
        WAIT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline(always)]
    pub fn dbg(&self) -> DBG_R {
        DBG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline(always)]
    pub fn tst(&self) -> TST_R {
        TST_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline(always)]
    pub fn update(&self) -> UPDATE_R {
        UPDATE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn int(&self) -> INT_R {
        INT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Watchdog Clock"]
    #[inline(always)]
    pub fn clk(&self) -> CLK_R {
        CLK_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Reconfiguration Success"]
    #[inline(always)]
    pub fn rcs(&self) -> RCS_R {
        RCS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Unlock status"]
    #[inline(always)]
    pub fn ulk(&self) -> ULK_R {
        ULK_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Watchdog prescaler"]
    #[inline(always)]
    pub fn pres(&self) -> PRES_R {
        PRES_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline(always)]
    pub fn cmd32en(&self) -> CMD32EN_R {
        CMD32EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Watchdog Interrupt Flag"]
    #[inline(always)]
    pub fn flg(&self) -> FLG_R {
        FLG_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Watchdog Window"]
    #[inline(always)]
    pub fn win(&self) -> WIN_R {
        WIN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Stop Enable"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<0> {
        STOP_W::new(self)
    }
    #[doc = "Bit 1 - Wait Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wait(&mut self) -> WAIT_W<1> {
        WAIT_W::new(self)
    }
    #[doc = "Bit 2 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbg(&mut self) -> DBG_W<2> {
        DBG_W::new(self)
    }
    #[doc = "Bits 3:4 - Watchdog Test"]
    #[inline(always)]
    #[must_use]
    pub fn tst(&mut self) -> TST_W<3> {
        TST_W::new(self)
    }
    #[doc = "Bit 5 - Allow updates"]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<5> {
        UPDATE_W::new(self)
    }
    #[doc = "Bit 6 - Watchdog Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn int(&mut self) -> INT_W<6> {
        INT_W::new(self)
    }
    #[doc = "Bit 7 - Watchdog Enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<7> {
        EN_W::new(self)
    }
    #[doc = "Bits 8:9 - Watchdog Clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk(&mut self) -> CLK_W<8> {
        CLK_W::new(self)
    }
    #[doc = "Bit 12 - Watchdog prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn pres(&mut self) -> PRES_W<12> {
        PRES_W::new(self)
    }
    #[doc = "Bit 13 - Enables or disables WDOG support for 32-bit (otherwise 16-bit or 8-bit) refresh/unlock command write words"]
    #[inline(always)]
    #[must_use]
    pub fn cmd32en(&mut self) -> CMD32EN_W<13> {
        CMD32EN_W::new(self)
    }
    #[doc = "Bit 14 - Watchdog Interrupt Flag"]
    #[inline(always)]
    #[must_use]
    pub fn flg(&mut self) -> FLG_W<14> {
        FLG_W::new(self)
    }
    #[doc = "Bit 15 - Watchdog Window"]
    #[inline(always)]
    #[must_use]
    pub fn win(&mut self) -> WIN_W<15> {
        WIN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Control and Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cs](index.html) module"]
pub struct CS_SPEC;
impl crate::RegisterSpec for CS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cs::R](R) reader structure"]
impl crate::Readable for CS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cs::W](W) writer structure"]
impl crate::Writable for CS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x4000;
}
#[doc = "`reset()` method sets CS to value 0x2980"]
impl crate::Resettable for CS_SPEC {
    const RESET_VALUE: Self::Ux = 0x2980;
}
