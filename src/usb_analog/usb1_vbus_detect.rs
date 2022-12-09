#[doc = "Register `USB1_VBUS_DETECT` reader"]
pub struct R(crate::R<USB1_VBUS_DETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_VBUS_DETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_VBUS_DETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_VBUS_DETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_VBUS_DETECT` writer"]
pub struct W(crate::W<USB1_VBUS_DETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_VBUS_DETECT_SPEC>;
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
impl From<crate::W<USB1_VBUS_DETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_VBUS_DETECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VBUSVALID_THRESH` reader - Set the threshold for the VBUSVALID comparator"]
pub type VBUSVALID_THRESH_R = crate::FieldReader<u8, VBUSVALID_THRESH_A>;
#[doc = "Set the threshold for the VBUSVALID comparator\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VBUSVALID_THRESH_A {
    #[doc = "0: 4.0V"]
    _4V0 = 0,
    #[doc = "1: 4.1V"]
    _4V1 = 1,
    #[doc = "2: 4.2V"]
    _4V2 = 2,
    #[doc = "3: 4.3V"]
    _4V3 = 3,
    #[doc = "4: 4.4V (default)"]
    _4V4 = 4,
    #[doc = "5: 4.5V"]
    _4V5 = 5,
    #[doc = "6: 4.6V"]
    _4V6 = 6,
    #[doc = "7: 4.7V"]
    _4V7 = 7,
}
impl From<VBUSVALID_THRESH_A> for u8 {
    #[inline(always)]
    fn from(variant: VBUSVALID_THRESH_A) -> Self {
        variant as _
    }
}
impl VBUSVALID_THRESH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBUSVALID_THRESH_A {
        match self.bits {
            0 => VBUSVALID_THRESH_A::_4V0,
            1 => VBUSVALID_THRESH_A::_4V1,
            2 => VBUSVALID_THRESH_A::_4V2,
            3 => VBUSVALID_THRESH_A::_4V3,
            4 => VBUSVALID_THRESH_A::_4V4,
            5 => VBUSVALID_THRESH_A::_4V5,
            6 => VBUSVALID_THRESH_A::_4V6,
            7 => VBUSVALID_THRESH_A::_4V7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_4V0`"]
    #[inline(always)]
    pub fn is_4v0(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V0
    }
    #[doc = "Checks if the value of the field is `_4V1`"]
    #[inline(always)]
    pub fn is_4v1(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V1
    }
    #[doc = "Checks if the value of the field is `_4V2`"]
    #[inline(always)]
    pub fn is_4v2(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V2
    }
    #[doc = "Checks if the value of the field is `_4V3`"]
    #[inline(always)]
    pub fn is_4v3(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V3
    }
    #[doc = "Checks if the value of the field is `_4V4`"]
    #[inline(always)]
    pub fn is_4v4(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V4
    }
    #[doc = "Checks if the value of the field is `_4V5`"]
    #[inline(always)]
    pub fn is_4v5(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V5
    }
    #[doc = "Checks if the value of the field is `_4V6`"]
    #[inline(always)]
    pub fn is_4v6(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V6
    }
    #[doc = "Checks if the value of the field is `_4V7`"]
    #[inline(always)]
    pub fn is_4v7(&self) -> bool {
        *self == VBUSVALID_THRESH_A::_4V7
    }
}
#[doc = "Field `VBUSVALID_THRESH` writer - Set the threshold for the VBUSVALID comparator"]
pub type VBUSVALID_THRESH_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, USB1_VBUS_DETECT_SPEC, u8, VBUSVALID_THRESH_A, 3, O>;
impl<'a, const O: u8> VBUSVALID_THRESH_W<'a, O> {
    #[doc = "4.0V"]
    #[inline(always)]
    pub fn _4v0(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V0)
    }
    #[doc = "4.1V"]
    #[inline(always)]
    pub fn _4v1(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V1)
    }
    #[doc = "4.2V"]
    #[inline(always)]
    pub fn _4v2(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V2)
    }
    #[doc = "4.3V"]
    #[inline(always)]
    pub fn _4v3(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V3)
    }
    #[doc = "4.4V (default)"]
    #[inline(always)]
    pub fn _4v4(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V4)
    }
    #[doc = "4.5V"]
    #[inline(always)]
    pub fn _4v5(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V5)
    }
    #[doc = "4.6V"]
    #[inline(always)]
    pub fn _4v6(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V6)
    }
    #[doc = "4.7V"]
    #[inline(always)]
    pub fn _4v7(self) -> &'a mut W {
        self.variant(VBUSVALID_THRESH_A::_4V7)
    }
}
#[doc = "Field `VBUSVALID_PWRUP_CMPS` reader - Powers up comparators for vbus_valid detector."]
pub type VBUSVALID_PWRUP_CMPS_R = crate::BitReader<bool>;
#[doc = "Field `VBUSVALID_PWRUP_CMPS` writer - Powers up comparators for vbus_valid detector."]
pub type VBUSVALID_PWRUP_CMPS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, bool, O>;
#[doc = "Field `DISCHARGE_VBUS` reader - USB OTG discharge VBUS."]
pub type DISCHARGE_VBUS_R = crate::BitReader<bool>;
#[doc = "Field `DISCHARGE_VBUS` writer - USB OTG discharge VBUS."]
pub type DISCHARGE_VBUS_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, bool, O>;
#[doc = "Field `CHARGE_VBUS` reader - USB OTG charge VBUS."]
pub type CHARGE_VBUS_R = crate::BitReader<bool>;
#[doc = "Field `CHARGE_VBUS` writer - USB OTG charge VBUS."]
pub type CHARGE_VBUS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1_VBUS_DETECT_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    pub fn vbusvalid_thresh(&self) -> VBUSVALID_THRESH_R {
        VBUSVALID_THRESH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 20 - Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    pub fn vbusvalid_pwrup_cmps(&self) -> VBUSVALID_PWRUP_CMPS_R {
        VBUSVALID_PWRUP_CMPS_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 26 - USB OTG discharge VBUS."]
    #[inline(always)]
    pub fn discharge_vbus(&self) -> DISCHARGE_VBUS_R {
        DISCHARGE_VBUS_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - USB OTG charge VBUS."]
    #[inline(always)]
    pub fn charge_vbus(&self) -> CHARGE_VBUS_R {
        CHARGE_VBUS_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set the threshold for the VBUSVALID comparator"]
    #[inline(always)]
    #[must_use]
    pub fn vbusvalid_thresh(&mut self) -> VBUSVALID_THRESH_W<0> {
        VBUSVALID_THRESH_W::new(self)
    }
    #[doc = "Bit 20 - Powers up comparators for vbus_valid detector."]
    #[inline(always)]
    #[must_use]
    pub fn vbusvalid_pwrup_cmps(&mut self) -> VBUSVALID_PWRUP_CMPS_W<20> {
        VBUSVALID_PWRUP_CMPS_W::new(self)
    }
    #[doc = "Bit 26 - USB OTG discharge VBUS."]
    #[inline(always)]
    #[must_use]
    pub fn discharge_vbus(&mut self) -> DISCHARGE_VBUS_W<26> {
        DISCHARGE_VBUS_W::new(self)
    }
    #[doc = "Bit 27 - USB OTG charge VBUS."]
    #[inline(always)]
    #[must_use]
    pub fn charge_vbus(&mut self) -> CHARGE_VBUS_W<27> {
        CHARGE_VBUS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB VBUS Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_vbus_detect](index.html) module"]
pub struct USB1_VBUS_DETECT_SPEC;
impl crate::RegisterSpec for USB1_VBUS_DETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_vbus_detect::R](R) reader structure"]
impl crate::Readable for USB1_VBUS_DETECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_vbus_detect::W](W) writer structure"]
impl crate::Writable for USB1_VBUS_DETECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_VBUS_DETECT to value 0x0010_0004"]
impl crate::Resettable for USB1_VBUS_DETECT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0010_0004;
}
