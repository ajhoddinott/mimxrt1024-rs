#[doc = "Register `CMEOR` reader"]
pub struct R(crate::R<CMEOR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMEOR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMEOR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMEOR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMEOR` writer"]
pub struct W(crate::W<CMEOR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMEOR_SPEC>;
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
impl From<crate::W<CMEOR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMEOR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MOD_EN_OV_GPT` reader - Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
pub type MOD_EN_OV_GPT_R = crate::BitReader<MOD_EN_OV_GPT_A>;
#[doc = "Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOD_EN_OV_GPT_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_OV_GPT_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_OV_GPT_1 = 1,
}
impl From<MOD_EN_OV_GPT_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_GPT_A) -> Self {
        variant as u8 != 0
    }
}
impl MOD_EN_OV_GPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_GPT_A {
        match self.bits {
            false => MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_0,
            true => MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_GPT_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_gpt_0(&self) -> bool {
        *self == MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_GPT_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_gpt_1(&self) -> bool {
        *self == MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_1
    }
}
#[doc = "Field `MOD_EN_OV_GPT` writer - Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
pub type MOD_EN_OV_GPT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CMEOR_SPEC, MOD_EN_OV_GPT_A, O>;
impl<'a, const O: u8> MOD_EN_OV_GPT_W<'a, O> {
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_gpt_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_gpt_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_GPT_A::MOD_EN_OV_GPT_1)
    }
}
#[doc = "Field `MOD_EN_OV_PIT` reader - Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
pub type MOD_EN_OV_PIT_R = crate::BitReader<MOD_EN_OV_PIT_A>;
#[doc = "Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOD_EN_OV_PIT_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_OV_PIT_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_OV_PIT_1 = 1,
}
impl From<MOD_EN_OV_PIT_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_PIT_A) -> Self {
        variant as u8 != 0
    }
}
impl MOD_EN_OV_PIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_PIT_A {
        match self.bits {
            false => MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_0,
            true => MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_PIT_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_pit_0(&self) -> bool {
        *self == MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_PIT_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_pit_1(&self) -> bool {
        *self == MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_1
    }
}
#[doc = "Field `MOD_EN_OV_PIT` writer - Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
pub type MOD_EN_OV_PIT_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CMEOR_SPEC, MOD_EN_OV_PIT_A, O>;
impl<'a, const O: u8> MOD_EN_OV_PIT_W<'a, O> {
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_pit_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_pit_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_PIT_A::MOD_EN_OV_PIT_1)
    }
}
#[doc = "Field `MOD_EN_USDHC` reader - overide clock enable signal from USDHC."]
pub type MOD_EN_USDHC_R = crate::BitReader<MOD_EN_USDHC_A>;
#[doc = "overide clock enable signal from USDHC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOD_EN_USDHC_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_USDHC_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_USDHC_1 = 1,
}
impl From<MOD_EN_USDHC_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_USDHC_A) -> Self {
        variant as u8 != 0
    }
}
impl MOD_EN_USDHC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_USDHC_A {
        match self.bits {
            false => MOD_EN_USDHC_A::MOD_EN_USDHC_0,
            true => MOD_EN_USDHC_A::MOD_EN_USDHC_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_USDHC_0`"]
    #[inline(always)]
    pub fn is_mod_en_usdhc_0(&self) -> bool {
        *self == MOD_EN_USDHC_A::MOD_EN_USDHC_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_USDHC_1`"]
    #[inline(always)]
    pub fn is_mod_en_usdhc_1(&self) -> bool {
        *self == MOD_EN_USDHC_A::MOD_EN_USDHC_1
    }
}
#[doc = "Field `MOD_EN_USDHC` writer - overide clock enable signal from USDHC."]
pub type MOD_EN_USDHC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CMEOR_SPEC, MOD_EN_USDHC_A, O>;
impl<'a, const O: u8> MOD_EN_USDHC_W<'a, O> {
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_usdhc_0(self) -> &'a mut W {
        self.variant(MOD_EN_USDHC_A::MOD_EN_USDHC_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_usdhc_1(self) -> &'a mut W {
        self.variant(MOD_EN_USDHC_A::MOD_EN_USDHC_1)
    }
}
#[doc = "Field `MOD_EN_OV_TRNG` reader - Overide clock enable signal from TRNG"]
pub type MOD_EN_OV_TRNG_R = crate::BitReader<MOD_EN_OV_TRNG_A>;
#[doc = "Overide clock enable signal from TRNG\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOD_EN_OV_TRNG_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_OV_TRNG_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_OV_TRNG_1 = 1,
}
impl From<MOD_EN_OV_TRNG_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_TRNG_A) -> Self {
        variant as u8 != 0
    }
}
impl MOD_EN_OV_TRNG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_TRNG_A {
        match self.bits {
            false => MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_0,
            true => MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_TRNG_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_trng_0(&self) -> bool {
        *self == MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_TRNG_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_trng_1(&self) -> bool {
        *self == MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_1
    }
}
#[doc = "Field `MOD_EN_OV_TRNG` writer - Overide clock enable signal from TRNG"]
pub type MOD_EN_OV_TRNG_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CMEOR_SPEC, MOD_EN_OV_TRNG_A, O>;
impl<'a, const O: u8> MOD_EN_OV_TRNG_W<'a, O> {
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_trng_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_trng_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_TRNG_A::MOD_EN_OV_TRNG_1)
    }
}
#[doc = "Field `MOD_EN_OV_CAN2_CPI` reader - Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
pub type MOD_EN_OV_CAN2_CPI_R = crate::BitReader<MOD_EN_OV_CAN2_CPI_A>;
#[doc = "Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOD_EN_OV_CAN2_CPI_A {
    #[doc = "0: don't override module enable signal"]
    MOD_EN_OV_CAN2_CPI_0 = 0,
    #[doc = "1: override module enable signal"]
    MOD_EN_OV_CAN2_CPI_1 = 1,
}
impl From<MOD_EN_OV_CAN2_CPI_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_CAN2_CPI_A) -> Self {
        variant as u8 != 0
    }
}
impl MOD_EN_OV_CAN2_CPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_CAN2_CPI_A {
        match self.bits {
            false => MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_0,
            true => MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN2_CPI_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_can2_cpi_0(&self) -> bool {
        *self == MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN2_CPI_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_can2_cpi_1(&self) -> bool {
        *self == MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_1
    }
}
#[doc = "Field `MOD_EN_OV_CAN2_CPI` writer - Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
pub type MOD_EN_OV_CAN2_CPI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CMEOR_SPEC, MOD_EN_OV_CAN2_CPI_A, O>;
impl<'a, const O: u8> MOD_EN_OV_CAN2_CPI_W<'a, O> {
    #[doc = "don't override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_can2_cpi_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_0)
    }
    #[doc = "override module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_can2_cpi_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN2_CPI_A::MOD_EN_OV_CAN2_CPI_1)
    }
}
#[doc = "Field `MOD_EN_OV_CAN1_CPI` reader - Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
pub type MOD_EN_OV_CAN1_CPI_R = crate::BitReader<MOD_EN_OV_CAN1_CPI_A>;
#[doc = "Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MOD_EN_OV_CAN1_CPI_A {
    #[doc = "0: don't overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_0 = 0,
    #[doc = "1: overide module enable signal"]
    MOD_EN_OV_CAN1_CPI_1 = 1,
}
impl From<MOD_EN_OV_CAN1_CPI_A> for bool {
    #[inline(always)]
    fn from(variant: MOD_EN_OV_CAN1_CPI_A) -> Self {
        variant as u8 != 0
    }
}
impl MOD_EN_OV_CAN1_CPI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MOD_EN_OV_CAN1_CPI_A {
        match self.bits {
            false => MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_0,
            true => MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_1,
        }
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN1_CPI_0`"]
    #[inline(always)]
    pub fn is_mod_en_ov_can1_cpi_0(&self) -> bool {
        *self == MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_0
    }
    #[doc = "Checks if the value of the field is `MOD_EN_OV_CAN1_CPI_1`"]
    #[inline(always)]
    pub fn is_mod_en_ov_can1_cpi_1(&self) -> bool {
        *self == MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_1
    }
}
#[doc = "Field `MOD_EN_OV_CAN1_CPI` writer - Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
pub type MOD_EN_OV_CAN1_CPI_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CMEOR_SPEC, MOD_EN_OV_CAN1_CPI_A, O>;
impl<'a, const O: u8> MOD_EN_OV_CAN1_CPI_W<'a, O> {
    #[doc = "don't overide module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_can1_cpi_0(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_0)
    }
    #[doc = "overide module enable signal"]
    #[inline(always)]
    pub fn mod_en_ov_can1_cpi_1(self) -> &'a mut W {
        self.variant(MOD_EN_OV_CAN1_CPI_A::MOD_EN_OV_CAN1_CPI_1)
    }
}
impl R {
    #[doc = "Bit 5 - Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub fn mod_en_ov_gpt(&self) -> MOD_EN_OV_GPT_R {
        MOD_EN_OV_GPT_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    pub fn mod_en_ov_pit(&self) -> MOD_EN_OV_PIT_R {
        MOD_EN_OV_PIT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - overide clock enable signal from USDHC."]
    #[inline(always)]
    pub fn mod_en_usdhc(&self) -> MOD_EN_USDHC_R {
        MOD_EN_USDHC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Overide clock enable signal from TRNG"]
    #[inline(always)]
    pub fn mod_en_ov_trng(&self) -> MOD_EN_OV_TRNG_R {
        MOD_EN_OV_TRNG_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 28 - Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub fn mod_en_ov_can2_cpi(&self) -> MOD_EN_OV_CAN2_CPI_R {
        MOD_EN_OV_CAN2_CPI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    pub fn mod_en_ov_can1_cpi(&self) -> MOD_EN_OV_CAN1_CPI_R {
        MOD_EN_OV_CAN1_CPI_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Overide clock enable signal from GPT - clock will not be gated based on GPT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    #[must_use]
    pub fn mod_en_ov_gpt(&mut self) -> MOD_EN_OV_GPT_W<5> {
        MOD_EN_OV_GPT_W::new(self)
    }
    #[doc = "Bit 6 - Overide clock enable signal from PIT - clock will not be gated based on PIT's signal 'ipg_enable_clk'"]
    #[inline(always)]
    #[must_use]
    pub fn mod_en_ov_pit(&mut self) -> MOD_EN_OV_PIT_W<6> {
        MOD_EN_OV_PIT_W::new(self)
    }
    #[doc = "Bit 7 - overide clock enable signal from USDHC."]
    #[inline(always)]
    #[must_use]
    pub fn mod_en_usdhc(&mut self) -> MOD_EN_USDHC_W<7> {
        MOD_EN_USDHC_W::new(self)
    }
    #[doc = "Bit 9 - Overide clock enable signal from TRNG"]
    #[inline(always)]
    #[must_use]
    pub fn mod_en_ov_trng(&mut self) -> MOD_EN_OV_TRNG_W<9> {
        MOD_EN_OV_TRNG_W::new(self)
    }
    #[doc = "Bit 28 - Overide clock enable signal from CAN2 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    #[must_use]
    pub fn mod_en_ov_can2_cpi(&mut self) -> MOD_EN_OV_CAN2_CPI_W<28> {
        MOD_EN_OV_CAN2_CPI_W::new(self)
    }
    #[doc = "Bit 30 - Overide clock enable signal from CAN1 - clock will not be gated based on CAN's signal 'enable_clk_cpi'"]
    #[inline(always)]
    #[must_use]
    pub fn mod_en_ov_can1_cpi(&mut self) -> MOD_EN_OV_CAN1_CPI_W<30> {
        MOD_EN_OV_CAN1_CPI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Module Enable Overide Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmeor](index.html) module"]
pub struct CMEOR_SPEC;
impl crate::RegisterSpec for CMEOR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmeor::R](R) reader structure"]
impl crate::Readable for CMEOR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmeor::W](W) writer structure"]
impl crate::Writable for CMEOR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMEOR to value 0xffff_ffff"]
impl crate::Resettable for CMEOR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
