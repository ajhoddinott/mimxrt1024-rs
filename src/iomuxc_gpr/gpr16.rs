#[doc = "Register `GPR16` reader"]
pub struct R(crate::R<GPR16_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR16_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR16_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR16_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR16` writer"]
pub struct W(crate::W<GPR16_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR16_SPEC>;
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
impl From<crate::W<GPR16_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR16_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXRAM_BANK_CFG_SEL` reader - FlexRAM bank config source select"]
pub type FLEXRAM_BANK_CFG_SEL_R = crate::BitReader<FLEXRAM_BANK_CFG_SEL_A>;
#[doc = "FlexRAM bank config source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXRAM_BANK_CFG_SEL_A {
    #[doc = "0: use fuse value to config"]
    FLEXRAM_BANK_CFG_SEL_0 = 0,
    #[doc = "1: use FLEXRAM_BANK_CFG to config"]
    FLEXRAM_BANK_CFG_SEL_1 = 1,
}
impl From<FLEXRAM_BANK_CFG_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXRAM_BANK_CFG_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXRAM_BANK_CFG_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXRAM_BANK_CFG_SEL_A {
        match self.bits {
            false => FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_0,
            true => FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FLEXRAM_BANK_CFG_SEL_0`"]
    #[inline(always)]
    pub fn is_flexram_bank_cfg_sel_0(&self) -> bool {
        *self == FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_0
    }
    #[doc = "Checks if the value of the field is `FLEXRAM_BANK_CFG_SEL_1`"]
    #[inline(always)]
    pub fn is_flexram_bank_cfg_sel_1(&self) -> bool {
        *self == FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_1
    }
}
#[doc = "Field `FLEXRAM_BANK_CFG_SEL` writer - FlexRAM bank config source select"]
pub type FLEXRAM_BANK_CFG_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR16_SPEC, FLEXRAM_BANK_CFG_SEL_A, O>;
impl<'a, const O: u8> FLEXRAM_BANK_CFG_SEL_W<'a, O> {
    #[doc = "use fuse value to config"]
    #[inline(always)]
    pub fn flexram_bank_cfg_sel_0(self) -> &'a mut W {
        self.variant(FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_0)
    }
    #[doc = "use FLEXRAM_BANK_CFG to config"]
    #[inline(always)]
    pub fn flexram_bank_cfg_sel_1(self) -> &'a mut W {
        self.variant(FLEXRAM_BANK_CFG_SEL_A::FLEXRAM_BANK_CFG_SEL_1)
    }
}
#[doc = "Field `CM7_INIT_VTOR` reader - Vector table offset register out of reset"]
pub type CM7_INIT_VTOR_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CM7_INIT_VTOR` writer - Vector table offset register out of reset"]
pub type CM7_INIT_VTOR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPR16_SPEC, u32, u32, 25, O>;
impl R {
    #[doc = "Bit 2 - FlexRAM bank config source select"]
    #[inline(always)]
    pub fn flexram_bank_cfg_sel(&self) -> FLEXRAM_BANK_CFG_SEL_R {
        FLEXRAM_BANK_CFG_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 7:31 - Vector table offset register out of reset"]
    #[inline(always)]
    pub fn cm7_init_vtor(&self) -> CM7_INIT_VTOR_R {
        CM7_INIT_VTOR_R::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bit 2 - FlexRAM bank config source select"]
    #[inline(always)]
    #[must_use]
    pub fn flexram_bank_cfg_sel(&mut self) -> FLEXRAM_BANK_CFG_SEL_W<2> {
        FLEXRAM_BANK_CFG_SEL_W::new(self)
    }
    #[doc = "Bits 7:31 - Vector table offset register out of reset"]
    #[inline(always)]
    #[must_use]
    pub fn cm7_init_vtor(&mut self) -> CM7_INIT_VTOR_W<7> {
        CM7_INIT_VTOR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR16 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr16](index.html) module"]
pub struct GPR16_SPEC;
impl crate::RegisterSpec for GPR16_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr16::R](R) reader structure"]
impl crate::Readable for GPR16_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr16::W](W) writer structure"]
impl crate::Writable for GPR16_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR16 to value 0x0020_0003"]
impl crate::Resettable for GPR16_SPEC {
    const RESET_VALUE: Self::Ux = 0x0020_0003;
}
