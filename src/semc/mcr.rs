#[doc = "Register `MCR` reader"]
pub struct R(crate::R<MCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR` writer"]
pub struct W(crate::W<MCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR_SPEC>;
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
impl From<crate::W<MCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<SWRST_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRST_A {
    #[doc = "0: No reset"]
    SWRST_0 = 0,
    #[doc = "1: Reset"]
    SWRST_1 = 1,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRST_A {
        match self.bits {
            false => SWRST_A::SWRST_0,
            true => SWRST_A::SWRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SWRST_0`"]
    #[inline(always)]
    pub fn is_swrst_0(&self) -> bool {
        *self == SWRST_A::SWRST_0
    }
    #[doc = "Checks if the value of the field is `SWRST_1`"]
    #[inline(always)]
    pub fn is_swrst_1(&self) -> bool {
        *self == SWRST_A::SWRST_1
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, SWRST_A, O>;
impl<'a, const O: u8> SWRST_W<'a, O> {
    #[doc = "No reset"]
    #[inline(always)]
    pub fn swrst_0(self) -> &'a mut W {
        self.variant(SWRST_A::SWRST_0)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn swrst_1(self) -> &'a mut W {
        self.variant(SWRST_A::SWRST_1)
    }
}
#[doc = "Field `MDIS` reader - Module Disable"]
pub type MDIS_R = crate::BitReader<MDIS_A>;
#[doc = "Module Disable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MDIS_A {
    #[doc = "0: Module enabled"]
    MDIS_0 = 0,
    #[doc = "1: Module disabled"]
    MDIS_1 = 1,
}
impl From<MDIS_A> for bool {
    #[inline(always)]
    fn from(variant: MDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl MDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MDIS_A {
        match self.bits {
            false => MDIS_A::MDIS_0,
            true => MDIS_A::MDIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `MDIS_0`"]
    #[inline(always)]
    pub fn is_mdis_0(&self) -> bool {
        *self == MDIS_A::MDIS_0
    }
    #[doc = "Checks if the value of the field is `MDIS_1`"]
    #[inline(always)]
    pub fn is_mdis_1(&self) -> bool {
        *self == MDIS_A::MDIS_1
    }
}
#[doc = "Field `MDIS` writer - Module Disable"]
pub type MDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, MDIS_A, O>;
impl<'a, const O: u8> MDIS_W<'a, O> {
    #[doc = "Module enabled"]
    #[inline(always)]
    pub fn mdis_0(self) -> &'a mut W {
        self.variant(MDIS_A::MDIS_0)
    }
    #[doc = "Module disabled"]
    #[inline(always)]
    pub fn mdis_1(self) -> &'a mut W {
        self.variant(MDIS_A::MDIS_1)
    }
}
#[doc = "Field `DQSMD` reader - DQS (read strobe) mode"]
pub type DQSMD_R = crate::BitReader<DQSMD_A>;
#[doc = "DQS (read strobe) mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DQSMD_A {
    #[doc = "0: Dummy read strobe loopbacked internally"]
    DQSMD_0 = 0,
    #[doc = "1: Dummy read strobe loopbacked from DQS pad"]
    DQSMD_1 = 1,
}
impl From<DQSMD_A> for bool {
    #[inline(always)]
    fn from(variant: DQSMD_A) -> Self {
        variant as u8 != 0
    }
}
impl DQSMD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DQSMD_A {
        match self.bits {
            false => DQSMD_A::DQSMD_0,
            true => DQSMD_A::DQSMD_1,
        }
    }
    #[doc = "Checks if the value of the field is `DQSMD_0`"]
    #[inline(always)]
    pub fn is_dqsmd_0(&self) -> bool {
        *self == DQSMD_A::DQSMD_0
    }
    #[doc = "Checks if the value of the field is `DQSMD_1`"]
    #[inline(always)]
    pub fn is_dqsmd_1(&self) -> bool {
        *self == DQSMD_A::DQSMD_1
    }
}
#[doc = "Field `DQSMD` writer - DQS (read strobe) mode"]
pub type DQSMD_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, DQSMD_A, O>;
impl<'a, const O: u8> DQSMD_W<'a, O> {
    #[doc = "Dummy read strobe loopbacked internally"]
    #[inline(always)]
    pub fn dqsmd_0(self) -> &'a mut W {
        self.variant(DQSMD_A::DQSMD_0)
    }
    #[doc = "Dummy read strobe loopbacked from DQS pad"]
    #[inline(always)]
    pub fn dqsmd_1(self) -> &'a mut W {
        self.variant(DQSMD_A::DQSMD_1)
    }
}
#[doc = "Field `WPOL0` reader - WAIT/RDY polarity for SRAM/NOR"]
pub type WPOL0_R = crate::BitReader<WPOL0_A>;
#[doc = "WAIT/RDY polarity for SRAM/NOR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPOL0_A {
    #[doc = "0: WAIT/RDY polarity is not changed."]
    WPOL0_0 = 0,
    #[doc = "1: WAIT/RDY polarity is inverted."]
    WPOL0_1 = 1,
}
impl From<WPOL0_A> for bool {
    #[inline(always)]
    fn from(variant: WPOL0_A) -> Self {
        variant as u8 != 0
    }
}
impl WPOL0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPOL0_A {
        match self.bits {
            false => WPOL0_A::WPOL0_0,
            true => WPOL0_A::WPOL0_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPOL0_0`"]
    #[inline(always)]
    pub fn is_wpol0_0(&self) -> bool {
        *self == WPOL0_A::WPOL0_0
    }
    #[doc = "Checks if the value of the field is `WPOL0_1`"]
    #[inline(always)]
    pub fn is_wpol0_1(&self) -> bool {
        *self == WPOL0_A::WPOL0_1
    }
}
#[doc = "Field `WPOL0` writer - WAIT/RDY polarity for SRAM/NOR"]
pub type WPOL0_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, WPOL0_A, O>;
impl<'a, const O: u8> WPOL0_W<'a, O> {
    #[doc = "WAIT/RDY polarity is not changed."]
    #[inline(always)]
    pub fn wpol0_0(self) -> &'a mut W {
        self.variant(WPOL0_A::WPOL0_0)
    }
    #[doc = "WAIT/RDY polarity is inverted."]
    #[inline(always)]
    pub fn wpol0_1(self) -> &'a mut W {
        self.variant(WPOL0_A::WPOL0_1)
    }
}
#[doc = "Field `WPOL1` reader - R/B# polarity for NAND device"]
pub type WPOL1_R = crate::BitReader<WPOL1_A>;
#[doc = "R/B# polarity for NAND device\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WPOL1_A {
    #[doc = "0: R/B# polarity is not changed."]
    WPOL1_0 = 0,
    #[doc = "1: R/B# polarity is inverted."]
    WPOL1_1 = 1,
}
impl From<WPOL1_A> for bool {
    #[inline(always)]
    fn from(variant: WPOL1_A) -> Self {
        variant as u8 != 0
    }
}
impl WPOL1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WPOL1_A {
        match self.bits {
            false => WPOL1_A::WPOL1_0,
            true => WPOL1_A::WPOL1_1,
        }
    }
    #[doc = "Checks if the value of the field is `WPOL1_0`"]
    #[inline(always)]
    pub fn is_wpol1_0(&self) -> bool {
        *self == WPOL1_A::WPOL1_0
    }
    #[doc = "Checks if the value of the field is `WPOL1_1`"]
    #[inline(always)]
    pub fn is_wpol1_1(&self) -> bool {
        *self == WPOL1_A::WPOL1_1
    }
}
#[doc = "Field `WPOL1` writer - R/B# polarity for NAND device"]
pub type WPOL1_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR_SPEC, WPOL1_A, O>;
impl<'a, const O: u8> WPOL1_W<'a, O> {
    #[doc = "R/B# polarity is not changed."]
    #[inline(always)]
    pub fn wpol1_0(self) -> &'a mut W {
        self.variant(WPOL1_A::WPOL1_0)
    }
    #[doc = "R/B# polarity is inverted."]
    #[inline(always)]
    pub fn wpol1_1(self) -> &'a mut W {
        self.variant(WPOL1_A::WPOL1_1)
    }
}
#[doc = "Field `CTO` reader - Command Execution timeout cycles"]
pub type CTO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CTO` writer - Command Execution timeout cycles"]
pub type CTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, u8, 8, O>;
#[doc = "Field `BTO` reader - Bus timeout cycles"]
pub type BTO_R = crate::FieldReader<u8, BTO_A>;
#[doc = "Bus timeout cycles\n\nValue on reset: 16"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BTO_A {
    #[doc = "0: 255*1"]
    BTO_0 = 0,
    #[doc = "1: 255*2"]
    BTO_1 = 1,
    #[doc = "31: 255*231"]
    BTO_31 = 31,
}
impl From<BTO_A> for u8 {
    #[inline(always)]
    fn from(variant: BTO_A) -> Self {
        variant as _
    }
}
impl BTO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BTO_A> {
        match self.bits {
            0 => Some(BTO_A::BTO_0),
            1 => Some(BTO_A::BTO_1),
            31 => Some(BTO_A::BTO_31),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BTO_0`"]
    #[inline(always)]
    pub fn is_bto_0(&self) -> bool {
        *self == BTO_A::BTO_0
    }
    #[doc = "Checks if the value of the field is `BTO_1`"]
    #[inline(always)]
    pub fn is_bto_1(&self) -> bool {
        *self == BTO_A::BTO_1
    }
    #[doc = "Checks if the value of the field is `BTO_31`"]
    #[inline(always)]
    pub fn is_bto_31(&self) -> bool {
        *self == BTO_A::BTO_31
    }
}
#[doc = "Field `BTO` writer - Bus timeout cycles"]
pub type BTO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR_SPEC, u8, BTO_A, 5, O>;
impl<'a, const O: u8> BTO_W<'a, O> {
    #[doc = "255*1"]
    #[inline(always)]
    pub fn bto_0(self) -> &'a mut W {
        self.variant(BTO_A::BTO_0)
    }
    #[doc = "255*2"]
    #[inline(always)]
    pub fn bto_1(self) -> &'a mut W {
        self.variant(BTO_A::BTO_1)
    }
    #[doc = "255*231"]
    #[inline(always)]
    pub fn bto_31(self) -> &'a mut W {
        self.variant(BTO_A::BTO_31)
    }
}
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline(always)]
    pub fn mdis(&self) -> MDIS_R {
        MDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DQS (read strobe) mode"]
    #[inline(always)]
    pub fn dqsmd(&self) -> DQSMD_R {
        DQSMD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - WAIT/RDY polarity for SRAM/NOR"]
    #[inline(always)]
    pub fn wpol0(&self) -> WPOL0_R {
        WPOL0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - R/B# polarity for NAND device"]
    #[inline(always)]
    pub fn wpol1(&self) -> WPOL1_R {
        WPOL1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Command Execution timeout cycles"]
    #[inline(always)]
    pub fn cto(&self) -> CTO_R {
        CTO_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Bus timeout cycles"]
    #[inline(always)]
    pub fn bto(&self) -> BTO_R {
        BTO_R::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<0> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 1 - Module Disable"]
    #[inline(always)]
    #[must_use]
    pub fn mdis(&mut self) -> MDIS_W<1> {
        MDIS_W::new(self)
    }
    #[doc = "Bit 2 - DQS (read strobe) mode"]
    #[inline(always)]
    #[must_use]
    pub fn dqsmd(&mut self) -> DQSMD_W<2> {
        DQSMD_W::new(self)
    }
    #[doc = "Bit 6 - WAIT/RDY polarity for SRAM/NOR"]
    #[inline(always)]
    #[must_use]
    pub fn wpol0(&mut self) -> WPOL0_W<6> {
        WPOL0_W::new(self)
    }
    #[doc = "Bit 7 - R/B# polarity for NAND device"]
    #[inline(always)]
    #[must_use]
    pub fn wpol1(&mut self) -> WPOL1_W<7> {
        WPOL1_W::new(self)
    }
    #[doc = "Bits 16:23 - Command Execution timeout cycles"]
    #[inline(always)]
    #[must_use]
    pub fn cto(&mut self) -> CTO_W<16> {
        CTO_W::new(self)
    }
    #[doc = "Bits 24:28 - Bus timeout cycles"]
    #[inline(always)]
    #[must_use]
    pub fn bto(&mut self) -> BTO_W<24> {
        BTO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr](index.html) module"]
pub struct MCR_SPEC;
impl crate::RegisterSpec for MCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr::R](R) reader structure"]
impl crate::Readable for MCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr::W](W) writer structure"]
impl crate::Writable for MCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR to value 0x1000_0002"]
impl crate::Resettable for MCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0002;
}
