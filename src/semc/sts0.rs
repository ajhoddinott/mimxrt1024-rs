#[doc = "Register `STS0` reader"]
pub struct R(crate::R<STS0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STS0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STS0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STS0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `IDLE` reader - Indicating whether the SEMC is in idle state."]
pub type IDLE_R = crate::BitReader<bool>;
#[doc = "Field `NARDY` reader - Indicating NAND device Ready/WAIT# pin level."]
pub type NARDY_R = crate::BitReader<NARDY_A>;
#[doc = "Indicating NAND device Ready/WAIT# pin level.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NARDY_A {
    #[doc = "0: NAND device is not ready"]
    NARDY_0 = 0,
    #[doc = "1: NAND device is ready"]
    NARDY_1 = 1,
}
impl From<NARDY_A> for bool {
    #[inline(always)]
    fn from(variant: NARDY_A) -> Self {
        variant as u8 != 0
    }
}
impl NARDY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NARDY_A {
        match self.bits {
            false => NARDY_A::NARDY_0,
            true => NARDY_A::NARDY_1,
        }
    }
    #[doc = "Checks if the value of the field is `NARDY_0`"]
    #[inline(always)]
    pub fn is_nardy_0(&self) -> bool {
        *self == NARDY_A::NARDY_0
    }
    #[doc = "Checks if the value of the field is `NARDY_1`"]
    #[inline(always)]
    pub fn is_nardy_1(&self) -> bool {
        *self == NARDY_A::NARDY_1
    }
}
impl R {
    #[doc = "Bit 0 - Indicating whether the SEMC is in idle state."]
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicating NAND device Ready/WAIT# pin level."]
    #[inline(always)]
    pub fn nardy(&self) -> NARDY_R {
        NARDY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "Status Register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sts0](index.html) module"]
pub struct STS0_SPEC;
impl crate::RegisterSpec for STS0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sts0::R](R) reader structure"]
impl crate::Readable for STS0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STS0 to value 0x01"]
impl crate::Resettable for STS0_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
