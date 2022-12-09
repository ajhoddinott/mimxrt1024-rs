#[doc = "Register `MIX_CTRL` reader"]
pub struct R(crate::R<MIX_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIX_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIX_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIX_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIX_CTRL` writer"]
pub struct W(crate::W<MIX_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIX_CTRL_SPEC>;
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
impl From<crate::W<MIX_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIX_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMAEN` reader - DMA enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: Disable"]
    DMAEN_0 = 0,
    #[doc = "1: Enable"]
    DMAEN_1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DMAEN_0,
            true => DMAEN_A::DMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEN_0`"]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        *self == DMAEN_A::DMAEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEN_1`"]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        *self == DMAEN_A::DMAEN_1
    }
}
#[doc = "Field `DMAEN` writer - DMA enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_1)
    }
}
#[doc = "Field `BCEN` reader - Block count enable"]
pub type BCEN_R = crate::BitReader<BCEN_A>;
#[doc = "Block count enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCEN_A {
    #[doc = "0: Disable"]
    BCEN_0 = 0,
    #[doc = "1: Enable"]
    BCEN_1 = 1,
}
impl From<BCEN_A> for bool {
    #[inline(always)]
    fn from(variant: BCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl BCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCEN_A {
        match self.bits {
            false => BCEN_A::BCEN_0,
            true => BCEN_A::BCEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BCEN_0`"]
    #[inline(always)]
    pub fn is_bcen_0(&self) -> bool {
        *self == BCEN_A::BCEN_0
    }
    #[doc = "Checks if the value of the field is `BCEN_1`"]
    #[inline(always)]
    pub fn is_bcen_1(&self) -> bool {
        *self == BCEN_A::BCEN_1
    }
}
#[doc = "Field `BCEN` writer - Block count enable"]
pub type BCEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, BCEN_A, O>;
impl<'a, const O: u8> BCEN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn bcen_0(self) -> &'a mut W {
        self.variant(BCEN_A::BCEN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn bcen_1(self) -> &'a mut W {
        self.variant(BCEN_A::BCEN_1)
    }
}
#[doc = "Field `AC12EN` reader - Auto CMD12 enable"]
pub type AC12EN_R = crate::BitReader<AC12EN_A>;
#[doc = "Auto CMD12 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12EN_A {
    #[doc = "0: Disable"]
    AC12EN_0 = 0,
    #[doc = "1: Enable"]
    AC12EN_1 = 1,
}
impl From<AC12EN_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EN_A {
        match self.bits {
            false => AC12EN_A::AC12EN_0,
            true => AC12EN_A::AC12EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12EN_0`"]
    #[inline(always)]
    pub fn is_ac12en_0(&self) -> bool {
        *self == AC12EN_A::AC12EN_0
    }
    #[doc = "Checks if the value of the field is `AC12EN_1`"]
    #[inline(always)]
    pub fn is_ac12en_1(&self) -> bool {
        *self == AC12EN_A::AC12EN_1
    }
}
#[doc = "Field `AC12EN` writer - Auto CMD12 enable"]
pub type AC12EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, AC12EN_A, O>;
impl<'a, const O: u8> AC12EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn ac12en_0(self) -> &'a mut W {
        self.variant(AC12EN_A::AC12EN_0)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn ac12en_1(self) -> &'a mut W {
        self.variant(AC12EN_A::AC12EN_1)
    }
}
#[doc = "Field `DDR_EN` reader - Dual data rate mode selection"]
pub type DDR_EN_R = crate::BitReader<bool>;
#[doc = "Field `DDR_EN` writer - Dual data rate mode selection"]
pub type DDR_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, bool, O>;
#[doc = "Field `DTDSEL` reader - Data transfer direction select"]
pub type DTDSEL_R = crate::BitReader<DTDSEL_A>;
#[doc = "Data transfer direction select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DTDSEL_A {
    #[doc = "0: Write (Host to card)"]
    DTDSEL_0 = 0,
    #[doc = "1: Read (Card to host)"]
    DTDSEL_1 = 1,
}
impl From<DTDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: DTDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DTDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DTDSEL_A {
        match self.bits {
            false => DTDSEL_A::DTDSEL_0,
            true => DTDSEL_A::DTDSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DTDSEL_0`"]
    #[inline(always)]
    pub fn is_dtdsel_0(&self) -> bool {
        *self == DTDSEL_A::DTDSEL_0
    }
    #[doc = "Checks if the value of the field is `DTDSEL_1`"]
    #[inline(always)]
    pub fn is_dtdsel_1(&self) -> bool {
        *self == DTDSEL_A::DTDSEL_1
    }
}
#[doc = "Field `DTDSEL` writer - Data transfer direction select"]
pub type DTDSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, DTDSEL_A, O>;
impl<'a, const O: u8> DTDSEL_W<'a, O> {
    #[doc = "Write (Host to card)"]
    #[inline(always)]
    pub fn dtdsel_0(self) -> &'a mut W {
        self.variant(DTDSEL_A::DTDSEL_0)
    }
    #[doc = "Read (Card to host)"]
    #[inline(always)]
    pub fn dtdsel_1(self) -> &'a mut W {
        self.variant(DTDSEL_A::DTDSEL_1)
    }
}
#[doc = "Field `MSBSEL` reader - Multi / Single block select"]
pub type MSBSEL_R = crate::BitReader<MSBSEL_A>;
#[doc = "Multi / Single block select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSBSEL_A {
    #[doc = "0: Single block"]
    MSBSEL_0 = 0,
    #[doc = "1: Multiple blocks"]
    MSBSEL_1 = 1,
}
impl From<MSBSEL_A> for bool {
    #[inline(always)]
    fn from(variant: MSBSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl MSBSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSBSEL_A {
        match self.bits {
            false => MSBSEL_A::MSBSEL_0,
            true => MSBSEL_A::MSBSEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `MSBSEL_0`"]
    #[inline(always)]
    pub fn is_msbsel_0(&self) -> bool {
        *self == MSBSEL_A::MSBSEL_0
    }
    #[doc = "Checks if the value of the field is `MSBSEL_1`"]
    #[inline(always)]
    pub fn is_msbsel_1(&self) -> bool {
        *self == MSBSEL_A::MSBSEL_1
    }
}
#[doc = "Field `MSBSEL` writer - Multi / Single block select"]
pub type MSBSEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, MSBSEL_A, O>;
impl<'a, const O: u8> MSBSEL_W<'a, O> {
    #[doc = "Single block"]
    #[inline(always)]
    pub fn msbsel_0(self) -> &'a mut W {
        self.variant(MSBSEL_A::MSBSEL_0)
    }
    #[doc = "Multiple blocks"]
    #[inline(always)]
    pub fn msbsel_1(self) -> &'a mut W {
        self.variant(MSBSEL_A::MSBSEL_1)
    }
}
#[doc = "Field `NIBBLE_POS` reader - Nibble position indication"]
pub type NIBBLE_POS_R = crate::BitReader<bool>;
#[doc = "Field `NIBBLE_POS` writer - Nibble position indication"]
pub type NIBBLE_POS_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, bool, O>;
#[doc = "Field `AC23EN` reader - Auto CMD23 enable"]
pub type AC23EN_R = crate::BitReader<bool>;
#[doc = "Field `AC23EN` writer - Auto CMD23 enable"]
pub type AC23EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, bool, O>;
#[doc = "Field `EXE_TUNE` reader - Execute tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
pub type EXE_TUNE_R = crate::BitReader<EXE_TUNE_A>;
#[doc = "Execute tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXE_TUNE_A {
    #[doc = "0: Not tuned or tuning completed"]
    EXE_TUNE_0 = 0,
    #[doc = "1: Execute tuning"]
    EXE_TUNE_1 = 1,
}
impl From<EXE_TUNE_A> for bool {
    #[inline(always)]
    fn from(variant: EXE_TUNE_A) -> Self {
        variant as u8 != 0
    }
}
impl EXE_TUNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXE_TUNE_A {
        match self.bits {
            false => EXE_TUNE_A::EXE_TUNE_0,
            true => EXE_TUNE_A::EXE_TUNE_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXE_TUNE_0`"]
    #[inline(always)]
    pub fn is_exe_tune_0(&self) -> bool {
        *self == EXE_TUNE_A::EXE_TUNE_0
    }
    #[doc = "Checks if the value of the field is `EXE_TUNE_1`"]
    #[inline(always)]
    pub fn is_exe_tune_1(&self) -> bool {
        *self == EXE_TUNE_A::EXE_TUNE_1
    }
}
#[doc = "Field `EXE_TUNE` writer - Execute tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
pub type EXE_TUNE_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, EXE_TUNE_A, O>;
impl<'a, const O: u8> EXE_TUNE_W<'a, O> {
    #[doc = "Not tuned or tuning completed"]
    #[inline(always)]
    pub fn exe_tune_0(self) -> &'a mut W {
        self.variant(EXE_TUNE_A::EXE_TUNE_0)
    }
    #[doc = "Execute tuning"]
    #[inline(always)]
    pub fn exe_tune_1(self) -> &'a mut W {
        self.variant(EXE_TUNE_A::EXE_TUNE_1)
    }
}
#[doc = "Field `SMP_CLK_SEL` reader - Clock selection"]
pub type SMP_CLK_SEL_R = crate::BitReader<SMP_CLK_SEL_A>;
#[doc = "Clock selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMP_CLK_SEL_A {
    #[doc = "0: Fixed clock is used to sample data / cmd"]
    SMP_CLK_SEL_0 = 0,
    #[doc = "1: Tuned clock is used to sample data / cmd"]
    SMP_CLK_SEL_1 = 1,
}
impl From<SMP_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SMP_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SMP_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_CLK_SEL_A {
        match self.bits {
            false => SMP_CLK_SEL_A::SMP_CLK_SEL_0,
            true => SMP_CLK_SEL_A::SMP_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_smp_clk_sel_0(&self) -> bool {
        *self == SMP_CLK_SEL_A::SMP_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_smp_clk_sel_1(&self) -> bool {
        *self == SMP_CLK_SEL_A::SMP_CLK_SEL_1
    }
}
#[doc = "Field `SMP_CLK_SEL` writer - Clock selection"]
pub type SMP_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIX_CTRL_SPEC, SMP_CLK_SEL_A, O>;
impl<'a, const O: u8> SMP_CLK_SEL_W<'a, O> {
    #[doc = "Fixed clock is used to sample data / cmd"]
    #[inline(always)]
    pub fn smp_clk_sel_0(self) -> &'a mut W {
        self.variant(SMP_CLK_SEL_A::SMP_CLK_SEL_0)
    }
    #[doc = "Tuned clock is used to sample data / cmd"]
    #[inline(always)]
    pub fn smp_clk_sel_1(self) -> &'a mut W {
        self.variant(SMP_CLK_SEL_A::SMP_CLK_SEL_1)
    }
}
#[doc = "Field `AUTO_TUNE_EN` reader - Auto tuning enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
pub type AUTO_TUNE_EN_R = crate::BitReader<AUTO_TUNE_EN_A>;
#[doc = "Auto tuning enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTO_TUNE_EN_A {
    #[doc = "0: Disable auto tuning"]
    AUTO_TUNE_EN_0 = 0,
    #[doc = "1: Enable auto tuning"]
    AUTO_TUNE_EN_1 = 1,
}
impl From<AUTO_TUNE_EN_A> for bool {
    #[inline(always)]
    fn from(variant: AUTO_TUNE_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTO_TUNE_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTO_TUNE_EN_A {
        match self.bits {
            false => AUTO_TUNE_EN_A::AUTO_TUNE_EN_0,
            true => AUTO_TUNE_EN_A::AUTO_TUNE_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AUTO_TUNE_EN_0`"]
    #[inline(always)]
    pub fn is_auto_tune_en_0(&self) -> bool {
        *self == AUTO_TUNE_EN_A::AUTO_TUNE_EN_0
    }
    #[doc = "Checks if the value of the field is `AUTO_TUNE_EN_1`"]
    #[inline(always)]
    pub fn is_auto_tune_en_1(&self) -> bool {
        *self == AUTO_TUNE_EN_A::AUTO_TUNE_EN_1
    }
}
#[doc = "Field `AUTO_TUNE_EN` writer - Auto tuning enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
pub type AUTO_TUNE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, MIX_CTRL_SPEC, AUTO_TUNE_EN_A, O>;
impl<'a, const O: u8> AUTO_TUNE_EN_W<'a, O> {
    #[doc = "Disable auto tuning"]
    #[inline(always)]
    pub fn auto_tune_en_0(self) -> &'a mut W {
        self.variant(AUTO_TUNE_EN_A::AUTO_TUNE_EN_0)
    }
    #[doc = "Enable auto tuning"]
    #[inline(always)]
    pub fn auto_tune_en_1(self) -> &'a mut W {
        self.variant(AUTO_TUNE_EN_A::AUTO_TUNE_EN_1)
    }
}
#[doc = "Field `FBCLK_SEL` reader - Feedback clock source selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
pub type FBCLK_SEL_R = crate::BitReader<FBCLK_SEL_A>;
#[doc = "Feedback clock source selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FBCLK_SEL_A {
    #[doc = "0: Feedback clock comes from the loopback CLK"]
    FBCLK_SEL_0 = 0,
    #[doc = "1: Feedback clock comes from the ipp_card_clk_out"]
    FBCLK_SEL_1 = 1,
}
impl From<FBCLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: FBCLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl FBCLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FBCLK_SEL_A {
        match self.bits {
            false => FBCLK_SEL_A::FBCLK_SEL_0,
            true => FBCLK_SEL_A::FBCLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `FBCLK_SEL_0`"]
    #[inline(always)]
    pub fn is_fbclk_sel_0(&self) -> bool {
        *self == FBCLK_SEL_A::FBCLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `FBCLK_SEL_1`"]
    #[inline(always)]
    pub fn is_fbclk_sel_1(&self) -> bool {
        *self == FBCLK_SEL_A::FBCLK_SEL_1
    }
}
#[doc = "Field `FBCLK_SEL` writer - Feedback clock source selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
pub type FBCLK_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, MIX_CTRL_SPEC, FBCLK_SEL_A, O>;
impl<'a, const O: u8> FBCLK_SEL_W<'a, O> {
    #[doc = "Feedback clock comes from the loopback CLK"]
    #[inline(always)]
    pub fn fbclk_sel_0(self) -> &'a mut W {
        self.variant(FBCLK_SEL_A::FBCLK_SEL_0)
    }
    #[doc = "Feedback clock comes from the ipp_card_clk_out"]
    #[inline(always)]
    pub fn fbclk_sel_1(self) -> &'a mut W {
        self.variant(FBCLK_SEL_A::FBCLK_SEL_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Block count enable"]
    #[inline(always)]
    pub fn bcen(&self) -> BCEN_R {
        BCEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD12 enable"]
    #[inline(always)]
    pub fn ac12en(&self) -> AC12EN_R {
        AC12EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Dual data rate mode selection"]
    #[inline(always)]
    pub fn ddr_en(&self) -> DDR_EN_R {
        DDR_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction select"]
    #[inline(always)]
    pub fn dtdsel(&self) -> DTDSEL_R {
        DTDSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Multi / Single block select"]
    #[inline(always)]
    pub fn msbsel(&self) -> MSBSEL_R {
        MSBSEL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Nibble position indication"]
    #[inline(always)]
    pub fn nibble_pos(&self) -> NIBBLE_POS_R {
        NIBBLE_POS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto CMD23 enable"]
    #[inline(always)]
    pub fn ac23en(&self) -> AC23EN_R {
        AC23EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 22 - Execute tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn exe_tune(&self) -> EXE_TUNE_R {
        EXE_TUNE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Clock selection"]
    #[inline(always)]
    pub fn smp_clk_sel(&self) -> SMP_CLK_SEL_R {
        SMP_CLK_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Auto tuning enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn auto_tune_en(&self) -> AUTO_TUNE_EN_R {
        AUTO_TUNE_EN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Feedback clock source selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    pub fn fbclk_sel(&self) -> FBCLK_SEL_R {
        FBCLK_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<0> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Block count enable"]
    #[inline(always)]
    #[must_use]
    pub fn bcen(&mut self) -> BCEN_W<1> {
        BCEN_W::new(self)
    }
    #[doc = "Bit 2 - Auto CMD12 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac12en(&mut self) -> AC12EN_W<2> {
        AC12EN_W::new(self)
    }
    #[doc = "Bit 3 - Dual data rate mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn ddr_en(&mut self) -> DDR_EN_W<3> {
        DDR_EN_W::new(self)
    }
    #[doc = "Bit 4 - Data transfer direction select"]
    #[inline(always)]
    #[must_use]
    pub fn dtdsel(&mut self) -> DTDSEL_W<4> {
        DTDSEL_W::new(self)
    }
    #[doc = "Bit 5 - Multi / Single block select"]
    #[inline(always)]
    #[must_use]
    pub fn msbsel(&mut self) -> MSBSEL_W<5> {
        MSBSEL_W::new(self)
    }
    #[doc = "Bit 6 - Nibble position indication"]
    #[inline(always)]
    #[must_use]
    pub fn nibble_pos(&mut self) -> NIBBLE_POS_W<6> {
        NIBBLE_POS_W::new(self)
    }
    #[doc = "Bit 7 - Auto CMD23 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ac23en(&mut self) -> AC23EN_W<7> {
        AC23EN_W::new(self)
    }
    #[doc = "Bit 22 - Execute tuning: (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    #[must_use]
    pub fn exe_tune(&mut self) -> EXE_TUNE_W<22> {
        EXE_TUNE_W::new(self)
    }
    #[doc = "Bit 23 - Clock selection"]
    #[inline(always)]
    #[must_use]
    pub fn smp_clk_sel(&mut self) -> SMP_CLK_SEL_W<23> {
        SMP_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 24 - Auto tuning enable (Only used for SD3.0, SDR104 mode and and EMMC HS200 mode)"]
    #[inline(always)]
    #[must_use]
    pub fn auto_tune_en(&mut self) -> AUTO_TUNE_EN_W<24> {
        AUTO_TUNE_EN_W::new(self)
    }
    #[doc = "Bit 25 - Feedback clock source selection (Only used for SD3.0, SDR104 mode and EMMC HS200 mode)"]
    #[inline(always)]
    #[must_use]
    pub fn fbclk_sel(&mut self) -> FBCLK_SEL_W<25> {
        FBCLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mixer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mix_ctrl](index.html) module"]
pub struct MIX_CTRL_SPEC;
impl crate::RegisterSpec for MIX_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mix_ctrl::R](R) reader structure"]
impl crate::Readable for MIX_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mix_ctrl::W](W) writer structure"]
impl crate::Writable for MIX_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIX_CTRL to value 0x8000_0000"]
impl crate::Resettable for MIX_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
