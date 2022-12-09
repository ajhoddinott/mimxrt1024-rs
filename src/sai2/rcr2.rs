#[doc = "Register `RCR2` reader"]
pub struct R(crate::R<RCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RCR2` writer"]
pub struct W(crate::W<RCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RCR2_SPEC>;
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
impl From<crate::W<RCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIV` reader - Bit Clock Divide"]
pub type DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIV` writer - Bit Clock Divide"]
pub type DIV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RCR2_SPEC, u8, u8, 8, O>;
#[doc = "Field `BCD` reader - Bit Clock Direction"]
pub type BCD_R = crate::BitReader<BCD_A>;
#[doc = "Bit Clock Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCD_A {
    #[doc = "0: Bit clock is generated externally in Slave mode."]
    EXT_SLAVE_MODE = 0,
    #[doc = "1: Bit clock is generated internally in Master mode."]
    INT_MASTER_MODE = 1,
}
impl From<BCD_A> for bool {
    #[inline(always)]
    fn from(variant: BCD_A) -> Self {
        variant as u8 != 0
    }
}
impl BCD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCD_A {
        match self.bits {
            false => BCD_A::EXT_SLAVE_MODE,
            true => BCD_A::INT_MASTER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `EXT_SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_ext_slave_mode(&self) -> bool {
        *self == BCD_A::EXT_SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `INT_MASTER_MODE`"]
    #[inline(always)]
    pub fn is_int_master_mode(&self) -> bool {
        *self == BCD_A::INT_MASTER_MODE
    }
}
#[doc = "Field `BCD` writer - Bit Clock Direction"]
pub type BCD_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR2_SPEC, BCD_A, O>;
impl<'a, const O: u8> BCD_W<'a, O> {
    #[doc = "Bit clock is generated externally in Slave mode."]
    #[inline(always)]
    pub fn ext_slave_mode(self) -> &'a mut W {
        self.variant(BCD_A::EXT_SLAVE_MODE)
    }
    #[doc = "Bit clock is generated internally in Master mode."]
    #[inline(always)]
    pub fn int_master_mode(self) -> &'a mut W {
        self.variant(BCD_A::INT_MASTER_MODE)
    }
}
#[doc = "Field `BCP` reader - Bit Clock Polarity"]
pub type BCP_R = crate::BitReader<BCP_A>;
#[doc = "Bit Clock Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCP_A {
    #[doc = "0: Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    ACTIVE_HIGH = 0,
    #[doc = "1: Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    ACTIVE_LOW = 1,
}
impl From<BCP_A> for bool {
    #[inline(always)]
    fn from(variant: BCP_A) -> Self {
        variant as u8 != 0
    }
}
impl BCP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCP_A {
        match self.bits {
            false => BCP_A::ACTIVE_HIGH,
            true => BCP_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == BCP_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == BCP_A::ACTIVE_LOW
    }
}
#[doc = "Field `BCP` writer - Bit Clock Polarity"]
pub type BCP_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR2_SPEC, BCP_A, O>;
impl<'a, const O: u8> BCP_W<'a, O> {
    #[doc = "Bit Clock is active high with drive outputs on rising edge and sample inputs on falling edge."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(BCP_A::ACTIVE_HIGH)
    }
    #[doc = "Bit Clock is active low with drive outputs on falling edge and sample inputs on rising edge."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(BCP_A::ACTIVE_LOW)
    }
}
#[doc = "Field `MSEL` reader - MCLK Select"]
pub type MSEL_R = crate::FieldReader<u8, MSEL_A>;
#[doc = "MCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: Bus Clock selected."]
    BUS_CLOCK = 0,
    #[doc = "1: Master Clock (MCLK) 1 option selected."]
    MCLK1 = 1,
    #[doc = "2: Master Clock (MCLK) 2 option selected."]
    MCLK2 = 2,
    #[doc = "3: Master Clock (MCLK) 3 option selected."]
    MCLK3 = 3,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
impl MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::BUS_CLOCK,
            1 => MSEL_A::MCLK1,
            2 => MSEL_A::MCLK2,
            3 => MSEL_A::MCLK3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `BUS_CLOCK`"]
    #[inline(always)]
    pub fn is_bus_clock(&self) -> bool {
        *self == MSEL_A::BUS_CLOCK
    }
    #[doc = "Checks if the value of the field is `MCLK1`"]
    #[inline(always)]
    pub fn is_mclk1(&self) -> bool {
        *self == MSEL_A::MCLK1
    }
    #[doc = "Checks if the value of the field is `MCLK2`"]
    #[inline(always)]
    pub fn is_mclk2(&self) -> bool {
        *self == MSEL_A::MCLK2
    }
    #[doc = "Checks if the value of the field is `MCLK3`"]
    #[inline(always)]
    pub fn is_mclk3(&self) -> bool {
        *self == MSEL_A::MCLK3
    }
}
#[doc = "Field `MSEL` writer - MCLK Select"]
pub type MSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, RCR2_SPEC, u8, MSEL_A, 2, O>;
impl<'a, const O: u8> MSEL_W<'a, O> {
    #[doc = "Bus Clock selected."]
    #[inline(always)]
    pub fn bus_clock(self) -> &'a mut W {
        self.variant(MSEL_A::BUS_CLOCK)
    }
    #[doc = "Master Clock (MCLK) 1 option selected."]
    #[inline(always)]
    pub fn mclk1(self) -> &'a mut W {
        self.variant(MSEL_A::MCLK1)
    }
    #[doc = "Master Clock (MCLK) 2 option selected."]
    #[inline(always)]
    pub fn mclk2(self) -> &'a mut W {
        self.variant(MSEL_A::MCLK2)
    }
    #[doc = "Master Clock (MCLK) 3 option selected."]
    #[inline(always)]
    pub fn mclk3(self) -> &'a mut W {
        self.variant(MSEL_A::MCLK3)
    }
}
#[doc = "Field `BCI` reader - Bit Clock Input"]
pub type BCI_R = crate::BitReader<BCI_A>;
#[doc = "Bit Clock Input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCI_A {
    #[doc = "0: No effect."]
    NO_EFFECT = 0,
    #[doc = "1: Internal logic is clocked as if bit clock was externally generated."]
    CLOCKED_AS_IF_EXT_GENERATED = 1,
}
impl From<BCI_A> for bool {
    #[inline(always)]
    fn from(variant: BCI_A) -> Self {
        variant as u8 != 0
    }
}
impl BCI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCI_A {
        match self.bits {
            false => BCI_A::NO_EFFECT,
            true => BCI_A::CLOCKED_AS_IF_EXT_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EFFECT`"]
    #[inline(always)]
    pub fn is_no_effect(&self) -> bool {
        *self == BCI_A::NO_EFFECT
    }
    #[doc = "Checks if the value of the field is `CLOCKED_AS_IF_EXT_GENERATED`"]
    #[inline(always)]
    pub fn is_clocked_as_if_ext_generated(&self) -> bool {
        *self == BCI_A::CLOCKED_AS_IF_EXT_GENERATED
    }
}
#[doc = "Field `BCI` writer - Bit Clock Input"]
pub type BCI_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR2_SPEC, BCI_A, O>;
impl<'a, const O: u8> BCI_W<'a, O> {
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect(self) -> &'a mut W {
        self.variant(BCI_A::NO_EFFECT)
    }
    #[doc = "Internal logic is clocked as if bit clock was externally generated."]
    #[inline(always)]
    pub fn clocked_as_if_ext_generated(self) -> &'a mut W {
        self.variant(BCI_A::CLOCKED_AS_IF_EXT_GENERATED)
    }
}
#[doc = "Field `BCS` reader - Bit Clock Swap"]
pub type BCS_R = crate::BitReader<BCS_A>;
#[doc = "Bit Clock Swap\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BCS_A {
    #[doc = "0: Use the normal bit clock source."]
    NORMAL = 0,
    #[doc = "1: Swap the bit clock source."]
    SWAP_BIT_CLK_SOURCE = 1,
}
impl From<BCS_A> for bool {
    #[inline(always)]
    fn from(variant: BCS_A) -> Self {
        variant as u8 != 0
    }
}
impl BCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BCS_A {
        match self.bits {
            false => BCS_A::NORMAL,
            true => BCS_A::SWAP_BIT_CLK_SOURCE,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == BCS_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `SWAP_BIT_CLK_SOURCE`"]
    #[inline(always)]
    pub fn is_swap_bit_clk_source(&self) -> bool {
        *self == BCS_A::SWAP_BIT_CLK_SOURCE
    }
}
#[doc = "Field `BCS` writer - Bit Clock Swap"]
pub type BCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR2_SPEC, BCS_A, O>;
impl<'a, const O: u8> BCS_W<'a, O> {
    #[doc = "Use the normal bit clock source."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(BCS_A::NORMAL)
    }
    #[doc = "Swap the bit clock source."]
    #[inline(always)]
    pub fn swap_bit_clk_source(self) -> &'a mut W {
        self.variant(BCS_A::SWAP_BIT_CLK_SOURCE)
    }
}
#[doc = "Field `SYNC` reader - Synchronous Mode"]
pub type SYNC_R = crate::BitReader<SYNC_A>;
#[doc = "Synchronous Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SYNC_A {
    #[doc = "0: Asynchronous mode."]
    ASYNC = 0,
    #[doc = "1: Synchronous with transmitter."]
    SYNC_W_TX = 1,
}
impl From<SYNC_A> for bool {
    #[inline(always)]
    fn from(variant: SYNC_A) -> Self {
        variant as u8 != 0
    }
}
impl SYNC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SYNC_A {
        match self.bits {
            false => SYNC_A::ASYNC,
            true => SYNC_A::SYNC_W_TX,
        }
    }
    #[doc = "Checks if the value of the field is `ASYNC`"]
    #[inline(always)]
    pub fn is_async(&self) -> bool {
        *self == SYNC_A::ASYNC
    }
    #[doc = "Checks if the value of the field is `SYNC_W_TX`"]
    #[inline(always)]
    pub fn is_sync_w_tx(&self) -> bool {
        *self == SYNC_A::SYNC_W_TX
    }
}
#[doc = "Field `SYNC` writer - Synchronous Mode"]
pub type SYNC_W<'a, const O: u8> = crate::BitWriter<'a, u32, RCR2_SPEC, SYNC_A, O>;
impl<'a, const O: u8> SYNC_W<'a, O> {
    #[doc = "Asynchronous mode."]
    #[inline(always)]
    pub fn async_(self) -> &'a mut W {
        self.variant(SYNC_A::ASYNC)
    }
    #[doc = "Synchronous with transmitter."]
    #[inline(always)]
    pub fn sync_w_tx(self) -> &'a mut W {
        self.variant(SYNC_A::SYNC_W_TX)
    }
}
impl R {
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline(always)]
    pub fn div(&self) -> DIV_R {
        DIV_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline(always)]
    pub fn bcd(&self) -> BCD_R {
        BCD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline(always)]
    pub fn bcp(&self) -> BCP_R {
        BCP_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline(always)]
    pub fn bci(&self) -> BCI_R {
        BCI_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline(always)]
    pub fn bcs(&self) -> BCS_R {
        BCS_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Synchronous Mode"]
    #[inline(always)]
    pub fn sync(&self) -> SYNC_R {
        SYNC_R::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bit Clock Divide"]
    #[inline(always)]
    #[must_use]
    pub fn div(&mut self) -> DIV_W<0> {
        DIV_W::new(self)
    }
    #[doc = "Bit 24 - Bit Clock Direction"]
    #[inline(always)]
    #[must_use]
    pub fn bcd(&mut self) -> BCD_W<24> {
        BCD_W::new(self)
    }
    #[doc = "Bit 25 - Bit Clock Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn bcp(&mut self) -> BCP_W<25> {
        BCP_W::new(self)
    }
    #[doc = "Bits 26:27 - MCLK Select"]
    #[inline(always)]
    #[must_use]
    pub fn msel(&mut self) -> MSEL_W<26> {
        MSEL_W::new(self)
    }
    #[doc = "Bit 28 - Bit Clock Input"]
    #[inline(always)]
    #[must_use]
    pub fn bci(&mut self) -> BCI_W<28> {
        BCI_W::new(self)
    }
    #[doc = "Bit 29 - Bit Clock Swap"]
    #[inline(always)]
    #[must_use]
    pub fn bcs(&mut self) -> BCS_W<29> {
        BCS_W::new(self)
    }
    #[doc = "Bit 30 - Synchronous Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sync(&mut self) -> SYNC_W<30> {
        SYNC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rcr2](index.html) module"]
pub struct RCR2_SPEC;
impl crate::RegisterSpec for RCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rcr2::R](R) reader structure"]
impl crate::Readable for RCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rcr2::W](W) writer structure"]
impl crate::Writable for RCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RCR2 to value 0"]
impl crate::Resettable for RCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
