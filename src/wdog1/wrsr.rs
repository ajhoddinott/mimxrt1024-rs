#[doc = "Register `WRSR` reader"]
pub struct R(crate::R<WRSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WRSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WRSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WRSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SFTW` reader - SFTW"]
pub type SFTW_R = crate::BitReader<SFTW_A>;
#[doc = "SFTW\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SFTW_A {
    #[doc = "0: Reset is not the result of a software reset."]
    SFTW_0 = 0,
    #[doc = "1: Reset is the result of a software reset."]
    SFTW_1 = 1,
}
impl From<SFTW_A> for bool {
    #[inline(always)]
    fn from(variant: SFTW_A) -> Self {
        variant as u8 != 0
    }
}
impl SFTW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SFTW_A {
        match self.bits {
            false => SFTW_A::SFTW_0,
            true => SFTW_A::SFTW_1,
        }
    }
    #[doc = "Checks if the value of the field is `SFTW_0`"]
    #[inline(always)]
    pub fn is_sftw_0(&self) -> bool {
        *self == SFTW_A::SFTW_0
    }
    #[doc = "Checks if the value of the field is `SFTW_1`"]
    #[inline(always)]
    pub fn is_sftw_1(&self) -> bool {
        *self == SFTW_A::SFTW_1
    }
}
#[doc = "Field `TOUT` reader - TOUT"]
pub type TOUT_R = crate::BitReader<TOUT_A>;
#[doc = "TOUT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TOUT_A {
    #[doc = "0: Reset is not the result of a WDOG timeout."]
    TOUT_0 = 0,
    #[doc = "1: Reset is the result of a WDOG timeout."]
    TOUT_1 = 1,
}
impl From<TOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl TOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TOUT_A {
        match self.bits {
            false => TOUT_A::TOUT_0,
            true => TOUT_A::TOUT_1,
        }
    }
    #[doc = "Checks if the value of the field is `TOUT_0`"]
    #[inline(always)]
    pub fn is_tout_0(&self) -> bool {
        *self == TOUT_A::TOUT_0
    }
    #[doc = "Checks if the value of the field is `TOUT_1`"]
    #[inline(always)]
    pub fn is_tout_1(&self) -> bool {
        *self == TOUT_A::TOUT_1
    }
}
#[doc = "Field `POR` reader - POR"]
pub type POR_R = crate::BitReader<POR_A>;
#[doc = "POR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum POR_A {
    #[doc = "0: Reset is not the result of a power on reset."]
    POR_0 = 0,
    #[doc = "1: Reset is the result of a power on reset."]
    POR_1 = 1,
}
impl From<POR_A> for bool {
    #[inline(always)]
    fn from(variant: POR_A) -> Self {
        variant as u8 != 0
    }
}
impl POR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_A {
        match self.bits {
            false => POR_A::POR_0,
            true => POR_A::POR_1,
        }
    }
    #[doc = "Checks if the value of the field is `POR_0`"]
    #[inline(always)]
    pub fn is_por_0(&self) -> bool {
        *self == POR_A::POR_0
    }
    #[doc = "Checks if the value of the field is `POR_1`"]
    #[inline(always)]
    pub fn is_por_1(&self) -> bool {
        *self == POR_A::POR_1
    }
}
impl R {
    #[doc = "Bit 0 - SFTW"]
    #[inline(always)]
    pub fn sftw(&self) -> SFTW_R {
        SFTW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOUT"]
    #[inline(always)]
    pub fn tout(&self) -> TOUT_R {
        TOUT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - POR"]
    #[inline(always)]
    pub fn por(&self) -> POR_R {
        POR_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Watchdog Reset Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wrsr](index.html) module"]
pub struct WRSR_SPEC;
impl crate::RegisterSpec for WRSR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wrsr::R](R) reader structure"]
impl crate::Readable for WRSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets WRSR to value 0"]
impl crate::Resettable for WRSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
