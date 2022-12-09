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
#[doc = "Field `CMPIE` reader - Compare Interrupt Enable"]
pub type CMPIE_R = crate::BitReader<CMPIE_A>;
#[doc = "Compare Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPIE_A {
    #[doc = "0: Disabled"]
    CMPIE_0 = 0,
    #[doc = "1: Enabled"]
    CMPIE_1 = 1,
}
impl From<CMPIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPIE_A {
        match self.bits {
            false => CMPIE_A::CMPIE_0,
            true => CMPIE_A::CMPIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMPIE_0`"]
    #[inline(always)]
    pub fn is_cmpie_0(&self) -> bool {
        *self == CMPIE_A::CMPIE_0
    }
    #[doc = "Checks if the value of the field is `CMPIE_1`"]
    #[inline(always)]
    pub fn is_cmpie_1(&self) -> bool {
        *self == CMPIE_A::CMPIE_1
    }
}
#[doc = "Field `CMPIE` writer - Compare Interrupt Enable"]
pub type CMPIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, CMPIE_A, O>;
impl<'a, const O: u8> CMPIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn cmpie_0(self) -> &'a mut W {
        self.variant(CMPIE_A::CMPIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn cmpie_1(self) -> &'a mut W {
        self.variant(CMPIE_A::CMPIE_1)
    }
}
#[doc = "Field `CMPIRQ` reader - Compare Interrupt Request"]
pub type CMPIRQ_R = crate::BitReader<CMPIRQ_A>;
#[doc = "Compare Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMPIRQ_A {
    #[doc = "0: No match has occurred (the counter does not match the COMP value)"]
    CMPIRQ_0 = 0,
    #[doc = "1: COMP match has occurred (the counter matches the COMP value)"]
    CMPIRQ_1 = 1,
}
impl From<CMPIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: CMPIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl CMPIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMPIRQ_A {
        match self.bits {
            false => CMPIRQ_A::CMPIRQ_0,
            true => CMPIRQ_A::CMPIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `CMPIRQ_0`"]
    #[inline(always)]
    pub fn is_cmpirq_0(&self) -> bool {
        *self == CMPIRQ_A::CMPIRQ_0
    }
    #[doc = "Checks if the value of the field is `CMPIRQ_1`"]
    #[inline(always)]
    pub fn is_cmpirq_1(&self) -> bool {
        *self == CMPIRQ_A::CMPIRQ_1
    }
}
#[doc = "Field `CMPIRQ` writer - Compare Interrupt Request"]
pub type CMPIRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL_SPEC, CMPIRQ_A, O>;
impl<'a, const O: u8> CMPIRQ_W<'a, O> {
    #[doc = "No match has occurred (the counter does not match the COMP value)"]
    #[inline(always)]
    pub fn cmpirq_0(self) -> &'a mut W {
        self.variant(CMPIRQ_A::CMPIRQ_0)
    }
    #[doc = "COMP match has occurred (the counter matches the COMP value)"]
    #[inline(always)]
    pub fn cmpirq_1(self) -> &'a mut W {
        self.variant(CMPIRQ_A::CMPIRQ_1)
    }
}
#[doc = "Field `WDE` reader - Watchdog Enable"]
pub type WDE_R = crate::BitReader<WDE_A>;
#[doc = "Watchdog Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDE_A {
    #[doc = "0: Disabled"]
    WDE_0 = 0,
    #[doc = "1: Enabled"]
    WDE_1 = 1,
}
impl From<WDE_A> for bool {
    #[inline(always)]
    fn from(variant: WDE_A) -> Self {
        variant as u8 != 0
    }
}
impl WDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDE_A {
        match self.bits {
            false => WDE_A::WDE_0,
            true => WDE_A::WDE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDE_0`"]
    #[inline(always)]
    pub fn is_wde_0(&self) -> bool {
        *self == WDE_A::WDE_0
    }
    #[doc = "Checks if the value of the field is `WDE_1`"]
    #[inline(always)]
    pub fn is_wde_1(&self) -> bool {
        *self == WDE_A::WDE_1
    }
}
#[doc = "Field `WDE` writer - Watchdog Enable"]
pub type WDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, WDE_A, O>;
impl<'a, const O: u8> WDE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn wde_0(self) -> &'a mut W {
        self.variant(WDE_A::WDE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn wde_1(self) -> &'a mut W {
        self.variant(WDE_A::WDE_1)
    }
}
#[doc = "Field `DIE` reader - Watchdog Timeout Interrupt Enable"]
pub type DIE_R = crate::BitReader<DIE_A>;
#[doc = "Watchdog Timeout Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIE_A {
    #[doc = "0: Disabled"]
    DIE_0 = 0,
    #[doc = "1: Enabled"]
    DIE_1 = 1,
}
impl From<DIE_A> for bool {
    #[inline(always)]
    fn from(variant: DIE_A) -> Self {
        variant as u8 != 0
    }
}
impl DIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIE_A {
        match self.bits {
            false => DIE_A::DIE_0,
            true => DIE_A::DIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIE_0`"]
    #[inline(always)]
    pub fn is_die_0(&self) -> bool {
        *self == DIE_A::DIE_0
    }
    #[doc = "Checks if the value of the field is `DIE_1`"]
    #[inline(always)]
    pub fn is_die_1(&self) -> bool {
        *self == DIE_A::DIE_1
    }
}
#[doc = "Field `DIE` writer - Watchdog Timeout Interrupt Enable"]
pub type DIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, DIE_A, O>;
impl<'a, const O: u8> DIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn die_0(self) -> &'a mut W {
        self.variant(DIE_A::DIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn die_1(self) -> &'a mut W {
        self.variant(DIE_A::DIE_1)
    }
}
#[doc = "Field `DIRQ` reader - Watchdog Timeout Interrupt Request"]
pub type DIRQ_R = crate::BitReader<DIRQ_A>;
#[doc = "Watchdog Timeout Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIRQ_A {
    #[doc = "0: No Watchdog timeout interrupt has occurred"]
    DIRQ_0 = 0,
    #[doc = "1: Watchdog timeout interrupt has occurred"]
    DIRQ_1 = 1,
}
impl From<DIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: DIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl DIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIRQ_A {
        match self.bits {
            false => DIRQ_A::DIRQ_0,
            true => DIRQ_A::DIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIRQ_0`"]
    #[inline(always)]
    pub fn is_dirq_0(&self) -> bool {
        *self == DIRQ_A::DIRQ_0
    }
    #[doc = "Checks if the value of the field is `DIRQ_1`"]
    #[inline(always)]
    pub fn is_dirq_1(&self) -> bool {
        *self == DIRQ_A::DIRQ_1
    }
}
#[doc = "Field `DIRQ` writer - Watchdog Timeout Interrupt Request"]
pub type DIRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL_SPEC, DIRQ_A, O>;
impl<'a, const O: u8> DIRQ_W<'a, O> {
    #[doc = "No Watchdog timeout interrupt has occurred"]
    #[inline(always)]
    pub fn dirq_0(self) -> &'a mut W {
        self.variant(DIRQ_A::DIRQ_0)
    }
    #[doc = "Watchdog timeout interrupt has occurred"]
    #[inline(always)]
    pub fn dirq_1(self) -> &'a mut W {
        self.variant(DIRQ_A::DIRQ_1)
    }
}
#[doc = "Field `XNE` reader - Use Negative Edge of INDEX Pulse"]
pub type XNE_R = crate::BitReader<XNE_A>;
#[doc = "Use Negative Edge of INDEX Pulse\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XNE_A {
    #[doc = "0: Use positive edge of INDEX pulse"]
    XNE_0 = 0,
    #[doc = "1: Use negative edge of INDEX pulse"]
    XNE_1 = 1,
}
impl From<XNE_A> for bool {
    #[inline(always)]
    fn from(variant: XNE_A) -> Self {
        variant as u8 != 0
    }
}
impl XNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XNE_A {
        match self.bits {
            false => XNE_A::XNE_0,
            true => XNE_A::XNE_1,
        }
    }
    #[doc = "Checks if the value of the field is `XNE_0`"]
    #[inline(always)]
    pub fn is_xne_0(&self) -> bool {
        *self == XNE_A::XNE_0
    }
    #[doc = "Checks if the value of the field is `XNE_1`"]
    #[inline(always)]
    pub fn is_xne_1(&self) -> bool {
        *self == XNE_A::XNE_1
    }
}
#[doc = "Field `XNE` writer - Use Negative Edge of INDEX Pulse"]
pub type XNE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, XNE_A, O>;
impl<'a, const O: u8> XNE_W<'a, O> {
    #[doc = "Use positive edge of INDEX pulse"]
    #[inline(always)]
    pub fn xne_0(self) -> &'a mut W {
        self.variant(XNE_A::XNE_0)
    }
    #[doc = "Use negative edge of INDEX pulse"]
    #[inline(always)]
    pub fn xne_1(self) -> &'a mut W {
        self.variant(XNE_A::XNE_1)
    }
}
#[doc = "Field `XIP` reader - INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
pub type XIP_R = crate::BitReader<XIP_A>;
#[doc = "INDEX Triggered Initialization of Position Counters UPOS and LPOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XIP_A {
    #[doc = "0: INDEX pulse does not initialize the position counter"]
    XIP_0 = 0,
    #[doc = "1: INDEX pulse initializes the position counter"]
    XIP_1 = 1,
}
impl From<XIP_A> for bool {
    #[inline(always)]
    fn from(variant: XIP_A) -> Self {
        variant as u8 != 0
    }
}
impl XIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIP_A {
        match self.bits {
            false => XIP_A::XIP_0,
            true => XIP_A::XIP_1,
        }
    }
    #[doc = "Checks if the value of the field is `XIP_0`"]
    #[inline(always)]
    pub fn is_xip_0(&self) -> bool {
        *self == XIP_A::XIP_0
    }
    #[doc = "Checks if the value of the field is `XIP_1`"]
    #[inline(always)]
    pub fn is_xip_1(&self) -> bool {
        *self == XIP_A::XIP_1
    }
}
#[doc = "Field `XIP` writer - INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
pub type XIP_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, XIP_A, O>;
impl<'a, const O: u8> XIP_W<'a, O> {
    #[doc = "INDEX pulse does not initialize the position counter"]
    #[inline(always)]
    pub fn xip_0(self) -> &'a mut W {
        self.variant(XIP_A::XIP_0)
    }
    #[doc = "INDEX pulse initializes the position counter"]
    #[inline(always)]
    pub fn xip_1(self) -> &'a mut W {
        self.variant(XIP_A::XIP_1)
    }
}
#[doc = "Field `XIE` reader - INDEX Pulse Interrupt Enable"]
pub type XIE_R = crate::BitReader<XIE_A>;
#[doc = "INDEX Pulse Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XIE_A {
    #[doc = "0: Disabled"]
    XIE_0 = 0,
    #[doc = "1: Enabled"]
    XIE_1 = 1,
}
impl From<XIE_A> for bool {
    #[inline(always)]
    fn from(variant: XIE_A) -> Self {
        variant as u8 != 0
    }
}
impl XIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIE_A {
        match self.bits {
            false => XIE_A::XIE_0,
            true => XIE_A::XIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `XIE_0`"]
    #[inline(always)]
    pub fn is_xie_0(&self) -> bool {
        *self == XIE_A::XIE_0
    }
    #[doc = "Checks if the value of the field is `XIE_1`"]
    #[inline(always)]
    pub fn is_xie_1(&self) -> bool {
        *self == XIE_A::XIE_1
    }
}
#[doc = "Field `XIE` writer - INDEX Pulse Interrupt Enable"]
pub type XIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, XIE_A, O>;
impl<'a, const O: u8> XIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn xie_0(self) -> &'a mut W {
        self.variant(XIE_A::XIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn xie_1(self) -> &'a mut W {
        self.variant(XIE_A::XIE_1)
    }
}
#[doc = "Field `XIRQ` reader - INDEX Pulse Interrupt Request"]
pub type XIRQ_R = crate::BitReader<XIRQ_A>;
#[doc = "INDEX Pulse Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XIRQ_A {
    #[doc = "0: INDEX pulse has not occurred"]
    XIRQ_0 = 0,
    #[doc = "1: INDEX pulse has occurred"]
    XIRQ_1 = 1,
}
impl From<XIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: XIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl XIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> XIRQ_A {
        match self.bits {
            false => XIRQ_A::XIRQ_0,
            true => XIRQ_A::XIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `XIRQ_0`"]
    #[inline(always)]
    pub fn is_xirq_0(&self) -> bool {
        *self == XIRQ_A::XIRQ_0
    }
    #[doc = "Checks if the value of the field is `XIRQ_1`"]
    #[inline(always)]
    pub fn is_xirq_1(&self) -> bool {
        *self == XIRQ_A::XIRQ_1
    }
}
#[doc = "Field `XIRQ` writer - INDEX Pulse Interrupt Request"]
pub type XIRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL_SPEC, XIRQ_A, O>;
impl<'a, const O: u8> XIRQ_W<'a, O> {
    #[doc = "INDEX pulse has not occurred"]
    #[inline(always)]
    pub fn xirq_0(self) -> &'a mut W {
        self.variant(XIRQ_A::XIRQ_0)
    }
    #[doc = "INDEX pulse has occurred"]
    #[inline(always)]
    pub fn xirq_1(self) -> &'a mut W {
        self.variant(XIRQ_A::XIRQ_1)
    }
}
#[doc = "Field `PH1` reader - Enable Signal Phase Count Mode"]
pub type PH1_R = crate::BitReader<PH1_A>;
#[doc = "Enable Signal Phase Count Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PH1_A {
    #[doc = "0: Use the standard quadrature decoder, where PHASEA and PHASEB represent a two-phase quadrature signal."]
    PH1_0 = 0,
    #[doc = "1: Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction: If CTRL\\[REV\\]
= 0, PHASEB = 0, then count up If CTRL\\[REV\\]
= 1, PHASEB = 1, then count up If CTRL\\[REV\\]
= 0, PHASEB = 1, then count down If CTRL\\[REV\\]
= 1, PHASEB = 0, then count down"]
    PH1_1 = 1,
}
impl From<PH1_A> for bool {
    #[inline(always)]
    fn from(variant: PH1_A) -> Self {
        variant as u8 != 0
    }
}
impl PH1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PH1_A {
        match self.bits {
            false => PH1_A::PH1_0,
            true => PH1_A::PH1_1,
        }
    }
    #[doc = "Checks if the value of the field is `PH1_0`"]
    #[inline(always)]
    pub fn is_ph1_0(&self) -> bool {
        *self == PH1_A::PH1_0
    }
    #[doc = "Checks if the value of the field is `PH1_1`"]
    #[inline(always)]
    pub fn is_ph1_1(&self) -> bool {
        *self == PH1_A::PH1_1
    }
}
#[doc = "Field `PH1` writer - Enable Signal Phase Count Mode"]
pub type PH1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, PH1_A, O>;
impl<'a, const O: u8> PH1_W<'a, O> {
    #[doc = "Use the standard quadrature decoder, where PHASEA and PHASEB represent a two-phase quadrature signal."]
    #[inline(always)]
    pub fn ph1_0(self) -> &'a mut W {
        self.variant(PH1_A::PH1_0)
    }
    #[doc = "Bypass the quadrature decoder. A positive transition of the PHASEA input generates a count signal. The PHASEB input and the REV bit control the counter direction: If CTRL\\[REV\\]
= 0, PHASEB = 0, then count up If CTRL\\[REV\\]
= 1, PHASEB = 1, then count up If CTRL\\[REV\\]
= 0, PHASEB = 1, then count down If CTRL\\[REV\\]
= 1, PHASEB = 0, then count down"]
    #[inline(always)]
    pub fn ph1_1(self) -> &'a mut W {
        self.variant(PH1_A::PH1_1)
    }
}
#[doc = "Field `REV` reader - Enable Reverse Direction Counting"]
pub type REV_R = crate::BitReader<REV_A>;
#[doc = "Enable Reverse Direction Counting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV_A {
    #[doc = "0: Count normally"]
    REV_0 = 0,
    #[doc = "1: Count in the reverse direction"]
    REV_1 = 1,
}
impl From<REV_A> for bool {
    #[inline(always)]
    fn from(variant: REV_A) -> Self {
        variant as u8 != 0
    }
}
impl REV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV_A {
        match self.bits {
            false => REV_A::REV_0,
            true => REV_A::REV_1,
        }
    }
    #[doc = "Checks if the value of the field is `REV_0`"]
    #[inline(always)]
    pub fn is_rev_0(&self) -> bool {
        *self == REV_A::REV_0
    }
    #[doc = "Checks if the value of the field is `REV_1`"]
    #[inline(always)]
    pub fn is_rev_1(&self) -> bool {
        *self == REV_A::REV_1
    }
}
#[doc = "Field `REV` writer - Enable Reverse Direction Counting"]
pub type REV_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, REV_A, O>;
impl<'a, const O: u8> REV_W<'a, O> {
    #[doc = "Count normally"]
    #[inline(always)]
    pub fn rev_0(self) -> &'a mut W {
        self.variant(REV_A::REV_0)
    }
    #[doc = "Count in the reverse direction"]
    #[inline(always)]
    pub fn rev_1(self) -> &'a mut W {
        self.variant(REV_A::REV_1)
    }
}
#[doc = "Field `SWIP` reader - Software-Triggered Initialization of Position Counters UPOS and LPOS"]
pub type SWIP_R = crate::BitReader<SWIP_A>;
#[doc = "Software-Triggered Initialization of Position Counters UPOS and LPOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWIP_A {
    #[doc = "0: No action"]
    SWIP_0 = 0,
    #[doc = "1: Initialize position counter (using upper and lower initialization registers, UINIT and LINIT)"]
    SWIP_1 = 1,
}
impl From<SWIP_A> for bool {
    #[inline(always)]
    fn from(variant: SWIP_A) -> Self {
        variant as u8 != 0
    }
}
impl SWIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWIP_A {
        match self.bits {
            false => SWIP_A::SWIP_0,
            true => SWIP_A::SWIP_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWIP_0`"]
    #[inline(always)]
    pub fn is_swip_0(&self) -> bool {
        *self == SWIP_A::SWIP_0
    }
    #[doc = "Checks if the value of the field is `SWIP_1`"]
    #[inline(always)]
    pub fn is_swip_1(&self) -> bool {
        *self == SWIP_A::SWIP_1
    }
}
#[doc = "Field `SWIP` writer - Software-Triggered Initialization of Position Counters UPOS and LPOS"]
pub type SWIP_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, SWIP_A, O>;
impl<'a, const O: u8> SWIP_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn swip_0(self) -> &'a mut W {
        self.variant(SWIP_A::SWIP_0)
    }
    #[doc = "Initialize position counter (using upper and lower initialization registers, UINIT and LINIT)"]
    #[inline(always)]
    pub fn swip_1(self) -> &'a mut W {
        self.variant(SWIP_A::SWIP_1)
    }
}
#[doc = "Field `HNE` reader - Use Negative Edge of HOME Input"]
pub type HNE_R = crate::BitReader<HNE_A>;
#[doc = "Use Negative Edge of HOME Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HNE_A {
    #[doc = "0: Use positive-going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_0 = 0,
    #[doc = "1: Use negative-going edge-to-trigger initialization of position counters UPOS and LPOS"]
    HNE_1 = 1,
}
impl From<HNE_A> for bool {
    #[inline(always)]
    fn from(variant: HNE_A) -> Self {
        variant as u8 != 0
    }
}
impl HNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HNE_A {
        match self.bits {
            false => HNE_A::HNE_0,
            true => HNE_A::HNE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HNE_0`"]
    #[inline(always)]
    pub fn is_hne_0(&self) -> bool {
        *self == HNE_A::HNE_0
    }
    #[doc = "Checks if the value of the field is `HNE_1`"]
    #[inline(always)]
    pub fn is_hne_1(&self) -> bool {
        *self == HNE_A::HNE_1
    }
}
#[doc = "Field `HNE` writer - Use Negative Edge of HOME Input"]
pub type HNE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, HNE_A, O>;
impl<'a, const O: u8> HNE_W<'a, O> {
    #[doc = "Use positive-going edge-to-trigger initialization of position counters UPOS and LPOS"]
    #[inline(always)]
    pub fn hne_0(self) -> &'a mut W {
        self.variant(HNE_A::HNE_0)
    }
    #[doc = "Use negative-going edge-to-trigger initialization of position counters UPOS and LPOS"]
    #[inline(always)]
    pub fn hne_1(self) -> &'a mut W {
        self.variant(HNE_A::HNE_1)
    }
}
#[doc = "Field `HIP` reader - Enable HOME to Initialize Position Counters UPOS and LPOS"]
pub type HIP_R = crate::BitReader<HIP_A>;
#[doc = "Enable HOME to Initialize Position Counters UPOS and LPOS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIP_A {
    #[doc = "0: No action"]
    HIP_0 = 0,
    #[doc = "1: HOME signal initializes the position counter"]
    HIP_1 = 1,
}
impl From<HIP_A> for bool {
    #[inline(always)]
    fn from(variant: HIP_A) -> Self {
        variant as u8 != 0
    }
}
impl HIP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIP_A {
        match self.bits {
            false => HIP_A::HIP_0,
            true => HIP_A::HIP_1,
        }
    }
    #[doc = "Checks if the value of the field is `HIP_0`"]
    #[inline(always)]
    pub fn is_hip_0(&self) -> bool {
        *self == HIP_A::HIP_0
    }
    #[doc = "Checks if the value of the field is `HIP_1`"]
    #[inline(always)]
    pub fn is_hip_1(&self) -> bool {
        *self == HIP_A::HIP_1
    }
}
#[doc = "Field `HIP` writer - Enable HOME to Initialize Position Counters UPOS and LPOS"]
pub type HIP_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, HIP_A, O>;
impl<'a, const O: u8> HIP_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn hip_0(self) -> &'a mut W {
        self.variant(HIP_A::HIP_0)
    }
    #[doc = "HOME signal initializes the position counter"]
    #[inline(always)]
    pub fn hip_1(self) -> &'a mut W {
        self.variant(HIP_A::HIP_1)
    }
}
#[doc = "Field `HIE` reader - HOME Interrupt Enable"]
pub type HIE_R = crate::BitReader<HIE_A>;
#[doc = "HOME Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIE_A {
    #[doc = "0: Disabled"]
    HIE_0 = 0,
    #[doc = "1: Enabled"]
    HIE_1 = 1,
}
impl From<HIE_A> for bool {
    #[inline(always)]
    fn from(variant: HIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIE_A {
        match self.bits {
            false => HIE_A::HIE_0,
            true => HIE_A::HIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `HIE_0`"]
    #[inline(always)]
    pub fn is_hie_0(&self) -> bool {
        *self == HIE_A::HIE_0
    }
    #[doc = "Checks if the value of the field is `HIE_1`"]
    #[inline(always)]
    pub fn is_hie_1(&self) -> bool {
        *self == HIE_A::HIE_1
    }
}
#[doc = "Field `HIE` writer - HOME Interrupt Enable"]
pub type HIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL_SPEC, HIE_A, O>;
impl<'a, const O: u8> HIE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn hie_0(self) -> &'a mut W {
        self.variant(HIE_A::HIE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn hie_1(self) -> &'a mut W {
        self.variant(HIE_A::HIE_1)
    }
}
#[doc = "Field `HIRQ` reader - HOME Signal Transition Interrupt Request"]
pub type HIRQ_R = crate::BitReader<HIRQ_A>;
#[doc = "HOME Signal Transition Interrupt Request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HIRQ_A {
    #[doc = "0: No transition on the HOME signal has occurred"]
    HIRQ_0 = 0,
    #[doc = "1: A transition on the HOME signal has occurred"]
    HIRQ_1 = 1,
}
impl From<HIRQ_A> for bool {
    #[inline(always)]
    fn from(variant: HIRQ_A) -> Self {
        variant as u8 != 0
    }
}
impl HIRQ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HIRQ_A {
        match self.bits {
            false => HIRQ_A::HIRQ_0,
            true => HIRQ_A::HIRQ_1,
        }
    }
    #[doc = "Checks if the value of the field is `HIRQ_0`"]
    #[inline(always)]
    pub fn is_hirq_0(&self) -> bool {
        *self == HIRQ_A::HIRQ_0
    }
    #[doc = "Checks if the value of the field is `HIRQ_1`"]
    #[inline(always)]
    pub fn is_hirq_1(&self) -> bool {
        *self == HIRQ_A::HIRQ_1
    }
}
#[doc = "Field `HIRQ` writer - HOME Signal Transition Interrupt Request"]
pub type HIRQ_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL_SPEC, HIRQ_A, O>;
impl<'a, const O: u8> HIRQ_W<'a, O> {
    #[doc = "No transition on the HOME signal has occurred"]
    #[inline(always)]
    pub fn hirq_0(self) -> &'a mut W {
        self.variant(HIRQ_A::HIRQ_0)
    }
    #[doc = "A transition on the HOME signal has occurred"]
    #[inline(always)]
    pub fn hirq_1(self) -> &'a mut W {
        self.variant(HIRQ_A::HIRQ_1)
    }
}
impl R {
    #[doc = "Bit 0 - Compare Interrupt Enable"]
    #[inline(always)]
    pub fn cmpie(&self) -> CMPIE_R {
        CMPIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Compare Interrupt Request"]
    #[inline(always)]
    pub fn cmpirq(&self) -> CMPIRQ_R {
        CMPIRQ_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Watchdog Enable"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    pub fn die(&self) -> DIE_R {
        DIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Watchdog Timeout Interrupt Request"]
    #[inline(always)]
    pub fn dirq(&self) -> DIRQ_R {
        DIRQ_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Use Negative Edge of INDEX Pulse"]
    #[inline(always)]
    pub fn xne(&self) -> XNE_R {
        XNE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub fn xip(&self) -> XIP_R {
        XIP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - INDEX Pulse Interrupt Enable"]
    #[inline(always)]
    pub fn xie(&self) -> XIE_R {
        XIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - INDEX Pulse Interrupt Request"]
    #[inline(always)]
    pub fn xirq(&self) -> XIRQ_R {
        XIRQ_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Signal Phase Count Mode"]
    #[inline(always)]
    pub fn ph1(&self) -> PH1_R {
        PH1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable Reverse Direction Counting"]
    #[inline(always)]
    pub fn rev(&self) -> REV_R {
        REV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Software-Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub fn swip(&self) -> SWIP_R {
        SWIP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Use Negative Edge of HOME Input"]
    #[inline(always)]
    pub fn hne(&self) -> HNE_R {
        HNE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable HOME to Initialize Position Counters UPOS and LPOS"]
    #[inline(always)]
    pub fn hip(&self) -> HIP_R {
        HIP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - HOME Interrupt Enable"]
    #[inline(always)]
    pub fn hie(&self) -> HIE_R {
        HIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - HOME Signal Transition Interrupt Request"]
    #[inline(always)]
    pub fn hirq(&self) -> HIRQ_R {
        HIRQ_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Compare Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmpie(&mut self) -> CMPIE_W<0> {
        CMPIE_W::new(self)
    }
    #[doc = "Bit 1 - Compare Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn cmpirq(&mut self) -> CMPIRQ_W<1> {
        CMPIRQ_W::new(self)
    }
    #[doc = "Bit 2 - Watchdog Enable"]
    #[inline(always)]
    #[must_use]
    pub fn wde(&mut self) -> WDE_W<2> {
        WDE_W::new(self)
    }
    #[doc = "Bit 3 - Watchdog Timeout Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn die(&mut self) -> DIE_W<3> {
        DIE_W::new(self)
    }
    #[doc = "Bit 4 - Watchdog Timeout Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn dirq(&mut self) -> DIRQ_W<4> {
        DIRQ_W::new(self)
    }
    #[doc = "Bit 5 - Use Negative Edge of INDEX Pulse"]
    #[inline(always)]
    #[must_use]
    pub fn xne(&mut self) -> XNE_W<5> {
        XNE_W::new(self)
    }
    #[doc = "Bit 6 - INDEX Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    #[must_use]
    pub fn xip(&mut self) -> XIP_W<6> {
        XIP_W::new(self)
    }
    #[doc = "Bit 7 - INDEX Pulse Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn xie(&mut self) -> XIE_W<7> {
        XIE_W::new(self)
    }
    #[doc = "Bit 8 - INDEX Pulse Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn xirq(&mut self) -> XIRQ_W<8> {
        XIRQ_W::new(self)
    }
    #[doc = "Bit 9 - Enable Signal Phase Count Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ph1(&mut self) -> PH1_W<9> {
        PH1_W::new(self)
    }
    #[doc = "Bit 10 - Enable Reverse Direction Counting"]
    #[inline(always)]
    #[must_use]
    pub fn rev(&mut self) -> REV_W<10> {
        REV_W::new(self)
    }
    #[doc = "Bit 11 - Software-Triggered Initialization of Position Counters UPOS and LPOS"]
    #[inline(always)]
    #[must_use]
    pub fn swip(&mut self) -> SWIP_W<11> {
        SWIP_W::new(self)
    }
    #[doc = "Bit 12 - Use Negative Edge of HOME Input"]
    #[inline(always)]
    #[must_use]
    pub fn hne(&mut self) -> HNE_W<12> {
        HNE_W::new(self)
    }
    #[doc = "Bit 13 - Enable HOME to Initialize Position Counters UPOS and LPOS"]
    #[inline(always)]
    #[must_use]
    pub fn hip(&mut self) -> HIP_W<13> {
        HIP_W::new(self)
    }
    #[doc = "Bit 14 - HOME Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn hie(&mut self) -> HIE_W<14> {
        HIE_W::new(self)
    }
    #[doc = "Bit 15 - HOME Signal Transition Interrupt Request"]
    #[inline(always)]
    #[must_use]
    pub fn hirq(&mut self) -> HIRQ_W<15> {
        HIRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x8112;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
