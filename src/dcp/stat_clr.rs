#[doc = "Register `STAT_CLR` reader"]
pub struct R(crate::R<STAT_CLR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STAT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STAT_CLR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STAT_CLR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `STAT_CLR` writer"]
pub struct W(crate::W<STAT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<STAT_CLR_SPEC>;
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
impl From<crate::W<STAT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<STAT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IRQ` reader - Indicates which channels have pending interrupt requests"]
pub type IRQ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `IRQ` writer - Indicates which channels have pending interrupt requests"]
pub type IRQ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, STAT_CLR_SPEC, u8, u8, 4, O>;
#[doc = "Field `READY_CHANNELS` reader - Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
pub type READY_CHANNELS_R = crate::FieldReader<u8, READY_CHANNELS_A>;
#[doc = "Indicates which channels are ready to proceed with a transfer (the active channel is also included)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum READY_CHANNELS_A {
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "4: CH2"]
    CH2 = 4,
    #[doc = "8: CH3"]
    CH3 = 8,
}
impl From<READY_CHANNELS_A> for u8 {
    #[inline(always)]
    fn from(variant: READY_CHANNELS_A) -> Self {
        variant as _
    }
}
impl READY_CHANNELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<READY_CHANNELS_A> {
        match self.bits {
            1 => Some(READY_CHANNELS_A::CH0),
            2 => Some(READY_CHANNELS_A::CH1),
            4 => Some(READY_CHANNELS_A::CH2),
            8 => Some(READY_CHANNELS_A::CH3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == READY_CHANNELS_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == READY_CHANNELS_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == READY_CHANNELS_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == READY_CHANNELS_A::CH3
    }
}
#[doc = "Field `CUR_CHANNEL` reader - Current (active) channel (encoded)"]
pub type CUR_CHANNEL_R = crate::FieldReader<u8, CUR_CHANNEL_A>;
#[doc = "Current (active) channel (encoded)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CUR_CHANNEL_A {
    #[doc = "0: None"]
    NONE = 0,
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "3: CH2"]
    CH2 = 3,
    #[doc = "4: CH3"]
    CH3 = 4,
}
impl From<CUR_CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CUR_CHANNEL_A) -> Self {
        variant as _
    }
}
impl CUR_CHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CUR_CHANNEL_A> {
        match self.bits {
            0 => Some(CUR_CHANNEL_A::NONE),
            1 => Some(CUR_CHANNEL_A::CH0),
            2 => Some(CUR_CHANNEL_A::CH1),
            3 => Some(CUR_CHANNEL_A::CH2),
            4 => Some(CUR_CHANNEL_A::CH3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CUR_CHANNEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == CUR_CHANNEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == CUR_CHANNEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == CUR_CHANNEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == CUR_CHANNEL_A::CH3
    }
}
#[doc = "Field `OTP_KEY_READY` reader - When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
pub type OTP_KEY_READY_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3 - Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    pub fn irq(&self) -> IRQ_R {
        IRQ_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Indicates which channels are ready to proceed with a transfer (the active channel is also included)"]
    #[inline(always)]
    pub fn ready_channels(&self) -> READY_CHANNELS_R {
        READY_CHANNELS_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:27 - Current (active) channel (encoded)"]
    #[inline(always)]
    pub fn cur_channel(&self) -> CUR_CHANNEL_R {
        CUR_CHANNEL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 28 - When set, it indicates that the OTP key is shifted from the fuse block and is ready for use."]
    #[inline(always)]
    pub fn otp_key_ready(&self) -> OTP_KEY_READY_R {
        OTP_KEY_READY_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Indicates which channels have pending interrupt requests"]
    #[inline(always)]
    #[must_use]
    pub fn irq(&mut self) -> IRQ_W<0> {
        IRQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [stat_clr](index.html) module"]
pub struct STAT_CLR_SPEC;
impl crate::RegisterSpec for STAT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [stat_clr::R](R) reader structure"]
impl crate::Readable for STAT_CLR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [stat_clr::W](W) writer structure"]
impl crate::Writable for STAT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets STAT_CLR to value 0x1000_0000"]
impl crate::Resettable for STAT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_0000;
}
