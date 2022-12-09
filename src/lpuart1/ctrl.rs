#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PT` reader - Parity Type"]
pub type PT_R = crate::BitReader<PT_A>;
#[doc = "Parity Type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PT_A {
    #[doc = "0: Even parity."]
    EVEN = 0,
    #[doc = "1: Odd parity."]
    ODD = 1,
}
impl From<PT_A> for bool {
    #[inline(always)]
    fn from(variant: PT_A) -> Self {
        variant as u8 != 0
    }
}
impl PT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PT_A {
        match self.bits {
            false => PT_A::EVEN,
            true => PT_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PT_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PT_A::ODD
    }
}
#[doc = "Field `PT` writer - Parity Type"]
pub type PT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, PT_A, O>;
impl<'a, const O: u8> PT_W<'a, O> {
    #[doc = "Even parity."]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PT_A::EVEN)
    }
    #[doc = "Odd parity."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PT_A::ODD)
    }
}
#[doc = "Field `PE` reader - Parity Enable"]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Parity Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: No hardware parity generation or checking."]
    DISABLED = 0,
    #[doc = "1: Parity enabled."]
    ENABLED = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::DISABLED,
            true => PE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PE_A::ENABLED
    }
}
#[doc = "Field `PE` writer - Parity Enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, PE_A, O>;
impl<'a, const O: u8> PE_W<'a, O> {
    #[doc = "No hardware parity generation or checking."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PE_A::DISABLED)
    }
    #[doc = "Parity enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PE_A::ENABLED)
    }
}
#[doc = "Field `ILT` reader - Idle Line Type Select"]
pub type ILT_R = crate::BitReader<ILT_A>;
#[doc = "Idle Line Type Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILT_A {
    #[doc = "0: Idle character bit count starts after start bit."]
    FROM_START = 0,
    #[doc = "1: Idle character bit count starts after stop bit."]
    FROM_STOP = 1,
}
impl From<ILT_A> for bool {
    #[inline(always)]
    fn from(variant: ILT_A) -> Self {
        variant as u8 != 0
    }
}
impl ILT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILT_A {
        match self.bits {
            false => ILT_A::FROM_START,
            true => ILT_A::FROM_STOP,
        }
    }
    #[doc = "Checks if the value of the field is `FROM_START`"]
    #[inline(always)]
    pub fn is_from_start(&self) -> bool {
        *self == ILT_A::FROM_START
    }
    #[doc = "Checks if the value of the field is `FROM_STOP`"]
    #[inline(always)]
    pub fn is_from_stop(&self) -> bool {
        *self == ILT_A::FROM_STOP
    }
}
#[doc = "Field `ILT` writer - Idle Line Type Select"]
pub type ILT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ILT_A, O>;
impl<'a, const O: u8> ILT_W<'a, O> {
    #[doc = "Idle character bit count starts after start bit."]
    #[inline(always)]
    pub fn from_start(self) -> &'a mut W {
        self.variant(ILT_A::FROM_START)
    }
    #[doc = "Idle character bit count starts after stop bit."]
    #[inline(always)]
    pub fn from_stop(self) -> &'a mut W {
        self.variant(ILT_A::FROM_STOP)
    }
}
#[doc = "Field `WAKE` reader - Receiver Wakeup Method Select"]
pub type WAKE_R = crate::BitReader<WAKE_A>;
#[doc = "Receiver Wakeup Method Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE_A {
    #[doc = "0: Configures RWU for idle-line wakeup."]
    IDLE = 0,
    #[doc = "1: Configures RWU with address-mark wakeup."]
    MARK = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::IDLE,
            true => WAKE_A::MARK,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WAKE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `MARK`"]
    #[inline(always)]
    pub fn is_mark(&self) -> bool {
        *self == WAKE_A::MARK
    }
}
#[doc = "Field `WAKE` writer - Receiver Wakeup Method Select"]
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, WAKE_A, O>;
impl<'a, const O: u8> WAKE_W<'a, O> {
    #[doc = "Configures RWU for idle-line wakeup."]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(WAKE_A::IDLE)
    }
    #[doc = "Configures RWU with address-mark wakeup."]
    #[inline(always)]
    pub fn mark(self) -> &'a mut W {
        self.variant(WAKE_A::MARK)
    }
}
#[doc = "Field `M` reader - 9-Bit or 8-Bit Mode Select"]
pub type M_R = crate::BitReader<M_A>;
#[doc = "9-Bit or 8-Bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_A {
    #[doc = "0: Receiver and transmitter use 8-bit data characters."]
    DATA8 = 0,
    #[doc = "1: Receiver and transmitter use 9-bit data characters."]
    DATA9 = 1,
}
impl From<M_A> for bool {
    #[inline(always)]
    fn from(variant: M_A) -> Self {
        variant as u8 != 0
    }
}
impl M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_A {
        match self.bits {
            false => M_A::DATA8,
            true => M_A::DATA9,
        }
    }
    #[doc = "Checks if the value of the field is `DATA8`"]
    #[inline(always)]
    pub fn is_data8(&self) -> bool {
        *self == M_A::DATA8
    }
    #[doc = "Checks if the value of the field is `DATA9`"]
    #[inline(always)]
    pub fn is_data9(&self) -> bool {
        *self == M_A::DATA9
    }
}
#[doc = "Field `M` writer - 9-Bit or 8-Bit Mode Select"]
pub type M_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, M_A, O>;
impl<'a, const O: u8> M_W<'a, O> {
    #[doc = "Receiver and transmitter use 8-bit data characters."]
    #[inline(always)]
    pub fn data8(self) -> &'a mut W {
        self.variant(M_A::DATA8)
    }
    #[doc = "Receiver and transmitter use 9-bit data characters."]
    #[inline(always)]
    pub fn data9(self) -> &'a mut W {
        self.variant(M_A::DATA9)
    }
}
#[doc = "Field `RSRC` reader - Receiver Source Select"]
pub type RSRC_R = crate::BitReader<RSRC_A>;
#[doc = "Receiver Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RSRC_A {
    #[doc = "0: Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    NO_EFFECT = 0,
    #[doc = "1: Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    ONEWIRE = 1,
}
impl From<RSRC_A> for bool {
    #[inline(always)]
    fn from(variant: RSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl RSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RSRC_A {
        match self.bits {
            false => RSRC_A::NO_EFFECT,
            true => RSRC_A::ONEWIRE,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RSRC_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `ONEWIRE`"]
    #[inline(always)]
    pub fn is_onewire(&self) -> bool {
        *self == RSRC_A::ONEWIRE
    }
}
#[doc = "Field `RSRC` writer - Receiver Source Select"]
pub type RSRC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RSRC_A, O>;
impl<'a, const O: u8> RSRC_W<'a, O> {
    #[doc = "Provided LOOPS is set, RSRC is cleared, selects internal loop back mode and the LPUART does not use the RXD pin."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RSRC_A::NO_EFFECT)
    }
    #[doc = "Single-wire LPUART mode where the TXD pin is connected to the transmitter output and receiver input."]
    #[inline(always)]
    pub fn onewire(self) -> &'a mut W {
        self.variant(RSRC_A::ONEWIRE)
    }
}
#[doc = "Field `DOZEEN` reader - Doze Enable"]
pub type DOZEEN_R = crate::BitReader<DOZEEN_A>;
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOZEEN_A {
    #[doc = "0: LPUART is enabled in Doze mode."]
    ENABLED = 0,
    #[doc = "1: LPUART is disabled in Doze mode ."]
    DISABLED = 1,
}
impl From<DOZEEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DOZEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEEN_A {
        match self.bits {
            false => DOZEEN_A::ENABLED,
            true => DOZEEN_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DOZEEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DOZEEN_A::DISABLED
    }
}
#[doc = "Field `DOZEEN` writer - Doze Enable"]
pub type DOZEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DOZEEN_A, O>;
impl<'a, const O: u8> DOZEEN_W<'a, O> {
    #[doc = "LPUART is enabled in Doze mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DOZEEN_A::ENABLED)
    }
    #[doc = "LPUART is disabled in Doze mode ."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DOZEEN_A::DISABLED)
    }
}
#[doc = "Field `LOOPS` reader - Loop Mode Select"]
pub type LOOPS_R = crate::BitReader<LOOPS_A>;
#[doc = "Loop Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LOOPS_A {
    #[doc = "0: Normal operation - RXD and TXD use separate pins."]
    NOFFECT = 0,
    #[doc = "1: Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    LOOPBACK = 1,
}
impl From<LOOPS_A> for bool {
    #[inline(always)]
    fn from(variant: LOOPS_A) -> Self {
        variant as u8 != 0
    }
}
impl LOOPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LOOPS_A {
        match self.bits {
            false => LOOPS_A::NOFFECT,
            true => LOOPS_A::LOOPBACK,
        }
    }
    #[doc = "Checks if the value of the field is `NOFFECT`"]
    #[inline(always)]
    pub fn is_noffect(&self) -> bool {
        *self == LOOPS_A::NOFFECT
    }
    #[doc = "Checks if the value of the field is `LOOPBACK`"]
    #[inline(always)]
    pub fn is_loopback(&self) -> bool {
        *self == LOOPS_A::LOOPBACK
    }
}
#[doc = "Field `LOOPS` writer - Loop Mode Select"]
pub type LOOPS_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, LOOPS_A, O>;
impl<'a, const O: u8> LOOPS_W<'a, O> {
    #[doc = "Normal operation - RXD and TXD use separate pins."]
    #[inline(always)]
    pub fn noffect(self) -> &'a mut W {
        self.variant(LOOPS_A::NOFFECT)
    }
    #[doc = "Loop mode or single-wire mode where transmitter outputs are internally connected to receiver input (see RSRC bit)."]
    #[inline(always)]
    pub fn loopback(self) -> &'a mut W {
        self.variant(LOOPS_A::LOOPBACK)
    }
}
#[doc = "Field `IDLECFG` reader - Idle Configuration"]
pub type IDLECFG_R = crate::FieldReader<u8, IDLECFG_A>;
#[doc = "Idle Configuration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IDLECFG_A {
    #[doc = "0: 1 idle character"]
    IDLE_1 = 0,
    #[doc = "1: 2 idle characters"]
    IDLE_2 = 1,
    #[doc = "2: 4 idle characters"]
    IDLE_4 = 2,
    #[doc = "3: 8 idle characters"]
    IDLE_8 = 3,
    #[doc = "4: 16 idle characters"]
    IDLE_16 = 4,
    #[doc = "5: 32 idle characters"]
    IDLE_32 = 5,
    #[doc = "6: 64 idle characters"]
    IDLE_64 = 6,
    #[doc = "7: 128 idle characters"]
    IDLE_128 = 7,
}
impl From<IDLECFG_A> for u8 {
    #[inline(always)]
    fn from(variant: IDLECFG_A) -> Self {
        variant as _
    }
}
impl IDLECFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLECFG_A {
        match self.bits {
            0 => IDLECFG_A::IDLE_1,
            1 => IDLECFG_A::IDLE_2,
            2 => IDLECFG_A::IDLE_4,
            3 => IDLECFG_A::IDLE_8,
            4 => IDLECFG_A::IDLE_16,
            5 => IDLECFG_A::IDLE_32,
            6 => IDLECFG_A::IDLE_64,
            7 => IDLECFG_A::IDLE_128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_1`"]
    #[inline(always)]
    pub fn is_idle_1(&self) -> bool {
        *self == IDLECFG_A::IDLE_1
    }
    #[doc = "Checks if the value of the field is `IDLE_2`"]
    #[inline(always)]
    pub fn is_idle_2(&self) -> bool {
        *self == IDLECFG_A::IDLE_2
    }
    #[doc = "Checks if the value of the field is `IDLE_4`"]
    #[inline(always)]
    pub fn is_idle_4(&self) -> bool {
        *self == IDLECFG_A::IDLE_4
    }
    #[doc = "Checks if the value of the field is `IDLE_8`"]
    #[inline(always)]
    pub fn is_idle_8(&self) -> bool {
        *self == IDLECFG_A::IDLE_8
    }
    #[doc = "Checks if the value of the field is `IDLE_16`"]
    #[inline(always)]
    pub fn is_idle_16(&self) -> bool {
        *self == IDLECFG_A::IDLE_16
    }
    #[doc = "Checks if the value of the field is `IDLE_32`"]
    #[inline(always)]
    pub fn is_idle_32(&self) -> bool {
        *self == IDLECFG_A::IDLE_32
    }
    #[doc = "Checks if the value of the field is `IDLE_64`"]
    #[inline(always)]
    pub fn is_idle_64(&self) -> bool {
        *self == IDLECFG_A::IDLE_64
    }
    #[doc = "Checks if the value of the field is `IDLE_128`"]
    #[inline(always)]
    pub fn is_idle_128(&self) -> bool {
        *self == IDLECFG_A::IDLE_128
    }
}
#[doc = "Field `IDLECFG` writer - Idle Configuration"]
pub type IDLECFG_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CTRL_SPEC, u8, IDLECFG_A, 3, O>;
impl<'a, const O: u8> IDLECFG_W<'a, O> {
    #[doc = "1 idle character"]
    #[inline(always)]
    pub fn idle_1(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLE_1)
    }
    #[doc = "2 idle characters"]
    #[inline(always)]
    pub fn idle_2(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLE_2)
    }
    #[doc = "4 idle characters"]
    #[inline(always)]
    pub fn idle_4(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLE_4)
    }
    #[doc = "8 idle characters"]
    #[inline(always)]
    pub fn idle_8(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLE_8)
    }
    #[doc = "16 idle characters"]
    #[inline(always)]
    pub fn idle_16(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLE_16)
    }
    #[doc = "32 idle characters"]
    #[inline(always)]
    pub fn idle_32(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLE_32)
    }
    #[doc = "64 idle characters"]
    #[inline(always)]
    pub fn idle_64(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLE_64)
    }
    #[doc = "128 idle characters"]
    #[inline(always)]
    pub fn idle_128(self) -> &'a mut W {
        self.variant(IDLECFG_A::IDLE_128)
    }
}
#[doc = "Field `M7` reader - 7-Bit Mode Select"]
pub type M7_R = crate::BitReader<M7_A>;
#[doc = "7-Bit Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M7_A {
    #[doc = "0: Receiver and transmitter use 8-bit to 10-bit data characters."]
    NO_EFFECT = 0,
    #[doc = "1: Receiver and transmitter use 7-bit data characters."]
    DATA7 = 1,
}
impl From<M7_A> for bool {
    #[inline(always)]
    fn from(variant: M7_A) -> Self {
        variant as u8 != 0
    }
}
impl M7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M7_A {
        match self.bits {
            false => M7_A::NO_EFFECT,
            true => M7_A::DATA7,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == M7_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `DATA7`"]
    #[inline(always)]
    pub fn is_data7(&self) -> bool {
        *self == M7_A::DATA7
    }
}
#[doc = "Field `M7` writer - 7-Bit Mode Select"]
pub type M7_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, M7_A, O>;
impl<'a, const O: u8> M7_W<'a, O> {
    #[doc = "Receiver and transmitter use 8-bit to 10-bit data characters."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(M7_A::NO_EFFECT)
    }
    #[doc = "Receiver and transmitter use 7-bit data characters."]
    #[inline(always)]
    pub fn data7(self) -> &'a mut W {
        self.variant(M7_A::DATA7)
    }
}
#[doc = "Field `MA2IE` reader - Match 2 Interrupt Enable"]
pub type MA2IE_R = crate::BitReader<MA2IE_A>;
#[doc = "Match 2 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MA2IE_A {
    #[doc = "0: MA2F interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: MA2F interrupt enabled"]
    ENABLED = 1,
}
impl From<MA2IE_A> for bool {
    #[inline(always)]
    fn from(variant: MA2IE_A) -> Self {
        variant as u8 != 0
    }
}
impl MA2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MA2IE_A {
        match self.bits {
            false => MA2IE_A::DISABLED,
            true => MA2IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MA2IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MA2IE_A::ENABLED
    }
}
#[doc = "Field `MA2IE` writer - Match 2 Interrupt Enable"]
pub type MA2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, MA2IE_A, O>;
impl<'a, const O: u8> MA2IE_W<'a, O> {
    #[doc = "MA2F interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MA2IE_A::DISABLED)
    }
    #[doc = "MA2F interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MA2IE_A::ENABLED)
    }
}
#[doc = "Field `MA1IE` reader - Match 1 Interrupt Enable"]
pub type MA1IE_R = crate::BitReader<MA1IE_A>;
#[doc = "Match 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MA1IE_A {
    #[doc = "0: MA1F interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: MA1F interrupt enabled"]
    ENABLED = 1,
}
impl From<MA1IE_A> for bool {
    #[inline(always)]
    fn from(variant: MA1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl MA1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MA1IE_A {
        match self.bits {
            false => MA1IE_A::DISABLED,
            true => MA1IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MA1IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MA1IE_A::ENABLED
    }
}
#[doc = "Field `MA1IE` writer - Match 1 Interrupt Enable"]
pub type MA1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, MA1IE_A, O>;
impl<'a, const O: u8> MA1IE_W<'a, O> {
    #[doc = "MA1F interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MA1IE_A::DISABLED)
    }
    #[doc = "MA1F interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MA1IE_A::ENABLED)
    }
}
#[doc = "Field `SBK` reader - Send Break"]
pub type SBK_R = crate::BitReader<SBK_A>;
#[doc = "Send Break\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBK_A {
    #[doc = "0: Normal transmitter operation."]
    NO_EFFECT = 0,
    #[doc = "1: Queue break character(s) to be sent."]
    TX_BREAK = 1,
}
impl From<SBK_A> for bool {
    #[inline(always)]
    fn from(variant: SBK_A) -> Self {
        variant as u8 != 0
    }
}
impl SBK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBK_A {
        match self.bits {
            false => SBK_A::NO_EFFECT,
            true => SBK_A::TX_BREAK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == SBK_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `TX_BREAK`"]
    #[inline(always)]
    pub fn is_tx_break(&self) -> bool {
        *self == SBK_A::TX_BREAK
    }
}
#[doc = "Field `SBK` writer - Send Break"]
pub type SBK_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SBK_A, O>;
impl<'a, const O: u8> SBK_W<'a, O> {
    #[doc = "Normal transmitter operation."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(SBK_A::NO_EFFECT)
    }
    #[doc = "Queue break character(s) to be sent."]
    #[inline(always)]
    pub fn tx_break(self) -> &'a mut W {
        self.variant(SBK_A::TX_BREAK)
    }
}
#[doc = "Field `RWU` reader - Receiver Wakeup Control"]
pub type RWU_R = crate::BitReader<RWU_A>;
#[doc = "Receiver Wakeup Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RWU_A {
    #[doc = "0: Normal receiver operation."]
    NO_EFFECT = 0,
    #[doc = "1: LPUART receiver in standby waiting for wakeup condition."]
    RX_WAKEUP = 1,
}
impl From<RWU_A> for bool {
    #[inline(always)]
    fn from(variant: RWU_A) -> Self {
        variant as u8 != 0
    }
}
impl RWU_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RWU_A {
        match self.bits {
            false => RWU_A::NO_EFFECT,
            true => RWU_A::RX_WAKEUP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == RWU_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `RX_WAKEUP`"]
    #[inline(always)]
    pub fn is_rx_wakeup(&self) -> bool {
        *self == RWU_A::RX_WAKEUP
    }
}
#[doc = "Field `RWU` writer - Receiver Wakeup Control"]
pub type RWU_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RWU_A, O>;
impl<'a, const O: u8> RWU_W<'a, O> {
    #[doc = "Normal receiver operation."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(RWU_A::NO_EFFECT)
    }
    #[doc = "LPUART receiver in standby waiting for wakeup condition."]
    #[inline(always)]
    pub fn rx_wakeup(self) -> &'a mut W {
        self.variant(RWU_A::RX_WAKEUP)
    }
}
#[doc = "Field `RE` reader - Receiver Enable"]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Receiver Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: Receiver disabled."]
    DISABLED = 0,
    #[doc = "1: Receiver enabled."]
    ENABLED = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::DISABLED,
            true => RE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE_A::ENABLED
    }
}
#[doc = "Field `RE` writer - Receiver Enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RE_A, O>;
impl<'a, const O: u8> RE_W<'a, O> {
    #[doc = "Receiver disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RE_A::DISABLED)
    }
    #[doc = "Receiver enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RE_A::ENABLED)
    }
}
#[doc = "Field `TE` reader - Transmitter Enable"]
pub type TE_R = crate::BitReader<TE_A>;
#[doc = "Transmitter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    #[doc = "0: Transmitter disabled."]
    DISABLED = 0,
    #[doc = "1: Transmitter enabled."]
    ENABLED = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::DISABLED,
            true => TE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE_A::ENABLED
    }
}
#[doc = "Field `TE` writer - Transmitter Enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TE_A, O>;
impl<'a, const O: u8> TE_W<'a, O> {
    #[doc = "Transmitter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TE_A::DISABLED)
    }
    #[doc = "Transmitter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TE_A::ENABLED)
    }
}
#[doc = "Field `ILIE` reader - Idle Line Interrupt Enable"]
pub type ILIE_R = crate::BitReader<ILIE_A>;
#[doc = "Idle Line Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ILIE_A {
    #[doc = "0: Hardware interrupts from IDLE disabled; use polling."]
    DISABLED = 0,
    #[doc = "1: Hardware interrupt is requested when IDLE flag is 1."]
    ENABLED = 1,
}
impl From<ILIE_A> for bool {
    #[inline(always)]
    fn from(variant: ILIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ILIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ILIE_A {
        match self.bits {
            false => ILIE_A::DISABLED,
            true => ILIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ILIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ILIE_A::ENABLED
    }
}
#[doc = "Field `ILIE` writer - Idle Line Interrupt Enable"]
pub type ILIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ILIE_A, O>;
impl<'a, const O: u8> ILIE_W<'a, O> {
    #[doc = "Hardware interrupts from IDLE disabled; use polling."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ILIE_A::DISABLED)
    }
    #[doc = "Hardware interrupt is requested when IDLE flag is 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ILIE_A::ENABLED)
    }
}
#[doc = "Field `RIE` reader - Receiver Interrupt Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Receiver Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: Hardware interrupts from RDRF disabled."]
    DISABLED = 0,
    #[doc = "1: Hardware interrupt is requested when RDRF flag is 1."]
    ENABLED = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::DISABLED,
            true => RIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RIE_A::ENABLED
    }
}
#[doc = "Field `RIE` writer - Receiver Interrupt Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "Hardware interrupts from RDRF disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RIE_A::DISABLED)
    }
    #[doc = "Hardware interrupt is requested when RDRF flag is 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RIE_A::ENABLED)
    }
}
#[doc = "Field `TCIE` reader - Transmission Complete Interrupt Enable for"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "Transmission Complete Interrupt Enable for\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: Hardware interrupts from TC disabled."]
    DISABLED = 0,
    #[doc = "1: Hardware interrupt is requested when TC flag is 1."]
    ENABLED = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::DISABLED,
            true => TCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::ENABLED
    }
}
#[doc = "Field `TCIE` writer - Transmission Complete Interrupt Enable for"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TCIE_A, O>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    #[doc = "Hardware interrupts from TC disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    #[doc = "Hardware interrupt is requested when TC flag is 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::ENABLED)
    }
}
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TIE_R = crate::BitReader<TIE_A>;
#[doc = "Transmit Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIE_A {
    #[doc = "0: Hardware interrupts from TDRE disabled."]
    DISABLED = 0,
    #[doc = "1: Hardware interrupt is requested when TDRE flag is 1."]
    ENABLED = 1,
}
impl From<TIE_A> for bool {
    #[inline(always)]
    fn from(variant: TIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIE_A {
        match self.bits {
            false => TIE_A::DISABLED,
            true => TIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TIE_A::ENABLED
    }
}
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TIE_A, O>;
impl<'a, const O: u8> TIE_W<'a, O> {
    #[doc = "Hardware interrupts from TDRE disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TIE_A::DISABLED)
    }
    #[doc = "Hardware interrupt is requested when TDRE flag is 1."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TIE_A::ENABLED)
    }
}
#[doc = "Field `PEIE` reader - Parity Error Interrupt Enable"]
pub type PEIE_R = crate::BitReader<PEIE_A>;
#[doc = "Parity Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE_A {
    #[doc = "0: PF interrupts disabled; use polling)."]
    DISABLED = 0,
    #[doc = "1: Hardware interrupt is requested when PF is set."]
    ENABLED = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::DISABLED,
            true => PEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEIE_A::ENABLED
    }
}
#[doc = "Field `PEIE` writer - Parity Error Interrupt Enable"]
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, PEIE_A, O>;
impl<'a, const O: u8> PEIE_W<'a, O> {
    #[doc = "PF interrupts disabled; use polling)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEIE_A::DISABLED)
    }
    #[doc = "Hardware interrupt is requested when PF is set."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEIE_A::ENABLED)
    }
}
#[doc = "Field `FEIE` reader - Framing Error Interrupt Enable"]
pub type FEIE_R = crate::BitReader<FEIE_A>;
#[doc = "Framing Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FEIE_A {
    #[doc = "0: FE interrupts disabled; use polling."]
    DISABLED = 0,
    #[doc = "1: Hardware interrupt is requested when FE is set."]
    ENABLED = 1,
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
            false => FEIE_A::DISABLED,
            true => FEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == FEIE_A::ENABLED
    }
}
#[doc = "Field `FEIE` writer - Framing Error Interrupt Enable"]
pub type FEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, FEIE_A, O>;
impl<'a, const O: u8> FEIE_W<'a, O> {
    #[doc = "FE interrupts disabled; use polling."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FEIE_A::DISABLED)
    }
    #[doc = "Hardware interrupt is requested when FE is set."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(FEIE_A::ENABLED)
    }
}
#[doc = "Field `NEIE` reader - Noise Error Interrupt Enable"]
pub type NEIE_R = crate::BitReader<NEIE_A>;
#[doc = "Noise Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NEIE_A {
    #[doc = "0: NF interrupts disabled; use polling."]
    DISABLED = 0,
    #[doc = "1: Hardware interrupt is requested when NF is set."]
    ENABLED = 1,
}
impl From<NEIE_A> for bool {
    #[inline(always)]
    fn from(variant: NEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl NEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NEIE_A {
        match self.bits {
            false => NEIE_A::DISABLED,
            true => NEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NEIE_A::ENABLED
    }
}
#[doc = "Field `NEIE` writer - Noise Error Interrupt Enable"]
pub type NEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, NEIE_A, O>;
impl<'a, const O: u8> NEIE_W<'a, O> {
    #[doc = "NF interrupts disabled; use polling."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NEIE_A::DISABLED)
    }
    #[doc = "Hardware interrupt is requested when NF is set."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NEIE_A::ENABLED)
    }
}
#[doc = "Field `ORIE` reader - Overrun Interrupt Enable"]
pub type ORIE_R = crate::BitReader<ORIE_A>;
#[doc = "Overrun Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ORIE_A {
    #[doc = "0: OR interrupts disabled; use polling."]
    DISABLED = 0,
    #[doc = "1: Hardware interrupt is requested when OR is set."]
    ENABLED = 1,
}
impl From<ORIE_A> for bool {
    #[inline(always)]
    fn from(variant: ORIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ORIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ORIE_A {
        match self.bits {
            false => ORIE_A::DISABLED,
            true => ORIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ORIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ORIE_A::ENABLED
    }
}
#[doc = "Field `ORIE` writer - Overrun Interrupt Enable"]
pub type ORIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, ORIE_A, O>;
impl<'a, const O: u8> ORIE_W<'a, O> {
    #[doc = "OR interrupts disabled; use polling."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ORIE_A::DISABLED)
    }
    #[doc = "Hardware interrupt is requested when OR is set."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ORIE_A::ENABLED)
    }
}
#[doc = "Field `TXINV` reader - Transmit Data Inversion"]
pub type TXINV_R = crate::BitReader<TXINV_A>;
#[doc = "Transmit Data Inversion\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXINV_A {
    #[doc = "0: Transmit data not inverted."]
    NOT_INVERTED = 0,
    #[doc = "1: Transmit data inverted."]
    INVERTED = 1,
}
impl From<TXINV_A> for bool {
    #[inline(always)]
    fn from(variant: TXINV_A) -> Self {
        variant as u8 != 0
    }
}
impl TXINV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXINV_A {
        match self.bits {
            false => TXINV_A::NOT_INVERTED,
            true => TXINV_A::INVERTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_INVERTED`"]
    #[inline(always)]
    pub fn is_not_inverted(&self) -> bool {
        *self == TXINV_A::NOT_INVERTED
    }
    #[doc = "Checks if the value of the field is `INVERTED`"]
    #[inline(always)]
    pub fn is_inverted(&self) -> bool {
        *self == TXINV_A::INVERTED
    }
}
#[doc = "Field `TXINV` writer - Transmit Data Inversion"]
pub type TXINV_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TXINV_A, O>;
impl<'a, const O: u8> TXINV_W<'a, O> {
    #[doc = "Transmit data not inverted."]
    #[inline(always)]
    pub fn not_inverted(self) -> &'a mut W {
        self.variant(TXINV_A::NOT_INVERTED)
    }
    #[doc = "Transmit data inverted."]
    #[inline(always)]
    pub fn inverted(self) -> &'a mut W {
        self.variant(TXINV_A::INVERTED)
    }
}
#[doc = "Field `TXDIR` reader - TXD Pin Direction in Single-Wire Mode"]
pub type TXDIR_R = crate::BitReader<TXDIR_A>;
#[doc = "TXD Pin Direction in Single-Wire Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDIR_A {
    #[doc = "0: TXD pin is an input in single-wire mode."]
    TX_INPUT = 0,
    #[doc = "1: TXD pin is an output in single-wire mode."]
    TX_OUTPUT = 1,
}
impl From<TXDIR_A> for bool {
    #[inline(always)]
    fn from(variant: TXDIR_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDIR_A {
        match self.bits {
            false => TXDIR_A::TX_INPUT,
            true => TXDIR_A::TX_OUTPUT,
        }
    }
    #[doc = "Checks if the value of the field is `TX_INPUT`"]
    #[inline(always)]
    pub fn is_tx_input(&self) -> bool {
        *self == TXDIR_A::TX_INPUT
    }
    #[doc = "Checks if the value of the field is `TX_OUTPUT`"]
    #[inline(always)]
    pub fn is_tx_output(&self) -> bool {
        *self == TXDIR_A::TX_OUTPUT
    }
}
#[doc = "Field `TXDIR` writer - TXD Pin Direction in Single-Wire Mode"]
pub type TXDIR_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, TXDIR_A, O>;
impl<'a, const O: u8> TXDIR_W<'a, O> {
    #[doc = "TXD pin is an input in single-wire mode."]
    #[inline(always)]
    pub fn tx_input(self) -> &'a mut W {
        self.variant(TXDIR_A::TX_INPUT)
    }
    #[doc = "TXD pin is an output in single-wire mode."]
    #[inline(always)]
    pub fn tx_output(self) -> &'a mut W {
        self.variant(TXDIR_A::TX_OUTPUT)
    }
}
#[doc = "Field `R9T8` reader - Receive Bit 9 / Transmit Bit 8"]
pub type R9T8_R = crate::BitReader<bool>;
#[doc = "Field `R9T8` writer - Receive Bit 9 / Transmit Bit 8"]
pub type R9T8_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
#[doc = "Field `R8T9` reader - Receive Bit 8 / Transmit Bit 9"]
pub type R8T9_R = crate::BitReader<bool>;
#[doc = "Field `R8T9` writer - Receive Bit 8 / Transmit Bit 9"]
pub type R8T9_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    pub fn pt(&self) -> PT_R {
        PT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    pub fn ilt(&self) -> ILT_R {
        ILT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    pub fn rsrc(&self) -> RSRC_R {
        RSRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline(always)]
    pub fn dozeen(&self) -> DOZEEN_R {
        DOZEEN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    pub fn loops(&self) -> LOOPS_R {
        LOOPS_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline(always)]
    pub fn idlecfg(&self) -> IDLECFG_R {
        IDLECFG_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - 7-Bit Mode Select"]
    #[inline(always)]
    pub fn m7(&self) -> M7_R {
        M7_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline(always)]
    pub fn ma2ie(&self) -> MA2IE_R {
        MA2IE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ma1ie(&self) -> MA1IE_R {
        MA1IE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline(always)]
    pub fn sbk(&self) -> SBK_R {
        SBK_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline(always)]
    pub fn rwu(&self) -> RWU_R {
        RWU_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline(always)]
    pub fn ilie(&self) -> ILIE_R {
        ILIE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TIE_R {
        TIE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline(always)]
    pub fn feie(&self) -> FEIE_R {
        FEIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline(always)]
    pub fn neie(&self) -> NEIE_R {
        NEIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline(always)]
    pub fn orie(&self) -> ORIE_R {
        ORIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline(always)]
    pub fn txinv(&self) -> TXINV_R {
        TXINV_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - TXD Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    pub fn txdir(&self) -> TXDIR_R {
        TXDIR_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    pub fn r9t8(&self) -> R9T8_R {
        R9T8_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    pub fn r8t9(&self) -> R8T9_R {
        R8T9_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Parity Type"]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PT_W<0> {
        PT_W::new(self)
    }
    #[doc = "Bit 1 - Parity Enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<1> {
        PE_W::new(self)
    }
    #[doc = "Bit 2 - Idle Line Type Select"]
    #[inline(always)]
    #[must_use]
    pub fn ilt(&mut self) -> ILT_W<2> {
        ILT_W::new(self)
    }
    #[doc = "Bit 3 - Receiver Wakeup Method Select"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<3> {
        WAKE_W::new(self)
    }
    #[doc = "Bit 4 - 9-Bit or 8-Bit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<4> {
        M_W::new(self)
    }
    #[doc = "Bit 5 - Receiver Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn rsrc(&mut self) -> RSRC_W<5> {
        RSRC_W::new(self)
    }
    #[doc = "Bit 6 - Doze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dozeen(&mut self) -> DOZEEN_W<6> {
        DOZEEN_W::new(self)
    }
    #[doc = "Bit 7 - Loop Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn loops(&mut self) -> LOOPS_W<7> {
        LOOPS_W::new(self)
    }
    #[doc = "Bits 8:10 - Idle Configuration"]
    #[inline(always)]
    #[must_use]
    pub fn idlecfg(&mut self) -> IDLECFG_W<8> {
        IDLECFG_W::new(self)
    }
    #[doc = "Bit 11 - 7-Bit Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn m7(&mut self) -> M7_W<11> {
        M7_W::new(self)
    }
    #[doc = "Bit 14 - Match 2 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ma2ie(&mut self) -> MA2IE_W<14> {
        MA2IE_W::new(self)
    }
    #[doc = "Bit 15 - Match 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ma1ie(&mut self) -> MA1IE_W<15> {
        MA1IE_W::new(self)
    }
    #[doc = "Bit 16 - Send Break"]
    #[inline(always)]
    #[must_use]
    pub fn sbk(&mut self) -> SBK_W<16> {
        SBK_W::new(self)
    }
    #[doc = "Bit 17 - Receiver Wakeup Control"]
    #[inline(always)]
    #[must_use]
    pub fn rwu(&mut self) -> RWU_W<17> {
        RWU_W::new(self)
    }
    #[doc = "Bit 18 - Receiver Enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<18> {
        RE_W::new(self)
    }
    #[doc = "Bit 19 - Transmitter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<19> {
        TE_W::new(self)
    }
    #[doc = "Bit 20 - Idle Line Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ilie(&mut self) -> ILIE_W<20> {
        ILIE_W::new(self)
    }
    #[doc = "Bit 21 - Receiver Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<21> {
        RIE_W::new(self)
    }
    #[doc = "Bit 22 - Transmission Complete Interrupt Enable for"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<22> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 23 - Transmit Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tie(&mut self) -> TIE_W<23> {
        TIE_W::new(self)
    }
    #[doc = "Bit 24 - Parity Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<24> {
        PEIE_W::new(self)
    }
    #[doc = "Bit 25 - Framing Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn feie(&mut self) -> FEIE_W<25> {
        FEIE_W::new(self)
    }
    #[doc = "Bit 26 - Noise Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn neie(&mut self) -> NEIE_W<26> {
        NEIE_W::new(self)
    }
    #[doc = "Bit 27 - Overrun Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn orie(&mut self) -> ORIE_W<27> {
        ORIE_W::new(self)
    }
    #[doc = "Bit 28 - Transmit Data Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn txinv(&mut self) -> TXINV_W<28> {
        TXINV_W::new(self)
    }
    #[doc = "Bit 29 - TXD Pin Direction in Single-Wire Mode"]
    #[inline(always)]
    #[must_use]
    pub fn txdir(&mut self) -> TXDIR_W<29> {
        TXDIR_W::new(self)
    }
    #[doc = "Bit 30 - Receive Bit 9 / Transmit Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn r9t8(&mut self) -> R9T8_W<30> {
        R9T8_W::new(self)
    }
    #[doc = "Bit 31 - Receive Bit 8 / Transmit Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn r8t9(&mut self) -> R8T9_W<31> {
        R8T9_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LPUART Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
