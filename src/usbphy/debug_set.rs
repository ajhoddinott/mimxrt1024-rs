#[doc = "Register `DEBUG_SET` reader"]
pub struct R(crate::R<DEBUG_SET_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DEBUG_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DEBUG_SET_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DEBUG_SET_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DEBUG_SET` writer"]
pub struct W(crate::W<DEBUG_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DEBUG_SET_SPEC>;
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
impl From<crate::W<DEBUG_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DEBUG_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OTGIDPIOLOCK` reader - Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
pub type OTGIDPIOLOCK_R = crate::BitReader<bool>;
#[doc = "Field `OTGIDPIOLOCK` writer - Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
pub type OTGIDPIOLOCK_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_SET_SPEC, bool, O>;
#[doc = "Field `DEBUG_INTERFACE_HOLD` reader - Use holding registers to assist in timing for external UTMI interface."]
pub type DEBUG_INTERFACE_HOLD_R = crate::BitReader<bool>;
#[doc = "Field `DEBUG_INTERFACE_HOLD` writer - Use holding registers to assist in timing for external UTMI interface."]
pub type DEBUG_INTERFACE_HOLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, DEBUG_SET_SPEC, bool, O>;
#[doc = "Field `HSTPULLDOWN` reader - Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
pub type HSTPULLDOWN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HSTPULLDOWN` writer - Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
pub type HSTPULLDOWN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEBUG_SET_SPEC, u8, u8, 2, O>;
#[doc = "Field `ENHSTPULLDOWN` reader - Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
pub type ENHSTPULLDOWN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENHSTPULLDOWN` writer - Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
pub type ENHSTPULLDOWN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG_SET_SPEC, u8, u8, 2, O>;
#[doc = "Field `RSVD0` reader - Reserved."]
pub type RSVD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX2RXCOUNT` reader - Delay in between the end of transmit to the beginning of receive"]
pub type TX2RXCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TX2RXCOUNT` writer - Delay in between the end of transmit to the beginning of receive"]
pub type TX2RXCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DEBUG_SET_SPEC, u8, u8, 4, O>;
#[doc = "Field `ENTX2RXCOUNT` reader - Set this bit to allow a countdown to transition in between TX and RX."]
pub type ENTX2RXCOUNT_R = crate::BitReader<bool>;
#[doc = "Field `ENTX2RXCOUNT` writer - Set this bit to allow a countdown to transition in between TX and RX."]
pub type ENTX2RXCOUNT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_SET_SPEC, bool, O>;
#[doc = "Field `RSVD1` reader - Reserved."]
pub type RSVD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQUELCHRESETCOUNT` reader - Delay in between the detection of squelch to the reset of high-speed RX."]
pub type SQUELCHRESETCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQUELCHRESETCOUNT` writer - Delay in between the detection of squelch to the reset of high-speed RX."]
pub type SQUELCHRESETCOUNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG_SET_SPEC, u8, u8, 5, O>;
#[doc = "Field `RSVD2` reader - Reserved."]
pub type RSVD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ENSQUELCHRESET` reader - Set bit to allow squelch to reset high-speed receive."]
pub type ENSQUELCHRESET_R = crate::BitReader<bool>;
#[doc = "Field `ENSQUELCHRESET` writer - Set bit to allow squelch to reset high-speed receive."]
pub type ENSQUELCHRESET_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_SET_SPEC, bool, O>;
#[doc = "Field `SQUELCHRESETLENGTH` reader - Duration of RESET in terms of the number of 480-MHz cycles."]
pub type SQUELCHRESETLENGTH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SQUELCHRESETLENGTH` writer - Duration of RESET in terms of the number of 480-MHz cycles."]
pub type SQUELCHRESETLENGTH_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DEBUG_SET_SPEC, u8, u8, 4, O>;
#[doc = "Field `HOST_RESUME_DEBUG` reader - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
pub type HOST_RESUME_DEBUG_R = crate::BitReader<bool>;
#[doc = "Field `HOST_RESUME_DEBUG` writer - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
pub type HOST_RESUME_DEBUG_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_SET_SPEC, bool, O>;
#[doc = "Field `CLKGATE` reader - Gate Test Clocks"]
pub type CLKGATE_R = crate::BitReader<bool>;
#[doc = "Field `CLKGATE` writer - Gate Test Clocks"]
pub type CLKGATE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DEBUG_SET_SPEC, bool, O>;
#[doc = "Field `RSVD3` reader - Reserved."]
pub type RSVD3_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    pub fn otgidpiolock(&self) -> OTGIDPIOLOCK_R {
        OTGIDPIOLOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    pub fn debug_interface_hold(&self) -> DEBUG_INTERFACE_HOLD_R {
        DEBUG_INTERFACE_HOLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    pub fn hstpulldown(&self) -> HSTPULLDOWN_R {
        HSTPULLDOWN_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    pub fn enhstpulldown(&self) -> ENHSTPULLDOWN_R {
        ENHSTPULLDOWN_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    pub fn tx2rxcount(&self) -> TX2RXCOUNT_R {
        TX2RXCOUNT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    pub fn entx2rxcount(&self) -> ENTX2RXCOUNT_R {
        ENTX2RXCOUNT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    pub fn squelchresetcount(&self) -> SQUELCHRESETCOUNT_R {
        SQUELCHRESETCOUNT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    pub fn ensquelchreset(&self) -> ENSQUELCHRESET_R {
        ENSQUELCHRESET_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:28 - Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    pub fn squelchresetlength(&self) -> SQUELCHRESETLENGTH_R {
        SQUELCHRESETLENGTH_R::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    pub fn host_resume_debug(&self) -> HOST_RESUME_DEBUG_R {
        HOST_RESUME_DEBUG_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Gate Test Clocks"]
    #[inline(always)]
    pub fn clkgate(&self) -> CLKGATE_R {
        CLKGATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved."]
    #[inline(always)]
    pub fn rsvd3(&self) -> RSVD3_R {
        RSVD3_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Once OTG ID from USBPHYx_STATUS_OTGID_STATUS, use this to hold the value"]
    #[inline(always)]
    #[must_use]
    pub fn otgidpiolock(&mut self) -> OTGIDPIOLOCK_W<0> {
        OTGIDPIOLOCK_W::new(self)
    }
    #[doc = "Bit 1 - Use holding registers to assist in timing for external UTMI interface."]
    #[inline(always)]
    #[must_use]
    pub fn debug_interface_hold(&mut self) -> DEBUG_INTERFACE_HOLD_W<1> {
        DEBUG_INTERFACE_HOLD_W::new(self)
    }
    #[doc = "Bits 2:3 - Set bit 3 to 1 to pull down 15-KOhm on USB_DP line"]
    #[inline(always)]
    #[must_use]
    pub fn hstpulldown(&mut self) -> HSTPULLDOWN_W<2> {
        HSTPULLDOWN_W::new(self)
    }
    #[doc = "Bits 4:5 - Set bit 5 to 1 to override the control of the USB_DP 15-KOhm pulldown"]
    #[inline(always)]
    #[must_use]
    pub fn enhstpulldown(&mut self) -> ENHSTPULLDOWN_W<4> {
        ENHSTPULLDOWN_W::new(self)
    }
    #[doc = "Bits 8:11 - Delay in between the end of transmit to the beginning of receive"]
    #[inline(always)]
    #[must_use]
    pub fn tx2rxcount(&mut self) -> TX2RXCOUNT_W<8> {
        TX2RXCOUNT_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to allow a countdown to transition in between TX and RX."]
    #[inline(always)]
    #[must_use]
    pub fn entx2rxcount(&mut self) -> ENTX2RXCOUNT_W<12> {
        ENTX2RXCOUNT_W::new(self)
    }
    #[doc = "Bits 16:20 - Delay in between the detection of squelch to the reset of high-speed RX."]
    #[inline(always)]
    #[must_use]
    pub fn squelchresetcount(&mut self) -> SQUELCHRESETCOUNT_W<16> {
        SQUELCHRESETCOUNT_W::new(self)
    }
    #[doc = "Bit 24 - Set bit to allow squelch to reset high-speed receive."]
    #[inline(always)]
    #[must_use]
    pub fn ensquelchreset(&mut self) -> ENSQUELCHRESET_W<24> {
        ENSQUELCHRESET_W::new(self)
    }
    #[doc = "Bits 25:28 - Duration of RESET in terms of the number of 480-MHz cycles."]
    #[inline(always)]
    #[must_use]
    pub fn squelchresetlength(&mut self) -> SQUELCHRESETLENGTH_W<25> {
        SQUELCHRESETLENGTH_W::new(self)
    }
    #[doc = "Bit 29 - Choose to trigger the host resume SE0 with HOST_FORCE_LS_SE0 = 0 or UTMI_SUSPEND = 1."]
    #[inline(always)]
    #[must_use]
    pub fn host_resume_debug(&mut self) -> HOST_RESUME_DEBUG_W<29> {
        HOST_RESUME_DEBUG_W::new(self)
    }
    #[doc = "Bit 30 - Gate Test Clocks"]
    #[inline(always)]
    #[must_use]
    pub fn clkgate(&mut self) -> CLKGATE_W<30> {
        CLKGATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Debug Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [debug_set](index.html) module"]
pub struct DEBUG_SET_SPEC;
impl crate::RegisterSpec for DEBUG_SET_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [debug_set::R](R) reader structure"]
impl crate::Readable for DEBUG_SET_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [debug_set::W](W) writer structure"]
impl crate::Writable for DEBUG_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DEBUG_SET to value 0x7f18_0000"]
impl crate::Resettable for DEBUG_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0x7f18_0000;
}
