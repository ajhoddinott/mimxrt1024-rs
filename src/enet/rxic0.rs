#[doc = "Register `RXIC0` reader"]
pub struct R(crate::R<RXIC0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXIC0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXIC0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXIC0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RXIC0` writer"]
pub struct W(crate::W<RXIC0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RXIC0_SPEC>;
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
impl From<crate::W<RXIC0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RXIC0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICTT` reader - Interrupt coalescing timer threshold"]
pub type ICTT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ICTT` writer - Interrupt coalescing timer threshold"]
pub type ICTT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXIC0_SPEC, u16, u16, 16, O>;
#[doc = "Field `ICFT` reader - Interrupt coalescing frame count threshold"]
pub type ICFT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ICFT` writer - Interrupt coalescing frame count threshold"]
pub type ICFT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, RXIC0_SPEC, u8, u8, 8, O>;
#[doc = "Field `ICCS` reader - Interrupt Coalescing Timer Clock Source Select"]
pub type ICCS_R = crate::BitReader<ICCS_A>;
#[doc = "Interrupt Coalescing Timer Clock Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICCS_A {
    #[doc = "0: Use MII/GMII TX clocks."]
    ZERO = 0,
    #[doc = "1: Use ENET system clock."]
    ONE = 1,
}
impl From<ICCS_A> for bool {
    #[inline(always)]
    fn from(variant: ICCS_A) -> Self {
        variant as u8 != 0
    }
}
impl ICCS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICCS_A {
        match self.bits {
            false => ICCS_A::ZERO,
            true => ICCS_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ICCS_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ICCS_A::ONE
    }
}
#[doc = "Field `ICCS` writer - Interrupt Coalescing Timer Clock Source Select"]
pub type ICCS_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIC0_SPEC, ICCS_A, O>;
impl<'a, const O: u8> ICCS_W<'a, O> {
    #[doc = "Use MII/GMII TX clocks."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ICCS_A::ZERO)
    }
    #[doc = "Use ENET system clock."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ICCS_A::ONE)
    }
}
#[doc = "Field `ICEN` reader - Interrupt Coalescing Enable"]
pub type ICEN_R = crate::BitReader<ICEN_A>;
#[doc = "Interrupt Coalescing Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ICEN_A {
    #[doc = "0: Disable Interrupt coalescing."]
    ZERO = 0,
    #[doc = "1: Enable Interrupt coalescing."]
    ONE = 1,
}
impl From<ICEN_A> for bool {
    #[inline(always)]
    fn from(variant: ICEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ICEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ICEN_A {
        match self.bits {
            false => ICEN_A::ZERO,
            true => ICEN_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == ICEN_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == ICEN_A::ONE
    }
}
#[doc = "Field `ICEN` writer - Interrupt Coalescing Enable"]
pub type ICEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, RXIC0_SPEC, ICEN_A, O>;
impl<'a, const O: u8> ICEN_W<'a, O> {
    #[doc = "Disable Interrupt coalescing."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(ICEN_A::ZERO)
    }
    #[doc = "Enable Interrupt coalescing."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(ICEN_A::ONE)
    }
}
impl R {
    #[doc = "Bits 0:15 - Interrupt coalescing timer threshold"]
    #[inline(always)]
    pub fn ictt(&self) -> ICTT_R {
        ICTT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 20:27 - Interrupt coalescing frame count threshold"]
    #[inline(always)]
    pub fn icft(&self) -> ICFT_R {
        ICFT_R::new(((self.bits >> 20) & 0xff) as u8)
    }
    #[doc = "Bit 30 - Interrupt Coalescing Timer Clock Source Select"]
    #[inline(always)]
    pub fn iccs(&self) -> ICCS_R {
        ICCS_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable"]
    #[inline(always)]
    pub fn icen(&self) -> ICEN_R {
        ICEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Interrupt coalescing timer threshold"]
    #[inline(always)]
    #[must_use]
    pub fn ictt(&mut self) -> ICTT_W<0> {
        ICTT_W::new(self)
    }
    #[doc = "Bits 20:27 - Interrupt coalescing frame count threshold"]
    #[inline(always)]
    #[must_use]
    pub fn icft(&mut self) -> ICFT_W<20> {
        ICFT_W::new(self)
    }
    #[doc = "Bit 30 - Interrupt Coalescing Timer Clock Source Select"]
    #[inline(always)]
    #[must_use]
    pub fn iccs(&mut self) -> ICCS_W<30> {
        ICCS_W::new(self)
    }
    #[doc = "Bit 31 - Interrupt Coalescing Enable"]
    #[inline(always)]
    #[must_use]
    pub fn icen(&mut self) -> ICEN_W<31> {
        ICEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Receive Interrupt Coalescing Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxic0](index.html) module"]
pub struct RXIC0_SPEC;
impl crate::RegisterSpec for RXIC0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxic0::R](R) reader structure"]
impl crate::Readable for RXIC0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rxic0::W](W) writer structure"]
impl crate::Writable for RXIC0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RXIC0 to value 0"]
impl crate::Resettable for RXIC0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
