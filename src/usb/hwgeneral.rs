#[doc = "Register `HWGENERAL` reader"]
pub struct R(crate::R<HWGENERAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HWGENERAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HWGENERAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HWGENERAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `PHYW` reader - Data width of the transciever connected to the controller core. PHYW bit reset value is"]
pub type PHYW_R = crate::FieldReader<u8, PHYW_A>;
#[doc = "Data width of the transciever connected to the controller core. PHYW bit reset value is\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PHYW_A {
    #[doc = "0: 8 bit wide data bus Software non-programmable"]
    PHYW_0 = 0,
    #[doc = "1: 16 bit wide data bus Software non-programmable"]
    PHYW_1 = 1,
    #[doc = "2: Reset to 8 bit wide data bus Software programmable"]
    PHYW_2 = 2,
    #[doc = "3: Reset to 16 bit wide data bus Software programmable"]
    PHYW_3 = 3,
}
impl From<PHYW_A> for u8 {
    #[inline(always)]
    fn from(variant: PHYW_A) -> Self {
        variant as _
    }
}
impl PHYW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYW_A {
        match self.bits {
            0 => PHYW_A::PHYW_0,
            1 => PHYW_A::PHYW_1,
            2 => PHYW_A::PHYW_2,
            3 => PHYW_A::PHYW_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHYW_0`"]
    #[inline(always)]
    pub fn is_phyw_0(&self) -> bool {
        *self == PHYW_A::PHYW_0
    }
    #[doc = "Checks if the value of the field is `PHYW_1`"]
    #[inline(always)]
    pub fn is_phyw_1(&self) -> bool {
        *self == PHYW_A::PHYW_1
    }
    #[doc = "Checks if the value of the field is `PHYW_2`"]
    #[inline(always)]
    pub fn is_phyw_2(&self) -> bool {
        *self == PHYW_A::PHYW_2
    }
    #[doc = "Checks if the value of the field is `PHYW_3`"]
    #[inline(always)]
    pub fn is_phyw_3(&self) -> bool {
        *self == PHYW_A::PHYW_3
    }
}
#[doc = "Field `PHYM` reader - Transciever type"]
pub type PHYM_R = crate::FieldReader<u8, PHYM_A>;
#[doc = "Transciever type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PHYM_A {
    #[doc = "0: UTMI/UMTI+"]
    PHYM_0 = 0,
    #[doc = "1: ULPI DDR"]
    PHYM_1 = 1,
    #[doc = "2: ULPI"]
    PHYM_2 = 2,
    #[doc = "3: Serial Only"]
    PHYM_3 = 3,
    #[doc = "4: Software programmable - reset to UTMI/UTMI+"]
    PHYM_4 = 4,
    #[doc = "5: Software programmable - reset to ULPI DDR"]
    PHYM_5 = 5,
    #[doc = "6: Software programmable - reset to ULPI"]
    PHYM_6 = 6,
    #[doc = "7: Software programmable - reset to Serial"]
    PHYM_7 = 7,
}
impl From<PHYM_A> for u8 {
    #[inline(always)]
    fn from(variant: PHYM_A) -> Self {
        variant as _
    }
}
impl PHYM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PHYM_A {
        match self.bits {
            0 => PHYM_A::PHYM_0,
            1 => PHYM_A::PHYM_1,
            2 => PHYM_A::PHYM_2,
            3 => PHYM_A::PHYM_3,
            4 => PHYM_A::PHYM_4,
            5 => PHYM_A::PHYM_5,
            6 => PHYM_A::PHYM_6,
            7 => PHYM_A::PHYM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PHYM_0`"]
    #[inline(always)]
    pub fn is_phym_0(&self) -> bool {
        *self == PHYM_A::PHYM_0
    }
    #[doc = "Checks if the value of the field is `PHYM_1`"]
    #[inline(always)]
    pub fn is_phym_1(&self) -> bool {
        *self == PHYM_A::PHYM_1
    }
    #[doc = "Checks if the value of the field is `PHYM_2`"]
    #[inline(always)]
    pub fn is_phym_2(&self) -> bool {
        *self == PHYM_A::PHYM_2
    }
    #[doc = "Checks if the value of the field is `PHYM_3`"]
    #[inline(always)]
    pub fn is_phym_3(&self) -> bool {
        *self == PHYM_A::PHYM_3
    }
    #[doc = "Checks if the value of the field is `PHYM_4`"]
    #[inline(always)]
    pub fn is_phym_4(&self) -> bool {
        *self == PHYM_A::PHYM_4
    }
    #[doc = "Checks if the value of the field is `PHYM_5`"]
    #[inline(always)]
    pub fn is_phym_5(&self) -> bool {
        *self == PHYM_A::PHYM_5
    }
    #[doc = "Checks if the value of the field is `PHYM_6`"]
    #[inline(always)]
    pub fn is_phym_6(&self) -> bool {
        *self == PHYM_A::PHYM_6
    }
    #[doc = "Checks if the value of the field is `PHYM_7`"]
    #[inline(always)]
    pub fn is_phym_7(&self) -> bool {
        *self == PHYM_A::PHYM_7
    }
}
#[doc = "Field `SM` reader - Serial interface mode capability"]
pub type SM_R = crate::FieldReader<u8, SM_A>;
#[doc = "Serial interface mode capability\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SM_A {
    #[doc = "0: No Serial Engine, always use parallel signalling."]
    SM_0 = 0,
    #[doc = "1: Serial Engine present, always use serial signalling for FS/LS."]
    SM_1 = 1,
    #[doc = "2: Software programmable - Reset to use parallel signalling for FS/LS"]
    SM_2 = 2,
    #[doc = "3: Software programmable - Reset to use serial signalling for FS/LS"]
    SM_3 = 3,
}
impl From<SM_A> for u8 {
    #[inline(always)]
    fn from(variant: SM_A) -> Self {
        variant as _
    }
}
impl SM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SM_A {
        match self.bits {
            0 => SM_A::SM_0,
            1 => SM_A::SM_1,
            2 => SM_A::SM_2,
            3 => SM_A::SM_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SM_0`"]
    #[inline(always)]
    pub fn is_sm_0(&self) -> bool {
        *self == SM_A::SM_0
    }
    #[doc = "Checks if the value of the field is `SM_1`"]
    #[inline(always)]
    pub fn is_sm_1(&self) -> bool {
        *self == SM_A::SM_1
    }
    #[doc = "Checks if the value of the field is `SM_2`"]
    #[inline(always)]
    pub fn is_sm_2(&self) -> bool {
        *self == SM_A::SM_2
    }
    #[doc = "Checks if the value of the field is `SM_3`"]
    #[inline(always)]
    pub fn is_sm_3(&self) -> bool {
        *self == SM_A::SM_3
    }
}
impl R {
    #[doc = "Bits 4:5 - Data width of the transciever connected to the controller core. PHYW bit reset value is"]
    #[inline(always)]
    pub fn phyw(&self) -> PHYW_R {
        PHYW_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:8 - Transciever type"]
    #[inline(always)]
    pub fn phym(&self) -> PHYM_R {
        PHYM_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:10 - Serial interface mode capability"]
    #[inline(always)]
    pub fn sm(&self) -> SM_R {
        SM_R::new(((self.bits >> 9) & 3) as u8)
    }
}
#[doc = "Hardware General\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hwgeneral](index.html) module"]
pub struct HWGENERAL_SPEC;
impl crate::RegisterSpec for HWGENERAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hwgeneral::R](R) reader structure"]
impl crate::Readable for HWGENERAL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HWGENERAL to value 0x35"]
impl crate::Resettable for HWGENERAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x35;
}
