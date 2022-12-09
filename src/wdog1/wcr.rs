#[doc = "Register `WCR` reader"]
pub struct R(crate::R<WCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WCR` writer"]
pub struct W(crate::W<WCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WCR_SPEC>;
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
impl From<crate::W<WCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDZST` reader - WDZST"]
pub type WDZST_R = crate::BitReader<WDZST_A>;
#[doc = "WDZST\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDZST_A {
    #[doc = "0: Continue timer operation (Default)."]
    WDZST_0 = 0,
    #[doc = "1: Suspend the watchdog timer."]
    WDZST_1 = 1,
}
impl From<WDZST_A> for bool {
    #[inline(always)]
    fn from(variant: WDZST_A) -> Self {
        variant as u8 != 0
    }
}
impl WDZST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDZST_A {
        match self.bits {
            false => WDZST_A::WDZST_0,
            true => WDZST_A::WDZST_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDZST_0`"]
    #[inline(always)]
    pub fn is_wdzst_0(&self) -> bool {
        *self == WDZST_A::WDZST_0
    }
    #[doc = "Checks if the value of the field is `WDZST_1`"]
    #[inline(always)]
    pub fn is_wdzst_1(&self) -> bool {
        *self == WDZST_A::WDZST_1
    }
}
#[doc = "Field `WDZST` writer - WDZST"]
pub type WDZST_W<'a, const O: u8> = crate::BitWriter<'a, u16, WCR_SPEC, WDZST_A, O>;
impl<'a, const O: u8> WDZST_W<'a, O> {
    #[doc = "Continue timer operation (Default)."]
    #[inline(always)]
    pub fn wdzst_0(self) -> &'a mut W {
        self.variant(WDZST_A::WDZST_0)
    }
    #[doc = "Suspend the watchdog timer."]
    #[inline(always)]
    pub fn wdzst_1(self) -> &'a mut W {
        self.variant(WDZST_A::WDZST_1)
    }
}
#[doc = "Field `WDBG` reader - WDBG"]
pub type WDBG_R = crate::BitReader<WDBG_A>;
#[doc = "WDBG\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDBG_A {
    #[doc = "0: Continue WDOG timer operation (Default)."]
    WDBG_0 = 0,
    #[doc = "1: Suspend the watchdog timer."]
    WDBG_1 = 1,
}
impl From<WDBG_A> for bool {
    #[inline(always)]
    fn from(variant: WDBG_A) -> Self {
        variant as u8 != 0
    }
}
impl WDBG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDBG_A {
        match self.bits {
            false => WDBG_A::WDBG_0,
            true => WDBG_A::WDBG_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDBG_0`"]
    #[inline(always)]
    pub fn is_wdbg_0(&self) -> bool {
        *self == WDBG_A::WDBG_0
    }
    #[doc = "Checks if the value of the field is `WDBG_1`"]
    #[inline(always)]
    pub fn is_wdbg_1(&self) -> bool {
        *self == WDBG_A::WDBG_1
    }
}
#[doc = "Field `WDBG` writer - WDBG"]
pub type WDBG_W<'a, const O: u8> = crate::BitWriter<'a, u16, WCR_SPEC, WDBG_A, O>;
impl<'a, const O: u8> WDBG_W<'a, O> {
    #[doc = "Continue WDOG timer operation (Default)."]
    #[inline(always)]
    pub fn wdbg_0(self) -> &'a mut W {
        self.variant(WDBG_A::WDBG_0)
    }
    #[doc = "Suspend the watchdog timer."]
    #[inline(always)]
    pub fn wdbg_1(self) -> &'a mut W {
        self.variant(WDBG_A::WDBG_1)
    }
}
#[doc = "Field `WDE` reader - WDE"]
pub type WDE_R = crate::BitReader<WDE_A>;
#[doc = "WDE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDE_A {
    #[doc = "0: Disable the Watchdog (Default)."]
    WDE_0 = 0,
    #[doc = "1: Enable the Watchdog."]
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
#[doc = "Field `WDE` writer - WDE"]
pub type WDE_W<'a, const O: u8> = crate::BitWriter<'a, u16, WCR_SPEC, WDE_A, O>;
impl<'a, const O: u8> WDE_W<'a, O> {
    #[doc = "Disable the Watchdog (Default)."]
    #[inline(always)]
    pub fn wde_0(self) -> &'a mut W {
        self.variant(WDE_A::WDE_0)
    }
    #[doc = "Enable the Watchdog."]
    #[inline(always)]
    pub fn wde_1(self) -> &'a mut W {
        self.variant(WDE_A::WDE_1)
    }
}
#[doc = "Field `WDT` reader - WDT"]
pub type WDT_R = crate::BitReader<WDT_A>;
#[doc = "WDT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDT_A {
    #[doc = "0: No effect on WDOG_B (Default)."]
    WDT_0 = 0,
    #[doc = "1: Assert WDOG_B upon a Watchdog Time-out event."]
    WDT_1 = 1,
}
impl From<WDT_A> for bool {
    #[inline(always)]
    fn from(variant: WDT_A) -> Self {
        variant as u8 != 0
    }
}
impl WDT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDT_A {
        match self.bits {
            false => WDT_A::WDT_0,
            true => WDT_A::WDT_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDT_0`"]
    #[inline(always)]
    pub fn is_wdt_0(&self) -> bool {
        *self == WDT_A::WDT_0
    }
    #[doc = "Checks if the value of the field is `WDT_1`"]
    #[inline(always)]
    pub fn is_wdt_1(&self) -> bool {
        *self == WDT_A::WDT_1
    }
}
#[doc = "Field `WDT` writer - WDT"]
pub type WDT_W<'a, const O: u8> = crate::BitWriter<'a, u16, WCR_SPEC, WDT_A, O>;
impl<'a, const O: u8> WDT_W<'a, O> {
    #[doc = "No effect on WDOG_B (Default)."]
    #[inline(always)]
    pub fn wdt_0(self) -> &'a mut W {
        self.variant(WDT_A::WDT_0)
    }
    #[doc = "Assert WDOG_B upon a Watchdog Time-out event."]
    #[inline(always)]
    pub fn wdt_1(self) -> &'a mut W {
        self.variant(WDT_A::WDT_1)
    }
}
#[doc = "Field `SRS` reader - SRS"]
pub type SRS_R = crate::BitReader<SRS_A>;
#[doc = "SRS\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRS_A {
    #[doc = "0: Assert system reset signal."]
    SRS_0 = 0,
    #[doc = "1: No effect on the system (Default)."]
    SRS_1 = 1,
}
impl From<SRS_A> for bool {
    #[inline(always)]
    fn from(variant: SRS_A) -> Self {
        variant as u8 != 0
    }
}
impl SRS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRS_A {
        match self.bits {
            false => SRS_A::SRS_0,
            true => SRS_A::SRS_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRS_0`"]
    #[inline(always)]
    pub fn is_srs_0(&self) -> bool {
        *self == SRS_A::SRS_0
    }
    #[doc = "Checks if the value of the field is `SRS_1`"]
    #[inline(always)]
    pub fn is_srs_1(&self) -> bool {
        *self == SRS_A::SRS_1
    }
}
#[doc = "Field `SRS` writer - SRS"]
pub type SRS_W<'a, const O: u8> = crate::BitWriter<'a, u16, WCR_SPEC, SRS_A, O>;
impl<'a, const O: u8> SRS_W<'a, O> {
    #[doc = "Assert system reset signal."]
    #[inline(always)]
    pub fn srs_0(self) -> &'a mut W {
        self.variant(SRS_A::SRS_0)
    }
    #[doc = "No effect on the system (Default)."]
    #[inline(always)]
    pub fn srs_1(self) -> &'a mut W {
        self.variant(SRS_A::SRS_1)
    }
}
#[doc = "Field `WDA` reader - WDA"]
pub type WDA_R = crate::BitReader<WDA_A>;
#[doc = "WDA\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDA_A {
    #[doc = "0: Assert WDOG_B output."]
    WDA_0 = 0,
    #[doc = "1: No effect on system (Default)."]
    WDA_1 = 1,
}
impl From<WDA_A> for bool {
    #[inline(always)]
    fn from(variant: WDA_A) -> Self {
        variant as u8 != 0
    }
}
impl WDA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDA_A {
        match self.bits {
            false => WDA_A::WDA_0,
            true => WDA_A::WDA_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDA_0`"]
    #[inline(always)]
    pub fn is_wda_0(&self) -> bool {
        *self == WDA_A::WDA_0
    }
    #[doc = "Checks if the value of the field is `WDA_1`"]
    #[inline(always)]
    pub fn is_wda_1(&self) -> bool {
        *self == WDA_A::WDA_1
    }
}
#[doc = "Field `WDA` writer - WDA"]
pub type WDA_W<'a, const O: u8> = crate::BitWriter<'a, u16, WCR_SPEC, WDA_A, O>;
impl<'a, const O: u8> WDA_W<'a, O> {
    #[doc = "Assert WDOG_B output."]
    #[inline(always)]
    pub fn wda_0(self) -> &'a mut W {
        self.variant(WDA_A::WDA_0)
    }
    #[doc = "No effect on system (Default)."]
    #[inline(always)]
    pub fn wda_1(self) -> &'a mut W {
        self.variant(WDA_A::WDA_1)
    }
}
#[doc = "Field `SRE` reader - software reset extension, an option way to generate software reset"]
pub type SRE_R = crate::BitReader<SRE_A>;
#[doc = "software reset extension, an option way to generate software reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SRE_A {
    #[doc = "0: using original way to generate software reset (default)"]
    SRE_0 = 0,
    #[doc = "1: using new way to generate software reset."]
    SRE_1 = 1,
}
impl From<SRE_A> for bool {
    #[inline(always)]
    fn from(variant: SRE_A) -> Self {
        variant as u8 != 0
    }
}
impl SRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SRE_A {
        match self.bits {
            false => SRE_A::SRE_0,
            true => SRE_A::SRE_1,
        }
    }
    #[doc = "Checks if the value of the field is `SRE_0`"]
    #[inline(always)]
    pub fn is_sre_0(&self) -> bool {
        *self == SRE_A::SRE_0
    }
    #[doc = "Checks if the value of the field is `SRE_1`"]
    #[inline(always)]
    pub fn is_sre_1(&self) -> bool {
        *self == SRE_A::SRE_1
    }
}
#[doc = "Field `SRE` writer - software reset extension, an option way to generate software reset"]
pub type SRE_W<'a, const O: u8> = crate::BitWriter<'a, u16, WCR_SPEC, SRE_A, O>;
impl<'a, const O: u8> SRE_W<'a, O> {
    #[doc = "using original way to generate software reset (default)"]
    #[inline(always)]
    pub fn sre_0(self) -> &'a mut W {
        self.variant(SRE_A::SRE_0)
    }
    #[doc = "using new way to generate software reset."]
    #[inline(always)]
    pub fn sre_1(self) -> &'a mut W {
        self.variant(SRE_A::SRE_1)
    }
}
#[doc = "Field `WDW` reader - WDW"]
pub type WDW_R = crate::BitReader<WDW_A>;
#[doc = "WDW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDW_A {
    #[doc = "0: Continue WDOG timer operation (Default)."]
    WDW_0 = 0,
    #[doc = "1: Suspend WDOG timer operation."]
    WDW_1 = 1,
}
impl From<WDW_A> for bool {
    #[inline(always)]
    fn from(variant: WDW_A) -> Self {
        variant as u8 != 0
    }
}
impl WDW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDW_A {
        match self.bits {
            false => WDW_A::WDW_0,
            true => WDW_A::WDW_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDW_0`"]
    #[inline(always)]
    pub fn is_wdw_0(&self) -> bool {
        *self == WDW_A::WDW_0
    }
    #[doc = "Checks if the value of the field is `WDW_1`"]
    #[inline(always)]
    pub fn is_wdw_1(&self) -> bool {
        *self == WDW_A::WDW_1
    }
}
#[doc = "Field `WDW` writer - WDW"]
pub type WDW_W<'a, const O: u8> = crate::BitWriter<'a, u16, WCR_SPEC, WDW_A, O>;
impl<'a, const O: u8> WDW_W<'a, O> {
    #[doc = "Continue WDOG timer operation (Default)."]
    #[inline(always)]
    pub fn wdw_0(self) -> &'a mut W {
        self.variant(WDW_A::WDW_0)
    }
    #[doc = "Suspend WDOG timer operation."]
    #[inline(always)]
    pub fn wdw_1(self) -> &'a mut W {
        self.variant(WDW_A::WDW_1)
    }
}
#[doc = "Field `WT` reader - WT"]
pub type WT_R = crate::FieldReader<u8, WT_A>;
#[doc = "WT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WT_A {
    #[doc = "0: - 0.5 Seconds (Default)."]
    WT_0 = 0,
    #[doc = "1: - 1.0 Seconds."]
    WT_1 = 1,
    #[doc = "2: - 1.5 Seconds."]
    WT_2 = 2,
    #[doc = "3: - 2.0 Seconds."]
    WT_3 = 3,
    #[doc = "255: - 128 Seconds."]
    WT_255 = 255,
}
impl From<WT_A> for u8 {
    #[inline(always)]
    fn from(variant: WT_A) -> Self {
        variant as _
    }
}
impl WT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WT_A> {
        match self.bits {
            0 => Some(WT_A::WT_0),
            1 => Some(WT_A::WT_1),
            2 => Some(WT_A::WT_2),
            3 => Some(WT_A::WT_3),
            255 => Some(WT_A::WT_255),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WT_0`"]
    #[inline(always)]
    pub fn is_wt_0(&self) -> bool {
        *self == WT_A::WT_0
    }
    #[doc = "Checks if the value of the field is `WT_1`"]
    #[inline(always)]
    pub fn is_wt_1(&self) -> bool {
        *self == WT_A::WT_1
    }
    #[doc = "Checks if the value of the field is `WT_2`"]
    #[inline(always)]
    pub fn is_wt_2(&self) -> bool {
        *self == WT_A::WT_2
    }
    #[doc = "Checks if the value of the field is `WT_3`"]
    #[inline(always)]
    pub fn is_wt_3(&self) -> bool {
        *self == WT_A::WT_3
    }
    #[doc = "Checks if the value of the field is `WT_255`"]
    #[inline(always)]
    pub fn is_wt_255(&self) -> bool {
        *self == WT_A::WT_255
    }
}
#[doc = "Field `WT` writer - WT"]
pub type WT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WCR_SPEC, u8, WT_A, 8, O>;
impl<'a, const O: u8> WT_W<'a, O> {
    #[doc = "- 0.5 Seconds (Default)."]
    #[inline(always)]
    pub fn wt_0(self) -> &'a mut W {
        self.variant(WT_A::WT_0)
    }
    #[doc = "- 1.0 Seconds."]
    #[inline(always)]
    pub fn wt_1(self) -> &'a mut W {
        self.variant(WT_A::WT_1)
    }
    #[doc = "- 1.5 Seconds."]
    #[inline(always)]
    pub fn wt_2(self) -> &'a mut W {
        self.variant(WT_A::WT_2)
    }
    #[doc = "- 2.0 Seconds."]
    #[inline(always)]
    pub fn wt_3(self) -> &'a mut W {
        self.variant(WT_A::WT_3)
    }
    #[doc = "- 128 Seconds."]
    #[inline(always)]
    pub fn wt_255(self) -> &'a mut W {
        self.variant(WT_A::WT_255)
    }
}
impl R {
    #[doc = "Bit 0 - WDZST"]
    #[inline(always)]
    pub fn wdzst(&self) -> WDZST_R {
        WDZST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - WDBG"]
    #[inline(always)]
    pub fn wdbg(&self) -> WDBG_R {
        WDBG_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - WDE"]
    #[inline(always)]
    pub fn wde(&self) -> WDE_R {
        WDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - WDT"]
    #[inline(always)]
    pub fn wdt(&self) -> WDT_R {
        WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - SRS"]
    #[inline(always)]
    pub fn srs(&self) -> SRS_R {
        SRS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - WDA"]
    #[inline(always)]
    pub fn wda(&self) -> WDA_R {
        WDA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - software reset extension, an option way to generate software reset"]
    #[inline(always)]
    pub fn sre(&self) -> SRE_R {
        SRE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WDW"]
    #[inline(always)]
    pub fn wdw(&self) -> WDW_R {
        WDW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - WT"]
    #[inline(always)]
    pub fn wt(&self) -> WT_R {
        WT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - WDZST"]
    #[inline(always)]
    #[must_use]
    pub fn wdzst(&mut self) -> WDZST_W<0> {
        WDZST_W::new(self)
    }
    #[doc = "Bit 1 - WDBG"]
    #[inline(always)]
    #[must_use]
    pub fn wdbg(&mut self) -> WDBG_W<1> {
        WDBG_W::new(self)
    }
    #[doc = "Bit 2 - WDE"]
    #[inline(always)]
    #[must_use]
    pub fn wde(&mut self) -> WDE_W<2> {
        WDE_W::new(self)
    }
    #[doc = "Bit 3 - WDT"]
    #[inline(always)]
    #[must_use]
    pub fn wdt(&mut self) -> WDT_W<3> {
        WDT_W::new(self)
    }
    #[doc = "Bit 4 - SRS"]
    #[inline(always)]
    #[must_use]
    pub fn srs(&mut self) -> SRS_W<4> {
        SRS_W::new(self)
    }
    #[doc = "Bit 5 - WDA"]
    #[inline(always)]
    #[must_use]
    pub fn wda(&mut self) -> WDA_W<5> {
        WDA_W::new(self)
    }
    #[doc = "Bit 6 - software reset extension, an option way to generate software reset"]
    #[inline(always)]
    #[must_use]
    pub fn sre(&mut self) -> SRE_W<6> {
        SRE_W::new(self)
    }
    #[doc = "Bit 7 - WDW"]
    #[inline(always)]
    #[must_use]
    pub fn wdw(&mut self) -> WDW_W<7> {
        WDW_W::new(self)
    }
    #[doc = "Bits 8:15 - WT"]
    #[inline(always)]
    #[must_use]
    pub fn wt(&mut self) -> WT_W<8> {
        WT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wcr](index.html) module"]
pub struct WCR_SPEC;
impl crate::RegisterSpec for WCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wcr::R](R) reader structure"]
impl crate::Readable for WCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wcr::W](W) writer structure"]
impl crate::Writable for WCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WCR to value 0x30"]
impl crate::Resettable for WCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x30;
}
