#[doc = "Register `REG_1P1_CLR` reader"]
pub struct R(crate::R<REG_1P1_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REG_1P1_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REG_1P1_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REG_1P1_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REG_1P1_CLR` writer"]
pub struct W(crate::W<REG_1P1_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REG_1P1_CLR_SPEC>;
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
impl From<crate::W<REG_1P1_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REG_1P1_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_LINREG` reader - Control bit to enable the regulator output."]
pub type ENABLE_LINREG_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_LINREG` writer - Control bit to enable the regulator output."]
pub type ENABLE_LINREG_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_1P1_CLR_SPEC, bool, O>;
#[doc = "Field `ENABLE_BO` reader - Control bit to enable the brownout circuitry in the regulator."]
pub type ENABLE_BO_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_BO` writer - Control bit to enable the brownout circuitry in the regulator."]
pub type ENABLE_BO_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_1P1_CLR_SPEC, bool, O>;
#[doc = "Field `ENABLE_ILIMIT` reader - Control bit to enable the current-limit circuitry in the regulator."]
pub type ENABLE_ILIMIT_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_ILIMIT` writer - Control bit to enable the current-limit circuitry in the regulator."]
pub type ENABLE_ILIMIT_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_1P1_CLR_SPEC, bool, O>;
#[doc = "Field `ENABLE_PULLDOWN` reader - Control bit to enable the pull-down circuitry in the regulator"]
pub type ENABLE_PULLDOWN_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_PULLDOWN` writer - Control bit to enable the pull-down circuitry in the regulator"]
pub type ENABLE_PULLDOWN_W<'a, const O: u8> = crate::BitWriter<'a, u32, REG_1P1_CLR_SPEC, bool, O>;
#[doc = "Field `BO_OFFSET` reader - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
pub type BO_OFFSET_R = crate::FieldReader<u8, u8>;
#[doc = "Field `BO_OFFSET` writer - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
pub type BO_OFFSET_W<'a, const O: u8> = crate::FieldWriter<'a, u32, REG_1P1_CLR_SPEC, u8, u8, 3, O>;
#[doc = "Field `OUTPUT_TRG` reader - Control bits to adjust the regulator output voltage"]
pub type OUTPUT_TRG_R = crate::FieldReader<u8, OUTPUT_TRG_A>;
#[doc = "Control bits to adjust the regulator output voltage\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OUTPUT_TRG_A {
    #[doc = "4: 0.8V"]
    OUTPUT_TRG_4 = 4,
    #[doc = "16: 1.1V"]
    OUTPUT_TRG_16 = 16,
}
impl From<OUTPUT_TRG_A> for u8 {
    #[inline(always)]
    fn from(variant: OUTPUT_TRG_A) -> Self {
        variant as _
    }
}
impl OUTPUT_TRG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OUTPUT_TRG_A> {
        match self.bits {
            4 => Some(OUTPUT_TRG_A::OUTPUT_TRG_4),
            16 => Some(OUTPUT_TRG_A::OUTPUT_TRG_16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_4`"]
    #[inline(always)]
    pub fn is_output_trg_4(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_4
    }
    #[doc = "Checks if the value of the field is `OUTPUT_TRG_16`"]
    #[inline(always)]
    pub fn is_output_trg_16(&self) -> bool {
        *self == OUTPUT_TRG_A::OUTPUT_TRG_16
    }
}
#[doc = "Field `OUTPUT_TRG` writer - Control bits to adjust the regulator output voltage"]
pub type OUTPUT_TRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, REG_1P1_CLR_SPEC, u8, OUTPUT_TRG_A, 5, O>;
impl<'a, const O: u8> OUTPUT_TRG_W<'a, O> {
    #[doc = "0.8V"]
    #[inline(always)]
    pub fn output_trg_4(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_4)
    }
    #[doc = "1.1V"]
    #[inline(always)]
    pub fn output_trg_16(self) -> &'a mut W {
        self.variant(OUTPUT_TRG_A::OUTPUT_TRG_16)
    }
}
#[doc = "Field `BO_VDD1P1` reader - Status bit that signals when a brownout is detected on the regulator output."]
pub type BO_VDD1P1_R = crate::BitReader<bool>;
#[doc = "Field `OK_VDD1P1` reader - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
pub type OK_VDD1P1_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_WEAK_LINREG` reader - Enables the weak 1p1 regulator"]
pub type ENABLE_WEAK_LINREG_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_WEAK_LINREG` writer - Enables the weak 1p1 regulator"]
pub type ENABLE_WEAK_LINREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_1P1_CLR_SPEC, bool, O>;
#[doc = "Field `SELREF_WEAK_LINREG` reader - Selects the source for the reference voltage of the weak 1p1 regulator."]
pub type SELREF_WEAK_LINREG_R = crate::BitReader<SELREF_WEAK_LINREG_A>;
#[doc = "Selects the source for the reference voltage of the weak 1p1 regulator.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELREF_WEAK_LINREG_A {
    #[doc = "0: Weak-linreg output tracks low-power-bandgap voltage"]
    SELREF_WEAK_LINREG_0 = 0,
    #[doc = "1: Weak-linreg output tracks VDD_SOC_IN voltage"]
    SELREF_WEAK_LINREG_1 = 1,
}
impl From<SELREF_WEAK_LINREG_A> for bool {
    #[inline(always)]
    fn from(variant: SELREF_WEAK_LINREG_A) -> Self {
        variant as u8 != 0
    }
}
impl SELREF_WEAK_LINREG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELREF_WEAK_LINREG_A {
        match self.bits {
            false => SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_0,
            true => SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_1,
        }
    }
    #[doc = "Checks if the value of the field is `SELREF_WEAK_LINREG_0`"]
    #[inline(always)]
    pub fn is_selref_weak_linreg_0(&self) -> bool {
        *self == SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_0
    }
    #[doc = "Checks if the value of the field is `SELREF_WEAK_LINREG_1`"]
    #[inline(always)]
    pub fn is_selref_weak_linreg_1(&self) -> bool {
        *self == SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_1
    }
}
#[doc = "Field `SELREF_WEAK_LINREG` writer - Selects the source for the reference voltage of the weak 1p1 regulator."]
pub type SELREF_WEAK_LINREG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REG_1P1_CLR_SPEC, SELREF_WEAK_LINREG_A, O>;
impl<'a, const O: u8> SELREF_WEAK_LINREG_W<'a, O> {
    #[doc = "Weak-linreg output tracks low-power-bandgap voltage"]
    #[inline(always)]
    pub fn selref_weak_linreg_0(self) -> &'a mut W {
        self.variant(SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_0)
    }
    #[doc = "Weak-linreg output tracks VDD_SOC_IN voltage"]
    #[inline(always)]
    pub fn selref_weak_linreg_1(self) -> &'a mut W {
        self.variant(SELREF_WEAK_LINREG_A::SELREF_WEAK_LINREG_1)
    }
}
impl R {
    #[doc = "Bit 0 - Control bit to enable the regulator output."]
    #[inline(always)]
    pub fn enable_linreg(&self) -> ENABLE_LINREG_R {
        ENABLE_LINREG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_bo(&self) -> ENABLE_BO_R {
        ENABLE_BO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    pub fn enable_ilimit(&self) -> ENABLE_ILIMIT_R {
        ENABLE_ILIMIT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    pub fn enable_pulldown(&self) -> ENABLE_PULLDOWN_R {
        ENABLE_PULLDOWN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    pub fn bo_offset(&self) -> BO_OFFSET_R {
        BO_OFFSET_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    pub fn output_trg(&self) -> OUTPUT_TRG_R {
        OUTPUT_TRG_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Status bit that signals when a brownout is detected on the regulator output."]
    #[inline(always)]
    pub fn bo_vdd1p1(&self) -> BO_VDD1P1_R {
        BO_VDD1P1_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Status bit that signals when the regulator output is ok. 1 = regulator output > brownout target"]
    #[inline(always)]
    pub fn ok_vdd1p1(&self) -> OK_VDD1P1_R {
        OK_VDD1P1_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enables the weak 1p1 regulator"]
    #[inline(always)]
    pub fn enable_weak_linreg(&self) -> ENABLE_WEAK_LINREG_R {
        ENABLE_WEAK_LINREG_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    pub fn selref_weak_linreg(&self) -> SELREF_WEAK_LINREG_R {
        SELREF_WEAK_LINREG_R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Control bit to enable the regulator output."]
    #[inline(always)]
    #[must_use]
    pub fn enable_linreg(&mut self) -> ENABLE_LINREG_W<0> {
        ENABLE_LINREG_W::new(self)
    }
    #[doc = "Bit 1 - Control bit to enable the brownout circuitry in the regulator."]
    #[inline(always)]
    #[must_use]
    pub fn enable_bo(&mut self) -> ENABLE_BO_W<1> {
        ENABLE_BO_W::new(self)
    }
    #[doc = "Bit 2 - Control bit to enable the current-limit circuitry in the regulator."]
    #[inline(always)]
    #[must_use]
    pub fn enable_ilimit(&mut self) -> ENABLE_ILIMIT_W<2> {
        ENABLE_ILIMIT_W::new(self)
    }
    #[doc = "Bit 3 - Control bit to enable the pull-down circuitry in the regulator"]
    #[inline(always)]
    #[must_use]
    pub fn enable_pulldown(&mut self) -> ENABLE_PULLDOWN_W<3> {
        ENABLE_PULLDOWN_W::new(self)
    }
    #[doc = "Bits 4:6 - Control bits to adjust the regulator brownout offset voltage in 25mV steps"]
    #[inline(always)]
    #[must_use]
    pub fn bo_offset(&mut self) -> BO_OFFSET_W<4> {
        BO_OFFSET_W::new(self)
    }
    #[doc = "Bits 8:12 - Control bits to adjust the regulator output voltage"]
    #[inline(always)]
    #[must_use]
    pub fn output_trg(&mut self) -> OUTPUT_TRG_W<8> {
        OUTPUT_TRG_W::new(self)
    }
    #[doc = "Bit 18 - Enables the weak 1p1 regulator"]
    #[inline(always)]
    #[must_use]
    pub fn enable_weak_linreg(&mut self) -> ENABLE_WEAK_LINREG_W<18> {
        ENABLE_WEAK_LINREG_W::new(self)
    }
    #[doc = "Bit 19 - Selects the source for the reference voltage of the weak 1p1 regulator."]
    #[inline(always)]
    #[must_use]
    pub fn selref_weak_linreg(&mut self) -> SELREF_WEAK_LINREG_W<19> {
        SELREF_WEAK_LINREG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Regulator 1P1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [reg_1p1_clr](index.html) module"]
pub struct REG_1P1_CLR_SPEC;
impl crate::RegisterSpec for REG_1P1_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [reg_1p1_clr::R](R) reader structure"]
impl crate::Readable for REG_1P1_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [reg_1p1_clr::W](W) writer structure"]
impl crate::Writable for REG_1P1_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REG_1P1_CLR to value 0x1073"]
impl crate::Resettable for REG_1P1_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1073;
}
