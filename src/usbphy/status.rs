#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STATUS` writer"]
pub struct W(crate::W<STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STATUS_SPEC>;
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
impl From<crate::W<STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSVD0` reader - Reserved."]
pub type RSVD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `HOSTDISCONDETECT_STATUS` reader - Indicates that the device has disconnected while in high-speed host mode."]
pub type HOSTDISCONDETECT_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `RSVD1` reader - Reserved."]
pub type RSVD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DEVPLUGIN_STATUS` reader - Indicates that the device has been connected on the USB_DP and USB_DM lines."]
pub type DEVPLUGIN_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `RSVD2` reader - Reserved."]
pub type RSVD2_R = crate::BitReader<bool>;
#[doc = "Field `OTGID_STATUS` reader - Indicates the results of ID pin on MiniAB plug"]
pub type OTGID_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `OTGID_STATUS` writer - Indicates the results of ID pin on MiniAB plug"]
pub type OTGID_STATUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, STATUS_SPEC, bool, O>;
#[doc = "Field `RSVD3` reader - Reserved."]
pub type RSVD3_R = crate::BitReader<bool>;
#[doc = "Field `RESUME_STATUS` reader - Indicates that the host is sending a wake-up after suspend and has triggered an interrupt."]
pub type RESUME_STATUS_R = crate::BitReader<bool>;
#[doc = "Field `RSVD4` reader - Reserved."]
pub type RSVD4_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:2 - Reserved."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Indicates that the device has disconnected while in high-speed host mode."]
    #[inline(always)]
    pub fn hostdiscondetect_status(&self) -> HOSTDISCONDETECT_STATUS_R {
        HOSTDISCONDETECT_STATUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Reserved."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Indicates that the device has been connected on the USB_DP and USB_DM lines."]
    #[inline(always)]
    pub fn devplugin_status(&self) -> DEVPLUGIN_STATUS_R {
        DEVPLUGIN_STATUS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates the results of ID pin on MiniAB plug"]
    #[inline(always)]
    pub fn otgid_status(&self) -> OTGID_STATUS_R {
        OTGID_STATUS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved."]
    #[inline(always)]
    pub fn rsvd3(&self) -> RSVD3_R {
        RSVD3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Indicates that the host is sending a wake-up after suspend and has triggered an interrupt."]
    #[inline(always)]
    pub fn resume_status(&self) -> RESUME_STATUS_R {
        RESUME_STATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd4(&self) -> RSVD4_R {
        RSVD4_R::new((self.bits >> 11) & 0x001f_ffff)
    }
}
impl W {
    #[doc = "Bit 8 - Indicates the results of ID pin on MiniAB plug"]
    #[inline(always)]
    #[must_use]
    pub fn otgid_status(&mut self) -> OTGID_STATUS_W<8> {
        OTGID_STATUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [status::W](W) writer structure"]
impl crate::Writable for STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
