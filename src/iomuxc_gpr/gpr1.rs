#[doc = "Register `GPR1` reader"]
pub struct R(crate::R<GPR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR1` writer"]
pub struct W(crate::W<GPR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR1_SPEC>;
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
impl From<crate::W<GPR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAI1_MCLK1_SEL` reader - SAI1 MCLK1 source select"]
pub type SAI1_MCLK1_SEL_R = crate::FieldReader<u8, SAI1_MCLK1_SEL_A>;
#[doc = "SAI1 MCLK1 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1_MCLK1_SEL_A {
    #[doc = "0: ccm.ssi1_clk_root"]
    SAI1_MCLK1_SEL_0 = 0,
    #[doc = "1: ccm.ssi2_clk_root"]
    SAI1_MCLK1_SEL_1 = 1,
    #[doc = "2: ccm.ssi3_clk_root"]
    SAI1_MCLK1_SEL_2 = 2,
    #[doc = "3: iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_3 = 3,
    #[doc = "4: iomux.sai2_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_4 = 4,
    #[doc = "5: iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK1_SEL_5 = 5,
}
impl From<SAI1_MCLK1_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1_MCLK1_SEL_A) -> Self {
        variant as _
    }
}
impl SAI1_MCLK1_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI1_MCLK1_SEL_A> {
        match self.bits {
            0 => Some(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_0),
            1 => Some(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_1),
            2 => Some(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_2),
            3 => Some(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_3),
            4 => Some(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_4),
            5 => Some(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_0`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_0(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_1`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_1(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_2`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_2(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_3`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_3(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_3
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_4`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_4(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_4
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK1_SEL_5`"]
    #[inline(always)]
    pub fn is_sai1_mclk1_sel_5(&self) -> bool {
        *self == SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_5
    }
}
#[doc = "Field `SAI1_MCLK1_SEL` writer - SAI1 MCLK1 source select"]
pub type SAI1_MCLK1_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR1_SPEC, u8, SAI1_MCLK1_SEL_A, 3, O>;
impl<'a, const O: u8> SAI1_MCLK1_SEL_W<'a, O> {
    #[doc = "ccm.ssi1_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_0)
    }
    #[doc = "ccm.ssi2_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_1)
    }
    #[doc = "ccm.ssi3_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_2(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_2)
    }
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_3(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_3)
    }
    #[doc = "iomux.sai2_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_4(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_4)
    }
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk1_sel_5(self) -> &'a mut W {
        self.variant(SAI1_MCLK1_SEL_A::SAI1_MCLK1_SEL_5)
    }
}
#[doc = "Field `SAI1_MCLK2_SEL` reader - SAI1 MCLK2 source select"]
pub type SAI1_MCLK2_SEL_R = crate::FieldReader<u8, SAI1_MCLK2_SEL_A>;
#[doc = "SAI1 MCLK2 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1_MCLK2_SEL_A {
    #[doc = "0: ccm.ssi1_clk_root"]
    SAI1_MCLK2_SEL_0 = 0,
    #[doc = "1: ccm.ssi2_clk_root"]
    SAI1_MCLK2_SEL_1 = 1,
    #[doc = "2: ccm.ssi3_clk_root"]
    SAI1_MCLK2_SEL_2 = 2,
    #[doc = "3: iomux.sai1_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_3 = 3,
    #[doc = "4: iomux.sai2_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_4 = 4,
    #[doc = "5: iomux.sai3_ipg_clk_sai_mclk"]
    SAI1_MCLK2_SEL_5 = 5,
}
impl From<SAI1_MCLK2_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1_MCLK2_SEL_A) -> Self {
        variant as _
    }
}
impl SAI1_MCLK2_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SAI1_MCLK2_SEL_A> {
        match self.bits {
            0 => Some(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_0),
            1 => Some(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_1),
            2 => Some(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_2),
            3 => Some(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_3),
            4 => Some(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_4),
            5 => Some(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_0`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_0(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_1`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_1(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_2`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_2(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_3`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_3(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_3
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_4`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_4(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_4
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK2_SEL_5`"]
    #[inline(always)]
    pub fn is_sai1_mclk2_sel_5(&self) -> bool {
        *self == SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_5
    }
}
#[doc = "Field `SAI1_MCLK2_SEL` writer - SAI1 MCLK2 source select"]
pub type SAI1_MCLK2_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR1_SPEC, u8, SAI1_MCLK2_SEL_A, 3, O>;
impl<'a, const O: u8> SAI1_MCLK2_SEL_W<'a, O> {
    #[doc = "ccm.ssi1_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_0)
    }
    #[doc = "ccm.ssi2_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_1)
    }
    #[doc = "ccm.ssi3_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_2(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_2)
    }
    #[doc = "iomux.sai1_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_3(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_3)
    }
    #[doc = "iomux.sai2_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_4(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_4)
    }
    #[doc = "iomux.sai3_ipg_clk_sai_mclk"]
    #[inline(always)]
    pub fn sai1_mclk2_sel_5(self) -> &'a mut W {
        self.variant(SAI1_MCLK2_SEL_A::SAI1_MCLK2_SEL_5)
    }
}
#[doc = "Field `SAI1_MCLK3_SEL` reader - SAI1 MCLK3 source select"]
pub type SAI1_MCLK3_SEL_R = crate::FieldReader<u8, SAI1_MCLK3_SEL_A>;
#[doc = "SAI1 MCLK3 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI1_MCLK3_SEL_A {
    #[doc = "0: ccm.spdif0_clk_root"]
    SAI1_MCLK3_SEL_0 = 0,
    #[doc = "1: SPDIF_EXT_CLK"]
    SAI1_MCLK3_SEL_1 = 1,
    #[doc = "2: spdif.spdif_srclk"]
    SAI1_MCLK3_SEL_2 = 2,
    #[doc = "3: spdif.spdif_outclock"]
    SAI1_MCLK3_SEL_3 = 3,
}
impl From<SAI1_MCLK3_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI1_MCLK3_SEL_A) -> Self {
        variant as _
    }
}
impl SAI1_MCLK3_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1_MCLK3_SEL_A {
        match self.bits {
            0 => SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_0,
            1 => SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_1,
            2 => SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_2,
            3 => SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_0`"]
    #[inline(always)]
    pub fn is_sai1_mclk3_sel_0(&self) -> bool {
        *self == SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_1`"]
    #[inline(always)]
    pub fn is_sai1_mclk3_sel_1(&self) -> bool {
        *self == SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_2`"]
    #[inline(always)]
    pub fn is_sai1_mclk3_sel_2(&self) -> bool {
        *self == SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK3_SEL_3`"]
    #[inline(always)]
    pub fn is_sai1_mclk3_sel_3(&self) -> bool {
        *self == SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_3
    }
}
#[doc = "Field `SAI1_MCLK3_SEL` writer - SAI1 MCLK3 source select"]
pub type SAI1_MCLK3_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPR1_SPEC, u8, SAI1_MCLK3_SEL_A, 2, O>;
impl<'a, const O: u8> SAI1_MCLK3_SEL_W<'a, O> {
    #[doc = "ccm.spdif0_clk_root"]
    #[inline(always)]
    pub fn sai1_mclk3_sel_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_0)
    }
    #[doc = "SPDIF_EXT_CLK"]
    #[inline(always)]
    pub fn sai1_mclk3_sel_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_1)
    }
    #[doc = "spdif.spdif_srclk"]
    #[inline(always)]
    pub fn sai1_mclk3_sel_2(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_2)
    }
    #[doc = "spdif.spdif_outclock"]
    #[inline(always)]
    pub fn sai1_mclk3_sel_3(self) -> &'a mut W {
        self.variant(SAI1_MCLK3_SEL_A::SAI1_MCLK3_SEL_3)
    }
}
#[doc = "Field `SAI2_MCLK3_SEL` reader - SAI2 MCLK3 source select"]
pub type SAI2_MCLK3_SEL_R = crate::FieldReader<u8, SAI2_MCLK3_SEL_A>;
#[doc = "SAI2 MCLK3 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI2_MCLK3_SEL_A {
    #[doc = "0: ccm.spdif0_clk_root"]
    SAI2_MCLK3_SEL_0 = 0,
    #[doc = "1: SPDIF_EXT_CLK"]
    SAI2_MCLK3_SEL_1 = 1,
    #[doc = "2: spdif.spdif_srclk"]
    SAI2_MCLK3_SEL_2 = 2,
    #[doc = "3: spdif.spdif_outclock"]
    SAI2_MCLK3_SEL_3 = 3,
}
impl From<SAI2_MCLK3_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI2_MCLK3_SEL_A) -> Self {
        variant as _
    }
}
impl SAI2_MCLK3_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_MCLK3_SEL_A {
        match self.bits {
            0 => SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_0,
            1 => SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_1,
            2 => SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_2,
            3 => SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_0`"]
    #[inline(always)]
    pub fn is_sai2_mclk3_sel_0(&self) -> bool {
        *self == SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_1`"]
    #[inline(always)]
    pub fn is_sai2_mclk3_sel_1(&self) -> bool {
        *self == SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_2`"]
    #[inline(always)]
    pub fn is_sai2_mclk3_sel_2(&self) -> bool {
        *self == SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK3_SEL_3`"]
    #[inline(always)]
    pub fn is_sai2_mclk3_sel_3(&self) -> bool {
        *self == SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_3
    }
}
#[doc = "Field `SAI2_MCLK3_SEL` writer - SAI2 MCLK3 source select"]
pub type SAI2_MCLK3_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPR1_SPEC, u8, SAI2_MCLK3_SEL_A, 2, O>;
impl<'a, const O: u8> SAI2_MCLK3_SEL_W<'a, O> {
    #[doc = "ccm.spdif0_clk_root"]
    #[inline(always)]
    pub fn sai2_mclk3_sel_0(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_0)
    }
    #[doc = "SPDIF_EXT_CLK"]
    #[inline(always)]
    pub fn sai2_mclk3_sel_1(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_1)
    }
    #[doc = "spdif.spdif_srclk"]
    #[inline(always)]
    pub fn sai2_mclk3_sel_2(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_2)
    }
    #[doc = "spdif.spdif_outclock"]
    #[inline(always)]
    pub fn sai2_mclk3_sel_3(self) -> &'a mut W {
        self.variant(SAI2_MCLK3_SEL_A::SAI2_MCLK3_SEL_3)
    }
}
#[doc = "Field `SAI3_MCLK3_SEL` reader - SAI3 MCLK3 source select"]
pub type SAI3_MCLK3_SEL_R = crate::FieldReader<u8, SAI3_MCLK3_SEL_A>;
#[doc = "SAI3 MCLK3 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SAI3_MCLK3_SEL_A {
    #[doc = "0: ccm.spdif0_clk_root"]
    SAI3_MCLK3_SEL_0 = 0,
    #[doc = "1: SPDIF_EXT_CLK"]
    SAI3_MCLK3_SEL_1 = 1,
    #[doc = "2: spdif.spdif_srclk"]
    SAI3_MCLK3_SEL_2 = 2,
    #[doc = "3: spdif.spdif_outclock"]
    SAI3_MCLK3_SEL_3 = 3,
}
impl From<SAI3_MCLK3_SEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SAI3_MCLK3_SEL_A) -> Self {
        variant as _
    }
}
impl SAI3_MCLK3_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3_MCLK3_SEL_A {
        match self.bits {
            0 => SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_0,
            1 => SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_1,
            2 => SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_2,
            3 => SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_0`"]
    #[inline(always)]
    pub fn is_sai3_mclk3_sel_0(&self) -> bool {
        *self == SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_0
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_1`"]
    #[inline(always)]
    pub fn is_sai3_mclk3_sel_1(&self) -> bool {
        *self == SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_1
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_2`"]
    #[inline(always)]
    pub fn is_sai3_mclk3_sel_2(&self) -> bool {
        *self == SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_2
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK3_SEL_3`"]
    #[inline(always)]
    pub fn is_sai3_mclk3_sel_3(&self) -> bool {
        *self == SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_3
    }
}
#[doc = "Field `SAI3_MCLK3_SEL` writer - SAI3 MCLK3 source select"]
pub type SAI3_MCLK3_SEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, GPR1_SPEC, u8, SAI3_MCLK3_SEL_A, 2, O>;
impl<'a, const O: u8> SAI3_MCLK3_SEL_W<'a, O> {
    #[doc = "ccm.spdif0_clk_root"]
    #[inline(always)]
    pub fn sai3_mclk3_sel_0(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_0)
    }
    #[doc = "SPDIF_EXT_CLK"]
    #[inline(always)]
    pub fn sai3_mclk3_sel_1(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_1)
    }
    #[doc = "spdif.spdif_srclk"]
    #[inline(always)]
    pub fn sai3_mclk3_sel_2(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_2)
    }
    #[doc = "spdif.spdif_outclock"]
    #[inline(always)]
    pub fn sai3_mclk3_sel_3(self) -> &'a mut W {
        self.variant(SAI3_MCLK3_SEL_A::SAI3_MCLK3_SEL_3)
    }
}
#[doc = "Field `GINT` reader - Global Interrupt"]
pub type GINT_R = crate::BitReader<GINT_A>;
#[doc = "Global Interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GINT_A {
    #[doc = "0: Global interrupt request is not asserted"]
    GINT_0 = 0,
    #[doc = "1: Global interrupt request is asserted"]
    GINT_1 = 1,
}
impl From<GINT_A> for bool {
    #[inline(always)]
    fn from(variant: GINT_A) -> Self {
        variant as u8 != 0
    }
}
impl GINT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GINT_A {
        match self.bits {
            false => GINT_A::GINT_0,
            true => GINT_A::GINT_1,
        }
    }
    #[doc = "Checks if the value of the field is `GINT_0`"]
    #[inline(always)]
    pub fn is_gint_0(&self) -> bool {
        *self == GINT_A::GINT_0
    }
    #[doc = "Checks if the value of the field is `GINT_1`"]
    #[inline(always)]
    pub fn is_gint_1(&self) -> bool {
        *self == GINT_A::GINT_1
    }
}
#[doc = "Field `GINT` writer - Global Interrupt"]
pub type GINT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR1_SPEC, GINT_A, O>;
impl<'a, const O: u8> GINT_W<'a, O> {
    #[doc = "Global interrupt request is not asserted"]
    #[inline(always)]
    pub fn gint_0(self) -> &'a mut W {
        self.variant(GINT_A::GINT_0)
    }
    #[doc = "Global interrupt request is asserted"]
    #[inline(always)]
    pub fn gint_1(self) -> &'a mut W {
        self.variant(GINT_A::GINT_1)
    }
}
#[doc = "Field `ENET_TX_CLK_SEL` reader - ENET_TX_CLK select"]
pub type ENET_TX_CLK_SEL_R = crate::BitReader<ENET_TX_CLK_SEL_A>;
#[doc = "ENET_TX_CLK select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENET_TX_CLK_SEL_A {
    #[doc = "0: Do not use."]
    ENET_TX_CLK_SEL_0 = 0,
    #[doc = "1: ENET_TX_CLK is the 25MHz MII clock."]
    ENET_TX_CLK_SEL_1 = 1,
}
impl From<ENET_TX_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_TX_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENET_TX_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_TX_CLK_SEL_A {
        match self.bits {
            false => ENET_TX_CLK_SEL_A::ENET_TX_CLK_SEL_0,
            true => ENET_TX_CLK_SEL_A::ENET_TX_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_TX_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_enet_tx_clk_sel_0(&self) -> bool {
        *self == ENET_TX_CLK_SEL_A::ENET_TX_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `ENET_TX_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_enet_tx_clk_sel_1(&self) -> bool {
        *self == ENET_TX_CLK_SEL_A::ENET_TX_CLK_SEL_1
    }
}
#[doc = "Field `ENET_TX_CLK_SEL` writer - ENET_TX_CLK select"]
pub type ENET_TX_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR1_SPEC, ENET_TX_CLK_SEL_A, O>;
impl<'a, const O: u8> ENET_TX_CLK_SEL_W<'a, O> {
    #[doc = "Do not use."]
    #[inline(always)]
    pub fn enet_tx_clk_sel_0(self) -> &'a mut W {
        self.variant(ENET_TX_CLK_SEL_A::ENET_TX_CLK_SEL_0)
    }
    #[doc = "ENET_TX_CLK is the 25MHz MII clock."]
    #[inline(always)]
    pub fn enet_tx_clk_sel_1(self) -> &'a mut W {
        self.variant(ENET_TX_CLK_SEL_A::ENET_TX_CLK_SEL_1)
    }
}
#[doc = "Field `ENET_REF_CLK_DIR` reader - This bitfield controls the direction of ENET_REF_CLK. ENET_REF_CLK is the 50MHz RMII clock."]
pub type ENET_REF_CLK_DIR_R = crate::BitReader<ENET_REF_CLK_DIR_A>;
#[doc = "This bitfield controls the direction of ENET_REF_CLK. ENET_REF_CLK is the 50MHz RMII clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENET_REF_CLK_DIR_A {
    #[doc = "0: ENET_REF_CLK is input"]
    ENET_REF_CLK_DIR_0 = 0,
    #[doc = "1: ENET_REF_CLK is output driven by ref_enetpll0"]
    ENET_REF_CLK_DIR_1 = 1,
}
impl From<ENET_REF_CLK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_REF_CLK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl ENET_REF_CLK_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_REF_CLK_DIR_A {
        match self.bits {
            false => ENET_REF_CLK_DIR_A::ENET_REF_CLK_DIR_0,
            true => ENET_REF_CLK_DIR_A::ENET_REF_CLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_REF_CLK_DIR_0`"]
    #[inline(always)]
    pub fn is_enet_ref_clk_dir_0(&self) -> bool {
        *self == ENET_REF_CLK_DIR_A::ENET_REF_CLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `ENET_REF_CLK_DIR_1`"]
    #[inline(always)]
    pub fn is_enet_ref_clk_dir_1(&self) -> bool {
        *self == ENET_REF_CLK_DIR_A::ENET_REF_CLK_DIR_1
    }
}
#[doc = "Field `ENET_REF_CLK_DIR` writer - This bitfield controls the direction of ENET_REF_CLK. ENET_REF_CLK is the 50MHz RMII clock."]
pub type ENET_REF_CLK_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR1_SPEC, ENET_REF_CLK_DIR_A, O>;
impl<'a, const O: u8> ENET_REF_CLK_DIR_W<'a, O> {
    #[doc = "ENET_REF_CLK is input"]
    #[inline(always)]
    pub fn enet_ref_clk_dir_0(self) -> &'a mut W {
        self.variant(ENET_REF_CLK_DIR_A::ENET_REF_CLK_DIR_0)
    }
    #[doc = "ENET_REF_CLK is output driven by ref_enetpll0"]
    #[inline(always)]
    pub fn enet_ref_clk_dir_1(self) -> &'a mut W {
        self.variant(ENET_REF_CLK_DIR_A::ENET_REF_CLK_DIR_1)
    }
}
#[doc = "Field `SAI1_MCLK_DIR` reader - sai1.MCLK signal direction control"]
pub type SAI1_MCLK_DIR_R = crate::BitReader<SAI1_MCLK_DIR_A>;
#[doc = "sai1.MCLK signal direction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI1_MCLK_DIR_A {
    #[doc = "0: sai1.MCLK is input signal"]
    SAI1_MCLK_DIR_0 = 0,
    #[doc = "1: sai1.MCLK is output signal"]
    SAI1_MCLK_DIR_1 = 1,
}
impl From<SAI1_MCLK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: SAI1_MCLK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI1_MCLK_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI1_MCLK_DIR_A {
        match self.bits {
            false => SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_0,
            true => SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK_DIR_0`"]
    #[inline(always)]
    pub fn is_sai1_mclk_dir_0(&self) -> bool {
        *self == SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `SAI1_MCLK_DIR_1`"]
    #[inline(always)]
    pub fn is_sai1_mclk_dir_1(&self) -> bool {
        *self == SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_1
    }
}
#[doc = "Field `SAI1_MCLK_DIR` writer - sai1.MCLK signal direction control"]
pub type SAI1_MCLK_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR1_SPEC, SAI1_MCLK_DIR_A, O>;
impl<'a, const O: u8> SAI1_MCLK_DIR_W<'a, O> {
    #[doc = "sai1.MCLK is input signal"]
    #[inline(always)]
    pub fn sai1_mclk_dir_0(self) -> &'a mut W {
        self.variant(SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_0)
    }
    #[doc = "sai1.MCLK is output signal"]
    #[inline(always)]
    pub fn sai1_mclk_dir_1(self) -> &'a mut W {
        self.variant(SAI1_MCLK_DIR_A::SAI1_MCLK_DIR_1)
    }
}
#[doc = "Field `SAI2_MCLK_DIR` reader - sai2.MCLK signal direction control"]
pub type SAI2_MCLK_DIR_R = crate::BitReader<SAI2_MCLK_DIR_A>;
#[doc = "sai2.MCLK signal direction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI2_MCLK_DIR_A {
    #[doc = "0: sai2.MCLK is input signal"]
    SAI2_MCLK_DIR_0 = 0,
    #[doc = "1: sai2.MCLK is output signal"]
    SAI2_MCLK_DIR_1 = 1,
}
impl From<SAI2_MCLK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: SAI2_MCLK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI2_MCLK_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI2_MCLK_DIR_A {
        match self.bits {
            false => SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_0,
            true => SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK_DIR_0`"]
    #[inline(always)]
    pub fn is_sai2_mclk_dir_0(&self) -> bool {
        *self == SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `SAI2_MCLK_DIR_1`"]
    #[inline(always)]
    pub fn is_sai2_mclk_dir_1(&self) -> bool {
        *self == SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_1
    }
}
#[doc = "Field `SAI2_MCLK_DIR` writer - sai2.MCLK signal direction control"]
pub type SAI2_MCLK_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR1_SPEC, SAI2_MCLK_DIR_A, O>;
impl<'a, const O: u8> SAI2_MCLK_DIR_W<'a, O> {
    #[doc = "sai2.MCLK is input signal"]
    #[inline(always)]
    pub fn sai2_mclk_dir_0(self) -> &'a mut W {
        self.variant(SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_0)
    }
    #[doc = "sai2.MCLK is output signal"]
    #[inline(always)]
    pub fn sai2_mclk_dir_1(self) -> &'a mut W {
        self.variant(SAI2_MCLK_DIR_A::SAI2_MCLK_DIR_1)
    }
}
#[doc = "Field `SAI3_MCLK_DIR` reader - sai3.MCLK signal direction control"]
pub type SAI3_MCLK_DIR_R = crate::BitReader<SAI3_MCLK_DIR_A>;
#[doc = "sai3.MCLK signal direction control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAI3_MCLK_DIR_A {
    #[doc = "0: sai3.MCLK is input signal"]
    SAI3_MCLK_DIR_0 = 0,
    #[doc = "1: sai3.MCLK is output signal"]
    SAI3_MCLK_DIR_1 = 1,
}
impl From<SAI3_MCLK_DIR_A> for bool {
    #[inline(always)]
    fn from(variant: SAI3_MCLK_DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl SAI3_MCLK_DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAI3_MCLK_DIR_A {
        match self.bits {
            false => SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_0,
            true => SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_1,
        }
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK_DIR_0`"]
    #[inline(always)]
    pub fn is_sai3_mclk_dir_0(&self) -> bool {
        *self == SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_0
    }
    #[doc = "Checks if the value of the field is `SAI3_MCLK_DIR_1`"]
    #[inline(always)]
    pub fn is_sai3_mclk_dir_1(&self) -> bool {
        *self == SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_1
    }
}
#[doc = "Field `SAI3_MCLK_DIR` writer - sai3.MCLK signal direction control"]
pub type SAI3_MCLK_DIR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR1_SPEC, SAI3_MCLK_DIR_A, O>;
impl<'a, const O: u8> SAI3_MCLK_DIR_W<'a, O> {
    #[doc = "sai3.MCLK is input signal"]
    #[inline(always)]
    pub fn sai3_mclk_dir_0(self) -> &'a mut W {
        self.variant(SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_0)
    }
    #[doc = "sai3.MCLK is output signal"]
    #[inline(always)]
    pub fn sai3_mclk_dir_1(self) -> &'a mut W {
        self.variant(SAI3_MCLK_DIR_A::SAI3_MCLK_DIR_1)
    }
}
#[doc = "Field `EXC_MON` reader - Exclusive monitor response select of illegal command"]
pub type EXC_MON_R = crate::BitReader<EXC_MON_A>;
#[doc = "Exclusive monitor response select of illegal command\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXC_MON_A {
    #[doc = "0: OKAY response"]
    EXC_MON_0 = 0,
    #[doc = "1: SLVError response"]
    EXC_MON_1 = 1,
}
impl From<EXC_MON_A> for bool {
    #[inline(always)]
    fn from(variant: EXC_MON_A) -> Self {
        variant as u8 != 0
    }
}
impl EXC_MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXC_MON_A {
        match self.bits {
            false => EXC_MON_A::EXC_MON_0,
            true => EXC_MON_A::EXC_MON_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXC_MON_0`"]
    #[inline(always)]
    pub fn is_exc_mon_0(&self) -> bool {
        *self == EXC_MON_A::EXC_MON_0
    }
    #[doc = "Checks if the value of the field is `EXC_MON_1`"]
    #[inline(always)]
    pub fn is_exc_mon_1(&self) -> bool {
        *self == EXC_MON_A::EXC_MON_1
    }
}
#[doc = "Field `EXC_MON` writer - Exclusive monitor response select of illegal command"]
pub type EXC_MON_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR1_SPEC, EXC_MON_A, O>;
impl<'a, const O: u8> EXC_MON_W<'a, O> {
    #[doc = "OKAY response"]
    #[inline(always)]
    pub fn exc_mon_0(self) -> &'a mut W {
        self.variant(EXC_MON_A::EXC_MON_0)
    }
    #[doc = "SLVError response"]
    #[inline(always)]
    pub fn exc_mon_1(self) -> &'a mut W {
        self.variant(EXC_MON_A::EXC_MON_1)
    }
}
#[doc = "Field `CM7_FORCE_HCLK_EN` reader - Arm CM7 platform AHB clock enable"]
pub type CM7_FORCE_HCLK_EN_R = crate::BitReader<CM7_FORCE_HCLK_EN_A>;
#[doc = "Arm CM7 platform AHB clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CM7_FORCE_HCLK_EN_A {
    #[doc = "0: AHB clock is not running (gated) when CM7 is sleeping and TCM is not accessible"]
    CM7_FORCE_HCLK_EN_0 = 0,
    #[doc = "1: AHB clock is running (enabled) when CM7 is sleeping and TCM is accessible"]
    CM7_FORCE_HCLK_EN_1 = 1,
}
impl From<CM7_FORCE_HCLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: CM7_FORCE_HCLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl CM7_FORCE_HCLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CM7_FORCE_HCLK_EN_A {
        match self.bits {
            false => CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_0,
            true => CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `CM7_FORCE_HCLK_EN_0`"]
    #[inline(always)]
    pub fn is_cm7_force_hclk_en_0(&self) -> bool {
        *self == CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_0
    }
    #[doc = "Checks if the value of the field is `CM7_FORCE_HCLK_EN_1`"]
    #[inline(always)]
    pub fn is_cm7_force_hclk_en_1(&self) -> bool {
        *self == CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_1
    }
}
#[doc = "Field `CM7_FORCE_HCLK_EN` writer - Arm CM7 platform AHB clock enable"]
pub type CM7_FORCE_HCLK_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR1_SPEC, CM7_FORCE_HCLK_EN_A, O>;
impl<'a, const O: u8> CM7_FORCE_HCLK_EN_W<'a, O> {
    #[doc = "AHB clock is not running (gated) when CM7 is sleeping and TCM is not accessible"]
    #[inline(always)]
    pub fn cm7_force_hclk_en_0(self) -> &'a mut W {
        self.variant(CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_0)
    }
    #[doc = "AHB clock is running (enabled) when CM7 is sleeping and TCM is accessible"]
    #[inline(always)]
    pub fn cm7_force_hclk_en_1(self) -> &'a mut W {
        self.variant(CM7_FORCE_HCLK_EN_A::CM7_FORCE_HCLK_EN_1)
    }
}
impl R {
    #[doc = "Bits 0:2 - SAI1 MCLK1 source select"]
    #[inline(always)]
    pub fn sai1_mclk1_sel(&self) -> SAI1_MCLK1_SEL_R {
        SAI1_MCLK1_SEL_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SAI1 MCLK2 source select"]
    #[inline(always)]
    pub fn sai1_mclk2_sel(&self) -> SAI1_MCLK2_SEL_R {
        SAI1_MCLK2_SEL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:7 - SAI1 MCLK3 source select"]
    #[inline(always)]
    pub fn sai1_mclk3_sel(&self) -> SAI1_MCLK3_SEL_R {
        SAI1_MCLK3_SEL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - SAI2 MCLK3 source select"]
    #[inline(always)]
    pub fn sai2_mclk3_sel(&self) -> SAI2_MCLK3_SEL_R {
        SAI2_MCLK3_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - SAI3 MCLK3 source select"]
    #[inline(always)]
    pub fn sai3_mclk3_sel(&self) -> SAI3_MCLK3_SEL_R {
        SAI3_MCLK3_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Global Interrupt"]
    #[inline(always)]
    pub fn gint(&self) -> GINT_R {
        GINT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ENET_TX_CLK select"]
    #[inline(always)]
    pub fn enet_tx_clk_sel(&self) -> ENET_TX_CLK_SEL_R {
        ENET_TX_CLK_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 17 - This bitfield controls the direction of ENET_REF_CLK. ENET_REF_CLK is the 50MHz RMII clock."]
    #[inline(always)]
    pub fn enet_ref_clk_dir(&self) -> ENET_REF_CLK_DIR_R {
        ENET_REF_CLK_DIR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - sai1.MCLK signal direction control"]
    #[inline(always)]
    pub fn sai1_mclk_dir(&self) -> SAI1_MCLK_DIR_R {
        SAI1_MCLK_DIR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - sai2.MCLK signal direction control"]
    #[inline(always)]
    pub fn sai2_mclk_dir(&self) -> SAI2_MCLK_DIR_R {
        SAI2_MCLK_DIR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - sai3.MCLK signal direction control"]
    #[inline(always)]
    pub fn sai3_mclk_dir(&self) -> SAI3_MCLK_DIR_R {
        SAI3_MCLK_DIR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Exclusive monitor response select of illegal command"]
    #[inline(always)]
    pub fn exc_mon(&self) -> EXC_MON_R {
        EXC_MON_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 31 - Arm CM7 platform AHB clock enable"]
    #[inline(always)]
    pub fn cm7_force_hclk_en(&self) -> CM7_FORCE_HCLK_EN_R {
        CM7_FORCE_HCLK_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI1 MCLK1 source select"]
    #[inline(always)]
    #[must_use]
    pub fn sai1_mclk1_sel(&mut self) -> SAI1_MCLK1_SEL_W<0> {
        SAI1_MCLK1_SEL_W::new(self)
    }
    #[doc = "Bits 3:5 - SAI1 MCLK2 source select"]
    #[inline(always)]
    #[must_use]
    pub fn sai1_mclk2_sel(&mut self) -> SAI1_MCLK2_SEL_W<3> {
        SAI1_MCLK2_SEL_W::new(self)
    }
    #[doc = "Bits 6:7 - SAI1 MCLK3 source select"]
    #[inline(always)]
    #[must_use]
    pub fn sai1_mclk3_sel(&mut self) -> SAI1_MCLK3_SEL_W<6> {
        SAI1_MCLK3_SEL_W::new(self)
    }
    #[doc = "Bits 8:9 - SAI2 MCLK3 source select"]
    #[inline(always)]
    #[must_use]
    pub fn sai2_mclk3_sel(&mut self) -> SAI2_MCLK3_SEL_W<8> {
        SAI2_MCLK3_SEL_W::new(self)
    }
    #[doc = "Bits 10:11 - SAI3 MCLK3 source select"]
    #[inline(always)]
    #[must_use]
    pub fn sai3_mclk3_sel(&mut self) -> SAI3_MCLK3_SEL_W<10> {
        SAI3_MCLK3_SEL_W::new(self)
    }
    #[doc = "Bit 12 - Global Interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn gint(&mut self) -> GINT_W<12> {
        GINT_W::new(self)
    }
    #[doc = "Bit 13 - ENET_TX_CLK select"]
    #[inline(always)]
    #[must_use]
    pub fn enet_tx_clk_sel(&mut self) -> ENET_TX_CLK_SEL_W<13> {
        ENET_TX_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 17 - This bitfield controls the direction of ENET_REF_CLK. ENET_REF_CLK is the 50MHz RMII clock."]
    #[inline(always)]
    #[must_use]
    pub fn enet_ref_clk_dir(&mut self) -> ENET_REF_CLK_DIR_W<17> {
        ENET_REF_CLK_DIR_W::new(self)
    }
    #[doc = "Bit 19 - sai1.MCLK signal direction control"]
    #[inline(always)]
    #[must_use]
    pub fn sai1_mclk_dir(&mut self) -> SAI1_MCLK_DIR_W<19> {
        SAI1_MCLK_DIR_W::new(self)
    }
    #[doc = "Bit 20 - sai2.MCLK signal direction control"]
    #[inline(always)]
    #[must_use]
    pub fn sai2_mclk_dir(&mut self) -> SAI2_MCLK_DIR_W<20> {
        SAI2_MCLK_DIR_W::new(self)
    }
    #[doc = "Bit 21 - sai3.MCLK signal direction control"]
    #[inline(always)]
    #[must_use]
    pub fn sai3_mclk_dir(&mut self) -> SAI3_MCLK_DIR_W<21> {
        SAI3_MCLK_DIR_W::new(self)
    }
    #[doc = "Bit 22 - Exclusive monitor response select of illegal command"]
    #[inline(always)]
    #[must_use]
    pub fn exc_mon(&mut self) -> EXC_MON_W<22> {
        EXC_MON_W::new(self)
    }
    #[doc = "Bit 31 - Arm CM7 platform AHB clock enable"]
    #[inline(always)]
    #[must_use]
    pub fn cm7_force_hclk_en(&mut self) -> CM7_FORCE_HCLK_EN_W<31> {
        CM7_FORCE_HCLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR1 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr1](index.html) module"]
pub struct GPR1_SPEC;
impl crate::RegisterSpec for GPR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr1::R](R) reader structure"]
impl crate::Readable for GPR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr1::W](W) writer structure"]
impl crate::Writable for GPR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR1 to value 0"]
impl crate::Resettable for GPR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
