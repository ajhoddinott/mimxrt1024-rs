#[doc = "Register `USB1_CHRG_DETECT_STAT` reader"]
pub struct R(crate::R<USB1_CHRG_DETECT_STAT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USB1_CHRG_DETECT_STAT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USB1_CHRG_DETECT_STAT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USB1_CHRG_DETECT_STAT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PLUG_CONTACT` reader - State of the USB plug contact detector."]
pub type PLUG_CONTACT_R = crate::BitReader<PLUG_CONTACT_A>;
#[doc = "State of the USB plug contact detector.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLUG_CONTACT_A {
    #[doc = "0: The USB plug has not made contact."]
    NO_CONTACT = 0,
    #[doc = "1: The USB plug has made good contact."]
    GOOD_CONTACT = 1,
}
impl From<PLUG_CONTACT_A> for bool {
    #[inline(always)]
    fn from(variant: PLUG_CONTACT_A) -> Self {
        variant as u8 != 0
    }
}
impl PLUG_CONTACT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLUG_CONTACT_A {
        match self.bits {
            false => PLUG_CONTACT_A::NO_CONTACT,
            true => PLUG_CONTACT_A::GOOD_CONTACT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_CONTACT`"]
    #[inline(always)]
    pub fn is_no_contact(&self) -> bool {
        *self == PLUG_CONTACT_A::NO_CONTACT
    }
    #[doc = "Checks if the value of the field is `GOOD_CONTACT`"]
    #[inline(always)]
    pub fn is_good_contact(&self) -> bool {
        *self == PLUG_CONTACT_A::GOOD_CONTACT
    }
}
#[doc = "Field `CHRG_DETECTED` reader - State of charger detection. This bit is a read only version of the state of the analog signal."]
pub type CHRG_DETECTED_R = crate::BitReader<CHRG_DETECTED_A>;
#[doc = "State of charger detection. This bit is a read only version of the state of the analog signal.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHRG_DETECTED_A {
    #[doc = "0: The USB port is not connected to a charger."]
    CHARGER_NOT_PRESENT = 0,
    #[doc = "1: A charger (either a dedicated charger or a host charger) is connected to the USB port."]
    CHARGER_PRESENT = 1,
}
impl From<CHRG_DETECTED_A> for bool {
    #[inline(always)]
    fn from(variant: CHRG_DETECTED_A) -> Self {
        variant as u8 != 0
    }
}
impl CHRG_DETECTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHRG_DETECTED_A {
        match self.bits {
            false => CHRG_DETECTED_A::CHARGER_NOT_PRESENT,
            true => CHRG_DETECTED_A::CHARGER_PRESENT,
        }
    }
    #[doc = "Checks if the value of the field is `CHARGER_NOT_PRESENT`"]
    #[inline(always)]
    pub fn is_charger_not_present(&self) -> bool {
        *self == CHRG_DETECTED_A::CHARGER_NOT_PRESENT
    }
    #[doc = "Checks if the value of the field is `CHARGER_PRESENT`"]
    #[inline(always)]
    pub fn is_charger_present(&self) -> bool {
        *self == CHRG_DETECTED_A::CHARGER_PRESENT
    }
}
#[doc = "Field `DM_STATE` reader - DM line state output of the charger detector."]
pub type DM_STATE_R = crate::BitReader<bool>;
#[doc = "Field `DP_STATE` reader - DP line state output of the charger detector."]
pub type DP_STATE_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - State of the USB plug contact detector."]
    #[inline(always)]
    pub fn plug_contact(&self) -> PLUG_CONTACT_R {
        PLUG_CONTACT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - State of charger detection. This bit is a read only version of the state of the analog signal."]
    #[inline(always)]
    pub fn chrg_detected(&self) -> CHRG_DETECTED_R {
        CHRG_DETECTED_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DM line state output of the charger detector."]
    #[inline(always)]
    pub fn dm_state(&self) -> DM_STATE_R {
        DM_STATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DP line state output of the charger detector."]
    #[inline(always)]
    pub fn dp_state(&self) -> DP_STATE_R {
        DP_STATE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "USB Charger Detect Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usb1_chrg_detect_stat](index.html) module"]
pub struct USB1_CHRG_DETECT_STAT_SPEC;
impl crate::RegisterSpec for USB1_CHRG_DETECT_STAT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usb1_chrg_detect_stat::R](R) reader structure"]
impl crate::Readable for USB1_CHRG_DETECT_STAT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets USB1_CHRG_DETECT_STAT to value 0"]
impl crate::Resettable for USB1_CHRG_DETECT_STAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
