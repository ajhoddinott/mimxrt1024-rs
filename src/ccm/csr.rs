#[doc = "Register `CSR` reader"]
pub struct R(crate::R<CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `REF_EN_B` reader - Status of the value of CCM_REF_EN_B output of ccm"]
pub type REF_EN_B_R = crate::BitReader<REF_EN_B_A>;
#[doc = "Status of the value of CCM_REF_EN_B output of ccm\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REF_EN_B_A {
    #[doc = "0: value of CCM_REF_EN_B is '0'"]
    REF_EN_B_0 = 0,
    #[doc = "1: value of CCM_REF_EN_B is '1'"]
    REF_EN_B_1 = 1,
}
impl From<REF_EN_B_A> for bool {
    #[inline(always)]
    fn from(variant: REF_EN_B_A) -> Self {
        variant as u8 != 0
    }
}
impl REF_EN_B_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_EN_B_A {
        match self.bits {
            false => REF_EN_B_A::REF_EN_B_0,
            true => REF_EN_B_A::REF_EN_B_1,
        }
    }
    #[doc = "Checks if the value of the field is `REF_EN_B_0`"]
    #[inline(always)]
    pub fn is_ref_en_b_0(&self) -> bool {
        *self == REF_EN_B_A::REF_EN_B_0
    }
    #[doc = "Checks if the value of the field is `REF_EN_B_1`"]
    #[inline(always)]
    pub fn is_ref_en_b_1(&self) -> bool {
        *self == REF_EN_B_A::REF_EN_B_1
    }
}
#[doc = "Field `CAMP2_READY` reader - Status indication of CAMP2."]
pub type CAMP2_READY_R = crate::BitReader<CAMP2_READY_A>;
#[doc = "Status indication of CAMP2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CAMP2_READY_A {
    #[doc = "0: CAMP2 is not ready."]
    CAMP2_READY_0 = 0,
    #[doc = "1: CAMP2 is ready."]
    CAMP2_READY_1 = 1,
}
impl From<CAMP2_READY_A> for bool {
    #[inline(always)]
    fn from(variant: CAMP2_READY_A) -> Self {
        variant as u8 != 0
    }
}
impl CAMP2_READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CAMP2_READY_A {
        match self.bits {
            false => CAMP2_READY_A::CAMP2_READY_0,
            true => CAMP2_READY_A::CAMP2_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `CAMP2_READY_0`"]
    #[inline(always)]
    pub fn is_camp2_ready_0(&self) -> bool {
        *self == CAMP2_READY_A::CAMP2_READY_0
    }
    #[doc = "Checks if the value of the field is `CAMP2_READY_1`"]
    #[inline(always)]
    pub fn is_camp2_ready_1(&self) -> bool {
        *self == CAMP2_READY_A::CAMP2_READY_1
    }
}
#[doc = "Field `COSC_READY` reader - Status indication of on board oscillator"]
pub type COSC_READY_R = crate::BitReader<COSC_READY_A>;
#[doc = "Status indication of on board oscillator\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum COSC_READY_A {
    #[doc = "0: on board oscillator is not ready."]
    COSC_READY_0 = 0,
    #[doc = "1: on board oscillator is ready."]
    COSC_READY_1 = 1,
}
impl From<COSC_READY_A> for bool {
    #[inline(always)]
    fn from(variant: COSC_READY_A) -> Self {
        variant as u8 != 0
    }
}
impl COSC_READY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> COSC_READY_A {
        match self.bits {
            false => COSC_READY_A::COSC_READY_0,
            true => COSC_READY_A::COSC_READY_1,
        }
    }
    #[doc = "Checks if the value of the field is `COSC_READY_0`"]
    #[inline(always)]
    pub fn is_cosc_ready_0(&self) -> bool {
        *self == COSC_READY_A::COSC_READY_0
    }
    #[doc = "Checks if the value of the field is `COSC_READY_1`"]
    #[inline(always)]
    pub fn is_cosc_ready_1(&self) -> bool {
        *self == COSC_READY_A::COSC_READY_1
    }
}
impl R {
    #[doc = "Bit 0 - Status of the value of CCM_REF_EN_B output of ccm"]
    #[inline(always)]
    pub fn ref_en_b(&self) -> REF_EN_B_R {
        REF_EN_B_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Status indication of CAMP2."]
    #[inline(always)]
    pub fn camp2_ready(&self) -> CAMP2_READY_R {
        CAMP2_READY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - Status indication of on board oscillator"]
    #[inline(always)]
    pub fn cosc_ready(&self) -> COSC_READY_R {
        COSC_READY_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[doc = "CCM Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csr](index.html) module"]
pub struct CSR_SPEC;
impl crate::RegisterSpec for CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [csr::R](R) reader structure"]
impl crate::Readable for CSR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CSR to value 0x10"]
impl crate::Resettable for CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x10;
}
