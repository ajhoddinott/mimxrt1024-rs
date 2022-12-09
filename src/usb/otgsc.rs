#[doc = "Register `OTGSC` reader"]
pub struct R(crate::R<OTGSC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OTGSC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OTGSC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OTGSC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OTGSC` writer"]
pub struct W(crate::W<OTGSC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OTGSC_SPEC>;
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
impl From<crate::W<OTGSC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OTGSC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VD` reader - VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
pub type VD_R = crate::BitReader<bool>;
#[doc = "Field `VD` writer - VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
pub type VD_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `VC` reader - VBUS Charge - Read/Write"]
pub type VC_R = crate::BitReader<bool>;
#[doc = "Field `VC` writer - VBUS Charge - Read/Write"]
pub type VC_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `OT` reader - OTG Termination - Read/Write"]
pub type OT_R = crate::BitReader<bool>;
#[doc = "Field `OT` writer - OTG Termination - Read/Write"]
pub type OT_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `DP` reader - Data Pulsing - Read/Write"]
pub type DP_R = crate::BitReader<bool>;
#[doc = "Field `DP` writer - Data Pulsing - Read/Write"]
pub type DP_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `IDPU` reader - ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]"]
pub type IDPU_R = crate::BitReader<bool>;
#[doc = "Field `IDPU` writer - ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]"]
pub type IDPU_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `ID` reader - USB ID - Read Only. 0 = A device, 1 = B device"]
pub type ID_R = crate::BitReader<bool>;
#[doc = "Field `AVV` reader - A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
pub type AVV_R = crate::BitReader<bool>;
#[doc = "Field `ASV` reader - A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
pub type ASV_R = crate::BitReader<bool>;
#[doc = "Field `BSV` reader - B Session Valid - Read Only. Indicates VBus is above the B session valid threshold."]
pub type BSV_R = crate::BitReader<bool>;
#[doc = "Field `BSE` reader - B Session End - Read Only. Indicates VBus is below the B session end threshold."]
pub type BSE_R = crate::BitReader<bool>;
#[doc = "Field `TOG_1MS` reader - 1 millisecond timer toggle - Read Only. This bit toggles once per millisecond."]
pub type TOG_1MS_R = crate::BitReader<bool>;
#[doc = "Field `DPS` reader - Data Bus Pulsing Status - Read Only"]
pub type DPS_R = crate::BitReader<bool>;
#[doc = "Field `IDIS` reader - USB ID Interrupt Status - Read/Write"]
pub type IDIS_R = crate::BitReader<bool>;
#[doc = "Field `IDIS` writer - USB ID Interrupt Status - Read/Write"]
pub type IDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `AVVIS` reader - A VBus Valid Interrupt Status - Read/Write to Clear"]
pub type AVVIS_R = crate::BitReader<bool>;
#[doc = "Field `AVVIS` writer - A VBus Valid Interrupt Status - Read/Write to Clear"]
pub type AVVIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `ASVIS` reader - A Session Valid Interrupt Status - Read/Write to Clear"]
pub type ASVIS_R = crate::BitReader<bool>;
#[doc = "Field `ASVIS` writer - A Session Valid Interrupt Status - Read/Write to Clear"]
pub type ASVIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `BSVIS` reader - B Session Valid Interrupt Status - Read/Write to Clear"]
pub type BSVIS_R = crate::BitReader<bool>;
#[doc = "Field `BSVIS` writer - B Session Valid Interrupt Status - Read/Write to Clear"]
pub type BSVIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `BSEIS` reader - B Session End Interrupt Status - Read/Write to Clear"]
pub type BSEIS_R = crate::BitReader<bool>;
#[doc = "Field `BSEIS` writer - B Session End Interrupt Status - Read/Write to Clear"]
pub type BSEIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `STATUS_1MS` reader - 1 millisecond timer Interrupt Status - Read/Write to Clear"]
pub type STATUS_1MS_R = crate::BitReader<bool>;
#[doc = "Field `STATUS_1MS` writer - 1 millisecond timer Interrupt Status - Read/Write to Clear"]
pub type STATUS_1MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `DPIS` reader - Data Pulse Interrupt Status - Read/Write to Clear"]
pub type DPIS_R = crate::BitReader<bool>;
#[doc = "Field `DPIS` writer - Data Pulse Interrupt Status - Read/Write to Clear"]
pub type DPIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `IDIE` reader - USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
pub type IDIE_R = crate::BitReader<bool>;
#[doc = "Field `IDIE` writer - USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
pub type IDIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `AVVIE` reader - A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
pub type AVVIE_R = crate::BitReader<bool>;
#[doc = "Field `AVVIE` writer - A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
pub type AVVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `ASVIE` reader - A Session Valid Interrupt Enable - Read/Write"]
pub type ASVIE_R = crate::BitReader<bool>;
#[doc = "Field `ASVIE` writer - A Session Valid Interrupt Enable - Read/Write"]
pub type ASVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `BSVIE` reader - B Session Valid Interrupt Enable - Read/Write"]
pub type BSVIE_R = crate::BitReader<bool>;
#[doc = "Field `BSVIE` writer - B Session Valid Interrupt Enable - Read/Write"]
pub type BSVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `BSEIE` reader - B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt."]
pub type BSEIE_R = crate::BitReader<bool>;
#[doc = "Field `BSEIE` writer - B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt."]
pub type BSEIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `EN_1MS` reader - 1 millisecond timer Interrupt Enable - Read/Write"]
pub type EN_1MS_R = crate::BitReader<bool>;
#[doc = "Field `EN_1MS` writer - 1 millisecond timer Interrupt Enable - Read/Write"]
pub type EN_1MS_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
#[doc = "Field `DPIE` reader - Data Pulse Interrupt Enable"]
pub type DPIE_R = crate::BitReader<bool>;
#[doc = "Field `DPIE` writer - Data Pulse Interrupt Enable"]
pub type DPIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, OTGSC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[inline(always)]
    pub fn vd(&self) -> VD_R {
        VD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VBUS Charge - Read/Write"]
    #[inline(always)]
    pub fn vc(&self) -> VC_R {
        VC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - OTG Termination - Read/Write"]
    #[inline(always)]
    pub fn ot(&self) -> OT_R {
        OT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data Pulsing - Read/Write"]
    #[inline(always)]
    pub fn dp(&self) -> DP_R {
        DP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]"]
    #[inline(always)]
    pub fn idpu(&self) -> IDPU_R {
        IDPU_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - USB ID - Read Only. 0 = A device, 1 = B device"]
    #[inline(always)]
    pub fn id(&self) -> ID_R {
        ID_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A VBus Valid - Read Only. Indicates VBus is above the A VBus valid threshold."]
    #[inline(always)]
    pub fn avv(&self) -> AVV_R {
        AVV_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A Session Valid - Read Only. Indicates VBus is above the A session valid threshold."]
    #[inline(always)]
    pub fn asv(&self) -> ASV_R {
        ASV_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - B Session Valid - Read Only. Indicates VBus is above the B session valid threshold."]
    #[inline(always)]
    pub fn bsv(&self) -> BSV_R {
        BSV_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - B Session End - Read Only. Indicates VBus is below the B session end threshold."]
    #[inline(always)]
    pub fn bse(&self) -> BSE_R {
        BSE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1 millisecond timer toggle - Read Only. This bit toggles once per millisecond."]
    #[inline(always)]
    pub fn tog_1ms(&self) -> TOG_1MS_R {
        TOG_1MS_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Data Bus Pulsing Status - Read Only"]
    #[inline(always)]
    pub fn dps(&self) -> DPS_R {
        DPS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - USB ID Interrupt Status - Read/Write"]
    #[inline(always)]
    pub fn idis(&self) -> IDIS_R {
        IDIS_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - A VBus Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn avvis(&self) -> AVVIS_R {
        AVVIS_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - A Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn asvis(&self) -> ASVIS_R {
        ASVIS_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - B Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn bsvis(&self) -> BSVIS_R {
        BSVIS_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - B Session End Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn bseis(&self) -> BSEIS_R {
        BSEIS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1 millisecond timer Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn status_1ms(&self) -> STATUS_1MS_R {
        STATUS_1MS_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Data Pulse Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    pub fn dpis(&self) -> DPIS_R {
        DPIS_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 24 - USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[inline(always)]
    pub fn idie(&self) -> IDIE_R {
        IDIE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[inline(always)]
    pub fn avvie(&self) -> AVVIE_R {
        AVVIE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - A Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub fn asvie(&self) -> ASVIE_R {
        ASVIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - B Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub fn bsvie(&self) -> BSVIE_R {
        BSVIE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt."]
    #[inline(always)]
    pub fn bseie(&self) -> BSEIE_R {
        BSEIE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1 millisecond timer Interrupt Enable - Read/Write"]
    #[inline(always)]
    pub fn en_1ms(&self) -> EN_1MS_R {
        EN_1MS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Data Pulse Interrupt Enable"]
    #[inline(always)]
    pub fn dpie(&self) -> DPIE_R {
        DPIE_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VBUS_Discharge - Read/Write. Setting this bit causes VBus to discharge through a resistor."]
    #[inline(always)]
    #[must_use]
    pub fn vd(&mut self) -> VD_W<0> {
        VD_W::new(self)
    }
    #[doc = "Bit 1 - VBUS Charge - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn vc(&mut self) -> VC_W<1> {
        VC_W::new(self)
    }
    #[doc = "Bit 3 - OTG Termination - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn ot(&mut self) -> OT_W<3> {
        OT_W::new(self)
    }
    #[doc = "Bit 4 - Data Pulsing - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn dp(&mut self) -> DP_W<4> {
        DP_W::new(self)
    }
    #[doc = "Bit 5 - ID Pullup - Read/Write This bit provide control over the ID pull-up resistor; 0 = off, 1 = on \\[default\\]"]
    #[inline(always)]
    #[must_use]
    pub fn idpu(&mut self) -> IDPU_W<5> {
        IDPU_W::new(self)
    }
    #[doc = "Bit 16 - USB ID Interrupt Status - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn idis(&mut self) -> IDIS_W<16> {
        IDIS_W::new(self)
    }
    #[doc = "Bit 17 - A VBus Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    #[must_use]
    pub fn avvis(&mut self) -> AVVIS_W<17> {
        AVVIS_W::new(self)
    }
    #[doc = "Bit 18 - A Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    #[must_use]
    pub fn asvis(&mut self) -> ASVIS_W<18> {
        ASVIS_W::new(self)
    }
    #[doc = "Bit 19 - B Session Valid Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bsvis(&mut self) -> BSVIS_W<19> {
        BSVIS_W::new(self)
    }
    #[doc = "Bit 20 - B Session End Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    #[must_use]
    pub fn bseis(&mut self) -> BSEIS_W<20> {
        BSEIS_W::new(self)
    }
    #[doc = "Bit 21 - 1 millisecond timer Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    #[must_use]
    pub fn status_1ms(&mut self) -> STATUS_1MS_W<21> {
        STATUS_1MS_W::new(self)
    }
    #[doc = "Bit 22 - Data Pulse Interrupt Status - Read/Write to Clear"]
    #[inline(always)]
    #[must_use]
    pub fn dpis(&mut self) -> DPIS_W<22> {
        DPIS_W::new(self)
    }
    #[doc = "Bit 24 - USB ID Interrupt Enable - Read/Write. Setting this bit enables the USB ID interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn idie(&mut self) -> IDIE_W<24> {
        IDIE_W::new(self)
    }
    #[doc = "Bit 25 - A VBus Valid Interrupt Enable - Read/Write. Setting this bit enables the A VBus valid interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn avvie(&mut self) -> AVVIE_W<25> {
        AVVIE_W::new(self)
    }
    #[doc = "Bit 26 - A Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn asvie(&mut self) -> ASVIE_W<26> {
        ASVIE_W::new(self)
    }
    #[doc = "Bit 27 - B Session Valid Interrupt Enable - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn bsvie(&mut self) -> BSVIE_W<27> {
        BSVIE_W::new(self)
    }
    #[doc = "Bit 28 - B Session End Interrupt Enable - Read/Write. Setting this bit enables the B session end interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn bseie(&mut self) -> BSEIE_W<28> {
        BSEIE_W::new(self)
    }
    #[doc = "Bit 29 - 1 millisecond timer Interrupt Enable - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn en_1ms(&mut self) -> EN_1MS_W<29> {
        EN_1MS_W::new(self)
    }
    #[doc = "Bit 30 - Data Pulse Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpie(&mut self) -> DPIE_W<30> {
        DPIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "On-The-Go Status & control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [otgsc](index.html) module"]
pub struct OTGSC_SPEC;
impl crate::RegisterSpec for OTGSC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [otgsc::R](R) reader structure"]
impl crate::Readable for OTGSC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [otgsc::W](W) writer structure"]
impl crate::Writable for OTGSC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OTGSC to value 0x1120"]
impl crate::Resettable for OTGSC_SPEC {
    const RESET_VALUE: Self::Ux = 0x1120;
}
