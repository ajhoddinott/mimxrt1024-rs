#[doc = "Register `STC` reader"]
pub struct R(crate::R<STC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STC` writer"]
pub struct W(crate::W<STC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STC_SPEC>;
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
impl From<crate::W<STC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TxClk_DF` reader - Divider factor (1-128)"]
pub type TX_CLK_DF_R = crate::FieldReader<u8, TX_CLK_DF_A>;
#[doc = "Divider factor (1-128)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_CLK_DF_A {
    #[doc = "0: divider factor is 1"]
    TX_CLK_DF_0 = 0,
    #[doc = "1: divider factor is 2"]
    TX_CLK_DF_1 = 1,
    #[doc = "127: divider factor is 128"]
    TX_CLK_DF_127 = 127,
}
impl From<TX_CLK_DF_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_CLK_DF_A) -> Self {
        variant as _
    }
}
impl TX_CLK_DF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_CLK_DF_A> {
        match self.bits {
            0 => Some(TX_CLK_DF_A::TX_CLK_DF_0),
            1 => Some(TX_CLK_DF_A::TX_CLK_DF_1),
            127 => Some(TX_CLK_DF_A::TX_CLK_DF_127),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TX_CLK_DF_0`"]
    #[inline(always)]
    pub fn is_tx_clk_df_0(&self) -> bool {
        *self == TX_CLK_DF_A::TX_CLK_DF_0
    }
    #[doc = "Checks if the value of the field is `TX_CLK_DF_1`"]
    #[inline(always)]
    pub fn is_tx_clk_df_1(&self) -> bool {
        *self == TX_CLK_DF_A::TX_CLK_DF_1
    }
    #[doc = "Checks if the value of the field is `TX_CLK_DF_127`"]
    #[inline(always)]
    pub fn is_tx_clk_df_127(&self) -> bool {
        *self == TX_CLK_DF_A::TX_CLK_DF_127
    }
}
#[doc = "Field `TxClk_DF` writer - Divider factor (1-128)"]
pub type TX_CLK_DF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STC_SPEC, u8, TX_CLK_DF_A, 7, O>;
impl<'a, const O: u8> TX_CLK_DF_W<'a, O> {
    #[doc = "divider factor is 1"]
    #[inline(always)]
    pub fn tx_clk_df_0(self) -> &'a mut W {
        self.variant(TX_CLK_DF_A::TX_CLK_DF_0)
    }
    #[doc = "divider factor is 2"]
    #[inline(always)]
    pub fn tx_clk_df_1(self) -> &'a mut W {
        self.variant(TX_CLK_DF_A::TX_CLK_DF_1)
    }
    #[doc = "divider factor is 128"]
    #[inline(always)]
    pub fn tx_clk_df_127(self) -> &'a mut W {
        self.variant(TX_CLK_DF_A::TX_CLK_DF_127)
    }
}
#[doc = "Field `tx_all_clk_en` reader - Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
pub type TX_ALL_CLK_EN_R = crate::BitReader<TX_ALL_CLK_EN_A>;
#[doc = "Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TX_ALL_CLK_EN_A {
    #[doc = "0: disable transfer clock."]
    TX_ALL_CLK_EN_0 = 0,
    #[doc = "1: enable transfer clock."]
    TX_ALL_CLK_EN_1 = 1,
}
impl From<TX_ALL_CLK_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TX_ALL_CLK_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TX_ALL_CLK_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TX_ALL_CLK_EN_A {
        match self.bits {
            false => TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_0,
            true => TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TX_ALL_CLK_EN_0`"]
    #[inline(always)]
    pub fn is_tx_all_clk_en_0(&self) -> bool {
        *self == TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_0
    }
    #[doc = "Checks if the value of the field is `TX_ALL_CLK_EN_1`"]
    #[inline(always)]
    pub fn is_tx_all_clk_en_1(&self) -> bool {
        *self == TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_1
    }
}
#[doc = "Field `tx_all_clk_en` writer - Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
pub type TX_ALL_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, STC_SPEC, TX_ALL_CLK_EN_A, O>;
impl<'a, const O: u8> TX_ALL_CLK_EN_W<'a, O> {
    #[doc = "disable transfer clock."]
    #[inline(always)]
    pub fn tx_all_clk_en_0(self) -> &'a mut W {
        self.variant(TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_0)
    }
    #[doc = "enable transfer clock."]
    #[inline(always)]
    pub fn tx_all_clk_en_1(self) -> &'a mut W {
        self.variant(TX_ALL_CLK_EN_A::TX_ALL_CLK_EN_1)
    }
}
#[doc = "Field `TxClk_Source` reader - no description available"]
pub type TX_CLK_SOURCE_R = crate::FieldReader<u8, TX_CLK_SOURCE_A>;
#[doc = "no description available\n\nValue on reset: 7"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TX_CLK_SOURCE_A {
    #[doc = "0: XTALOSC input (XTALOSC clock)"]
    TX_CLK_SOURCE_0 = 0,
    #[doc = "1: tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)"]
    TX_CLK_SOURCE_1 = 1,
    #[doc = "2: tx_clk1 (from SAI1)"]
    TX_CLK_SOURCE_2 = 2,
    #[doc = "3: tx_clk2 SPDIF_EXT_CLK, from pads"]
    TX_CLK_SOURCE_3 = 3,
    #[doc = "4: tx_clk3 (from SAI2)"]
    TX_CLK_SOURCE_4 = 4,
    #[doc = "5: ipg_clk input (frequency divided)"]
    TX_CLK_SOURCE_5 = 5,
    #[doc = "6: tx_clk4 (from SAI3)"]
    TX_CLK_SOURCE_6 = 6,
}
impl From<TX_CLK_SOURCE_A> for u8 {
    #[inline(always)]
    fn from(variant: TX_CLK_SOURCE_A) -> Self {
        variant as _
    }
}
impl TX_CLK_SOURCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TX_CLK_SOURCE_A> {
        match self.bits {
            0 => Some(TX_CLK_SOURCE_A::TX_CLK_SOURCE_0),
            1 => Some(TX_CLK_SOURCE_A::TX_CLK_SOURCE_1),
            2 => Some(TX_CLK_SOURCE_A::TX_CLK_SOURCE_2),
            3 => Some(TX_CLK_SOURCE_A::TX_CLK_SOURCE_3),
            4 => Some(TX_CLK_SOURCE_A::TX_CLK_SOURCE_4),
            5 => Some(TX_CLK_SOURCE_A::TX_CLK_SOURCE_5),
            6 => Some(TX_CLK_SOURCE_A::TX_CLK_SOURCE_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TX_CLK_SOURCE_0`"]
    #[inline(always)]
    pub fn is_tx_clk_source_0(&self) -> bool {
        *self == TX_CLK_SOURCE_A::TX_CLK_SOURCE_0
    }
    #[doc = "Checks if the value of the field is `TX_CLK_SOURCE_1`"]
    #[inline(always)]
    pub fn is_tx_clk_source_1(&self) -> bool {
        *self == TX_CLK_SOURCE_A::TX_CLK_SOURCE_1
    }
    #[doc = "Checks if the value of the field is `TX_CLK_SOURCE_2`"]
    #[inline(always)]
    pub fn is_tx_clk_source_2(&self) -> bool {
        *self == TX_CLK_SOURCE_A::TX_CLK_SOURCE_2
    }
    #[doc = "Checks if the value of the field is `TX_CLK_SOURCE_3`"]
    #[inline(always)]
    pub fn is_tx_clk_source_3(&self) -> bool {
        *self == TX_CLK_SOURCE_A::TX_CLK_SOURCE_3
    }
    #[doc = "Checks if the value of the field is `TX_CLK_SOURCE_4`"]
    #[inline(always)]
    pub fn is_tx_clk_source_4(&self) -> bool {
        *self == TX_CLK_SOURCE_A::TX_CLK_SOURCE_4
    }
    #[doc = "Checks if the value of the field is `TX_CLK_SOURCE_5`"]
    #[inline(always)]
    pub fn is_tx_clk_source_5(&self) -> bool {
        *self == TX_CLK_SOURCE_A::TX_CLK_SOURCE_5
    }
    #[doc = "Checks if the value of the field is `TX_CLK_SOURCE_6`"]
    #[inline(always)]
    pub fn is_tx_clk_source_6(&self) -> bool {
        *self == TX_CLK_SOURCE_A::TX_CLK_SOURCE_6
    }
}
#[doc = "Field `TxClk_Source` writer - no description available"]
pub type TX_CLK_SOURCE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STC_SPEC, u8, TX_CLK_SOURCE_A, 3, O>;
impl<'a, const O: u8> TX_CLK_SOURCE_W<'a, O> {
    #[doc = "XTALOSC input (XTALOSC clock)"]
    #[inline(always)]
    pub fn tx_clk_source_0(self) -> &'a mut W {
        self.variant(TX_CLK_SOURCE_A::TX_CLK_SOURCE_0)
    }
    #[doc = "tx_clk input (from SPDIF0_CLK_ROOT. See CCM.)"]
    #[inline(always)]
    pub fn tx_clk_source_1(self) -> &'a mut W {
        self.variant(TX_CLK_SOURCE_A::TX_CLK_SOURCE_1)
    }
    #[doc = "tx_clk1 (from SAI1)"]
    #[inline(always)]
    pub fn tx_clk_source_2(self) -> &'a mut W {
        self.variant(TX_CLK_SOURCE_A::TX_CLK_SOURCE_2)
    }
    #[doc = "tx_clk2 SPDIF_EXT_CLK, from pads"]
    #[inline(always)]
    pub fn tx_clk_source_3(self) -> &'a mut W {
        self.variant(TX_CLK_SOURCE_A::TX_CLK_SOURCE_3)
    }
    #[doc = "tx_clk3 (from SAI2)"]
    #[inline(always)]
    pub fn tx_clk_source_4(self) -> &'a mut W {
        self.variant(TX_CLK_SOURCE_A::TX_CLK_SOURCE_4)
    }
    #[doc = "ipg_clk input (frequency divided)"]
    #[inline(always)]
    pub fn tx_clk_source_5(self) -> &'a mut W {
        self.variant(TX_CLK_SOURCE_A::TX_CLK_SOURCE_5)
    }
    #[doc = "tx_clk4 (from SAI3)"]
    #[inline(always)]
    pub fn tx_clk_source_6(self) -> &'a mut W {
        self.variant(TX_CLK_SOURCE_A::TX_CLK_SOURCE_6)
    }
}
#[doc = "Field `SYSCLK_DF` reader - system clock divider factor, 2~512."]
pub type SYSCLK_DF_R = crate::FieldReader<u16, SYSCLK_DF_A>;
#[doc = "system clock divider factor, 2~512.\n\nValue on reset: 65"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum SYSCLK_DF_A {
    #[doc = "0: no clock signal"]
    SYSCLK_DF_0 = 0,
    #[doc = "1: divider factor is 2"]
    SYSCLK_DF_1 = 1,
    #[doc = "511: divider factor is 512"]
    SYSCLK_DF_511 = 511,
}
impl From<SYSCLK_DF_A> for u16 {
    #[inline(always)]
    fn from(variant: SYSCLK_DF_A) -> Self {
        variant as _
    }
}
impl SYSCLK_DF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSCLK_DF_A> {
        match self.bits {
            0 => Some(SYSCLK_DF_A::SYSCLK_DF_0),
            1 => Some(SYSCLK_DF_A::SYSCLK_DF_1),
            511 => Some(SYSCLK_DF_A::SYSCLK_DF_511),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK_DF_0`"]
    #[inline(always)]
    pub fn is_sysclk_df_0(&self) -> bool {
        *self == SYSCLK_DF_A::SYSCLK_DF_0
    }
    #[doc = "Checks if the value of the field is `SYSCLK_DF_1`"]
    #[inline(always)]
    pub fn is_sysclk_df_1(&self) -> bool {
        *self == SYSCLK_DF_A::SYSCLK_DF_1
    }
    #[doc = "Checks if the value of the field is `SYSCLK_DF_511`"]
    #[inline(always)]
    pub fn is_sysclk_df_511(&self) -> bool {
        *self == SYSCLK_DF_A::SYSCLK_DF_511
    }
}
#[doc = "Field `SYSCLK_DF` writer - system clock divider factor, 2~512."]
pub type SYSCLK_DF_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, STC_SPEC, u16, SYSCLK_DF_A, 9, O>;
impl<'a, const O: u8> SYSCLK_DF_W<'a, O> {
    #[doc = "no clock signal"]
    #[inline(always)]
    pub fn sysclk_df_0(self) -> &'a mut W {
        self.variant(SYSCLK_DF_A::SYSCLK_DF_0)
    }
    #[doc = "divider factor is 2"]
    #[inline(always)]
    pub fn sysclk_df_1(self) -> &'a mut W {
        self.variant(SYSCLK_DF_A::SYSCLK_DF_1)
    }
    #[doc = "divider factor is 512"]
    #[inline(always)]
    pub fn sysclk_df_511(self) -> &'a mut W {
        self.variant(SYSCLK_DF_A::SYSCLK_DF_511)
    }
}
impl R {
    #[doc = "Bits 0:6 - Divider factor (1-128)"]
    #[inline(always)]
    pub fn tx_clk_df(&self) -> TX_CLK_DF_R {
        TX_CLK_DF_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
    #[inline(always)]
    pub fn tx_all_clk_en(&self) -> TX_ALL_CLK_EN_R {
        TX_ALL_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - no description available"]
    #[inline(always)]
    pub fn tx_clk_source(&self) -> TX_CLK_SOURCE_R {
        TX_CLK_SOURCE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:19 - system clock divider factor, 2~512."]
    #[inline(always)]
    pub fn sysclk_df(&self) -> SYSCLK_DF_R {
        SYSCLK_DF_R::new(((self.bits >> 11) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - Divider factor (1-128)"]
    #[inline(always)]
    #[must_use]
    pub fn tx_clk_df(&mut self) -> TX_CLK_DF_W<0> {
        TX_CLK_DF_W::new(self)
    }
    #[doc = "Bit 7 - Spdif transfer clock enable. When data is going to be transfered, this bit should be set to1."]
    #[inline(always)]
    #[must_use]
    pub fn tx_all_clk_en(&mut self) -> TX_ALL_CLK_EN_W<7> {
        TX_ALL_CLK_EN_W::new(self)
    }
    #[doc = "Bits 8:10 - no description available"]
    #[inline(always)]
    #[must_use]
    pub fn tx_clk_source(&mut self) -> TX_CLK_SOURCE_W<8> {
        TX_CLK_SOURCE_W::new(self)
    }
    #[doc = "Bits 11:19 - system clock divider factor, 2~512."]
    #[inline(always)]
    #[must_use]
    pub fn sysclk_df(&mut self) -> SYSCLK_DF_W<11> {
        SYSCLK_DF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPDIFTxClk Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stc](index.html) module"]
pub struct STC_SPEC;
impl crate::RegisterSpec for STC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stc::R](R) reader structure"]
impl crate::Readable for STC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stc::W](W) writer structure"]
impl crate::Writable for STC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STC to value 0x0002_0f00"]
impl crate::Resettable for STC_SPEC {
    const RESET_VALUE: Self::Ux = 0x0002_0f00;
}
