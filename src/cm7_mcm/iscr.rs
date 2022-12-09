#[doc = "Register `ISCR` reader"]
pub struct R(crate::R<ISCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISCR` writer"]
pub struct W(crate::W<ISCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISCR_SPEC>;
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
impl From<crate::W<ISCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WABS` reader - Write Abort on Slave"]
pub type WABS_R = crate::BitReader<WABS_A>;
#[doc = "Write Abort on Slave\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WABS_A {
    #[doc = "0: No abort"]
    NOABORT = 0,
    #[doc = "1: Abort"]
    ABORT = 1,
}
impl From<WABS_A> for bool {
    #[inline(always)]
    fn from(variant: WABS_A) -> Self {
        variant as u8 != 0
    }
}
impl WABS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WABS_A {
        match self.bits {
            false => WABS_A::NOABORT,
            true => WABS_A::ABORT,
        }
    }
    #[doc = "Checks if the value of the field is `NOABORT`"]
    #[inline(always)]
    pub fn is_noabort(&self) -> bool {
        *self == WABS_A::NOABORT
    }
    #[doc = "Checks if the value of the field is `ABORT`"]
    #[inline(always)]
    pub fn is_abort(&self) -> bool {
        *self == WABS_A::ABORT
    }
}
#[doc = "Field `WABS` writer - Write Abort on Slave"]
pub type WABS_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, ISCR_SPEC, WABS_A, O>;
impl<'a, const O: u8> WABS_W<'a, O> {
    #[doc = "No abort"]
    #[inline(always)]
    pub fn noabort(self) -> &'a mut W {
        self.variant(WABS_A::NOABORT)
    }
    #[doc = "Abort"]
    #[inline(always)]
    pub fn abort(self) -> &'a mut W {
        self.variant(WABS_A::ABORT)
    }
}
#[doc = "Field `WABSO` reader - Write Abort on Slave Overrun"]
pub type WABSO_R = crate::BitReader<WABSO_A>;
#[doc = "Write Abort on Slave Overrun\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WABSO_A {
    #[doc = "0: No write abort overrun"]
    NO = 0,
    #[doc = "1: Write abort overrun occurred"]
    YES = 1,
}
impl From<WABSO_A> for bool {
    #[inline(always)]
    fn from(variant: WABSO_A) -> Self {
        variant as u8 != 0
    }
}
impl WABSO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WABSO_A {
        match self.bits {
            false => WABSO_A::NO,
            true => WABSO_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == WABSO_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == WABSO_A::YES
    }
}
#[doc = "Field `FIOC` reader - FPU Invalid Operation interrupt Status"]
pub type FIOC_R = crate::BitReader<FIOC_A>;
#[doc = "FPU Invalid Operation interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIOC_A {
    #[doc = "0: No interrupt"]
    NO = 0,
    #[doc = "1: Interrupt occured"]
    YES = 1,
}
impl From<FIOC_A> for bool {
    #[inline(always)]
    fn from(variant: FIOC_A) -> Self {
        variant as u8 != 0
    }
}
impl FIOC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIOC_A {
        match self.bits {
            false => FIOC_A::NO,
            true => FIOC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == FIOC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == FIOC_A::YES
    }
}
#[doc = "Field `FDZC` reader - FPU Divide-by-Zero Interrupt Status"]
pub type FDZC_R = crate::BitReader<FDZC_A>;
#[doc = "FPU Divide-by-Zero Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDZC_A {
    #[doc = "0: No interrupt"]
    NO = 0,
    #[doc = "1: Interrupt occured"]
    YES = 1,
}
impl From<FDZC_A> for bool {
    #[inline(always)]
    fn from(variant: FDZC_A) -> Self {
        variant as u8 != 0
    }
}
impl FDZC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDZC_A {
        match self.bits {
            false => FDZC_A::NO,
            true => FDZC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == FDZC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == FDZC_A::YES
    }
}
#[doc = "Field `FOFC` reader - FPU Overflow interrupt status"]
pub type FOFC_R = crate::BitReader<FOFC_A>;
#[doc = "FPU Overflow interrupt status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFC_A {
    #[doc = "0: No interrupt"]
    NO = 0,
    #[doc = "1: Interrupt occured"]
    YES = 1,
}
impl From<FOFC_A> for bool {
    #[inline(always)]
    fn from(variant: FOFC_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFC_A {
        match self.bits {
            false => FOFC_A::NO,
            true => FOFC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == FOFC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == FOFC_A::YES
    }
}
#[doc = "Field `FUFC` reader - FPU Underflow Interrupt Status"]
pub type FUFC_R = crate::BitReader<FUFC_A>;
#[doc = "FPU Underflow Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUFC_A {
    #[doc = "0: No interrupt"]
    NO = 0,
    #[doc = "1: Interrupt occured"]
    YES = 1,
}
impl From<FUFC_A> for bool {
    #[inline(always)]
    fn from(variant: FUFC_A) -> Self {
        variant as u8 != 0
    }
}
impl FUFC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUFC_A {
        match self.bits {
            false => FUFC_A::NO,
            true => FUFC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == FUFC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == FUFC_A::YES
    }
}
#[doc = "Field `FIXC` reader - FPU Inexact Interrupt Status"]
pub type FIXC_R = crate::BitReader<FIXC_A>;
#[doc = "FPU Inexact Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXC_A {
    #[doc = "0: No interrupt"]
    NO = 0,
    #[doc = "1: Interrupt occured"]
    YES = 1,
}
impl From<FIXC_A> for bool {
    #[inline(always)]
    fn from(variant: FIXC_A) -> Self {
        variant as u8 != 0
    }
}
impl FIXC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXC_A {
        match self.bits {
            false => FIXC_A::NO,
            true => FIXC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == FIXC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == FIXC_A::YES
    }
}
#[doc = "Field `FIDC` reader - FPU Input Denormal Interrupt Status"]
pub type FIDC_R = crate::BitReader<FIDC_A>;
#[doc = "FPU Input Denormal Interrupt Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIDC_A {
    #[doc = "0: No interrupt"]
    NO = 0,
    #[doc = "1: Interrupt occured"]
    YES = 1,
}
impl From<FIDC_A> for bool {
    #[inline(always)]
    fn from(variant: FIDC_A) -> Self {
        variant as u8 != 0
    }
}
impl FIDC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIDC_A {
        match self.bits {
            false => FIDC_A::NO,
            true => FIDC_A::YES,
        }
    }
    #[doc = "Checks if the value of the field is `NO`"]
    #[inline(always)]
    pub fn is_no(&self) -> bool {
        *self == FIDC_A::NO
    }
    #[doc = "Checks if the value of the field is `YES`"]
    #[inline(always)]
    pub fn is_yes(&self) -> bool {
        *self == FIDC_A::YES
    }
}
#[doc = "Field `WABE` reader - TCM Write Abort Interrupt enable"]
pub type WABE_R = crate::BitReader<WABE_A>;
#[doc = "TCM Write Abort Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WABE_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<WABE_A> for bool {
    #[inline(always)]
    fn from(variant: WABE_A) -> Self {
        variant as u8 != 0
    }
}
impl WABE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WABE_A {
        match self.bits {
            false => WABE_A::DISABLE,
            true => WABE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == WABE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == WABE_A::ENABLE
    }
}
#[doc = "Field `WABE` writer - TCM Write Abort Interrupt enable"]
pub type WABE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, WABE_A, O>;
impl<'a, const O: u8> WABE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(WABE_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(WABE_A::ENABLE)
    }
}
#[doc = "Field `FIOCE` reader - FPU Invalid Operation Interrupt Enable"]
pub type FIOCE_R = crate::BitReader<FIOCE_A>;
#[doc = "FPU Invalid Operation Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIOCE_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<FIOCE_A> for bool {
    #[inline(always)]
    fn from(variant: FIOCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIOCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIOCE_A {
        match self.bits {
            false => FIOCE_A::DISABLE,
            true => FIOCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIOCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIOCE_A::ENABLE
    }
}
#[doc = "Field `FIOCE` writer - FPU Invalid Operation Interrupt Enable"]
pub type FIOCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FIOCE_A, O>;
impl<'a, const O: u8> FIOCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIOCE_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIOCE_A::ENABLE)
    }
}
#[doc = "Field `FDZCE` reader - FPU Divide-by-Zero Interrupt Enable"]
pub type FDZCE_R = crate::BitReader<FDZCE_A>;
#[doc = "FPU Divide-by-Zero Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FDZCE_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<FDZCE_A> for bool {
    #[inline(always)]
    fn from(variant: FDZCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FDZCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FDZCE_A {
        match self.bits {
            false => FDZCE_A::DISABLE,
            true => FDZCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FDZCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FDZCE_A::ENABLE
    }
}
#[doc = "Field `FDZCE` writer - FPU Divide-by-Zero Interrupt Enable"]
pub type FDZCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FDZCE_A, O>;
impl<'a, const O: u8> FDZCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FDZCE_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FDZCE_A::ENABLE)
    }
}
#[doc = "Field `FOFCE` reader - FPU Overflow Interrupt Enable"]
pub type FOFCE_R = crate::BitReader<FOFCE_A>;
#[doc = "FPU Overflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOFCE_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<FOFCE_A> for bool {
    #[inline(always)]
    fn from(variant: FOFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FOFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOFCE_A {
        match self.bits {
            false => FOFCE_A::DISABLE,
            true => FOFCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FOFCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FOFCE_A::ENABLE
    }
}
#[doc = "Field `FOFCE` writer - FPU Overflow Interrupt Enable"]
pub type FOFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FOFCE_A, O>;
impl<'a, const O: u8> FOFCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FOFCE_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FOFCE_A::ENABLE)
    }
}
#[doc = "Field `FUFCE` reader - FPU Underflow Interrupt Enable"]
pub type FUFCE_R = crate::BitReader<FUFCE_A>;
#[doc = "FPU Underflow Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FUFCE_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<FUFCE_A> for bool {
    #[inline(always)]
    fn from(variant: FUFCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FUFCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FUFCE_A {
        match self.bits {
            false => FUFCE_A::DISABLE,
            true => FUFCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FUFCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FUFCE_A::ENABLE
    }
}
#[doc = "Field `FUFCE` writer - FPU Underflow Interrupt Enable"]
pub type FUFCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FUFCE_A, O>;
impl<'a, const O: u8> FUFCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FUFCE_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FUFCE_A::ENABLE)
    }
}
#[doc = "Field `FIXCE` reader - FPU Inexact Interrupt Enable"]
pub type FIXCE_R = crate::BitReader<FIXCE_A>;
#[doc = "FPU Inexact Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIXCE_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<FIXCE_A> for bool {
    #[inline(always)]
    fn from(variant: FIXCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIXCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIXCE_A {
        match self.bits {
            false => FIXCE_A::DISABLE,
            true => FIXCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIXCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIXCE_A::ENABLE
    }
}
#[doc = "Field `FIXCE` writer - FPU Inexact Interrupt Enable"]
pub type FIXCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FIXCE_A, O>;
impl<'a, const O: u8> FIXCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIXCE_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIXCE_A::ENABLE)
    }
}
#[doc = "Field `FIDCE` reader - FPU Input Denormal Interrupt Enable"]
pub type FIDCE_R = crate::BitReader<FIDCE_A>;
#[doc = "FPU Input Denormal Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FIDCE_A {
    #[doc = "0: Disable interrupt"]
    DISABLE = 0,
    #[doc = "1: Enable interrupt"]
    ENABLE = 1,
}
impl From<FIDCE_A> for bool {
    #[inline(always)]
    fn from(variant: FIDCE_A) -> Self {
        variant as u8 != 0
    }
}
impl FIDCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FIDCE_A {
        match self.bits {
            false => FIDCE_A::DISABLE,
            true => FIDCE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FIDCE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FIDCE_A::ENABLE
    }
}
#[doc = "Field `FIDCE` writer - FPU Input Denormal Interrupt Enable"]
pub type FIDCE_W<'a, const O: u8> = crate::BitWriter<'a, u32, ISCR_SPEC, FIDCE_A, O>;
impl<'a, const O: u8> FIDCE_W<'a, O> {
    #[doc = "Disable interrupt"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FIDCE_A::DISABLE)
    }
    #[doc = "Enable interrupt"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FIDCE_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 5 - Write Abort on Slave"]
    #[inline(always)]
    pub fn wabs(&self) -> WABS_R {
        WABS_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write Abort on Slave Overrun"]
    #[inline(always)]
    pub fn wabso(&self) -> WABSO_R {
        WABSO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - FPU Invalid Operation interrupt Status"]
    #[inline(always)]
    pub fn fioc(&self) -> FIOC_R {
        FIOC_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FPU Divide-by-Zero Interrupt Status"]
    #[inline(always)]
    pub fn fdzc(&self) -> FDZC_R {
        FDZC_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - FPU Overflow interrupt status"]
    #[inline(always)]
    pub fn fofc(&self) -> FOFC_R {
        FOFC_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - FPU Underflow Interrupt Status"]
    #[inline(always)]
    pub fn fufc(&self) -> FUFC_R {
        FUFC_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - FPU Inexact Interrupt Status"]
    #[inline(always)]
    pub fn fixc(&self) -> FIXC_R {
        FIXC_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - FPU Input Denormal Interrupt Status"]
    #[inline(always)]
    pub fn fidc(&self) -> FIDC_R {
        FIDC_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - TCM Write Abort Interrupt enable"]
    #[inline(always)]
    pub fn wabe(&self) -> WABE_R {
        WABE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - FPU Invalid Operation Interrupt Enable"]
    #[inline(always)]
    pub fn fioce(&self) -> FIOCE_R {
        FIOCE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - FPU Divide-by-Zero Interrupt Enable"]
    #[inline(always)]
    pub fn fdzce(&self) -> FDZCE_R {
        FDZCE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - FPU Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn fofce(&self) -> FOFCE_R {
        FOFCE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - FPU Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn fufce(&self) -> FUFCE_R {
        FUFCE_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - FPU Inexact Interrupt Enable"]
    #[inline(always)]
    pub fn fixce(&self) -> FIXCE_R {
        FIXCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 31 - FPU Input Denormal Interrupt Enable"]
    #[inline(always)]
    pub fn fidce(&self) -> FIDCE_R {
        FIDCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Write Abort on Slave"]
    #[inline(always)]
    #[must_use]
    pub fn wabs(&mut self) -> WABS_W<5> {
        WABS_W::new(self)
    }
    #[doc = "Bit 21 - TCM Write Abort Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wabe(&mut self) -> WABE_W<21> {
        WABE_W::new(self)
    }
    #[doc = "Bit 24 - FPU Invalid Operation Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fioce(&mut self) -> FIOCE_W<24> {
        FIOCE_W::new(self)
    }
    #[doc = "Bit 25 - FPU Divide-by-Zero Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fdzce(&mut self) -> FDZCE_W<25> {
        FDZCE_W::new(self)
    }
    #[doc = "Bit 26 - FPU Overflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fofce(&mut self) -> FOFCE_W<26> {
        FOFCE_W::new(self)
    }
    #[doc = "Bit 27 - FPU Underflow Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fufce(&mut self) -> FUFCE_W<27> {
        FUFCE_W::new(self)
    }
    #[doc = "Bit 28 - FPU Inexact Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fixce(&mut self) -> FIXCE_W<28> {
        FIXCE_W::new(self)
    }
    #[doc = "Bit 31 - FPU Input Denormal Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn fidce(&mut self) -> FIDCE_W<31> {
        FIDCE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Status and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iscr](index.html) module"]
pub struct ISCR_SPEC;
impl crate::RegisterSpec for ISCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iscr::R](R) reader structure"]
impl crate::Readable for ISCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iscr::W](W) writer structure"]
impl crate::Writable for ISCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x20;
}
#[doc = "`reset()` method sets ISCR to value 0"]
impl crate::Resettable for ISCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
