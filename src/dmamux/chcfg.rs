#[doc = "Register `CHCFG[%s]` reader"]
pub struct R(crate::R<CHCFG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHCFG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHCFG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHCFG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CHCFG[%s]` writer"]
pub struct W(crate::W<CHCFG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHCFG_SPEC>;
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
impl From<crate::W<CHCFG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHCFG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SOURCE` reader - DMA Channel Source (Slot Number)"]
pub type SOURCE_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SOURCE` writer - DMA Channel Source (Slot Number)"]
pub type SOURCE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CHCFG_SPEC, u8, u8, 7, O>;
#[doc = "Field `A_ON` reader - DMA Channel Always Enable"]
pub type A_ON_R = crate::BitReader<A_ON_A>;
#[doc = "DMA Channel Always Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum A_ON_A {
    #[doc = "0: DMA Channel Always ON function is disabled"]
    A_ON_0 = 0,
    #[doc = "1: DMA Channel Always ON function is enabled"]
    A_ON_1 = 1,
}
impl From<A_ON_A> for bool {
    #[inline(always)]
    fn from(variant: A_ON_A) -> Self {
        variant as u8 != 0
    }
}
impl A_ON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> A_ON_A {
        match self.bits {
            false => A_ON_A::A_ON_0,
            true => A_ON_A::A_ON_1,
        }
    }
    #[doc = "Checks if the value of the field is `A_ON_0`"]
    #[inline(always)]
    pub fn is_a_on_0(&self) -> bool {
        *self == A_ON_A::A_ON_0
    }
    #[doc = "Checks if the value of the field is `A_ON_1`"]
    #[inline(always)]
    pub fn is_a_on_1(&self) -> bool {
        *self == A_ON_A::A_ON_1
    }
}
#[doc = "Field `A_ON` writer - DMA Channel Always Enable"]
pub type A_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCFG_SPEC, A_ON_A, O>;
impl<'a, const O: u8> A_ON_W<'a, O> {
    #[doc = "DMA Channel Always ON function is disabled"]
    #[inline(always)]
    pub fn a_on_0(self) -> &'a mut W {
        self.variant(A_ON_A::A_ON_0)
    }
    #[doc = "DMA Channel Always ON function is enabled"]
    #[inline(always)]
    pub fn a_on_1(self) -> &'a mut W {
        self.variant(A_ON_A::A_ON_1)
    }
}
#[doc = "Field `TRIG` reader - DMA Channel Trigger Enable"]
pub type TRIG_R = crate::BitReader<TRIG_A>;
#[doc = "DMA Channel Trigger Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TRIG_A {
    #[doc = "0: Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    TRIG_0 = 0,
    #[doc = "1: Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode."]
    TRIG_1 = 1,
}
impl From<TRIG_A> for bool {
    #[inline(always)]
    fn from(variant: TRIG_A) -> Self {
        variant as u8 != 0
    }
}
impl TRIG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TRIG_A {
        match self.bits {
            false => TRIG_A::TRIG_0,
            true => TRIG_A::TRIG_1,
        }
    }
    #[doc = "Checks if the value of the field is `TRIG_0`"]
    #[inline(always)]
    pub fn is_trig_0(&self) -> bool {
        *self == TRIG_A::TRIG_0
    }
    #[doc = "Checks if the value of the field is `TRIG_1`"]
    #[inline(always)]
    pub fn is_trig_1(&self) -> bool {
        *self == TRIG_A::TRIG_1
    }
}
#[doc = "Field `TRIG` writer - DMA Channel Trigger Enable"]
pub type TRIG_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCFG_SPEC, TRIG_A, O>;
impl<'a, const O: u8> TRIG_W<'a, O> {
    #[doc = "Triggering is disabled. If triggering is disabled and ENBL is set, the DMA Channel will simply route the specified source to the DMA channel. (Normal mode)"]
    #[inline(always)]
    pub fn trig_0(self) -> &'a mut W {
        self.variant(TRIG_A::TRIG_0)
    }
    #[doc = "Triggering is enabled. If triggering is enabled and ENBL is set, the DMA_CH_MUX is in Periodic Trigger mode."]
    #[inline(always)]
    pub fn trig_1(self) -> &'a mut W {
        self.variant(TRIG_A::TRIG_1)
    }
}
#[doc = "Field `ENBL` reader - DMA Mux Channel Enable"]
pub type ENBL_R = crate::BitReader<ENBL_A>;
#[doc = "DMA Mux Channel Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENBL_A {
    #[doc = "0: DMA Mux channel is disabled"]
    ENBL_0 = 0,
    #[doc = "1: DMA Mux channel is enabled"]
    ENBL_1 = 1,
}
impl From<ENBL_A> for bool {
    #[inline(always)]
    fn from(variant: ENBL_A) -> Self {
        variant as u8 != 0
    }
}
impl ENBL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENBL_A {
        match self.bits {
            false => ENBL_A::ENBL_0,
            true => ENBL_A::ENBL_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENBL_0`"]
    #[inline(always)]
    pub fn is_enbl_0(&self) -> bool {
        *self == ENBL_A::ENBL_0
    }
    #[doc = "Checks if the value of the field is `ENBL_1`"]
    #[inline(always)]
    pub fn is_enbl_1(&self) -> bool {
        *self == ENBL_A::ENBL_1
    }
}
#[doc = "Field `ENBL` writer - DMA Mux Channel Enable"]
pub type ENBL_W<'a, const O: u8> = crate::BitWriter<'a, u32, CHCFG_SPEC, ENBL_A, O>;
impl<'a, const O: u8> ENBL_W<'a, O> {
    #[doc = "DMA Mux channel is disabled"]
    #[inline(always)]
    pub fn enbl_0(self) -> &'a mut W {
        self.variant(ENBL_A::ENBL_0)
    }
    #[doc = "DMA Mux channel is enabled"]
    #[inline(always)]
    pub fn enbl_1(self) -> &'a mut W {
        self.variant(ENBL_A::ENBL_1)
    }
}
impl R {
    #[doc = "Bits 0:6 - DMA Channel Source (Slot Number)"]
    #[inline(always)]
    pub fn source(&self) -> SOURCE_R {
        SOURCE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 29 - DMA Channel Always Enable"]
    #[inline(always)]
    pub fn a_on(&self) -> A_ON_R {
        A_ON_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - DMA Channel Trigger Enable"]
    #[inline(always)]
    pub fn trig(&self) -> TRIG_R {
        TRIG_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - DMA Mux Channel Enable"]
    #[inline(always)]
    pub fn enbl(&self) -> ENBL_R {
        ENBL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - DMA Channel Source (Slot Number)"]
    #[inline(always)]
    #[must_use]
    pub fn source(&mut self) -> SOURCE_W<0> {
        SOURCE_W::new(self)
    }
    #[doc = "Bit 29 - DMA Channel Always Enable"]
    #[inline(always)]
    #[must_use]
    pub fn a_on(&mut self) -> A_ON_W<29> {
        A_ON_W::new(self)
    }
    #[doc = "Bit 30 - DMA Channel Trigger Enable"]
    #[inline(always)]
    #[must_use]
    pub fn trig(&mut self) -> TRIG_W<30> {
        TRIG_W::new(self)
    }
    #[doc = "Bit 31 - DMA Mux Channel Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enbl(&mut self) -> ENBL_W<31> {
        ENBL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel index Configuration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chcfg](index.html) module"]
pub struct CHCFG_SPEC;
impl crate::RegisterSpec for CHCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chcfg::R](R) reader structure"]
impl crate::Readable for CHCFG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [chcfg::W](W) writer structure"]
impl crate::Writable for CHCFG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CHCFG[%s]
to value 0"]
impl crate::Resettable for CHCFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
