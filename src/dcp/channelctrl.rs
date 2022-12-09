#[doc = "Register `CHANNELCTRL` reader"]
pub struct R(crate::R<CHANNELCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHANNELCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHANNELCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHANNELCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHANNELCTRL` writer"]
pub struct W(crate::W<CHANNELCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHANNELCTRL_SPEC>;
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
impl From<crate::W<CHANNELCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHANNELCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_CHANNEL` reader - Setting a bit in this field enables the DMA channel associated with it"]
pub type ENABLE_CHANNEL_R = crate::FieldReader<u8, ENABLE_CHANNEL_A>;
#[doc = "Setting a bit in this field enables the DMA channel associated with it\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ENABLE_CHANNEL_A {
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "4: CH2"]
    CH2 = 4,
    #[doc = "8: CH3"]
    CH3 = 8,
}
impl From<ENABLE_CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: ENABLE_CHANNEL_A) -> Self {
        variant as _
    }
}
impl ENABLE_CHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ENABLE_CHANNEL_A> {
        match self.bits {
            1 => Some(ENABLE_CHANNEL_A::CH0),
            2 => Some(ENABLE_CHANNEL_A::CH1),
            4 => Some(ENABLE_CHANNEL_A::CH2),
            8 => Some(ENABLE_CHANNEL_A::CH3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == ENABLE_CHANNEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == ENABLE_CHANNEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == ENABLE_CHANNEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == ENABLE_CHANNEL_A::CH3
    }
}
#[doc = "Field `ENABLE_CHANNEL` writer - Setting a bit in this field enables the DMA channel associated with it"]
pub type ENABLE_CHANNEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNELCTRL_SPEC, u8, ENABLE_CHANNEL_A, 8, O>;
impl<'a, const O: u8> ENABLE_CHANNEL_W<'a, O> {
    #[doc = "CH0"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(ENABLE_CHANNEL_A::CH0)
    }
    #[doc = "CH1"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(ENABLE_CHANNEL_A::CH1)
    }
    #[doc = "CH2"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(ENABLE_CHANNEL_A::CH2)
    }
    #[doc = "CH3"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(ENABLE_CHANNEL_A::CH3)
    }
}
#[doc = "Field `HIGH_PRIORITY_CHANNEL` reader - Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
pub type HIGH_PRIORITY_CHANNEL_R = crate::FieldReader<u8, HIGH_PRIORITY_CHANNEL_A>;
#[doc = "Setting a bit in this field causes the corresponding channel to have high-priority arbitration\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HIGH_PRIORITY_CHANNEL_A {
    #[doc = "1: CH0"]
    CH0 = 1,
    #[doc = "2: CH1"]
    CH1 = 2,
    #[doc = "4: CH2"]
    CH2 = 4,
    #[doc = "8: CH3"]
    CH3 = 8,
}
impl From<HIGH_PRIORITY_CHANNEL_A> for u8 {
    #[inline(always)]
    fn from(variant: HIGH_PRIORITY_CHANNEL_A) -> Self {
        variant as _
    }
}
impl HIGH_PRIORITY_CHANNEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HIGH_PRIORITY_CHANNEL_A> {
        match self.bits {
            1 => Some(HIGH_PRIORITY_CHANNEL_A::CH0),
            2 => Some(HIGH_PRIORITY_CHANNEL_A::CH1),
            4 => Some(HIGH_PRIORITY_CHANNEL_A::CH2),
            8 => Some(HIGH_PRIORITY_CHANNEL_A::CH3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        *self == HIGH_PRIORITY_CHANNEL_A::CH3
    }
}
#[doc = "Field `HIGH_PRIORITY_CHANNEL` writer - Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
pub type HIGH_PRIORITY_CHANNEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CHANNELCTRL_SPEC, u8, HIGH_PRIORITY_CHANNEL_A, 8, O>;
impl<'a, const O: u8> HIGH_PRIORITY_CHANNEL_W<'a, O> {
    #[doc = "CH0"]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNEL_A::CH0)
    }
    #[doc = "CH1"]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNEL_A::CH1)
    }
    #[doc = "CH2"]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNEL_A::CH2)
    }
    #[doc = "CH3"]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(HIGH_PRIORITY_CHANNEL_A::CH3)
    }
}
#[doc = "Field `CH0_IRQ_MERGED` reader - Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
pub type CH0_IRQ_MERGED_R = crate::BitReader<bool>;
#[doc = "Field `CH0_IRQ_MERGED` writer - Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
pub type CH0_IRQ_MERGED_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHANNELCTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    pub fn enable_channel(&self) -> ENABLE_CHANNEL_R {
        ENABLE_CHANNEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    pub fn high_priority_channel(&self) -> HIGH_PRIORITY_CHANNEL_R {
        HIGH_PRIORITY_CHANNEL_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    pub fn ch0_irq_merged(&self) -> CH0_IRQ_MERGED_R {
        CH0_IRQ_MERGED_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Setting a bit in this field enables the DMA channel associated with it"]
    #[inline(always)]
    #[must_use]
    pub fn enable_channel(&mut self) -> ENABLE_CHANNEL_W<0> {
        ENABLE_CHANNEL_W::new(self)
    }
    #[doc = "Bits 8:15 - Setting a bit in this field causes the corresponding channel to have high-priority arbitration"]
    #[inline(always)]
    #[must_use]
    pub fn high_priority_channel(&mut self) -> HIGH_PRIORITY_CHANNEL_W<8> {
        HIGH_PRIORITY_CHANNEL_W::new(self)
    }
    #[doc = "Bit 16 - Indicates that the interrupt for channel 0 must be merged with the other interrupts on the shared dcp_irq interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn ch0_irq_merged(&mut self) -> CH0_IRQ_MERGED_W<16> {
        CH0_IRQ_MERGED_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP channel control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [channelctrl](index.html) module"]
pub struct CHANNELCTRL_SPEC;
impl crate::RegisterSpec for CHANNELCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [channelctrl::R](R) reader structure"]
impl crate::Readable for CHANNELCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [channelctrl::W](W) writer structure"]
impl crate::Writable for CHANNELCTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHANNELCTRL to value 0"]
impl crate::Resettable for CHANNELCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
