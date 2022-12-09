#[doc = "Register `DER` reader"]
pub struct R(crate::R<DER_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DER_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DER_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DER_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DER` writer"]
pub struct W(crate::W<DER_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DER_SPEC>;
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
impl From<crate::W<DER_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DER_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TDDE` reader - Transmit Data DMA Enable"]
pub type TDDE_R = crate::BitReader<TDDE_A>;
#[doc = "Transmit Data DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TDDE_A {
    #[doc = "0: DMA request is disabled"]
    DISABLED = 0,
    #[doc = "1: DMA request is enabled"]
    ENABLED = 1,
}
impl From<TDDE_A> for bool {
    #[inline(always)]
    fn from(variant: TDDE_A) -> Self {
        variant as u8 != 0
    }
}
impl TDDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TDDE_A {
        match self.bits {
            false => TDDE_A::DISABLED,
            true => TDDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TDDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TDDE_A::ENABLED
    }
}
#[doc = "Field `TDDE` writer - Transmit Data DMA Enable"]
pub type TDDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DER_SPEC, TDDE_A, O>;
impl<'a, const O: u8> TDDE_W<'a, O> {
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TDDE_A::DISABLED)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TDDE_A::ENABLED)
    }
}
#[doc = "Field `RDDE` reader - Receive Data DMA Enable"]
pub type RDDE_R = crate::BitReader<RDDE_A>;
#[doc = "Receive Data DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RDDE_A {
    #[doc = "0: DMA request is disabled"]
    DISABLED = 0,
    #[doc = "1: DMA request is enabled"]
    ENABLED = 1,
}
impl From<RDDE_A> for bool {
    #[inline(always)]
    fn from(variant: RDDE_A) -> Self {
        variant as u8 != 0
    }
}
impl RDDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RDDE_A {
        match self.bits {
            false => RDDE_A::DISABLED,
            true => RDDE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RDDE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RDDE_A::ENABLED
    }
}
#[doc = "Field `RDDE` writer - Receive Data DMA Enable"]
pub type RDDE_W<'a, const O: u8> = crate::BitWriter<'a, u32, DER_SPEC, RDDE_A, O>;
impl<'a, const O: u8> RDDE_W<'a, O> {
    #[doc = "DMA request is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RDDE_A::DISABLED)
    }
    #[doc = "DMA request is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RDDE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Data DMA Enable"]
    #[inline(always)]
    pub fn tdde(&self) -> TDDE_R {
        TDDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data DMA Enable"]
    #[inline(always)]
    pub fn rdde(&self) -> RDDE_R {
        RDDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Data DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tdde(&mut self) -> TDDE_W<0> {
        TDDE_W::new(self)
    }
    #[doc = "Bit 1 - Receive Data DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rdde(&mut self) -> RDDE_W<1> {
        RDDE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA Enable\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [der](index.html) module"]
pub struct DER_SPEC;
impl crate::RegisterSpec for DER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [der::R](R) reader structure"]
impl crate::Readable for DER_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [der::W](W) writer structure"]
impl crate::Writable for DER_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DER to value 0"]
impl crate::Resettable for DER_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
