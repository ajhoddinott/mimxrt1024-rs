#[doc = "Register `CTRL_SET` reader"]
pub struct R(crate::R<CTRL_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL_SET` writer"]
pub struct W(crate::W<CTRL_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SET_SPEC>;
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
impl From<crate::W<CTRL_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENOTG_ID_CHG_IRQ` reader - Enable OTG_ID_CHG_IRQ."]
pub type ENOTG_ID_CHG_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `ENOTG_ID_CHG_IRQ` writer - Enable OTG_ID_CHG_IRQ."]
pub type ENOTG_ID_CHG_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENHOSTDISCONDETECT` reader - For host mode, enables high-speed disconnect detector"]
pub type ENHOSTDISCONDETECT_R = crate::BitReader<bool>;
#[doc = "Field `ENHOSTDISCONDETECT` writer - For host mode, enables high-speed disconnect detector"]
pub type ENHOSTDISCONDETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENIRQHOSTDISCON` reader - Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
pub type ENIRQHOSTDISCON_R = crate::BitReader<bool>;
#[doc = "Field `ENIRQHOSTDISCON` writer - Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
pub type ENIRQHOSTDISCON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `HOSTDISCONDETECT_IRQ` reader - Indicates that the device has disconnected in high-speed mode"]
pub type HOSTDISCONDETECT_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `HOSTDISCONDETECT_IRQ` writer - Indicates that the device has disconnected in high-speed mode"]
pub type HOSTDISCONDETECT_IRQ_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENDEVPLUGINDETECT` reader - For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
pub type ENDEVPLUGINDETECT_R = crate::BitReader<bool>;
#[doc = "Field `ENDEVPLUGINDETECT` writer - For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
pub type ENDEVPLUGINDETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `DEVPLUGIN_POLARITY` reader - For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
pub type DEVPLUGIN_POLARITY_R = crate::BitReader<bool>;
#[doc = "Field `DEVPLUGIN_POLARITY` writer - For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
pub type DEVPLUGIN_POLARITY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `OTG_ID_CHG_IRQ` reader - OTG ID change interrupt. Indicates the value of ID pin changed."]
pub type OTG_ID_CHG_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `OTG_ID_CHG_IRQ` writer - OTG ID change interrupt. Indicates the value of ID pin changed."]
pub type OTG_ID_CHG_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENOTGIDDETECT` reader - Enables circuit to detect resistance of MiniAB ID pin."]
pub type ENOTGIDDETECT_R = crate::BitReader<bool>;
#[doc = "Field `ENOTGIDDETECT` writer - Enables circuit to detect resistance of MiniAB ID pin."]
pub type ENOTGIDDETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `RESUMEIRQSTICKY` reader - Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
pub type RESUMEIRQSTICKY_R = crate::BitReader<bool>;
#[doc = "Field `RESUMEIRQSTICKY` writer - Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
pub type RESUMEIRQSTICKY_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENIRQRESUMEDETECT` reader - Enables interrupt for detection of a non-J state on the USB line"]
pub type ENIRQRESUMEDETECT_R = crate::BitReader<bool>;
#[doc = "Field `ENIRQRESUMEDETECT` writer - Enables interrupt for detection of a non-J state on the USB line"]
pub type ENIRQRESUMEDETECT_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `RESUME_IRQ` reader - Indicates that the host is sending a wake-up after suspend"]
pub type RESUME_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `RESUME_IRQ` writer - Indicates that the host is sending a wake-up after suspend"]
pub type RESUME_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENIRQDEVPLUGIN` reader - Enables interrupt for the detection of connectivity to the USB line."]
pub type ENIRQDEVPLUGIN_R = crate::BitReader<bool>;
#[doc = "Field `ENIRQDEVPLUGIN` writer - Enables interrupt for the detection of connectivity to the USB line."]
pub type ENIRQDEVPLUGIN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `DEVPLUGIN_IRQ` reader - Indicates that the device is connected"]
pub type DEVPLUGIN_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `DEVPLUGIN_IRQ` writer - Indicates that the device is connected"]
pub type DEVPLUGIN_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `DATA_ON_LRADC` reader - Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
pub type DATA_ON_LRADC_R = crate::BitReader<bool>;
#[doc = "Field `DATA_ON_LRADC` writer - Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
pub type DATA_ON_LRADC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENUTMILEVEL2` reader - Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
pub type ENUTMILEVEL2_R = crate::BitReader<bool>;
#[doc = "Field `ENUTMILEVEL2` writer - Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
pub type ENUTMILEVEL2_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENUTMILEVEL3` reader - Enables UTMI+ Level3"]
pub type ENUTMILEVEL3_R = crate::BitReader<bool>;
#[doc = "Field `ENUTMILEVEL3` writer - Enables UTMI+ Level3"]
pub type ENUTMILEVEL3_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENIRQWAKEUP` reader - Enables interrupt for the wakeup events."]
pub type ENIRQWAKEUP_R = crate::BitReader<bool>;
#[doc = "Field `ENIRQWAKEUP` writer - Enables interrupt for the wakeup events."]
pub type ENIRQWAKEUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `WAKEUP_IRQ` reader - Indicates that there is a wakeup event"]
pub type WAKEUP_IRQ_R = crate::BitReader<bool>;
#[doc = "Field `WAKEUP_IRQ` writer - Indicates that there is a wakeup event"]
pub type WAKEUP_IRQ_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENAUTO_PWRON_PLL` reader - Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
pub type ENAUTO_PWRON_PLL_R = crate::BitReader<bool>;
#[doc = "Field `ENAUTO_PWRON_PLL` writer - Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
pub type ENAUTO_PWRON_PLL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENAUTOCLR_CLKGATE` reader - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub type ENAUTOCLR_CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `ENAUTOCLR_CLKGATE` writer - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
pub type ENAUTOCLR_CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENAUTOCLR_PHY_PWD` reader - Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
pub type ENAUTOCLR_PHY_PWD_R = crate::BitReader<bool>;
#[doc = "Field `ENAUTOCLR_PHY_PWD` writer - Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
pub type ENAUTOCLR_PHY_PWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENDPDMCHG_WKUP` reader - Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
pub type ENDPDMCHG_WKUP_R = crate::BitReader<bool>;
#[doc = "Field `ENDPDMCHG_WKUP` writer - Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
pub type ENDPDMCHG_WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENIDCHG_WKUP` reader - Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
pub type ENIDCHG_WKUP_R = crate::BitReader<bool>;
#[doc = "Field `ENIDCHG_WKUP` writer - Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
pub type ENIDCHG_WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `ENVBUSCHG_WKUP` reader - Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
pub type ENVBUSCHG_WKUP_R = crate::BitReader<bool>;
#[doc = "Field `ENVBUSCHG_WKUP` writer - Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
pub type ENVBUSCHG_WKUP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `FSDLL_RST_EN` reader - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
pub type FSDLL_RST_EN_R = crate::BitReader<bool>;
#[doc = "Field `FSDLL_RST_EN` writer - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
pub type FSDLL_RST_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `RSVD1` reader - Reserved."]
pub type RSVD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `OTG_ID_VALUE` reader - Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
pub type OTG_ID_VALUE_R = crate::BitReader<bool>;
#[doc = "Field `HOST_FORCE_LS_SE0` reader - Forces the next FS packet that is transmitted to have a EOP with LS timing"]
pub type HOST_FORCE_LS_SE0_R = crate::BitReader<bool>;
#[doc = "Field `HOST_FORCE_LS_SE0` writer - Forces the next FS packet that is transmitted to have a EOP with LS timing"]
pub type HOST_FORCE_LS_SE0_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `UTMI_SUSPENDM` reader - Used by the PHY to indicate a powered-down state"]
pub type UTMI_SUSPENDM_R = crate::BitReader<bool>;
#[doc = "Field `CLKGATE` reader - Gate UTMI Clocks"]
pub type CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `CLKGATE` writer - Gate UTMI Clocks"]
pub type CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
#[doc = "Field `SFTRST` reader - Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
pub type SFTRST_R = crate::BitReader<bool>;
#[doc = "Field `SFTRST` writer - Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
pub type SFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SET_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    pub fn enotg_id_chg_irq(&self) -> ENOTG_ID_CHG_IRQ_R {
        ENOTG_ID_CHG_IRQ_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    pub fn enhostdiscondetect(&self) -> ENHOSTDISCONDETECT_R {
        ENHOSTDISCONDETECT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    pub fn enirqhostdiscon(&self) -> ENIRQHOSTDISCON_R {
        ENIRQHOSTDISCON_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    pub fn hostdiscondetect_irq(&self) -> HOSTDISCONDETECT_IRQ_R {
        HOSTDISCONDETECT_IRQ_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    pub fn endevplugindetect(&self) -> ENDEVPLUGINDETECT_R {
        ENDEVPLUGINDETECT_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    pub fn devplugin_polarity(&self) -> DEVPLUGIN_POLARITY_R {
        DEVPLUGIN_POLARITY_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    pub fn otg_id_chg_irq(&self) -> OTG_ID_CHG_IRQ_R {
        OTG_ID_CHG_IRQ_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    pub fn enotgiddetect(&self) -> ENOTGIDDETECT_R {
        ENOTGIDDETECT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    pub fn resumeirqsticky(&self) -> RESUMEIRQSTICKY_R {
        RESUMEIRQSTICKY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    pub fn enirqresumedetect(&self) -> ENIRQRESUMEDETECT_R {
        ENIRQRESUMEDETECT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    pub fn resume_irq(&self) -> RESUME_IRQ_R {
        RESUME_IRQ_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    pub fn enirqdevplugin(&self) -> ENIRQDEVPLUGIN_R {
        ENIRQDEVPLUGIN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    pub fn devplugin_irq(&self) -> DEVPLUGIN_IRQ_R {
        DEVPLUGIN_IRQ_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    pub fn data_on_lradc(&self) -> DATA_ON_LRADC_R {
        DATA_ON_LRADC_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    pub fn enutmilevel2(&self) -> ENUTMILEVEL2_R {
        ENUTMILEVEL2_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level3"]
    #[inline(always)]
    pub fn enutmilevel3(&self) -> ENUTMILEVEL3_R {
        ENUTMILEVEL3_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables interrupt for the wakeup events."]
    #[inline(always)]
    pub fn enirqwakeup(&self) -> ENIRQWAKEUP_R {
        ENIRQWAKEUP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Indicates that there is a wakeup event"]
    #[inline(always)]
    pub fn wakeup_irq(&self) -> WAKEUP_IRQ_R {
        WAKEUP_IRQ_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    pub fn enauto_pwron_pll(&self) -> ENAUTO_PWRON_PLL_R {
        ENAUTO_PWRON_PLL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_clkgate(&self) -> ENAUTOCLR_CLKGATE_R {
        ENAUTOCLR_CLKGATE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    pub fn enautoclr_phy_pwd(&self) -> ENAUTOCLR_PHY_PWD_R {
        ENAUTOCLR_PHY_PWD_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    pub fn endpdmchg_wkup(&self) -> ENDPDMCHG_WKUP_R {
        ENDPDMCHG_WKUP_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    pub fn enidchg_wkup(&self) -> ENIDCHG_WKUP_R {
        ENIDCHG_WKUP_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    pub fn envbuschg_wkup(&self) -> ENVBUSCHG_WKUP_R {
        ENVBUSCHG_WKUP_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    pub fn fsdll_rst_en(&self) -> FSDLL_RST_EN_R {
        FSDLL_RST_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Almost same as OTGID_STATUS in USBPHYx_STATUS Register"]
    #[inline(always)]
    pub fn otg_id_value(&self) -> OTG_ID_VALUE_R {
        OTG_ID_VALUE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    pub fn host_force_ls_se0(&self) -> HOST_FORCE_LS_SE0_R {
        HOST_FORCE_LS_SE0_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Used by the PHY to indicate a powered-down state"]
    #[inline(always)]
    pub fn utmi_suspendm(&self) -> UTMI_SUSPENDM_R {
        UTMI_SUSPENDM_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    pub fn sftrst(&self) -> SFTRST_R {
        SFTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable OTG_ID_CHG_IRQ."]
    #[inline(always)]
    #[must_use]
    pub fn enotg_id_chg_irq(&mut self) -> ENOTG_ID_CHG_IRQ_W<0> {
        ENOTG_ID_CHG_IRQ_W::new(self)
    }
    #[doc = "Bit 1 - For host mode, enables high-speed disconnect detector"]
    #[inline(always)]
    #[must_use]
    pub fn enhostdiscondetect(&mut self) -> ENHOSTDISCONDETECT_W<1> {
        ENHOSTDISCONDETECT_W::new(self)
    }
    #[doc = "Bit 2 - Enables interrupt for detection of disconnection to Device when in high-speed host mode"]
    #[inline(always)]
    #[must_use]
    pub fn enirqhostdiscon(&mut self) -> ENIRQHOSTDISCON_W<2> {
        ENIRQHOSTDISCON_W::new(self)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected in high-speed mode"]
    #[inline(always)]
    #[must_use]
    pub fn hostdiscondetect_irq(&mut self) -> HOSTDISCONDETECT_IRQ_W<3> {
        HOSTDISCONDETECT_IRQ_W::new(self)
    }
    #[doc = "Bit 4 - For device mode, enables 200-KOhm pullups for detecting connectivity to the host."]
    #[inline(always)]
    #[must_use]
    pub fn endevplugindetect(&mut self) -> ENDEVPLUGINDETECT_W<4> {
        ENDEVPLUGINDETECT_W::new(self)
    }
    #[doc = "Bit 5 - For device mode, if this bit is cleared to 0, then it trips the interrupt if the device is plugged in"]
    #[inline(always)]
    #[must_use]
    pub fn devplugin_polarity(&mut self) -> DEVPLUGIN_POLARITY_W<5> {
        DEVPLUGIN_POLARITY_W::new(self)
    }
    #[doc = "Bit 6 - OTG ID change interrupt. Indicates the value of ID pin changed."]
    #[inline(always)]
    #[must_use]
    pub fn otg_id_chg_irq(&mut self) -> OTG_ID_CHG_IRQ_W<6> {
        OTG_ID_CHG_IRQ_W::new(self)
    }
    #[doc = "Bit 7 - Enables circuit to detect resistance of MiniAB ID pin."]
    #[inline(always)]
    #[must_use]
    pub fn enotgiddetect(&mut self) -> ENOTGIDDETECT_W<7> {
        ENOTGIDDETECT_W::new(self)
    }
    #[doc = "Bit 8 - Set to 1 will make RESUME_IRQ bit a sticky bit until software clear it"]
    #[inline(always)]
    #[must_use]
    pub fn resumeirqsticky(&mut self) -> RESUMEIRQSTICKY_W<8> {
        RESUMEIRQSTICKY_W::new(self)
    }
    #[doc = "Bit 9 - Enables interrupt for detection of a non-J state on the USB line"]
    #[inline(always)]
    #[must_use]
    pub fn enirqresumedetect(&mut self) -> ENIRQRESUMEDETECT_W<9> {
        ENIRQRESUMEDETECT_W::new(self)
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after suspend"]
    #[inline(always)]
    #[must_use]
    pub fn resume_irq(&mut self) -> RESUME_IRQ_W<10> {
        RESUME_IRQ_W::new(self)
    }
    #[doc = "Bit 11 - Enables interrupt for the detection of connectivity to the USB line."]
    #[inline(always)]
    #[must_use]
    pub fn enirqdevplugin(&mut self) -> ENIRQDEVPLUGIN_W<11> {
        ENIRQDEVPLUGIN_W::new(self)
    }
    #[doc = "Bit 12 - Indicates that the device is connected"]
    #[inline(always)]
    #[must_use]
    pub fn devplugin_irq(&mut self) -> DEVPLUGIN_IRQ_W<12> {
        DEVPLUGIN_IRQ_W::new(self)
    }
    #[doc = "Bit 13 - Enables the LRADC to monitor USB_DP and USB_DM. This is for use in non-USB modes only."]
    #[inline(always)]
    #[must_use]
    pub fn data_on_lradc(&mut self) -> DATA_ON_LRADC_W<13> {
        DATA_ON_LRADC_W::new(self)
    }
    #[doc = "Bit 14 - Enables UTMI+ Level2. This should be enabled if needs to support LS device"]
    #[inline(always)]
    #[must_use]
    pub fn enutmilevel2(&mut self) -> ENUTMILEVEL2_W<14> {
        ENUTMILEVEL2_W::new(self)
    }
    #[doc = "Bit 15 - Enables UTMI+ Level3"]
    #[inline(always)]
    #[must_use]
    pub fn enutmilevel3(&mut self) -> ENUTMILEVEL3_W<15> {
        ENUTMILEVEL3_W::new(self)
    }
    #[doc = "Bit 16 - Enables interrupt for the wakeup events."]
    #[inline(always)]
    #[must_use]
    pub fn enirqwakeup(&mut self) -> ENIRQWAKEUP_W<16> {
        ENIRQWAKEUP_W::new(self)
    }
    #[doc = "Bit 17 - Indicates that there is a wakeup event"]
    #[inline(always)]
    #[must_use]
    pub fn wakeup_irq(&mut self) -> WAKEUP_IRQ_W<17> {
        WAKEUP_IRQ_W::new(self)
    }
    #[doc = "Bit 18 - Enables the feature to auto-enable the POWER bit of HW_CLKCTRL_PLLxCTRL0 if there is wakeup event if USB is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn enauto_pwron_pll(&mut self) -> ENAUTO_PWRON_PLL_W<18> {
        ENAUTO_PWRON_PLL_W::new(self)
    }
    #[doc = "Bit 19 - Enables the feature to auto-clear the CLKGATE bit if there is wakeup event while USB is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn enautoclr_clkgate(&mut self) -> ENAUTOCLR_CLKGATE_W<19> {
        ENAUTOCLR_CLKGATE_W::new(self)
    }
    #[doc = "Bit 20 - Enables the feature to auto-clear the PWD register bits in USBPHYx_PWD if there is wakeup event while USB is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn enautoclr_phy_pwd(&mut self) -> ENAUTOCLR_PHY_PWD_W<20> {
        ENAUTOCLR_PHY_PWD_W::new(self)
    }
    #[doc = "Bit 21 - Enables the feature to wakeup USB if DP/DM is toggled when USB is suspended"]
    #[inline(always)]
    #[must_use]
    pub fn endpdmchg_wkup(&mut self) -> ENDPDMCHG_WKUP_W<21> {
        ENDPDMCHG_WKUP_W::new(self)
    }
    #[doc = "Bit 22 - Enables the feature to wakeup USB if ID is toggled when USB is suspended."]
    #[inline(always)]
    #[must_use]
    pub fn enidchg_wkup(&mut self) -> ENIDCHG_WKUP_W<22> {
        ENIDCHG_WKUP_W::new(self)
    }
    #[doc = "Bit 23 - Enables the feature to wakeup USB if VBUS is toggled when USB is suspended."]
    #[inline(always)]
    #[must_use]
    pub fn envbuschg_wkup(&mut self) -> ENVBUSCHG_WKUP_W<23> {
        ENVBUSCHG_WKUP_W::new(self)
    }
    #[doc = "Bit 24 - Enables the feature to reset the FSDLL lock detection logic at the end of each TX packet."]
    #[inline(always)]
    #[must_use]
    pub fn fsdll_rst_en(&mut self) -> FSDLL_RST_EN_W<24> {
        FSDLL_RST_EN_W::new(self)
    }
    #[doc = "Bit 28 - Forces the next FS packet that is transmitted to have a EOP with LS timing"]
    #[inline(always)]
    #[must_use]
    pub fn host_force_ls_se0(&mut self) -> HOST_FORCE_LS_SE0_W<28> {
        HOST_FORCE_LS_SE0_W::new(self)
    }
    #[doc = "Bit 30 - Gate UTMI Clocks"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> CLKGATE_W<30> {
        CLKGATE_W::new(self)
    }
    #[doc = "Bit 31 - Writing a 1 to this bit will soft-reset the USBPHYx_PWD, USBPHYx_TX, USBPHYx_RX, and USBPHYx_CTRL registers"]
    #[inline(always)]
    #[must_use]
    pub fn sftrst(&mut self) -> SFTRST_W<31> {
        SFTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY General Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl_set](index.html) module"]
pub struct CTRL_SET_SPEC;
impl crate::RegisterSpec for CTRL_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl_set::R](R) reader structure"]
impl crate::Readable for CTRL_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl_set::W](W) writer structure"]
impl crate::Writable for CTRL_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL_SET to value 0xc020_0000"]
impl crate::Resettable for CTRL_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0xc020_0000;
}
