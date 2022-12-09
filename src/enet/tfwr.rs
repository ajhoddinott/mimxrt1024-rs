#[doc = "Register `TFWR` reader"]
pub struct R(crate::R<TFWR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TFWR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TFWR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TFWR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TFWR` writer"]
pub struct W(crate::W<TFWR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TFWR_SPEC>;
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
impl From<crate::W<TFWR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TFWR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TFWR` reader - Transmit FIFO Write"]
pub type TFWR_R = crate::FieldReader<u8, TFWR_A>;
#[doc = "Transmit FIFO Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TFWR_A {
    #[doc = "0: 64 bytes written."]
    VAL64_0 = 0,
    #[doc = "1: 64 bytes written."]
    VAL64_1 = 1,
    #[doc = "2: 128 bytes written."]
    VAL128 = 2,
    #[doc = "3: 192 bytes written."]
    VAL192 = 3,
    #[doc = "31: 1984 bytes written."]
    VAL1984 = 31,
}
impl From<TFWR_A> for u8 {
    #[inline(always)]
    fn from(variant: TFWR_A) -> Self {
        variant as _
    }
}
impl TFWR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<TFWR_A> {
        match self.bits {
            0 => Some(TFWR_A::VAL64_0),
            1 => Some(TFWR_A::VAL64_1),
            2 => Some(TFWR_A::VAL128),
            3 => Some(TFWR_A::VAL192),
            31 => Some(TFWR_A::VAL1984),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `VAL64_0`"]
    #[inline(always)]
    pub fn is_val64_0(&self) -> bool {
        *self == TFWR_A::VAL64_0
    }
    #[doc = "Checks if the value of the field is `VAL64_1`"]
    #[inline(always)]
    pub fn is_val64_1(&self) -> bool {
        *self == TFWR_A::VAL64_1
    }
    #[doc = "Checks if the value of the field is `VAL128`"]
    #[inline(always)]
    pub fn is_val128(&self) -> bool {
        *self == TFWR_A::VAL128
    }
    #[doc = "Checks if the value of the field is `VAL192`"]
    #[inline(always)]
    pub fn is_val192(&self) -> bool {
        *self == TFWR_A::VAL192
    }
    #[doc = "Checks if the value of the field is `VAL1984`"]
    #[inline(always)]
    pub fn is_val1984(&self) -> bool {
        *self == TFWR_A::VAL1984
    }
}
#[doc = "Field `TFWR` writer - Transmit FIFO Write"]
pub type TFWR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TFWR_SPEC, u8, TFWR_A, 6, O>;
impl<'a, const O: u8> TFWR_W<'a, O> {
    #[doc = "64 bytes written."]
    #[inline(always)]
    pub fn val64_0(self) -> &'a mut W {
        self.variant(TFWR_A::VAL64_0)
    }
    #[doc = "64 bytes written."]
    #[inline(always)]
    pub fn val64_1(self) -> &'a mut W {
        self.variant(TFWR_A::VAL64_1)
    }
    #[doc = "128 bytes written."]
    #[inline(always)]
    pub fn val128(self) -> &'a mut W {
        self.variant(TFWR_A::VAL128)
    }
    #[doc = "192 bytes written."]
    #[inline(always)]
    pub fn val192(self) -> &'a mut W {
        self.variant(TFWR_A::VAL192)
    }
    #[doc = "1984 bytes written."]
    #[inline(always)]
    pub fn val1984(self) -> &'a mut W {
        self.variant(TFWR_A::VAL1984)
    }
}
#[doc = "Field `STRFWD` reader - Store And Forward Enable"]
pub type STRFWD_R = crate::BitReader<STRFWD_A>;
#[doc = "Store And Forward Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STRFWD_A {
    #[doc = "0: Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    ZERO = 0,
    #[doc = "1: Enabled."]
    ONE = 1,
}
impl From<STRFWD_A> for bool {
    #[inline(always)]
    fn from(variant: STRFWD_A) -> Self {
        variant as u8 != 0
    }
}
impl STRFWD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STRFWD_A {
        match self.bits {
            false => STRFWD_A::ZERO,
            true => STRFWD_A::ONE,
        }
    }
    #[doc = "Checks if the value of the field is `ZERO`"]
    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        *self == STRFWD_A::ZERO
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == STRFWD_A::ONE
    }
}
#[doc = "Field `STRFWD` writer - Store And Forward Enable"]
pub type STRFWD_W<'a, const O: u8> = crate::BitWriter<'a, u32, TFWR_SPEC, STRFWD_A, O>;
impl<'a, const O: u8> STRFWD_W<'a, O> {
    #[doc = "Reset. The transmission start threshold is programmed in TFWR\\[TFWR\\]."]
    #[inline(always)]
    pub fn zero(self) -> &'a mut W {
        self.variant(STRFWD_A::ZERO)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(STRFWD_A::ONE)
    }
}
impl R {
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline(always)]
    pub fn tfwr(&self) -> TFWR_R {
        TFWR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline(always)]
    pub fn strfwd(&self) -> STRFWD_R {
        STRFWD_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Transmit FIFO Write"]
    #[inline(always)]
    #[must_use]
    pub fn tfwr(&mut self) -> TFWR_W<0> {
        TFWR_W::new(self)
    }
    #[doc = "Bit 8 - Store And Forward Enable"]
    #[inline(always)]
    #[must_use]
    pub fn strfwd(&mut self) -> STRFWD_W<8> {
        STRFWD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Transmit FIFO Watermark Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tfwr](index.html) module"]
pub struct TFWR_SPEC;
impl crate::RegisterSpec for TFWR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tfwr::R](R) reader structure"]
impl crate::Readable for TFWR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tfwr::W](W) writer structure"]
impl crate::Writable for TFWR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TFWR to value 0"]
impl crate::Resettable for TFWR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
