#[doc = "Register `USBSTS` reader"]
pub struct R(crate::R<USBSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBSTS` writer"]
pub struct W(crate::W<USBSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBSTS_SPEC>;
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
impl From<crate::W<USBSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UI` reader - USB Interrupt (USBINT) - R/WC"]
pub type UI_R = crate::BitReader<bool>;
#[doc = "Field `UI` writer - USB Interrupt (USBINT) - R/WC"]
pub type UI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `UEI` reader - USB Error Interrupt (USBERRINT) - R/WC"]
pub type UEI_R = crate::BitReader<bool>;
#[doc = "Field `UEI` writer - USB Error Interrupt (USBERRINT) - R/WC"]
pub type UEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `PCI` reader - Port Change Detect - R/WC"]
pub type PCI_R = crate::BitReader<bool>;
#[doc = "Field `PCI` writer - Port Change Detect - R/WC"]
pub type PCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `FRI` reader - Frame List Rollover - R/WC"]
pub type FRI_R = crate::BitReader<bool>;
#[doc = "Field `FRI` writer - Frame List Rollover - R/WC"]
pub type FRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `SEI` reader - System Error- R/WC"]
pub type SEI_R = crate::BitReader<bool>;
#[doc = "Field `SEI` writer - System Error- R/WC"]
pub type SEI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `AAI` reader - Interrupt on Async Advance - R/WC"]
pub type AAI_R = crate::BitReader<bool>;
#[doc = "Field `AAI` writer - Interrupt on Async Advance - R/WC"]
pub type AAI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `URI` reader - USB Reset Received - R/WC"]
pub type URI_R = crate::BitReader<bool>;
#[doc = "Field `URI` writer - USB Reset Received - R/WC"]
pub type URI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `SRI` reader - SOF Received - R/WC"]
pub type SRI_R = crate::BitReader<bool>;
#[doc = "Field `SRI` writer - SOF Received - R/WC"]
pub type SRI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `SLI` reader - DCSuspend - R/WC"]
pub type SLI_R = crate::BitReader<bool>;
#[doc = "Field `SLI` writer - DCSuspend - R/WC"]
pub type SLI_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `ULPII` reader - ULPI Interrupt - R/WC"]
pub type ULPII_R = crate::BitReader<bool>;
#[doc = "Field `ULPII` writer - ULPI Interrupt - R/WC"]
pub type ULPII_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `HCH` reader - HCHaIted - Read Only"]
pub type HCH_R = crate::BitReader<bool>;
#[doc = "Field `HCH` writer - HCHaIted - Read Only"]
pub type HCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `RCL` reader - Reclamation - Read Only"]
pub type RCL_R = crate::BitReader<bool>;
#[doc = "Field `RCL` writer - Reclamation - Read Only"]
pub type RCL_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `PS` reader - Periodic Schedule Status - Read Only"]
pub type PS_R = crate::BitReader<bool>;
#[doc = "Field `PS` writer - Periodic Schedule Status - Read Only"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `AS` reader - Asynchronous Schedule Status - Read Only"]
pub type AS_R = crate::BitReader<bool>;
#[doc = "Field `AS` writer - Asynchronous Schedule Status - Read Only"]
pub type AS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `NAKI` reader - NAK Interrupt Bit--RO"]
pub type NAKI_R = crate::BitReader<bool>;
#[doc = "Field `TI0` reader - General Purpose Timer Interrupt 0(GPTINT0)--R/WC"]
pub type TI0_R = crate::BitReader<bool>;
#[doc = "Field `TI0` writer - General Purpose Timer Interrupt 0(GPTINT0)--R/WC"]
pub type TI0_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
#[doc = "Field `TI1` reader - General Purpose Timer Interrupt 1(GPTINT1)--R/WC"]
pub type TI1_R = crate::BitReader<bool>;
#[doc = "Field `TI1` writer - General Purpose Timer Interrupt 1(GPTINT1)--R/WC"]
pub type TI1_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBSTS_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - USB Interrupt (USBINT) - R/WC"]
    #[inline(always)]
    pub fn ui(&self) -> UI_R {
        UI_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USB Error Interrupt (USBERRINT) - R/WC"]
    #[inline(always)]
    pub fn uei(&self) -> UEI_R {
        UEI_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Port Change Detect - R/WC"]
    #[inline(always)]
    pub fn pci(&self) -> PCI_R {
        PCI_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame List Rollover - R/WC"]
    #[inline(always)]
    pub fn fri(&self) -> FRI_R {
        FRI_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - System Error- R/WC"]
    #[inline(always)]
    pub fn sei(&self) -> SEI_R {
        SEI_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance - R/WC"]
    #[inline(always)]
    pub fn aai(&self) -> AAI_R {
        AAI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - USB Reset Received - R/WC"]
    #[inline(always)]
    pub fn uri(&self) -> URI_R {
        URI_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SOF Received - R/WC"]
    #[inline(always)]
    pub fn sri(&self) -> SRI_R {
        SRI_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DCSuspend - R/WC"]
    #[inline(always)]
    pub fn sli(&self) -> SLI_R {
        SLI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - ULPI Interrupt - R/WC"]
    #[inline(always)]
    pub fn ulpii(&self) -> ULPII_R {
        ULPII_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - HCHaIted - Read Only"]
    #[inline(always)]
    pub fn hch(&self) -> HCH_R {
        HCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reclamation - Read Only"]
    #[inline(always)]
    pub fn rcl(&self) -> RCL_R {
        RCL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Periodic Schedule Status - Read Only"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Asynchronous Schedule Status - Read Only"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - NAK Interrupt Bit--RO"]
    #[inline(always)]
    pub fn naki(&self) -> NAKI_R {
        NAKI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - General Purpose Timer Interrupt 0(GPTINT0)--R/WC"]
    #[inline(always)]
    pub fn ti0(&self) -> TI0_R {
        TI0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - General Purpose Timer Interrupt 1(GPTINT1)--R/WC"]
    #[inline(always)]
    pub fn ti1(&self) -> TI1_R {
        TI1_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USB Interrupt (USBINT) - R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn ui(&mut self) -> UI_W<0> {
        UI_W::new(self)
    }
    #[doc = "Bit 1 - USB Error Interrupt (USBERRINT) - R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn uei(&mut self) -> UEI_W<1> {
        UEI_W::new(self)
    }
    #[doc = "Bit 2 - Port Change Detect - R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn pci(&mut self) -> PCI_W<2> {
        PCI_W::new(self)
    }
    #[doc = "Bit 3 - Frame List Rollover - R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn fri(&mut self) -> FRI_W<3> {
        FRI_W::new(self)
    }
    #[doc = "Bit 4 - System Error- R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn sei(&mut self) -> SEI_W<4> {
        SEI_W::new(self)
    }
    #[doc = "Bit 5 - Interrupt on Async Advance - R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn aai(&mut self) -> AAI_W<5> {
        AAI_W::new(self)
    }
    #[doc = "Bit 6 - USB Reset Received - R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn uri(&mut self) -> URI_W<6> {
        URI_W::new(self)
    }
    #[doc = "Bit 7 - SOF Received - R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn sri(&mut self) -> SRI_W<7> {
        SRI_W::new(self)
    }
    #[doc = "Bit 8 - DCSuspend - R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn sli(&mut self) -> SLI_W<8> {
        SLI_W::new(self)
    }
    #[doc = "Bit 10 - ULPI Interrupt - R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn ulpii(&mut self) -> ULPII_W<10> {
        ULPII_W::new(self)
    }
    #[doc = "Bit 12 - HCHaIted - Read Only"]
    #[inline(always)]
    #[must_use]
    pub fn hch(&mut self) -> HCH_W<12> {
        HCH_W::new(self)
    }
    #[doc = "Bit 13 - Reclamation - Read Only"]
    #[inline(always)]
    #[must_use]
    pub fn rcl(&mut self) -> RCL_W<13> {
        RCL_W::new(self)
    }
    #[doc = "Bit 14 - Periodic Schedule Status - Read Only"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<14> {
        PS_W::new(self)
    }
    #[doc = "Bit 15 - Asynchronous Schedule Status - Read Only"]
    #[inline(always)]
    #[must_use]
    pub fn as_(&mut self) -> AS_W<15> {
        AS_W::new(self)
    }
    #[doc = "Bit 24 - General Purpose Timer Interrupt 0(GPTINT0)--R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn ti0(&mut self) -> TI0_W<24> {
        TI0_W::new(self)
    }
    #[doc = "Bit 25 - General Purpose Timer Interrupt 1(GPTINT1)--R/WC"]
    #[inline(always)]
    #[must_use]
    pub fn ti1(&mut self) -> TI1_W<25> {
        TI1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbsts](index.html) module"]
pub struct USBSTS_SPEC;
impl crate::RegisterSpec for USBSTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbsts::R](R) reader structure"]
impl crate::Readable for USBSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbsts::W](W) writer structure"]
impl crate::Writable for USBSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBSTS to value 0"]
impl crate::Resettable for USBSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
