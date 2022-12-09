#[doc = "Register `SRPC` reader"]
pub struct R(crate::R<SRPC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRPC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRPC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRPC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRPC` writer"]
pub struct W(crate::W<SRPC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRPC_SPEC>;
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
impl From<crate::W<SRPC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRPC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GainSel` reader - Gain selection:"]
pub type GAIN_SEL_R = crate::FieldReader<u8, GAIN_SEL_A>;
#[doc = "Gain selection:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GAIN_SEL_A {
    #[doc = "0: 24*(2**10)"]
    GAIN_SEL_0 = 0,
    #[doc = "1: 16*(2**10)"]
    GAIN_SEL_1 = 1,
    #[doc = "2: 12*(2**10)"]
    GAIN_SEL_2 = 2,
    #[doc = "3: 8*(2**10)"]
    GAIN_SEL_3 = 3,
    #[doc = "4: 6*(2**10)"]
    GAIN_SEL_4 = 4,
    #[doc = "5: 4*(2**10)"]
    GAIN_SEL_5 = 5,
    #[doc = "6: 3*(2**10)"]
    GAIN_SEL_6 = 6,
}
impl From<GAIN_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: GAIN_SEL_A) -> Self {
        variant as _
    }
}
impl GAIN_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<GAIN_SEL_A> {
        match self.bits {
            0 => Some(GAIN_SEL_A::GAIN_SEL_0),
            1 => Some(GAIN_SEL_A::GAIN_SEL_1),
            2 => Some(GAIN_SEL_A::GAIN_SEL_2),
            3 => Some(GAIN_SEL_A::GAIN_SEL_3),
            4 => Some(GAIN_SEL_A::GAIN_SEL_4),
            5 => Some(GAIN_SEL_A::GAIN_SEL_5),
            6 => Some(GAIN_SEL_A::GAIN_SEL_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `GAIN_SEL_0`"]
    #[inline(always)]
    pub fn is_gain_sel_0(&self) -> bool {
        *self == GAIN_SEL_A::GAIN_SEL_0
    }
    #[doc = "Checks if the value of the field is `GAIN_SEL_1`"]
    #[inline(always)]
    pub fn is_gain_sel_1(&self) -> bool {
        *self == GAIN_SEL_A::GAIN_SEL_1
    }
    #[doc = "Checks if the value of the field is `GAIN_SEL_2`"]
    #[inline(always)]
    pub fn is_gain_sel_2(&self) -> bool {
        *self == GAIN_SEL_A::GAIN_SEL_2
    }
    #[doc = "Checks if the value of the field is `GAIN_SEL_3`"]
    #[inline(always)]
    pub fn is_gain_sel_3(&self) -> bool {
        *self == GAIN_SEL_A::GAIN_SEL_3
    }
    #[doc = "Checks if the value of the field is `GAIN_SEL_4`"]
    #[inline(always)]
    pub fn is_gain_sel_4(&self) -> bool {
        *self == GAIN_SEL_A::GAIN_SEL_4
    }
    #[doc = "Checks if the value of the field is `GAIN_SEL_5`"]
    #[inline(always)]
    pub fn is_gain_sel_5(&self) -> bool {
        *self == GAIN_SEL_A::GAIN_SEL_5
    }
    #[doc = "Checks if the value of the field is `GAIN_SEL_6`"]
    #[inline(always)]
    pub fn is_gain_sel_6(&self) -> bool {
        *self == GAIN_SEL_A::GAIN_SEL_6
    }
}
#[doc = "Field `GainSel` writer - Gain selection:"]
pub type GAIN_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRPC_SPEC, u8, GAIN_SEL_A, 3, O>;
impl<'a, const O: u8> GAIN_SEL_W<'a, O> {
    #[doc = "24*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_0(self) -> &'a mut W {
        self.variant(GAIN_SEL_A::GAIN_SEL_0)
    }
    #[doc = "16*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_1(self) -> &'a mut W {
        self.variant(GAIN_SEL_A::GAIN_SEL_1)
    }
    #[doc = "12*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_2(self) -> &'a mut W {
        self.variant(GAIN_SEL_A::GAIN_SEL_2)
    }
    #[doc = "8*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_3(self) -> &'a mut W {
        self.variant(GAIN_SEL_A::GAIN_SEL_3)
    }
    #[doc = "6*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_4(self) -> &'a mut W {
        self.variant(GAIN_SEL_A::GAIN_SEL_4)
    }
    #[doc = "4*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_5(self) -> &'a mut W {
        self.variant(GAIN_SEL_A::GAIN_SEL_5)
    }
    #[doc = "3*(2**10)"]
    #[inline(always)]
    pub fn gain_sel_6(self) -> &'a mut W {
        self.variant(GAIN_SEL_A::GAIN_SEL_6)
    }
}
#[doc = "Field `LOCK` reader - LOCK bit to show that the internal DPLL is locked, read only"]
pub type LOCK_R = crate::BitReader<bool>;
#[doc = "Field `ClkSrc_Sel` reader - Clock source selection, all other settings not shown are reserved:"]
pub type CLK_SRC_SEL_R = crate::FieldReader<u8, CLK_SRC_SEL_A>;
#[doc = "Clock source selection, all other settings not shown are reserved:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLK_SRC_SEL_A {
    #[doc = "0: if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
    CLK_SRC_SEL_0 = 0,
    #[doc = "1: if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
    CLK_SRC_SEL_1 = 1,
    #[doc = "3: if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
    CLK_SRC_SEL_3 = 3,
    #[doc = "5: REF_CLK_32K (XTALOSC)"]
    CLK_SRC_SEL_5 = 5,
    #[doc = "6: tx_clk (SPDIF0_CLK_ROOT)"]
    CLK_SRC_SEL_6 = 6,
    #[doc = "8: SPDIF_EXT_CLK"]
    CLK_SRC_SEL_8 = 8,
}
impl From<CLK_SRC_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CLK_SRC_SEL_A) -> Self {
        variant as _
    }
}
impl CLK_SRC_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLK_SRC_SEL_A> {
        match self.bits {
            0 => Some(CLK_SRC_SEL_A::CLK_SRC_SEL_0),
            1 => Some(CLK_SRC_SEL_A::CLK_SRC_SEL_1),
            3 => Some(CLK_SRC_SEL_A::CLK_SRC_SEL_3),
            5 => Some(CLK_SRC_SEL_A::CLK_SRC_SEL_5),
            6 => Some(CLK_SRC_SEL_A::CLK_SRC_SEL_6),
            8 => Some(CLK_SRC_SEL_A::CLK_SRC_SEL_8),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CLK_SRC_SEL_0`"]
    #[inline(always)]
    pub fn is_clk_src_sel_0(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK_SRC_SEL_0
    }
    #[doc = "Checks if the value of the field is `CLK_SRC_SEL_1`"]
    #[inline(always)]
    pub fn is_clk_src_sel_1(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK_SRC_SEL_1
    }
    #[doc = "Checks if the value of the field is `CLK_SRC_SEL_3`"]
    #[inline(always)]
    pub fn is_clk_src_sel_3(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK_SRC_SEL_3
    }
    #[doc = "Checks if the value of the field is `CLK_SRC_SEL_5`"]
    #[inline(always)]
    pub fn is_clk_src_sel_5(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK_SRC_SEL_5
    }
    #[doc = "Checks if the value of the field is `CLK_SRC_SEL_6`"]
    #[inline(always)]
    pub fn is_clk_src_sel_6(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK_SRC_SEL_6
    }
    #[doc = "Checks if the value of the field is `CLK_SRC_SEL_8`"]
    #[inline(always)]
    pub fn is_clk_src_sel_8(&self) -> bool {
        *self == CLK_SRC_SEL_A::CLK_SRC_SEL_8
    }
}
#[doc = "Field `ClkSrc_Sel` writer - Clock source selection, all other settings not shown are reserved:"]
pub type CLK_SRC_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SRPC_SPEC, u8, CLK_SRC_SEL_A, 4, O>;
impl<'a, const O: u8> CLK_SRC_SEL_W<'a, O> {
    #[doc = "if (DPLL Locked) SPDIF_RxClk else REF_CLK_32K (XTALOSC)"]
    #[inline(always)]
    pub fn clk_src_sel_0(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK_SRC_SEL_0)
    }
    #[doc = "if (DPLL Locked) SPDIF_RxClk else tx_clk (SPDIF0_CLK_ROOT)"]
    #[inline(always)]
    pub fn clk_src_sel_1(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK_SRC_SEL_1)
    }
    #[doc = "if (DPLL Locked) SPDIF_RxClk else SPDIF_EXT_CLK"]
    #[inline(always)]
    pub fn clk_src_sel_3(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK_SRC_SEL_3)
    }
    #[doc = "REF_CLK_32K (XTALOSC)"]
    #[inline(always)]
    pub fn clk_src_sel_5(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK_SRC_SEL_5)
    }
    #[doc = "tx_clk (SPDIF0_CLK_ROOT)"]
    #[inline(always)]
    pub fn clk_src_sel_6(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK_SRC_SEL_6)
    }
    #[doc = "SPDIF_EXT_CLK"]
    #[inline(always)]
    pub fn clk_src_sel_8(self) -> &'a mut W {
        self.variant(CLK_SRC_SEL_A::CLK_SRC_SEL_8)
    }
}
impl R {
    #[doc = "Bits 3:5 - Gain selection:"]
    #[inline(always)]
    pub fn gain_sel(&self) -> GAIN_SEL_R {
        GAIN_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - LOCK bit to show that the internal DPLL is locked, read only"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:10 - Clock source selection, all other settings not shown are reserved:"]
    #[inline(always)]
    pub fn clk_src_sel(&self) -> CLK_SRC_SEL_R {
        CLK_SRC_SEL_R::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - Gain selection:"]
    #[inline(always)]
    #[must_use]
    pub fn gain_sel(&mut self) -> GAIN_SEL_W<3> {
        GAIN_SEL_W::new(self)
    }
    #[doc = "Bits 7:10 - Clock source selection, all other settings not shown are reserved:"]
    #[inline(always)]
    #[must_use]
    pub fn clk_src_sel(&mut self) -> CLK_SRC_SEL_W<7> {
        CLK_SRC_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PhaseConfig Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [srpc](index.html) module"]
pub struct SRPC_SPEC;
impl crate::RegisterSpec for SRPC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [srpc::R](R) reader structure"]
impl crate::Readable for SRPC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [srpc::W](W) writer structure"]
impl crate::Writable for SRPC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRPC to value 0"]
impl crate::Resettable for SRPC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
