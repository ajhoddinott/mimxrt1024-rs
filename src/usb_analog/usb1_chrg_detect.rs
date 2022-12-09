#[doc = "Register `USB1_CHRG_DETECT` reader"]
pub struct R(crate::R<USB1_CHRG_DETECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_CHRG_DETECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_CHRG_DETECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_CHRG_DETECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USB1_CHRG_DETECT` writer"]
pub struct W(crate::W<USB1_CHRG_DETECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USB1_CHRG_DETECT_SPEC>;
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
impl From<crate::W<USB1_CHRG_DETECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USB1_CHRG_DETECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CHK_CONTACT` reader - Check the contact of USB plug"]
pub type CHK_CONTACT_R = crate::BitReader<CHK_CONTACT_A>;
#[doc = "Check the contact of USB plug\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHK_CONTACT_A {
    #[doc = "0: Do not check the contact of USB plug."]
    NO_CHECK = 0,
    #[doc = "1: Check whether the USB plug has been in contact with each other"]
    CHECK = 1,
}
impl From<CHK_CONTACT_A> for bool {
    #[inline(always)]
    fn from(variant: CHK_CONTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl CHK_CONTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHK_CONTACT_A {
        match self.bits {
            false => CHK_CONTACT_A::NO_CHECK,
            true => CHK_CONTACT_A::CHECK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CHECK`"]
    #[inline(always)]
    pub fn is_no_check(&self) -> bool {
        *self == CHK_CONTACT_A::NO_CHECK
    }
    #[doc = "Checks if the value of the field is `CHECK`"]
    #[inline(always)]
    pub fn is_check(&self) -> bool {
        *self == CHK_CONTACT_A::CHECK
    }
}
#[doc = "Field `CHK_CONTACT` writer - Check the contact of USB plug"]
pub type CHK_CONTACT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_CHRG_DETECT_SPEC, CHK_CONTACT_A, O>;
impl<'a, const O: u8> CHK_CONTACT_W<'a, O> {
    #[doc = "Do not check the contact of USB plug."]
    #[inline(always)]
    pub fn no_check(self) -> &'a mut W {
        self.variant(CHK_CONTACT_A::NO_CHECK)
    }
    #[doc = "Check whether the USB plug has been in contact with each other"]
    #[inline(always)]
    pub fn check(self) -> &'a mut W {
        self.variant(CHK_CONTACT_A::CHECK)
    }
}
#[doc = "Field `CHK_CHRG_B` reader - Check the charger connection"]
pub type CHK_CHRG_B_R = crate::BitReader<CHK_CHRG_B_A>;
#[doc = "Check the charger connection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHK_CHRG_B_A {
    #[doc = "0: Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    CHECK = 0,
    #[doc = "1: Do not check whether a charger is connected to the USB port."]
    NO_CHECK = 1,
}
impl From<CHK_CHRG_B_A> for bool {
    #[inline(always)]
    fn from(variant: CHK_CHRG_B_A) -> Self {
        variant as u8 != 0
    }
}
impl CHK_CHRG_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHK_CHRG_B_A {
        match self.bits {
            false => CHK_CHRG_B_A::CHECK,
            true => CHK_CHRG_B_A::NO_CHECK,
        }
    }
    #[doc = "Checks if the value of the field is `CHECK`"]
    #[inline(always)]
    pub fn is_check(&self) -> bool {
        *self == CHK_CHRG_B_A::CHECK
    }
    #[doc = "Checks if the value of the field is `NO_CHECK`"]
    #[inline(always)]
    pub fn is_no_check(&self) -> bool {
        *self == CHK_CHRG_B_A::NO_CHECK
    }
}
#[doc = "Field `CHK_CHRG_B` writer - Check the charger connection"]
pub type CHK_CHRG_B_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, USB1_CHRG_DETECT_SPEC, CHK_CHRG_B_A, O>;
impl<'a, const O: u8> CHK_CHRG_B_W<'a, O> {
    #[doc = "Check whether a charger (either a dedicated charger or a host charger) is connected to USB port."]
    #[inline(always)]
    pub fn check(self) -> &'a mut W {
        self.variant(CHK_CHRG_B_A::CHECK)
    }
    #[doc = "Do not check whether a charger is connected to the USB port."]
    #[inline(always)]
    pub fn no_check(self) -> &'a mut W {
        self.variant(CHK_CHRG_B_A::NO_CHECK)
    }
}
#[doc = "Field `EN_B` reader - Control the charger detector."]
pub type EN_B_R = crate::BitReader<EN_B_A>;
#[doc = "Control the charger detector.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_B_A {
    #[doc = "0: Enable the charger detector."]
    ENABLE = 0,
    #[doc = "1: Disable the charger detector."]
    DISABLE = 1,
}
impl From<EN_B_A> for bool {
    #[inline(always)]
    fn from(variant: EN_B_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_B_A {
        match self.bits {
            false => EN_B_A::ENABLE,
            true => EN_B_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == EN_B_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == EN_B_A::DISABLE
    }
}
#[doc = "Field `EN_B` writer - Control the charger detector."]
pub type EN_B_W<'a, const O: u8> = crate::BitWriter<'a, u32, USB1_CHRG_DETECT_SPEC, EN_B_A, O>;
impl<'a, const O: u8> EN_B_W<'a, O> {
    #[doc = "Enable the charger detector."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(EN_B_A::ENABLE)
    }
    #[doc = "Disable the charger detector."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(EN_B_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 18 - Check the contact of USB plug"]
    #[inline(always)]
    pub fn chk_contact(&self) -> CHK_CONTACT_R {
        CHK_CONTACT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Check the charger connection"]
    #[inline(always)]
    pub fn chk_chrg_b(&self) -> CHK_CHRG_B_R {
        CHK_CHRG_B_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Control the charger detector."]
    #[inline(always)]
    pub fn en_b(&self) -> EN_B_R {
        EN_B_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Check the contact of USB plug"]
    #[inline(always)]
    #[must_use]
    pub fn chk_contact(&mut self) -> CHK_CONTACT_W<18> {
        CHK_CONTACT_W::new(self)
    }
    #[doc = "Bit 19 - Check the charger connection"]
    #[inline(always)]
    #[must_use]
    pub fn chk_chrg_b(&mut self) -> CHK_CHRG_B_W<19> {
        CHK_CHRG_B_W::new(self)
    }
    #[doc = "Bit 20 - Control the charger detector."]
    #[inline(always)]
    #[must_use]
    pub fn en_b(&mut self) -> EN_B_W<20> {
        EN_B_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Charger Detect Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_chrg_detect](index.html) module"]
pub struct USB1_CHRG_DETECT_SPEC;
impl crate::RegisterSpec for USB1_CHRG_DETECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_chrg_detect::R](R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usb1_chrg_detect::W](W) writer structure"]
impl crate::Writable for USB1_CHRG_DETECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB1_CHRG_DETECT to value 0"]
impl crate::Resettable for USB1_CHRG_DETECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
