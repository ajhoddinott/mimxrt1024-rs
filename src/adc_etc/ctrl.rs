#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRIG_ENABLE` reader - TRIG enable register."]
pub type TRIG_ENABLE_R = crate::FieldReader<u8, TRIG_ENABLE_A>;
#[doc = "TRIG enable register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TRIG_ENABLE_A {
    #[doc = "0: disable all 8 external XBAR triggers."]
    TRIG_ENABLE_0 = 0,
    #[doc = "1: enable external XBAR trigger0."]
    TRIG_ENABLE_1 = 1,
    #[doc = "2: enable external XBAR trigger1."]
    TRIG_ENABLE_2 = 2,
    #[doc = "3: enable external XBAR trigger0 and trigger1."]
    TRIG_ENABLE_3 = 3,
    #[doc = "255: enable all 8 external XBAR triggers."]
    TRIG_ENABLE_255 = 255,
}
impl From<TRIG_ENABLE_A> for u8 {
    #[inline(always)]
    fn from(variant: TRIG_ENABLE_A) -> Self {
        variant as _
    }
}
impl TRIG_ENABLE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TRIG_ENABLE_A> {
        match self.bits {
            0 => Some(TRIG_ENABLE_A::TRIG_ENABLE_0),
            1 => Some(TRIG_ENABLE_A::TRIG_ENABLE_1),
            2 => Some(TRIG_ENABLE_A::TRIG_ENABLE_2),
            3 => Some(TRIG_ENABLE_A::TRIG_ENABLE_3),
            255 => Some(TRIG_ENABLE_A::TRIG_ENABLE_255),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_ENABLE_0`"]
    #[inline(always)]
    pub fn is_trig_enable_0(&self) -> bool {
        *self == TRIG_ENABLE_A::TRIG_ENABLE_0
    }
    #[doc = "Checks if the value of the field is `TRIG_ENABLE_1`"]
    #[inline(always)]
    pub fn is_trig_enable_1(&self) -> bool {
        *self == TRIG_ENABLE_A::TRIG_ENABLE_1
    }
    #[doc = "Checks if the value of the field is `TRIG_ENABLE_2`"]
    #[inline(always)]
    pub fn is_trig_enable_2(&self) -> bool {
        *self == TRIG_ENABLE_A::TRIG_ENABLE_2
    }
    #[doc = "Checks if the value of the field is `TRIG_ENABLE_3`"]
    #[inline(always)]
    pub fn is_trig_enable_3(&self) -> bool {
        *self == TRIG_ENABLE_A::TRIG_ENABLE_3
    }
    #[doc = "Checks if the value of the field is `TRIG_ENABLE_255`"]
    #[inline(always)]
    pub fn is_trig_enable_255(&self) -> bool {
        *self == TRIG_ENABLE_A::TRIG_ENABLE_255
    }
}
#[doc = "Field `TRIG_ENABLE` writer - TRIG enable register."]
pub type TRIG_ENABLE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, CTRL_SPEC, u8, TRIG_ENABLE_A, 8, O>;
impl<'a, const O: u8> TRIG_ENABLE_W<'a, O> {
    #[doc = "disable all 8 external XBAR triggers."]
    #[inline(always)]
    pub fn trig_enable_0(self) -> &'a mut W {
        self.variant(TRIG_ENABLE_A::TRIG_ENABLE_0)
    }
    #[doc = "enable external XBAR trigger0."]
    #[inline(always)]
    pub fn trig_enable_1(self) -> &'a mut W {
        self.variant(TRIG_ENABLE_A::TRIG_ENABLE_1)
    }
    #[doc = "enable external XBAR trigger1."]
    #[inline(always)]
    pub fn trig_enable_2(self) -> &'a mut W {
        self.variant(TRIG_ENABLE_A::TRIG_ENABLE_2)
    }
    #[doc = "enable external XBAR trigger0 and trigger1."]
    #[inline(always)]
    pub fn trig_enable_3(self) -> &'a mut W {
        self.variant(TRIG_ENABLE_A::TRIG_ENABLE_3)
    }
    #[doc = "enable all 8 external XBAR triggers."]
    #[inline(always)]
    pub fn trig_enable_255(self) -> &'a mut W {
        self.variant(TRIG_ENABLE_A::TRIG_ENABLE_255)
    }
}
#[doc = "Field `PRE_DIVIDER` reader - Pre-divider for trig delay and interval"]
pub type PRE_DIVIDER_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PRE_DIVIDER` writer - Pre-divider for trig delay and interval"]
pub type PRE_DIVIDER_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CTRL_SPEC, u8, u8, 8, O>;
#[doc = "Field `DMA_MODE_SEL` reader - Select the trigger type of the DMA_REQ."]
pub type DMA_MODE_SEL_R = crate::BitReader<DMA_MODE_SEL_A>;
#[doc = "Select the trigger type of the DMA_REQ.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMA_MODE_SEL_A {
    #[doc = "0: Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared."]
    DMA_MODE_SEL_0 = 0,
    #[doc = "1: Trig DMA_REQ with pulsed signal, REQ will be cleared by ACK only."]
    DMA_MODE_SEL_1 = 1,
}
impl From<DMA_MODE_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: DMA_MODE_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl DMA_MODE_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMA_MODE_SEL_A {
        match self.bits {
            false => DMA_MODE_SEL_A::DMA_MODE_SEL_0,
            true => DMA_MODE_SEL_A::DMA_MODE_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMA_MODE_SEL_0`"]
    #[inline(always)]
    pub fn is_dma_mode_sel_0(&self) -> bool {
        *self == DMA_MODE_SEL_A::DMA_MODE_SEL_0
    }
    #[doc = "Checks if the value of the field is `DMA_MODE_SEL_1`"]
    #[inline(always)]
    pub fn is_dma_mode_sel_1(&self) -> bool {
        *self == DMA_MODE_SEL_A::DMA_MODE_SEL_1
    }
}
#[doc = "Field `DMA_MODE_SEL` writer - Select the trigger type of the DMA_REQ."]
pub type DMA_MODE_SEL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DMA_MODE_SEL_A, O>;
impl<'a, const O: u8> DMA_MODE_SEL_W<'a, O> {
    #[doc = "Trig DMA_REQ with latched signal, REQ will be cleared when ACK and source request cleared."]
    #[inline(always)]
    pub fn dma_mode_sel_0(self) -> &'a mut W {
        self.variant(DMA_MODE_SEL_A::DMA_MODE_SEL_0)
    }
    #[doc = "Trig DMA_REQ with pulsed signal, REQ will be cleared by ACK only."]
    #[inline(always)]
    pub fn dma_mode_sel_1(self) -> &'a mut W {
        self.variant(DMA_MODE_SEL_A::DMA_MODE_SEL_1)
    }
}
#[doc = "Field `SOFTRST` reader - Software synchronous reset, active high."]
pub type SOFTRST_R = crate::BitReader<SOFTRST_A>;
#[doc = "Software synchronous reset, active high.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SOFTRST_A {
    #[doc = "0: ADC_ETC works normally."]
    SOFTRST_0 = 0,
    #[doc = "1: All registers inside ADC_ETC will be reset to the default value."]
    SOFTRST_1 = 1,
}
impl From<SOFTRST_A> for bool {
    #[inline(always)]
    fn from(variant: SOFTRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SOFTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SOFTRST_A {
        match self.bits {
            false => SOFTRST_A::SOFTRST_0,
            true => SOFTRST_A::SOFTRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTRST_0`"]
    #[inline(always)]
    pub fn is_softrst_0(&self) -> bool {
        *self == SOFTRST_A::SOFTRST_0
    }
    #[doc = "Checks if the value of the field is `SOFTRST_1`"]
    #[inline(always)]
    pub fn is_softrst_1(&self) -> bool {
        *self == SOFTRST_A::SOFTRST_1
    }
}
#[doc = "Field `SOFTRST` writer - Software synchronous reset, active high."]
pub type SOFTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SOFTRST_A, O>;
impl<'a, const O: u8> SOFTRST_W<'a, O> {
    #[doc = "ADC_ETC works normally."]
    #[inline(always)]
    pub fn softrst_0(self) -> &'a mut W {
        self.variant(SOFTRST_A::SOFTRST_0)
    }
    #[doc = "All registers inside ADC_ETC will be reset to the default value."]
    #[inline(always)]
    pub fn softrst_1(self) -> &'a mut W {
        self.variant(SOFTRST_A::SOFTRST_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - TRIG enable register."]
    #[inline(always)]
    pub fn trig_enable(&self) -> TRIG_ENABLE_R {
        TRIG_ENABLE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Pre-divider for trig delay and interval"]
    #[inline(always)]
    pub fn pre_divider(&self) -> PRE_DIVIDER_R {
        PRE_DIVIDER_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Select the trigger type of the DMA_REQ."]
    #[inline(always)]
    pub fn dma_mode_sel(&self) -> DMA_MODE_SEL_R {
        DMA_MODE_SEL_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 31 - Software synchronous reset, active high."]
    #[inline(always)]
    pub fn softrst(&self) -> SOFTRST_R {
        SOFTRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - TRIG enable register."]
    #[inline(always)]
    #[must_use]
    pub fn trig_enable(&mut self) -> TRIG_ENABLE_W<0> {
        TRIG_ENABLE_W::new(self)
    }
    #[doc = "Bits 16:23 - Pre-divider for trig delay and interval"]
    #[inline(always)]
    #[must_use]
    pub fn pre_divider(&mut self) -> PRE_DIVIDER_W<16> {
        PRE_DIVIDER_W::new(self)
    }
    #[doc = "Bit 29 - Select the trigger type of the DMA_REQ."]
    #[inline(always)]
    #[must_use]
    pub fn dma_mode_sel(&mut self) -> DMA_MODE_SEL_W<29> {
        DMA_MODE_SEL_W::new(self)
    }
    #[doc = "Bit 31 - Software synchronous reset, active high."]
    #[inline(always)]
    #[must_use]
    pub fn softrst(&mut self) -> SOFTRST_W<31> {
        SOFTRST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC_ETC Global Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x8000_0000"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x8000_0000;
}
