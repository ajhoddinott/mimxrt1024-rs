#[doc = "Register `CDCDR` reader"]
pub struct R(crate::R<CDCDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CDCDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CDCDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CDCDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CDCDR` writer"]
pub struct W(crate::W<CDCDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CDCDR_SPEC>;
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
impl From<crate::W<CDCDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CDCDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPDIF0_CLK_SEL` reader - Selector for spdif0 clock multiplexer"]
pub type SPDIF0_CLK_SEL_R = crate::FieldReader<u8, SPDIF0_CLK_SEL_A>;
#[doc = "Selector for spdif0 clock multiplexer\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDIF0_CLK_SEL_A {
    #[doc = "0: derive clock from PLL4"]
    SPDIF0_CLK_SEL_0 = 0,
    #[doc = "1: derive clock from PLL3 PFD2"]
    SPDIF0_CLK_SEL_1 = 1,
    #[doc = "3: derive clock from pll3_sw_clk"]
    SPDIF0_CLK_SEL_3 = 3,
}
impl From<SPDIF0_CLK_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIF0_CLK_SEL_A) -> Self {
        variant as _
    }
}
impl SPDIF0_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SPDIF0_CLK_SEL_A> {
        match self.bits {
            0 => Some(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_0),
            1 => Some(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_1),
            3 => Some(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_spdif0_clk_sel_0(&self) -> bool {
        *self == SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_spdif0_clk_sel_1(&self) -> bool {
        *self == SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_1
    }
    #[doc = "Checks if the value of the field is `SPDIF0_CLK_SEL_3`"]
    #[inline(always)]
    pub fn is_spdif0_clk_sel_3(&self) -> bool {
        *self == SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_3
    }
}
#[doc = "Field `SPDIF0_CLK_SEL` writer - Selector for spdif0 clock multiplexer"]
pub type SPDIF0_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CDCDR_SPEC, u8, SPDIF0_CLK_SEL_A, 2, O>;
impl<'a, const O: u8> SPDIF0_CLK_SEL_W<'a, O> {
    #[doc = "derive clock from PLL4"]
    #[inline(always)]
    pub fn spdif0_clk_sel_0(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_0)
    }
    #[doc = "derive clock from PLL3 PFD2"]
    #[inline(always)]
    pub fn spdif0_clk_sel_1(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_1)
    }
    #[doc = "derive clock from pll3_sw_clk"]
    #[inline(always)]
    pub fn spdif0_clk_sel_3(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_SEL_A::SPDIF0_CLK_SEL_3)
    }
}
#[doc = "Field `SPDIF0_CLK_PODF` reader - Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
pub type SPDIF0_CLK_PODF_R = crate::FieldReader<u8, SPDIF0_CLK_PODF_A>;
#[doc = "Divider for spdif0 clock podf. Divider should be updated when output clock is gated.\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDIF0_CLK_PODF_A {
    #[doc = "0: Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "1: Divide by 2"]
    DIVIDE_2 = 1,
    #[doc = "2: Divide by 3"]
    DIVIDE_3 = 2,
    #[doc = "3: Divide by 4"]
    DIVIDE_4 = 3,
    #[doc = "4: Divide by 5"]
    DIVIDE_5 = 4,
    #[doc = "5: Divide by 6"]
    DIVIDE_6 = 5,
    #[doc = "6: Divide by 7"]
    DIVIDE_7 = 6,
    #[doc = "7: Divide by 8"]
    DIVIDE_8 = 7,
}
impl From<SPDIF0_CLK_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIF0_CLK_PODF_A) -> Self {
        variant as _
    }
}
impl SPDIF0_CLK_PODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIF0_CLK_PODF_A {
        match self.bits {
            0 => SPDIF0_CLK_PODF_A::DIVIDE_1,
            1 => SPDIF0_CLK_PODF_A::DIVIDE_2,
            2 => SPDIF0_CLK_PODF_A::DIVIDE_3,
            3 => SPDIF0_CLK_PODF_A::DIVIDE_4,
            4 => SPDIF0_CLK_PODF_A::DIVIDE_5,
            5 => SPDIF0_CLK_PODF_A::DIVIDE_6,
            6 => SPDIF0_CLK_PODF_A::DIVIDE_7,
            7 => SPDIF0_CLK_PODF_A::DIVIDE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_8`"]
    #[inline(always)]
    pub fn is_divide_8(&self) -> bool {
        *self == SPDIF0_CLK_PODF_A::DIVIDE_8
    }
}
#[doc = "Field `SPDIF0_CLK_PODF` writer - Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
pub type SPDIF0_CLK_PODF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CDCDR_SPEC, u8, SPDIF0_CLK_PODF_A, 3, O>;
impl<'a, const O: u8> SPDIF0_CLK_PODF_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_8(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PODF_A::DIVIDE_8)
    }
}
#[doc = "Field `SPDIF0_CLK_PRED` reader - Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
pub type SPDIF0_CLK_PRED_R = crate::FieldReader<u8, SPDIF0_CLK_PRED_A>;
#[doc = "Divider for spdif0 clock pred. Divider should be updated when output clock is gated.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SPDIF0_CLK_PRED_A {
    #[doc = "0: Divide by 1"]
    DIVIDE_1 = 0,
    #[doc = "1: Divide by 2"]
    DIVIDE_2 = 1,
    #[doc = "2: Divide by 3"]
    DIVIDE_3 = 2,
    #[doc = "3: Divide by 4"]
    DIVIDE_4 = 3,
    #[doc = "4: Divide by 5"]
    DIVIDE_5 = 4,
    #[doc = "5: Divide by 6"]
    DIVIDE_6 = 5,
    #[doc = "6: Divide by 7"]
    DIVIDE_7 = 6,
    #[doc = "7: Divide by 8"]
    DIVIDE_8 = 7,
}
impl From<SPDIF0_CLK_PRED_A> for u8 {
    #[inline(always)]
    fn from(variant: SPDIF0_CLK_PRED_A) -> Self {
        variant as _
    }
}
impl SPDIF0_CLK_PRED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPDIF0_CLK_PRED_A {
        match self.bits {
            0 => SPDIF0_CLK_PRED_A::DIVIDE_1,
            1 => SPDIF0_CLK_PRED_A::DIVIDE_2,
            2 => SPDIF0_CLK_PRED_A::DIVIDE_3,
            3 => SPDIF0_CLK_PRED_A::DIVIDE_4,
            4 => SPDIF0_CLK_PRED_A::DIVIDE_5,
            5 => SPDIF0_CLK_PRED_A::DIVIDE_6,
            6 => SPDIF0_CLK_PRED_A::DIVIDE_7,
            7 => SPDIF0_CLK_PRED_A::DIVIDE_8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVIDE_1`"]
    #[inline(always)]
    pub fn is_divide_1(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_1
    }
    #[doc = "Checks if the value of the field is `DIVIDE_2`"]
    #[inline(always)]
    pub fn is_divide_2(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_2
    }
    #[doc = "Checks if the value of the field is `DIVIDE_3`"]
    #[inline(always)]
    pub fn is_divide_3(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_3
    }
    #[doc = "Checks if the value of the field is `DIVIDE_4`"]
    #[inline(always)]
    pub fn is_divide_4(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_4
    }
    #[doc = "Checks if the value of the field is `DIVIDE_5`"]
    #[inline(always)]
    pub fn is_divide_5(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_5
    }
    #[doc = "Checks if the value of the field is `DIVIDE_6`"]
    #[inline(always)]
    pub fn is_divide_6(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_6
    }
    #[doc = "Checks if the value of the field is `DIVIDE_7`"]
    #[inline(always)]
    pub fn is_divide_7(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_7
    }
    #[doc = "Checks if the value of the field is `DIVIDE_8`"]
    #[inline(always)]
    pub fn is_divide_8(&self) -> bool {
        *self == SPDIF0_CLK_PRED_A::DIVIDE_8
    }
}
#[doc = "Field `SPDIF0_CLK_PRED` writer - Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
pub type SPDIF0_CLK_PRED_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CDCDR_SPEC, u8, SPDIF0_CLK_PRED_A, 3, O>;
impl<'a, const O: u8> SPDIF0_CLK_PRED_W<'a, O> {
    #[doc = "Divide by 1"]
    #[inline(always)]
    pub fn divide_1(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn divide_2(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_2)
    }
    #[doc = "Divide by 3"]
    #[inline(always)]
    pub fn divide_3(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_3)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn divide_4(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_4)
    }
    #[doc = "Divide by 5"]
    #[inline(always)]
    pub fn divide_5(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_5)
    }
    #[doc = "Divide by 6"]
    #[inline(always)]
    pub fn divide_6(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_6)
    }
    #[doc = "Divide by 7"]
    #[inline(always)]
    pub fn divide_7(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_7)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn divide_8(self) -> &'a mut W {
        self.variant(SPDIF0_CLK_PRED_A::DIVIDE_8)
    }
}
impl R {
    #[doc = "Bits 20:21 - Selector for spdif0 clock multiplexer"]
    #[inline(always)]
    pub fn spdif0_clk_sel(&self) -> SPDIF0_CLK_SEL_R {
        SPDIF0_CLK_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:24 - Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn spdif0_clk_podf(&self) -> SPDIF0_CLK_PODF_R {
        SPDIF0_CLK_PODF_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:27 - Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    pub fn spdif0_clk_pred(&self) -> SPDIF0_CLK_PRED_R {
        SPDIF0_CLK_PRED_R::new(((self.bits >> 25) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 20:21 - Selector for spdif0 clock multiplexer"]
    #[inline(always)]
    #[must_use]
    pub fn spdif0_clk_sel(&mut self) -> SPDIF0_CLK_SEL_W<20> {
        SPDIF0_CLK_SEL_W::new(self)
    }
    #[doc = "Bits 22:24 - Divider for spdif0 clock podf. Divider should be updated when output clock is gated."]
    #[inline(always)]
    #[must_use]
    pub fn spdif0_clk_podf(&mut self) -> SPDIF0_CLK_PODF_W<22> {
        SPDIF0_CLK_PODF_W::new(self)
    }
    #[doc = "Bits 25:27 - Divider for spdif0 clock pred. Divider should be updated when output clock is gated."]
    #[inline(always)]
    #[must_use]
    pub fn spdif0_clk_pred(&mut self) -> SPDIF0_CLK_PRED_W<25> {
        SPDIF0_CLK_PRED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM D1 Clock Divider Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cdcdr](index.html) module"]
pub struct CDCDR_SPEC;
impl crate::RegisterSpec for CDCDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cdcdr::R](R) reader structure"]
impl crate::Readable for CDCDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cdcdr::W](W) writer structure"]
impl crate::Writable for CDCDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CDCDR to value 0x33f7_1f92"]
impl crate::Resettable for CDCDR_SPEC {
    const RESET_VALUE: Self::Ux = 0x33f7_1f92;
}
