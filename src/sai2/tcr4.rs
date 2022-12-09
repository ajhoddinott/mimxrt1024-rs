#[doc = "Register `TCR4` reader"]
pub struct R(crate::R<TCR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCR4` writer"]
pub struct W(crate::W<TCR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCR4_SPEC>;
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
impl From<crate::W<TCR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FSD` reader - Frame Sync Direction"]
pub type FSD_R = crate::BitReader<FSD_A>;
#[doc = "Frame Sync Direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSD_A {
    #[doc = "0: Frame sync is generated externally in Slave mode."]
    EXT_IN_SLAVE_MODE = 0,
    #[doc = "1: Frame sync is generated internally in Master mode."]
    INT_IN_MASTER_MODE = 1,
}
impl From<FSD_A> for bool {
    #[inline(always)]
    fn from(variant: FSD_A) -> Self {
        variant as u8 != 0
    }
}
impl FSD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSD_A {
        match self.bits {
            false => FSD_A::EXT_IN_SLAVE_MODE,
            true => FSD_A::INT_IN_MASTER_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `EXT_IN_SLAVE_MODE`"]
    #[inline(always)]
    pub fn is_ext_in_slave_mode(&self) -> bool {
        *self == FSD_A::EXT_IN_SLAVE_MODE
    }
    #[doc = "Checks if the value of the field is `INT_IN_MASTER_MODE`"]
    #[inline(always)]
    pub fn is_int_in_master_mode(&self) -> bool {
        *self == FSD_A::INT_IN_MASTER_MODE
    }
}
#[doc = "Field `FSD` writer - Frame Sync Direction"]
pub type FSD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR4_SPEC, FSD_A, O>;
impl<'a, const O: u8> FSD_W<'a, O> {
    #[doc = "Frame sync is generated externally in Slave mode."]
    #[inline(always)]
    pub fn ext_in_slave_mode(self) -> &'a mut W {
        self.variant(FSD_A::EXT_IN_SLAVE_MODE)
    }
    #[doc = "Frame sync is generated internally in Master mode."]
    #[inline(always)]
    pub fn int_in_master_mode(self) -> &'a mut W {
        self.variant(FSD_A::INT_IN_MASTER_MODE)
    }
}
#[doc = "Field `FSP` reader - Frame Sync Polarity"]
pub type FSP_R = crate::BitReader<FSP_A>;
#[doc = "Frame Sync Polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSP_A {
    #[doc = "0: Frame sync is active high."]
    ACTIVE_HIGH = 0,
    #[doc = "1: Frame sync is active low."]
    ACTIVE_LOW = 1,
}
impl From<FSP_A> for bool {
    #[inline(always)]
    fn from(variant: FSP_A) -> Self {
        variant as u8 != 0
    }
}
impl FSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSP_A {
        match self.bits {
            false => FSP_A::ACTIVE_HIGH,
            true => FSP_A::ACTIVE_LOW,
        }
    }
    #[doc = "Checks if the value of the field is `ACTIVE_HIGH`"]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == FSP_A::ACTIVE_HIGH
    }
    #[doc = "Checks if the value of the field is `ACTIVE_LOW`"]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == FSP_A::ACTIVE_LOW
    }
}
#[doc = "Field `FSP` writer - Frame Sync Polarity"]
pub type FSP_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR4_SPEC, FSP_A, O>;
impl<'a, const O: u8> FSP_W<'a, O> {
    #[doc = "Frame sync is active high."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut W {
        self.variant(FSP_A::ACTIVE_HIGH)
    }
    #[doc = "Frame sync is active low."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut W {
        self.variant(FSP_A::ACTIVE_LOW)
    }
}
#[doc = "Field `ONDEM` reader - On Demand Mode"]
pub type ONDEM_R = crate::BitReader<ONDEM_A>;
#[doc = "On Demand Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONDEM_A {
    #[doc = "0: Internal frame sync is generated continuously."]
    CONTINUOUS_FRAME_SYNC = 0,
    #[doc = "1: Internal frame sync is generated when the FIFO warning flag is clear."]
    ON_DEMAND_FRAME_SYNC = 1,
}
impl From<ONDEM_A> for bool {
    #[inline(always)]
    fn from(variant: ONDEM_A) -> Self {
        variant as u8 != 0
    }
}
impl ONDEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONDEM_A {
        match self.bits {
            false => ONDEM_A::CONTINUOUS_FRAME_SYNC,
            true => ONDEM_A::ON_DEMAND_FRAME_SYNC,
        }
    }
    #[doc = "Checks if the value of the field is `CONTINUOUS_FRAME_SYNC`"]
    #[inline(always)]
    pub fn is_continuous_frame_sync(&self) -> bool {
        *self == ONDEM_A::CONTINUOUS_FRAME_SYNC
    }
    #[doc = "Checks if the value of the field is `ON_DEMAND_FRAME_SYNC`"]
    #[inline(always)]
    pub fn is_on_demand_frame_sync(&self) -> bool {
        *self == ONDEM_A::ON_DEMAND_FRAME_SYNC
    }
}
#[doc = "Field `ONDEM` writer - On Demand Mode"]
pub type ONDEM_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR4_SPEC, ONDEM_A, O>;
impl<'a, const O: u8> ONDEM_W<'a, O> {
    #[doc = "Internal frame sync is generated continuously."]
    #[inline(always)]
    pub fn continuous_frame_sync(self) -> &'a mut W {
        self.variant(ONDEM_A::CONTINUOUS_FRAME_SYNC)
    }
    #[doc = "Internal frame sync is generated when the FIFO warning flag is clear."]
    #[inline(always)]
    pub fn on_demand_frame_sync(self) -> &'a mut W {
        self.variant(ONDEM_A::ON_DEMAND_FRAME_SYNC)
    }
}
#[doc = "Field `FSE` reader - Frame Sync Early"]
pub type FSE_R = crate::BitReader<FSE_A>;
#[doc = "Frame Sync Early\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FSE_A {
    #[doc = "0: Frame sync asserts with the first bit of the frame."]
    DISABLE = 0,
    #[doc = "1: Frame sync asserts one bit before the first bit of the frame."]
    ENABLE = 1,
}
impl From<FSE_A> for bool {
    #[inline(always)]
    fn from(variant: FSE_A) -> Self {
        variant as u8 != 0
    }
}
impl FSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FSE_A {
        match self.bits {
            false => FSE_A::DISABLE,
            true => FSE_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FSE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FSE_A::ENABLE
    }
}
#[doc = "Field `FSE` writer - Frame Sync Early"]
pub type FSE_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR4_SPEC, FSE_A, O>;
impl<'a, const O: u8> FSE_W<'a, O> {
    #[doc = "Frame sync asserts with the first bit of the frame."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FSE_A::DISABLE)
    }
    #[doc = "Frame sync asserts one bit before the first bit of the frame."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FSE_A::ENABLE)
    }
}
#[doc = "Field `MF` reader - MSB First"]
pub type MF_R = crate::BitReader<MF_A>;
#[doc = "MSB First\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MF_A {
    #[doc = "0: LSB is transmitted first."]
    DISABLE = 0,
    #[doc = "1: MSB is transmitted first."]
    ENABLE = 1,
}
impl From<MF_A> for bool {
    #[inline(always)]
    fn from(variant: MF_A) -> Self {
        variant as u8 != 0
    }
}
impl MF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MF_A {
        match self.bits {
            false => MF_A::DISABLE,
            true => MF_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == MF_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MF_A::ENABLE
    }
}
#[doc = "Field `MF` writer - MSB First"]
pub type MF_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR4_SPEC, MF_A, O>;
impl<'a, const O: u8> MF_W<'a, O> {
    #[doc = "LSB is transmitted first."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(MF_A::DISABLE)
    }
    #[doc = "MSB is transmitted first."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MF_A::ENABLE)
    }
}
#[doc = "Field `CHMOD` reader - Channel Mode"]
pub type CHMOD_R = crate::BitReader<CHMOD_A>;
#[doc = "Channel Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHMOD_A {
    #[doc = "0: TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled."]
    TDM_MODE = 0,
    #[doc = "1: Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled."]
    OUTPUT_MODE = 1,
}
impl From<CHMOD_A> for bool {
    #[inline(always)]
    fn from(variant: CHMOD_A) -> Self {
        variant as u8 != 0
    }
}
impl CHMOD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHMOD_A {
        match self.bits {
            false => CHMOD_A::TDM_MODE,
            true => CHMOD_A::OUTPUT_MODE,
        }
    }
    #[doc = "Checks if the value of the field is `TDM_MODE`"]
    #[inline(always)]
    pub fn is_tdm_mode(&self) -> bool {
        *self == CHMOD_A::TDM_MODE
    }
    #[doc = "Checks if the value of the field is `OUTPUT_MODE`"]
    #[inline(always)]
    pub fn is_output_mode(&self) -> bool {
        *self == CHMOD_A::OUTPUT_MODE
    }
}
#[doc = "Field `CHMOD` writer - Channel Mode"]
pub type CHMOD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR4_SPEC, CHMOD_A, O>;
impl<'a, const O: u8> CHMOD_W<'a, O> {
    #[doc = "TDM mode, transmit data pins are tri-stated when slots are masked or channels are disabled."]
    #[inline(always)]
    pub fn tdm_mode(self) -> &'a mut W {
        self.variant(CHMOD_A::TDM_MODE)
    }
    #[doc = "Output mode, transmit data pins are never tri-stated and will output zero when slots are masked or channels are disabled."]
    #[inline(always)]
    pub fn output_mode(self) -> &'a mut W {
        self.variant(CHMOD_A::OUTPUT_MODE)
    }
}
#[doc = "Field `SYWD` reader - Sync Width"]
pub type SYWD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SYWD` writer - Sync Width"]
pub type SYWD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `FRSZ` reader - Frame size"]
pub type FRSZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FRSZ` writer - Frame size"]
pub type FRSZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR4_SPEC, u8, u8, 5, O>;
#[doc = "Field `FPACK` reader - FIFO Packing Mode"]
pub type FPACK_R = crate::FieldReader<u8, FPACK_A>;
#[doc = "FIFO Packing Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FPACK_A {
    #[doc = "0: FIFO packing is disabled."]
    DISABLED = 0,
    #[doc = "2: 8-bit FIFO packing is enabled."]
    EIGHT_BIT_FIFO_PACKING = 2,
    #[doc = "3: 16-bit FIFO packing is enabled."]
    SIXTEEN_BIT_FIFO_PACKING = 3,
}
impl From<FPACK_A> for u8 {
    #[inline(always)]
    fn from(variant: FPACK_A) -> Self {
        variant as _
    }
}
impl FPACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FPACK_A> {
        match self.bits {
            0 => Some(FPACK_A::DISABLED),
            2 => Some(FPACK_A::EIGHT_BIT_FIFO_PACKING),
            3 => Some(FPACK_A::SIXTEEN_BIT_FIFO_PACKING),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == FPACK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `EIGHT_BIT_FIFO_PACKING`"]
    #[inline(always)]
    pub fn is_eight_bit_fifo_packing(&self) -> bool {
        *self == FPACK_A::EIGHT_BIT_FIFO_PACKING
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BIT_FIFO_PACKING`"]
    #[inline(always)]
    pub fn is_sixteen_bit_fifo_packing(&self) -> bool {
        *self == FPACK_A::SIXTEEN_BIT_FIFO_PACKING
    }
}
#[doc = "Field `FPACK` writer - FIFO Packing Mode"]
pub type FPACK_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TCR4_SPEC, u8, FPACK_A, 2, O>;
impl<'a, const O: u8> FPACK_W<'a, O> {
    #[doc = "FIFO packing is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(FPACK_A::DISABLED)
    }
    #[doc = "8-bit FIFO packing is enabled."]
    #[inline(always)]
    pub fn eight_bit_fifo_packing(self) -> &'a mut W {
        self.variant(FPACK_A::EIGHT_BIT_FIFO_PACKING)
    }
    #[doc = "16-bit FIFO packing is enabled."]
    #[inline(always)]
    pub fn sixteen_bit_fifo_packing(self) -> &'a mut W {
        self.variant(FPACK_A::SIXTEEN_BIT_FIFO_PACKING)
    }
}
#[doc = "Field `FCONT` reader - FIFO Continue on Error"]
pub type FCONT_R = crate::BitReader<FCONT_A>;
#[doc = "FIFO Continue on Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FCONT_A {
    #[doc = "0: On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    DISABLE = 0,
    #[doc = "1: On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    ENABLE = 1,
}
impl From<FCONT_A> for bool {
    #[inline(always)]
    fn from(variant: FCONT_A) -> Self {
        variant as u8 != 0
    }
}
impl FCONT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FCONT_A {
        match self.bits {
            false => FCONT_A::DISABLE,
            true => FCONT_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FCONT_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FCONT_A::ENABLE
    }
}
#[doc = "Field `FCONT` writer - FIFO Continue on Error"]
pub type FCONT_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCR4_SPEC, FCONT_A, O>;
impl<'a, const O: u8> FCONT_W<'a, O> {
    #[doc = "On FIFO error, the SAI will continue from the start of the next frame after the FIFO error flag has been cleared."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FCONT_A::DISABLE)
    }
    #[doc = "On FIFO error, the SAI will continue from the same word that caused the FIFO error to set after the FIFO warning flag has been cleared."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FCONT_A::ENABLE)
    }
}
impl R {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    pub fn fsd(&self) -> FSD_R {
        FSD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    pub fn fsp(&self) -> FSP_R {
        FSP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    pub fn ondem(&self) -> ONDEM_R {
        ONDEM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    pub fn fse(&self) -> FSE_R {
        FSE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    pub fn mf(&self) -> MF_R {
        MF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Channel Mode"]
    #[inline(always)]
    pub fn chmod(&self) -> CHMOD_R {
        CHMOD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    pub fn sywd(&self) -> SYWD_R {
        SYWD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline(always)]
    pub fn frsz(&self) -> FRSZ_R {
        FRSZ_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    pub fn fpack(&self) -> FPACK_R {
        FPACK_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    pub fn fcont(&self) -> FCONT_R {
        FCONT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Frame Sync Direction"]
    #[inline(always)]
    #[must_use]
    pub fn fsd(&mut self) -> FSD_W<0> {
        FSD_W::new(self)
    }
    #[doc = "Bit 1 - Frame Sync Polarity"]
    #[inline(always)]
    #[must_use]
    pub fn fsp(&mut self) -> FSP_W<1> {
        FSP_W::new(self)
    }
    #[doc = "Bit 2 - On Demand Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ondem(&mut self) -> ONDEM_W<2> {
        ONDEM_W::new(self)
    }
    #[doc = "Bit 3 - Frame Sync Early"]
    #[inline(always)]
    #[must_use]
    pub fn fse(&mut self) -> FSE_W<3> {
        FSE_W::new(self)
    }
    #[doc = "Bit 4 - MSB First"]
    #[inline(always)]
    #[must_use]
    pub fn mf(&mut self) -> MF_W<4> {
        MF_W::new(self)
    }
    #[doc = "Bit 5 - Channel Mode"]
    #[inline(always)]
    #[must_use]
    pub fn chmod(&mut self) -> CHMOD_W<5> {
        CHMOD_W::new(self)
    }
    #[doc = "Bits 8:12 - Sync Width"]
    #[inline(always)]
    #[must_use]
    pub fn sywd(&mut self) -> SYWD_W<8> {
        SYWD_W::new(self)
    }
    #[doc = "Bits 16:20 - Frame size"]
    #[inline(always)]
    #[must_use]
    pub fn frsz(&mut self) -> FRSZ_W<16> {
        FRSZ_W::new(self)
    }
    #[doc = "Bits 24:25 - FIFO Packing Mode"]
    #[inline(always)]
    #[must_use]
    pub fn fpack(&mut self) -> FPACK_W<24> {
        FPACK_W::new(self)
    }
    #[doc = "Bit 28 - FIFO Continue on Error"]
    #[inline(always)]
    #[must_use]
    pub fn fcont(&mut self) -> FCONT_W<28> {
        FCONT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit Configuration 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcr4](index.html) module"]
pub struct TCR4_SPEC;
impl crate::RegisterSpec for TCR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcr4::R](R) reader structure"]
impl crate::Readable for TCR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcr4::W](W) writer structure"]
impl crate::Writable for TCR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCR4 to value 0"]
impl crate::Resettable for TCR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
