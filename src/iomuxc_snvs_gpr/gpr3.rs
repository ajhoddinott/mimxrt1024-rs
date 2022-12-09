#[doc = "Register `GPR3` reader"]
pub struct R(crate::R<GPR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR3` writer"]
pub struct W(crate::W<GPR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR3_SPEC>;
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
impl From<crate::W<GPR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPSR_MODE_ENABLE` reader - Set to enable LPSR mode."]
pub type LPSR_MODE_ENABLE_R = crate::BitReader<LPSR_MODE_ENABLE_A>;
#[doc = "Set to enable LPSR mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LPSR_MODE_ENABLE_A {
    #[doc = "0: SNVS domain will reset when system reset happens"]
    LPSR_MODE_ENABLE_0 = 0,
    #[doc = "1: SNVS domain will only reset with SNVS POR"]
    LPSR_MODE_ENABLE_1 = 1,
}
impl From<LPSR_MODE_ENABLE_A> for bool {
    #[inline(always)]
    fn from(variant: LPSR_MODE_ENABLE_A) -> Self {
        variant as u8 != 0
    }
}
impl LPSR_MODE_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LPSR_MODE_ENABLE_A {
        match self.bits {
            false => LPSR_MODE_ENABLE_A::LPSR_MODE_ENABLE_0,
            true => LPSR_MODE_ENABLE_A::LPSR_MODE_ENABLE_1,
        }
    }
    #[doc = "Checks if the value of the field is `LPSR_MODE_ENABLE_0`"]
    #[inline(always)]
    pub fn is_lpsr_mode_enable_0(&self) -> bool {
        *self == LPSR_MODE_ENABLE_A::LPSR_MODE_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `LPSR_MODE_ENABLE_1`"]
    #[inline(always)]
    pub fn is_lpsr_mode_enable_1(&self) -> bool {
        *self == LPSR_MODE_ENABLE_A::LPSR_MODE_ENABLE_1
    }
}
#[doc = "Field `LPSR_MODE_ENABLE` writer - Set to enable LPSR mode."]
pub type LPSR_MODE_ENABLE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR3_SPEC, LPSR_MODE_ENABLE_A, O>;
impl<'a, const O: u8> LPSR_MODE_ENABLE_W<'a, O> {
    #[doc = "SNVS domain will reset when system reset happens"]
    #[inline(always)]
    pub fn lpsr_mode_enable_0(self) -> &'a mut W {
        self.variant(LPSR_MODE_ENABLE_A::LPSR_MODE_ENABLE_0)
    }
    #[doc = "SNVS domain will only reset with SNVS POR"]
    #[inline(always)]
    pub fn lpsr_mode_enable_1(self) -> &'a mut W {
        self.variant(LPSR_MODE_ENABLE_A::LPSR_MODE_ENABLE_1)
    }
}
#[doc = "Field `DCDC_STATUS_CAPT_CLR` reader - DCDC captured status clear"]
pub type DCDC_STATUS_CAPT_CLR_R = crate::BitReader<bool>;
#[doc = "Field `DCDC_STATUS_CAPT_CLR` writer - DCDC captured status clear"]
pub type DCDC_STATUS_CAPT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR3_SPEC, bool, O>;
#[doc = "Field `POR_PULL_TYPE` reader - POR_B pad control"]
pub type POR_PULL_TYPE_R = crate::FieldReader<u8, POR_PULL_TYPE_A>;
#[doc = "POR_B pad control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POR_PULL_TYPE_A {
    #[doc = "0: 100 Ohm pull up enabled for POR_B always"]
    POR_PULL_TYPE_0 = 0,
    #[doc = "1: Disable pull in SNVS mode, 100 Ohm pull up enabled otherwise"]
    POR_PULL_TYPE_1 = 1,
    #[doc = "2: Disable pull of POR_B always"]
    POR_PULL_TYPE_2 = 2,
    #[doc = "3: 100 Ohm pull down enabled in SNVS mode, 100 Ohm pull up enabled otherwise"]
    POR_PULL_TYPE_3 = 3,
}
impl From<POR_PULL_TYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: POR_PULL_TYPE_A) -> Self {
        variant as _
    }
}
impl POR_PULL_TYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POR_PULL_TYPE_A {
        match self.bits {
            0 => POR_PULL_TYPE_A::POR_PULL_TYPE_0,
            1 => POR_PULL_TYPE_A::POR_PULL_TYPE_1,
            2 => POR_PULL_TYPE_A::POR_PULL_TYPE_2,
            3 => POR_PULL_TYPE_A::POR_PULL_TYPE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POR_PULL_TYPE_0`"]
    #[inline(always)]
    pub fn is_por_pull_type_0(&self) -> bool {
        *self == POR_PULL_TYPE_A::POR_PULL_TYPE_0
    }
    #[doc = "Checks if the value of the field is `POR_PULL_TYPE_1`"]
    #[inline(always)]
    pub fn is_por_pull_type_1(&self) -> bool {
        *self == POR_PULL_TYPE_A::POR_PULL_TYPE_1
    }
    #[doc = "Checks if the value of the field is `POR_PULL_TYPE_2`"]
    #[inline(always)]
    pub fn is_por_pull_type_2(&self) -> bool {
        *self == POR_PULL_TYPE_A::POR_PULL_TYPE_2
    }
    #[doc = "Checks if the value of the field is `POR_PULL_TYPE_3`"]
    #[inline(always)]
    pub fn is_por_pull_type_3(&self) -> bool {
        *self == POR_PULL_TYPE_A::POR_PULL_TYPE_3
    }
}
#[doc = "Field `POR_PULL_TYPE` writer - POR_B pad control"]
pub type POR_PULL_TYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPR3_SPEC, u8, POR_PULL_TYPE_A, 2, O>;
impl<'a, const O: u8> POR_PULL_TYPE_W<'a, O> {
    #[doc = "100 Ohm pull up enabled for POR_B always"]
    #[inline(always)]
    pub fn por_pull_type_0(self) -> &'a mut W {
        self.variant(POR_PULL_TYPE_A::POR_PULL_TYPE_0)
    }
    #[doc = "Disable pull in SNVS mode, 100 Ohm pull up enabled otherwise"]
    #[inline(always)]
    pub fn por_pull_type_1(self) -> &'a mut W {
        self.variant(POR_PULL_TYPE_A::POR_PULL_TYPE_1)
    }
    #[doc = "Disable pull of POR_B always"]
    #[inline(always)]
    pub fn por_pull_type_2(self) -> &'a mut W {
        self.variant(POR_PULL_TYPE_A::POR_PULL_TYPE_2)
    }
    #[doc = "100 Ohm pull down enabled in SNVS mode, 100 Ohm pull up enabled otherwise"]
    #[inline(always)]
    pub fn por_pull_type_3(self) -> &'a mut W {
        self.variant(POR_PULL_TYPE_A::POR_PULL_TYPE_3)
    }
}
#[doc = "Field `DCDC_IN_LOW_VOL` reader - DCDC_IN low voltage detect."]
pub type DCDC_IN_LOW_VOL_R = crate::BitReader<DCDC_IN_LOW_VOL_A>;
#[doc = "DCDC_IN low voltage detect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDC_IN_LOW_VOL_A {
    #[doc = "0: DCDC_IN is ok"]
    DCDC_IN_LOW_VOL_0 = 0,
    #[doc = "1: DCDC_IN is too low"]
    DCDC_IN_LOW_VOL_1 = 1,
}
impl From<DCDC_IN_LOW_VOL_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_IN_LOW_VOL_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDC_IN_LOW_VOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_IN_LOW_VOL_A {
        match self.bits {
            false => DCDC_IN_LOW_VOL_A::DCDC_IN_LOW_VOL_0,
            true => DCDC_IN_LOW_VOL_A::DCDC_IN_LOW_VOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCDC_IN_LOW_VOL_0`"]
    #[inline(always)]
    pub fn is_dcdc_in_low_vol_0(&self) -> bool {
        *self == DCDC_IN_LOW_VOL_A::DCDC_IN_LOW_VOL_0
    }
    #[doc = "Checks if the value of the field is `DCDC_IN_LOW_VOL_1`"]
    #[inline(always)]
    pub fn is_dcdc_in_low_vol_1(&self) -> bool {
        *self == DCDC_IN_LOW_VOL_A::DCDC_IN_LOW_VOL_1
    }
}
#[doc = "Field `DCDC_OVER_CUR` reader - DCDC output over current alert"]
pub type DCDC_OVER_CUR_R = crate::BitReader<DCDC_OVER_CUR_A>;
#[doc = "DCDC output over current alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDC_OVER_CUR_A {
    #[doc = "0: No over current detected"]
    DCDC_OVER_CUR_0 = 0,
    #[doc = "1: Over current detected"]
    DCDC_OVER_CUR_1 = 1,
}
impl From<DCDC_OVER_CUR_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_OVER_CUR_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDC_OVER_CUR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_OVER_CUR_A {
        match self.bits {
            false => DCDC_OVER_CUR_A::DCDC_OVER_CUR_0,
            true => DCDC_OVER_CUR_A::DCDC_OVER_CUR_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCDC_OVER_CUR_0`"]
    #[inline(always)]
    pub fn is_dcdc_over_cur_0(&self) -> bool {
        *self == DCDC_OVER_CUR_A::DCDC_OVER_CUR_0
    }
    #[doc = "Checks if the value of the field is `DCDC_OVER_CUR_1`"]
    #[inline(always)]
    pub fn is_dcdc_over_cur_1(&self) -> bool {
        *self == DCDC_OVER_CUR_A::DCDC_OVER_CUR_1
    }
}
#[doc = "Field `DCDC_OVER_VOL` reader - DCDC output over voltage alert"]
pub type DCDC_OVER_VOL_R = crate::BitReader<DCDC_OVER_VOL_A>;
#[doc = "DCDC output over voltage alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDC_OVER_VOL_A {
    #[doc = "0: No over voltage detected"]
    DCDC_OVER_VOL_0 = 0,
    #[doc = "1: Over voltage detected"]
    DCDC_OVER_VOL_1 = 1,
}
impl From<DCDC_OVER_VOL_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_OVER_VOL_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDC_OVER_VOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_OVER_VOL_A {
        match self.bits {
            false => DCDC_OVER_VOL_A::DCDC_OVER_VOL_0,
            true => DCDC_OVER_VOL_A::DCDC_OVER_VOL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCDC_OVER_VOL_0`"]
    #[inline(always)]
    pub fn is_dcdc_over_vol_0(&self) -> bool {
        *self == DCDC_OVER_VOL_A::DCDC_OVER_VOL_0
    }
    #[doc = "Checks if the value of the field is `DCDC_OVER_VOL_1`"]
    #[inline(always)]
    pub fn is_dcdc_over_vol_1(&self) -> bool {
        *self == DCDC_OVER_VOL_A::DCDC_OVER_VOL_1
    }
}
#[doc = "Field `DCDC_STS_DC_OK` reader - DCDC status OK"]
pub type DCDC_STS_DC_OK_R = crate::BitReader<DCDC_STS_DC_OK_A>;
#[doc = "DCDC status OK\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DCDC_STS_DC_OK_A {
    #[doc = "0: DCDC is ramping up and not ready"]
    DCDC_STS_DC_OK_0 = 0,
    #[doc = "1: DCDC is ready"]
    DCDC_STS_DC_OK_1 = 1,
}
impl From<DCDC_STS_DC_OK_A> for bool {
    #[inline(always)]
    fn from(variant: DCDC_STS_DC_OK_A) -> Self {
        variant as u8 != 0
    }
}
impl DCDC_STS_DC_OK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCDC_STS_DC_OK_A {
        match self.bits {
            false => DCDC_STS_DC_OK_A::DCDC_STS_DC_OK_0,
            true => DCDC_STS_DC_OK_A::DCDC_STS_DC_OK_1,
        }
    }
    #[doc = "Checks if the value of the field is `DCDC_STS_DC_OK_0`"]
    #[inline(always)]
    pub fn is_dcdc_sts_dc_ok_0(&self) -> bool {
        *self == DCDC_STS_DC_OK_A::DCDC_STS_DC_OK_0
    }
    #[doc = "Checks if the value of the field is `DCDC_STS_DC_OK_1`"]
    #[inline(always)]
    pub fn is_dcdc_sts_dc_ok_1(&self) -> bool {
        *self == DCDC_STS_DC_OK_A::DCDC_STS_DC_OK_1
    }
}
impl R {
    #[doc = "Bit 0 - Set to enable LPSR mode."]
    #[inline(always)]
    pub fn lpsr_mode_enable(&self) -> LPSR_MODE_ENABLE_R {
        LPSR_MODE_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCDC captured status clear"]
    #[inline(always)]
    pub fn dcdc_status_capt_clr(&self) -> DCDC_STATUS_CAPT_CLR_R {
        DCDC_STATUS_CAPT_CLR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - POR_B pad control"]
    #[inline(always)]
    pub fn por_pull_type(&self) -> POR_PULL_TYPE_R {
        POR_PULL_TYPE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 16 - DCDC_IN low voltage detect."]
    #[inline(always)]
    pub fn dcdc_in_low_vol(&self) -> DCDC_IN_LOW_VOL_R {
        DCDC_IN_LOW_VOL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DCDC output over current alert"]
    #[inline(always)]
    pub fn dcdc_over_cur(&self) -> DCDC_OVER_CUR_R {
        DCDC_OVER_CUR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DCDC output over voltage alert"]
    #[inline(always)]
    pub fn dcdc_over_vol(&self) -> DCDC_OVER_VOL_R {
        DCDC_OVER_VOL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DCDC status OK"]
    #[inline(always)]
    pub fn dcdc_sts_dc_ok(&self) -> DCDC_STS_DC_OK_R {
        DCDC_STS_DC_OK_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set to enable LPSR mode."]
    #[inline(always)]
    #[must_use]
    pub fn lpsr_mode_enable(&mut self) -> LPSR_MODE_ENABLE_W<0> {
        LPSR_MODE_ENABLE_W::new(self)
    }
    #[doc = "Bit 1 - DCDC captured status clear"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_status_capt_clr(&mut self) -> DCDC_STATUS_CAPT_CLR_W<1> {
        DCDC_STATUS_CAPT_CLR_W::new(self)
    }
    #[doc = "Bits 2:3 - POR_B pad control"]
    #[inline(always)]
    #[must_use]
    pub fn por_pull_type(&mut self) -> POR_PULL_TYPE_W<2> {
        POR_PULL_TYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR3 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr3](index.html) module"]
pub struct GPR3_SPEC;
impl crate::RegisterSpec for GPR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr3::R](R) reader structure"]
impl crate::Readable for GPR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr3::W](W) writer structure"]
impl crate::Writable for GPR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR3 to value 0"]
impl crate::Resettable for GPR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
