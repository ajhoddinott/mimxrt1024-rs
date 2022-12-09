#[doc = "Register `REG3` reader"]
pub struct R(crate::R<REG3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG3` writer"]
pub struct W(crate::W<REG3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG3_SPEC>;
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
impl From<crate::W<REG3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRG` reader - Target value of VDD_SOC"]
pub type TRG_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRG` writer - Target value of VDD_SOC"]
pub type TRG_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG3_SPEC, u8, u8, 5, O>;
#[doc = "Field `TARGET_LP` reader - Low Power Target Value"]
pub type TARGET_LP_R = crate::FieldReader<u8, TARGET_LP_A>;
#[doc = "Low Power Target Value\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TARGET_LP_A {
    #[doc = "0: 0.9 V"]
    SEL0 = 0,
    #[doc = "1: 0.925 V"]
    SEL1 = 1,
    #[doc = "2: 0.95 V"]
    SEL2 = 2,
    #[doc = "3: 0.975 V"]
    SEL3 = 3,
    #[doc = "4: 1.0 V"]
    SEL4 = 4,
}
impl From<TARGET_LP_A> for u8 {
    #[inline(always)]
    fn from(variant: TARGET_LP_A) -> Self {
        variant as _
    }
}
impl TARGET_LP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TARGET_LP_A> {
        match self.bits {
            0 => Some(TARGET_LP_A::SEL0),
            1 => Some(TARGET_LP_A::SEL1),
            2 => Some(TARGET_LP_A::SEL2),
            3 => Some(TARGET_LP_A::SEL3),
            4 => Some(TARGET_LP_A::SEL4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SEL0`"]
    #[inline(always)]
    pub fn is_sel0(&self) -> bool {
        *self == TARGET_LP_A::SEL0
    }
    #[doc = "Checks if the value of the field is `SEL1`"]
    #[inline(always)]
    pub fn is_sel1(&self) -> bool {
        *self == TARGET_LP_A::SEL1
    }
    #[doc = "Checks if the value of the field is `SEL2`"]
    #[inline(always)]
    pub fn is_sel2(&self) -> bool {
        *self == TARGET_LP_A::SEL2
    }
    #[doc = "Checks if the value of the field is `SEL3`"]
    #[inline(always)]
    pub fn is_sel3(&self) -> bool {
        *self == TARGET_LP_A::SEL3
    }
    #[doc = "Checks if the value of the field is `SEL4`"]
    #[inline(always)]
    pub fn is_sel4(&self) -> bool {
        *self == TARGET_LP_A::SEL4
    }
}
#[doc = "Field `TARGET_LP` writer - Low Power Target Value"]
pub type TARGET_LP_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG3_SPEC, u8, TARGET_LP_A, 3, O>;
impl<'a, const O: u8> TARGET_LP_W<'a, O> {
    #[doc = "0.9 V"]
    #[inline(always)]
    pub fn sel0(self) -> &'a mut W {
        self.variant(TARGET_LP_A::SEL0)
    }
    #[doc = "0.925 V"]
    #[inline(always)]
    pub fn sel1(self) -> &'a mut W {
        self.variant(TARGET_LP_A::SEL1)
    }
    #[doc = "0.95 V"]
    #[inline(always)]
    pub fn sel2(self) -> &'a mut W {
        self.variant(TARGET_LP_A::SEL2)
    }
    #[doc = "0.975 V"]
    #[inline(always)]
    pub fn sel3(self) -> &'a mut W {
        self.variant(TARGET_LP_A::SEL3)
    }
    #[doc = "1.0 V"]
    #[inline(always)]
    pub fn sel4(self) -> &'a mut W {
        self.variant(TARGET_LP_A::SEL4)
    }
}
#[doc = "Field `MINPWR_DC_HALFCLK` reader - Set DCDC clock to half frequency for continuous mode"]
pub type MINPWR_DC_HALFCLK_R = crate::BitReader<MINPWR_DC_HALFCLK_A>;
#[doc = "Set DCDC clock to half frequency for continuous mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MINPWR_DC_HALFCLK_A {
    #[doc = "0: DCDC clock remains at full frequency for continuous mode"]
    FULLFREQ = 0,
    #[doc = "1: DCDC clock set to half frequency for continuous mode"]
    HALFFREQ = 1,
}
impl From<MINPWR_DC_HALFCLK_A> for bool {
    #[inline(always)]
    fn from(variant: MINPWR_DC_HALFCLK_A) -> Self {
        variant as u8 != 0
    }
}
impl MINPWR_DC_HALFCLK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MINPWR_DC_HALFCLK_A {
        match self.bits {
            false => MINPWR_DC_HALFCLK_A::FULLFREQ,
            true => MINPWR_DC_HALFCLK_A::HALFFREQ,
        }
    }
    #[doc = "Checks if the value of the field is `FULLFREQ`"]
    #[inline(always)]
    pub fn is_fullfreq(&self) -> bool {
        *self == MINPWR_DC_HALFCLK_A::FULLFREQ
    }
    #[doc = "Checks if the value of the field is `HALFFREQ`"]
    #[inline(always)]
    pub fn is_halffreq(&self) -> bool {
        *self == MINPWR_DC_HALFCLK_A::HALFFREQ
    }
}
#[doc = "Field `MINPWR_DC_HALFCLK` writer - Set DCDC clock to half frequency for continuous mode"]
pub type MINPWR_DC_HALFCLK_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG3_SPEC, MINPWR_DC_HALFCLK_A, O>;
impl<'a, const O: u8> MINPWR_DC_HALFCLK_W<'a, O> {
    #[doc = "DCDC clock remains at full frequency for continuous mode"]
    #[inline(always)]
    pub fn fullfreq(self) -> &'a mut W {
        self.variant(MINPWR_DC_HALFCLK_A::FULLFREQ)
    }
    #[doc = "DCDC clock set to half frequency for continuous mode"]
    #[inline(always)]
    pub fn halffreq(self) -> &'a mut W {
        self.variant(MINPWR_DC_HALFCLK_A::HALFFREQ)
    }
}
#[doc = "Field `DISABLE_STEP` reader - Disable Step"]
pub type DISABLE_STEP_R = crate::BitReader<DISABLE_STEP_A>;
#[doc = "Disable Step\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DISABLE_STEP_A {
    #[doc = "0: Enable stepping for the output of VDD_SOC of DCDC"]
    ENABLE = 0,
    #[doc = "1: Disable stepping for the output of VDD_SOC of DCDC"]
    DISABLE = 1,
}
impl From<DISABLE_STEP_A> for bool {
    #[inline(always)]
    fn from(variant: DISABLE_STEP_A) -> Self {
        variant as u8 != 0
    }
}
impl DISABLE_STEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DISABLE_STEP_A {
        match self.bits {
            false => DISABLE_STEP_A::ENABLE,
            true => DISABLE_STEP_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DISABLE_STEP_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DISABLE_STEP_A::DISABLE
    }
}
#[doc = "Field `DISABLE_STEP` writer - Disable Step"]
pub type DISABLE_STEP_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG3_SPEC, DISABLE_STEP_A, O>;
impl<'a, const O: u8> DISABLE_STEP_W<'a, O> {
    #[doc = "Enable stepping for the output of VDD_SOC of DCDC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DISABLE_STEP_A::ENABLE)
    }
    #[doc = "Disable stepping for the output of VDD_SOC of DCDC"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DISABLE_STEP_A::DISABLE)
    }
}
impl R {
    #[doc = "Bits 0:4 - Target value of VDD_SOC"]
    #[inline(always)]
    pub fn trg(&self) -> TRG_R {
        TRG_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:10 - Low Power Target Value"]
    #[inline(always)]
    pub fn target_lp(&self) -> TARGET_LP_R {
        TARGET_LP_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 24 - Set DCDC clock to half frequency for continuous mode"]
    #[inline(always)]
    pub fn minpwr_dc_halfclk(&self) -> MINPWR_DC_HALFCLK_R {
        MINPWR_DC_HALFCLK_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - Disable Step"]
    #[inline(always)]
    pub fn disable_step(&self) -> DISABLE_STEP_R {
        DISABLE_STEP_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Target value of VDD_SOC"]
    #[inline(always)]
    #[must_use]
    pub fn trg(&mut self) -> TRG_W<0> {
        TRG_W::new(self)
    }
    #[doc = "Bits 8:10 - Low Power Target Value"]
    #[inline(always)]
    #[must_use]
    pub fn target_lp(&mut self) -> TARGET_LP_W<8> {
        TARGET_LP_W::new(self)
    }
    #[doc = "Bit 24 - Set DCDC clock to half frequency for continuous mode"]
    #[inline(always)]
    #[must_use]
    pub fn minpwr_dc_halfclk(&mut self) -> MINPWR_DC_HALFCLK_W<24> {
        MINPWR_DC_HALFCLK_W::new(self)
    }
    #[doc = "Bit 30 - Disable Step"]
    #[inline(always)]
    #[must_use]
    pub fn disable_step(&mut self) -> DISABLE_STEP_W<30> {
        DISABLE_STEP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCDC Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg3](index.html) module"]
pub struct REG3_SPEC;
impl crate::RegisterSpec for REG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg3::R](R) reader structure"]
impl crate::Readable for REG3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg3::W](W) writer structure"]
impl crate::Writable for REG3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG3 to value 0x010e"]
impl crate::Resettable for REG3_SPEC {
    const RESET_VALUE: Self::Ux = 0x010e;
}
