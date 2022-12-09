#[doc = "Register `CGPR` reader"]
pub struct R(crate::R<CGPR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CGPR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CGPR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CGPR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CGPR` writer"]
pub struct W(crate::W<CGPR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CGPR_SPEC>;
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
impl From<crate::W<CGPR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CGPR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMIC_DELAY_SCALER` reader - Defines clock dividion of clock for stby_count (pmic delay counter)"]
pub type PMIC_DELAY_SCALER_R = crate::BitReader<PMIC_DELAY_SCALER_A>;
#[doc = "Defines clock dividion of clock for stby_count (pmic delay counter)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PMIC_DELAY_SCALER_A {
    #[doc = "0: clock is not divided"]
    PMIC_DELAY_SCALER_0 = 0,
    #[doc = "1: clock is divided /8"]
    PMIC_DELAY_SCALER_1 = 1,
}
impl From<PMIC_DELAY_SCALER_A> for bool {
    #[inline(always)]
    fn from(variant: PMIC_DELAY_SCALER_A) -> Self {
        variant as u8 != 0
    }
}
impl PMIC_DELAY_SCALER_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PMIC_DELAY_SCALER_A {
        match self.bits {
            false => PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_0,
            true => PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_1,
        }
    }
    #[doc = "Checks if the value of the field is `PMIC_DELAY_SCALER_0`"]
    #[inline(always)]
    pub fn is_pmic_delay_scaler_0(&self) -> bool {
        *self == PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_0
    }
    #[doc = "Checks if the value of the field is `PMIC_DELAY_SCALER_1`"]
    #[inline(always)]
    pub fn is_pmic_delay_scaler_1(&self) -> bool {
        *self == PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_1
    }
}
#[doc = "Field `PMIC_DELAY_SCALER` writer - Defines clock dividion of clock for stby_count (pmic delay counter)"]
pub type PMIC_DELAY_SCALER_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGPR_SPEC, PMIC_DELAY_SCALER_A, O>;
impl<'a, const O: u8> PMIC_DELAY_SCALER_W<'a, O> {
    #[doc = "clock is not divided"]
    #[inline(always)]
    pub fn pmic_delay_scaler_0(self) -> &'a mut W {
        self.variant(PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_0)
    }
    #[doc = "clock is divided /8"]
    #[inline(always)]
    pub fn pmic_delay_scaler_1(self) -> &'a mut W {
        self.variant(PMIC_DELAY_SCALER_A::PMIC_DELAY_SCALER_1)
    }
}
#[doc = "Field `EFUSE_PROG_SUPPLY_GATE` reader - Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
pub type EFUSE_PROG_SUPPLY_GATE_R = crate::BitReader<EFUSE_PROG_SUPPLY_GATE_A>;
#[doc = "Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EFUSE_PROG_SUPPLY_GATE_A {
    #[doc = "0: fuse programing supply voltage is gated off to the efuse module"]
    EFUSE_PROG_SUPPLY_GATE_0 = 0,
    #[doc = "1: allow fuse programing."]
    EFUSE_PROG_SUPPLY_GATE_1 = 1,
}
impl From<EFUSE_PROG_SUPPLY_GATE_A> for bool {
    #[inline(always)]
    fn from(variant: EFUSE_PROG_SUPPLY_GATE_A) -> Self {
        variant as u8 != 0
    }
}
impl EFUSE_PROG_SUPPLY_GATE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EFUSE_PROG_SUPPLY_GATE_A {
        match self.bits {
            false => EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_0,
            true => EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_1,
        }
    }
    #[doc = "Checks if the value of the field is `EFUSE_PROG_SUPPLY_GATE_0`"]
    #[inline(always)]
    pub fn is_efuse_prog_supply_gate_0(&self) -> bool {
        *self == EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_0
    }
    #[doc = "Checks if the value of the field is `EFUSE_PROG_SUPPLY_GATE_1`"]
    #[inline(always)]
    pub fn is_efuse_prog_supply_gate_1(&self) -> bool {
        *self == EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_1
    }
}
#[doc = "Field `EFUSE_PROG_SUPPLY_GATE` writer - Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
pub type EFUSE_PROG_SUPPLY_GATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGPR_SPEC, EFUSE_PROG_SUPPLY_GATE_A, O>;
impl<'a, const O: u8> EFUSE_PROG_SUPPLY_GATE_W<'a, O> {
    #[doc = "fuse programing supply voltage is gated off to the efuse module"]
    #[inline(always)]
    pub fn efuse_prog_supply_gate_0(self) -> &'a mut W {
        self.variant(EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_0)
    }
    #[doc = "allow fuse programing."]
    #[inline(always)]
    pub fn efuse_prog_supply_gate_1(self) -> &'a mut W {
        self.variant(EFUSE_PROG_SUPPLY_GATE_A::EFUSE_PROG_SUPPLY_GATE_1)
    }
}
#[doc = "Field `SYS_MEM_DS_CTRL` reader - System memory DS control"]
pub type SYS_MEM_DS_CTRL_R = crate::FieldReader<u8, SYS_MEM_DS_CTRL_A>;
#[doc = "System memory DS control\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYS_MEM_DS_CTRL_A {
    #[doc = "0: Disable memory DS mode always"]
    SYS_MEM_DS_CTRL_0 = 0,
    #[doc = "1: Enable memory (outside Arm platform) DS mode when system STOP and PLL are disabled"]
    SYS_MEM_DS_CTRL_1 = 1,
    #[doc = "2: enable memory (outside Arm platform) DS mode when system is in STOP mode"]
    SYS_MEM_DS_CTRL_2 = 2,
}
impl From<SYS_MEM_DS_CTRL_A> for u8 {
    #[inline(always)]
    fn from(variant: SYS_MEM_DS_CTRL_A) -> Self {
        variant as _
    }
}
impl SYS_MEM_DS_CTRL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYS_MEM_DS_CTRL_A> {
        match self.bits {
            0 => Some(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_0),
            1 => Some(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_1),
            2 => Some(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_2),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYS_MEM_DS_CTRL_0`"]
    #[inline(always)]
    pub fn is_sys_mem_ds_ctrl_0(&self) -> bool {
        *self == SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_0
    }
    #[doc = "Checks if the value of the field is `SYS_MEM_DS_CTRL_1`"]
    #[inline(always)]
    pub fn is_sys_mem_ds_ctrl_1(&self) -> bool {
        *self == SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_1
    }
    #[doc = "Checks if the value of the field is `SYS_MEM_DS_CTRL_2`"]
    #[inline(always)]
    pub fn is_sys_mem_ds_ctrl_2(&self) -> bool {
        *self == SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_2
    }
}
#[doc = "Field `SYS_MEM_DS_CTRL` writer - System memory DS control"]
pub type SYS_MEM_DS_CTRL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CGPR_SPEC, u8, SYS_MEM_DS_CTRL_A, 2, O>;
impl<'a, const O: u8> SYS_MEM_DS_CTRL_W<'a, O> {
    #[doc = "Disable memory DS mode always"]
    #[inline(always)]
    pub fn sys_mem_ds_ctrl_0(self) -> &'a mut W {
        self.variant(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_0)
    }
    #[doc = "Enable memory (outside Arm platform) DS mode when system STOP and PLL are disabled"]
    #[inline(always)]
    pub fn sys_mem_ds_ctrl_1(self) -> &'a mut W {
        self.variant(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_1)
    }
    #[doc = "enable memory (outside Arm platform) DS mode when system is in STOP mode"]
    #[inline(always)]
    pub fn sys_mem_ds_ctrl_2(self) -> &'a mut W {
        self.variant(SYS_MEM_DS_CTRL_A::SYS_MEM_DS_CTRL_2)
    }
}
#[doc = "Field `FPL` reader - Fast PLL enable."]
pub type FPL_R = crate::BitReader<FPL_A>;
#[doc = "Fast PLL enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FPL_A {
    #[doc = "0: Engage PLL enable default way."]
    FPL_0 = 0,
    #[doc = "1: Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
    FPL_1 = 1,
}
impl From<FPL_A> for bool {
    #[inline(always)]
    fn from(variant: FPL_A) -> Self {
        variant as u8 != 0
    }
}
impl FPL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FPL_A {
        match self.bits {
            false => FPL_A::FPL_0,
            true => FPL_A::FPL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FPL_0`"]
    #[inline(always)]
    pub fn is_fpl_0(&self) -> bool {
        *self == FPL_A::FPL_0
    }
    #[doc = "Checks if the value of the field is `FPL_1`"]
    #[inline(always)]
    pub fn is_fpl_1(&self) -> bool {
        *self == FPL_A::FPL_1
    }
}
#[doc = "Field `FPL` writer - Fast PLL enable."]
pub type FPL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CGPR_SPEC, FPL_A, O>;
impl<'a, const O: u8> FPL_W<'a, O> {
    #[doc = "Engage PLL enable default way."]
    #[inline(always)]
    pub fn fpl_0(self) -> &'a mut W {
        self.variant(FPL_A::FPL_0)
    }
    #[doc = "Engage PLL enable 3 CKIL clocks earlier at exiting low power mode (STOP). Should be used only if 24MHz OSC was active in low power mode."]
    #[inline(always)]
    pub fn fpl_1(self) -> &'a mut W {
        self.variant(FPL_A::FPL_1)
    }
}
#[doc = "Field `INT_MEM_CLK_LPM` reader - Control for the Deep Sleep signal to the Arm Platform memories with additional control logic based on the Arm WFI signal"]
pub type INT_MEM_CLK_LPM_R = crate::BitReader<INT_MEM_CLK_LPM_A>;
#[doc = "Control for the Deep Sleep signal to the Arm Platform memories with additional control logic based on the Arm WFI signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum INT_MEM_CLK_LPM_A {
    #[doc = "0: Disable the clock to the Arm platform memories when entering Low Power Mode"]
    INT_MEM_CLK_LPM_0 = 0,
    #[doc = "1: Keep the clocks to the Arm platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)"]
    INT_MEM_CLK_LPM_1 = 1,
}
impl From<INT_MEM_CLK_LPM_A> for bool {
    #[inline(always)]
    fn from(variant: INT_MEM_CLK_LPM_A) -> Self {
        variant as u8 != 0
    }
}
impl INT_MEM_CLK_LPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> INT_MEM_CLK_LPM_A {
        match self.bits {
            false => INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_0,
            true => INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_1,
        }
    }
    #[doc = "Checks if the value of the field is `INT_MEM_CLK_LPM_0`"]
    #[inline(always)]
    pub fn is_int_mem_clk_lpm_0(&self) -> bool {
        *self == INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_0
    }
    #[doc = "Checks if the value of the field is `INT_MEM_CLK_LPM_1`"]
    #[inline(always)]
    pub fn is_int_mem_clk_lpm_1(&self) -> bool {
        *self == INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_1
    }
}
#[doc = "Field `INT_MEM_CLK_LPM` writer - Control for the Deep Sleep signal to the Arm Platform memories with additional control logic based on the Arm WFI signal"]
pub type INT_MEM_CLK_LPM_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CGPR_SPEC, INT_MEM_CLK_LPM_A, O>;
impl<'a, const O: u8> INT_MEM_CLK_LPM_W<'a, O> {
    #[doc = "Disable the clock to the Arm platform memories when entering Low Power Mode"]
    #[inline(always)]
    pub fn int_mem_clk_lpm_0(self) -> &'a mut W {
        self.variant(INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_0)
    }
    #[doc = "Keep the clocks to the Arm platform memories enabled only if an interrupt is pending when entering Low Power Modes (WAIT and STOP without power gating)"]
    #[inline(always)]
    pub fn int_mem_clk_lpm_1(self) -> &'a mut W {
        self.variant(INT_MEM_CLK_LPM_A::INT_MEM_CLK_LPM_1)
    }
}
impl R {
    #[doc = "Bit 0 - Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[inline(always)]
    pub fn pmic_delay_scaler(&self) -> PMIC_DELAY_SCALER_R {
        PMIC_DELAY_SCALER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
    #[inline(always)]
    pub fn efuse_prog_supply_gate(&self) -> EFUSE_PROG_SUPPLY_GATE_R {
        EFUSE_PROG_SUPPLY_GATE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 14:15 - System memory DS control"]
    #[inline(always)]
    pub fn sys_mem_ds_ctrl(&self) -> SYS_MEM_DS_CTRL_R {
        SYS_MEM_DS_CTRL_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fast PLL enable."]
    #[inline(always)]
    pub fn fpl(&self) -> FPL_R {
        FPL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Control for the Deep Sleep signal to the Arm Platform memories with additional control logic based on the Arm WFI signal"]
    #[inline(always)]
    pub fn int_mem_clk_lpm(&self) -> INT_MEM_CLK_LPM_R {
        INT_MEM_CLK_LPM_R::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Defines clock dividion of clock for stby_count (pmic delay counter)"]
    #[inline(always)]
    #[must_use]
    pub fn pmic_delay_scaler(&mut self) -> PMIC_DELAY_SCALER_W<0> {
        PMIC_DELAY_SCALER_W::new(self)
    }
    #[doc = "Bit 4 - Defines the value of the output signal cgpr_dout\\[4\\]. Gate of program supply for efuse programing"]
    #[inline(always)]
    #[must_use]
    pub fn efuse_prog_supply_gate(&mut self) -> EFUSE_PROG_SUPPLY_GATE_W<4> {
        EFUSE_PROG_SUPPLY_GATE_W::new(self)
    }
    #[doc = "Bits 14:15 - System memory DS control"]
    #[inline(always)]
    #[must_use]
    pub fn sys_mem_ds_ctrl(&mut self) -> SYS_MEM_DS_CTRL_W<14> {
        SYS_MEM_DS_CTRL_W::new(self)
    }
    #[doc = "Bit 16 - Fast PLL enable."]
    #[inline(always)]
    #[must_use]
    pub fn fpl(&mut self) -> FPL_W<16> {
        FPL_W::new(self)
    }
    #[doc = "Bit 17 - Control for the Deep Sleep signal to the Arm Platform memories with additional control logic based on the Arm WFI signal"]
    #[inline(always)]
    #[must_use]
    pub fn int_mem_clk_lpm(&mut self) -> INT_MEM_CLK_LPM_W<17> {
        INT_MEM_CLK_LPM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cgpr](index.html) module"]
pub struct CGPR_SPEC;
impl crate::RegisterSpec for CGPR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cgpr::R](R) reader structure"]
impl crate::Readable for CGPR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cgpr::W](W) writer structure"]
impl crate::Writable for CGPR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CGPR to value 0xfe62"]
impl crate::Resettable for CGPR_SPEC {
    const RESET_VALUE: Self::Ux = 0xfe62;
}
