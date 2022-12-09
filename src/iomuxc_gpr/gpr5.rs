#[doc = "Register `GPR5` reader"]
pub struct R(crate::R<GPR5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR5` writer"]
pub struct W(crate::W<GPR5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR5_SPEC>;
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
impl From<crate::W<GPR5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDOG1_MASK` reader - WDOG1 Timeout Mask"]
pub type WDOG1_MASK_R = crate::BitReader<WDOG1_MASK_A>;
#[doc = "WDOG1 Timeout Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOG1_MASK_A {
    #[doc = "0: WDOG1 Timeout behaves normally"]
    WDOG1_MASK_0 = 0,
    #[doc = "1: WDOG1 Timeout is masked"]
    WDOG1_MASK_1 = 1,
}
impl From<WDOG1_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG1_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOG1_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG1_MASK_A {
        match self.bits {
            false => WDOG1_MASK_A::WDOG1_MASK_0,
            true => WDOG1_MASK_A::WDOG1_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG1_MASK_0`"]
    #[inline(always)]
    pub fn is_wdog1_mask_0(&self) -> bool {
        *self == WDOG1_MASK_A::WDOG1_MASK_0
    }
    #[doc = "Checks if the value of the field is `WDOG1_MASK_1`"]
    #[inline(always)]
    pub fn is_wdog1_mask_1(&self) -> bool {
        *self == WDOG1_MASK_A::WDOG1_MASK_1
    }
}
#[doc = "Field `WDOG1_MASK` writer - WDOG1 Timeout Mask"]
pub type WDOG1_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR5_SPEC, WDOG1_MASK_A, O>;
impl<'a, const O: u8> WDOG1_MASK_W<'a, O> {
    #[doc = "WDOG1 Timeout behaves normally"]
    #[inline(always)]
    pub fn wdog1_mask_0(self) -> &'a mut W {
        self.variant(WDOG1_MASK_A::WDOG1_MASK_0)
    }
    #[doc = "WDOG1 Timeout is masked"]
    #[inline(always)]
    pub fn wdog1_mask_1(self) -> &'a mut W {
        self.variant(WDOG1_MASK_A::WDOG1_MASK_1)
    }
}
#[doc = "Field `WDOG2_MASK` reader - WDOG2 Timeout Mask"]
pub type WDOG2_MASK_R = crate::BitReader<WDOG2_MASK_A>;
#[doc = "WDOG2 Timeout Mask\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDOG2_MASK_A {
    #[doc = "0: WDOG2 Timeout behaves normally"]
    WDOG2_MASK_0 = 0,
    #[doc = "1: WDOG2 Timeout is masked"]
    WDOG2_MASK_1 = 1,
}
impl From<WDOG2_MASK_A> for bool {
    #[inline(always)]
    fn from(variant: WDOG2_MASK_A) -> Self {
        variant as u8 != 0
    }
}
impl WDOG2_MASK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDOG2_MASK_A {
        match self.bits {
            false => WDOG2_MASK_A::WDOG2_MASK_0,
            true => WDOG2_MASK_A::WDOG2_MASK_1,
        }
    }
    #[doc = "Checks if the value of the field is `WDOG2_MASK_0`"]
    #[inline(always)]
    pub fn is_wdog2_mask_0(&self) -> bool {
        *self == WDOG2_MASK_A::WDOG2_MASK_0
    }
    #[doc = "Checks if the value of the field is `WDOG2_MASK_1`"]
    #[inline(always)]
    pub fn is_wdog2_mask_1(&self) -> bool {
        *self == WDOG2_MASK_A::WDOG2_MASK_1
    }
}
#[doc = "Field `WDOG2_MASK` writer - WDOG2 Timeout Mask"]
pub type WDOG2_MASK_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR5_SPEC, WDOG2_MASK_A, O>;
impl<'a, const O: u8> WDOG2_MASK_W<'a, O> {
    #[doc = "WDOG2 Timeout behaves normally"]
    #[inline(always)]
    pub fn wdog2_mask_0(self) -> &'a mut W {
        self.variant(WDOG2_MASK_A::WDOG2_MASK_0)
    }
    #[doc = "WDOG2 Timeout is masked"]
    #[inline(always)]
    pub fn wdog2_mask_1(self) -> &'a mut W {
        self.variant(WDOG2_MASK_A::WDOG2_MASK_1)
    }
}
#[doc = "Field `GPT2_CAPIN1_SEL` reader - GPT2 input capture channel 1 source select"]
pub type GPT2_CAPIN1_SEL_R = crate::BitReader<GPT2_CAPIN1_SEL_A>;
#[doc = "GPT2 input capture channel 1 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPT2_CAPIN1_SEL_A {
    #[doc = "0: source from GPT2_CAPTURE1"]
    GPT2_CAPIN1_SEL_0 = 0,
    #[doc = "1: source from ENET_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)"]
    GPT2_CAPIN1_SEL_1 = 1,
}
impl From<GPT2_CAPIN1_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: GPT2_CAPIN1_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl GPT2_CAPIN1_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPT2_CAPIN1_SEL_A {
        match self.bits {
            false => GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_0,
            true => GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN1_SEL_0`"]
    #[inline(always)]
    pub fn is_gpt2_capin1_sel_0(&self) -> bool {
        *self == GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_0
    }
    #[doc = "Checks if the value of the field is `GPT2_CAPIN1_SEL_1`"]
    #[inline(always)]
    pub fn is_gpt2_capin1_sel_1(&self) -> bool {
        *self == GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_1
    }
}
#[doc = "Field `GPT2_CAPIN1_SEL` writer - GPT2 input capture channel 1 source select"]
pub type GPT2_CAPIN1_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR5_SPEC, GPT2_CAPIN1_SEL_A, O>;
impl<'a, const O: u8> GPT2_CAPIN1_SEL_W<'a, O> {
    #[doc = "source from GPT2_CAPTURE1"]
    #[inline(always)]
    pub fn gpt2_capin1_sel_0(self) -> &'a mut W {
        self.variant(GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_0)
    }
    #[doc = "source from ENET_1588_EVENT3_OUT (chnnal 3 of IEEE 1588 timer)"]
    #[inline(always)]
    pub fn gpt2_capin1_sel_1(self) -> &'a mut W {
        self.variant(GPT2_CAPIN1_SEL_A::GPT2_CAPIN1_SEL_1)
    }
}
#[doc = "Field `ENET_EVENT3IN_SEL` reader - ENET input timer event3 source select"]
pub type ENET_EVENT3IN_SEL_R = crate::BitReader<ENET_EVENT3IN_SEL_A>;
#[doc = "ENET input timer event3 source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENET_EVENT3IN_SEL_A {
    #[doc = "0: event3 source input from ENET_1588_EVENT3_IN"]
    ENET_EVENT3IN_SEL_0 = 0,
    #[doc = "1: event3 source input from GPT2.GPT_COMPARE1"]
    ENET_EVENT3IN_SEL_1 = 1,
}
impl From<ENET_EVENT3IN_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: ENET_EVENT3IN_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENET_EVENT3IN_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENET_EVENT3IN_SEL_A {
        match self.bits {
            false => ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_0,
            true => ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENET_EVENT3IN_SEL_0`"]
    #[inline(always)]
    pub fn is_enet_event3in_sel_0(&self) -> bool {
        *self == ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_0
    }
    #[doc = "Checks if the value of the field is `ENET_EVENT3IN_SEL_1`"]
    #[inline(always)]
    pub fn is_enet_event3in_sel_1(&self) -> bool {
        *self == ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_1
    }
}
#[doc = "Field `ENET_EVENT3IN_SEL` writer - ENET input timer event3 source select"]
pub type ENET_EVENT3IN_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR5_SPEC, ENET_EVENT3IN_SEL_A, O>;
impl<'a, const O: u8> ENET_EVENT3IN_SEL_W<'a, O> {
    #[doc = "event3 source input from ENET_1588_EVENT3_IN"]
    #[inline(always)]
    pub fn enet_event3in_sel_0(self) -> &'a mut W {
        self.variant(ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_0)
    }
    #[doc = "event3 source input from GPT2.GPT_COMPARE1"]
    #[inline(always)]
    pub fn enet_event3in_sel_1(self) -> &'a mut W {
        self.variant(ENET_EVENT3IN_SEL_A::ENET_EVENT3IN_SEL_1)
    }
}
#[doc = "Field `VREF_1M_CLK_GPT1` reader - GPT1 1 MHz clock source select"]
pub type VREF_1M_CLK_GPT1_R = crate::BitReader<VREF_1M_CLK_GPT1_A>;
#[doc = "GPT1 1 MHz clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREF_1M_CLK_GPT1_A {
    #[doc = "0: GPT1 ipg_clk_highfreq driven by IPG_PERCLK. IPG_PERCLK is derived from either BUS clock or OSC_24M clock. See CCM chapter for more information"]
    VREF_1M_CLK_GPT1_0 = 0,
    #[doc = "1: GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock. Anatop 1M clock is derived from the OSC_RC_24M clock. It has two versions: corrected by 32k clock or un-corrected. See the XTALOSC24M_OSC_CONFIG2 register for more details"]
    VREF_1M_CLK_GPT1_1 = 1,
}
impl From<VREF_1M_CLK_GPT1_A> for bool {
    #[inline(always)]
    fn from(variant: VREF_1M_CLK_GPT1_A) -> Self {
        variant as u8 != 0
    }
}
impl VREF_1M_CLK_GPT1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF_1M_CLK_GPT1_A {
        match self.bits {
            false => VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_0,
            true => VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_1,
        }
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT1_0`"]
    #[inline(always)]
    pub fn is_vref_1m_clk_gpt1_0(&self) -> bool {
        *self == VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_0
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT1_1`"]
    #[inline(always)]
    pub fn is_vref_1m_clk_gpt1_1(&self) -> bool {
        *self == VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_1
    }
}
#[doc = "Field `VREF_1M_CLK_GPT1` writer - GPT1 1 MHz clock source select"]
pub type VREF_1M_CLK_GPT1_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR5_SPEC, VREF_1M_CLK_GPT1_A, O>;
impl<'a, const O: u8> VREF_1M_CLK_GPT1_W<'a, O> {
    #[doc = "GPT1 ipg_clk_highfreq driven by IPG_PERCLK. IPG_PERCLK is derived from either BUS clock or OSC_24M clock. See CCM chapter for more information"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt1_0(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_0)
    }
    #[doc = "GPT1 ipg_clk_highfreq driven by anatop 1 MHz clock. Anatop 1M clock is derived from the OSC_RC_24M clock. It has two versions: corrected by 32k clock or un-corrected. See the XTALOSC24M_OSC_CONFIG2 register for more details"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt1_1(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT1_A::VREF_1M_CLK_GPT1_1)
    }
}
#[doc = "Field `VREF_1M_CLK_GPT2` reader - GPT2 1 MHz clock source select"]
pub type VREF_1M_CLK_GPT2_R = crate::BitReader<VREF_1M_CLK_GPT2_A>;
#[doc = "GPT2 1 MHz clock source select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREF_1M_CLK_GPT2_A {
    #[doc = "0: GPT2 ipg_clk_highfreq driven by IPG_PERCLK. IPG_PERCLK is derived from either BUS clock or OSC_24M clock. See CCM chapter for more information"]
    VREF_1M_CLK_GPT2_0 = 0,
    #[doc = "1: GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock. Anatop 1M clock is derived from the OSC_RC_24M clock. It has two versions: corrected by 32k clock or un-corrected. See the XTALOSC24M_OSC_CONFIG2 register for more details"]
    VREF_1M_CLK_GPT2_1 = 1,
}
impl From<VREF_1M_CLK_GPT2_A> for bool {
    #[inline(always)]
    fn from(variant: VREF_1M_CLK_GPT2_A) -> Self {
        variant as u8 != 0
    }
}
impl VREF_1M_CLK_GPT2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREF_1M_CLK_GPT2_A {
        match self.bits {
            false => VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_0,
            true => VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_1,
        }
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT2_0`"]
    #[inline(always)]
    pub fn is_vref_1m_clk_gpt2_0(&self) -> bool {
        *self == VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_0
    }
    #[doc = "Checks if the value of the field is `VREF_1M_CLK_GPT2_1`"]
    #[inline(always)]
    pub fn is_vref_1m_clk_gpt2_1(&self) -> bool {
        *self == VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_1
    }
}
#[doc = "Field `VREF_1M_CLK_GPT2` writer - GPT2 1 MHz clock source select"]
pub type VREF_1M_CLK_GPT2_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR5_SPEC, VREF_1M_CLK_GPT2_A, O>;
impl<'a, const O: u8> VREF_1M_CLK_GPT2_W<'a, O> {
    #[doc = "GPT2 ipg_clk_highfreq driven by IPG_PERCLK. IPG_PERCLK is derived from either BUS clock or OSC_24M clock. See CCM chapter for more information"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt2_0(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_0)
    }
    #[doc = "GPT2 ipg_clk_highfreq driven by anatop 1 MHz clock. Anatop 1M clock is derived from the OSC_RC_24M clock. It has two versions: corrected by 32k clock or un-corrected. See the XTALOSC24M_OSC_CONFIG2 register for more details"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt2_1(self) -> &'a mut W {
        self.variant(VREF_1M_CLK_GPT2_A::VREF_1M_CLK_GPT2_1)
    }
}
impl R {
    #[doc = "Bit 6 - WDOG1 Timeout Mask"]
    #[inline(always)]
    pub fn wdog1_mask(&self) -> WDOG1_MASK_R {
        WDOG1_MASK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - WDOG2 Timeout Mask"]
    #[inline(always)]
    pub fn wdog2_mask(&self) -> WDOG2_MASK_R {
        WDOG2_MASK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 23 - GPT2 input capture channel 1 source select"]
    #[inline(always)]
    pub fn gpt2_capin1_sel(&self) -> GPT2_CAPIN1_SEL_R {
        GPT2_CAPIN1_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 25 - ENET input timer event3 source select"]
    #[inline(always)]
    pub fn enet_event3in_sel(&self) -> ENET_EVENT3IN_SEL_R {
        ENET_EVENT3IN_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 28 - GPT1 1 MHz clock source select"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt1(&self) -> VREF_1M_CLK_GPT1_R {
        VREF_1M_CLK_GPT1_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - GPT2 1 MHz clock source select"]
    #[inline(always)]
    pub fn vref_1m_clk_gpt2(&self) -> VREF_1M_CLK_GPT2_R {
        VREF_1M_CLK_GPT2_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - WDOG1 Timeout Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wdog1_mask(&mut self) -> WDOG1_MASK_W<6> {
        WDOG1_MASK_W::new(self)
    }
    #[doc = "Bit 7 - WDOG2 Timeout Mask"]
    #[inline(always)]
    #[must_use]
    pub fn wdog2_mask(&mut self) -> WDOG2_MASK_W<7> {
        WDOG2_MASK_W::new(self)
    }
    #[doc = "Bit 23 - GPT2 input capture channel 1 source select"]
    #[inline(always)]
    #[must_use]
    pub fn gpt2_capin1_sel(&mut self) -> GPT2_CAPIN1_SEL_W<23> {
        GPT2_CAPIN1_SEL_W::new(self)
    }
    #[doc = "Bit 25 - ENET input timer event3 source select"]
    #[inline(always)]
    #[must_use]
    pub fn enet_event3in_sel(&mut self) -> ENET_EVENT3IN_SEL_W<25> {
        ENET_EVENT3IN_SEL_W::new(self)
    }
    #[doc = "Bit 28 - GPT1 1 MHz clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn vref_1m_clk_gpt1(&mut self) -> VREF_1M_CLK_GPT1_W<28> {
        VREF_1M_CLK_GPT1_W::new(self)
    }
    #[doc = "Bit 29 - GPT2 1 MHz clock source select"]
    #[inline(always)]
    #[must_use]
    pub fn vref_1m_clk_gpt2(&mut self) -> VREF_1M_CLK_GPT2_W<29> {
        VREF_1M_CLK_GPT2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR5 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr5](index.html) module"]
pub struct GPR5_SPEC;
impl crate::RegisterSpec for GPR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr5::R](R) reader structure"]
impl crate::Readable for GPR5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr5::W](W) writer structure"]
impl crate::Writable for GPR5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR5 to value 0"]
impl crate::Resettable for GPR5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
