#[doc = "Register `OPACR4` reader"]
pub struct R(crate::R<OPACR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR4` writer"]
pub struct W(crate::W<OPACR4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR4_SPEC>;
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
impl From<crate::W<OPACR4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAC33` reader - Off-platform Peripheral Access Control 33"]
pub type OPAC33_R = crate::FieldReader<u8, OPAC33_A>;
#[doc = "Off-platform Peripheral Access Control 33\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC33_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC33_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC33_A) -> Self {
        variant as _
    }
}
impl OPAC33_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC33_A> {
        match self.bits {
            0 => Some(OPAC33_A::TP0),
            1 => Some(OPAC33_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC33_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC33_A::TP1
    }
}
#[doc = "Field `OPAC33` writer - Off-platform Peripheral Access Control 33"]
pub type OPAC33_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR4_SPEC, u8, OPAC33_A, 4, O>;
impl<'a, const O: u8> OPAC33_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC33_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC33_A::TP1)
    }
}
#[doc = "Field `OPAC32` reader - Off-platform Peripheral Access Control 32"]
pub type OPAC32_R = crate::FieldReader<u8, OPAC32_A>;
#[doc = "Off-platform Peripheral Access Control 32\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC32_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC32_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC32_A) -> Self {
        variant as _
    }
}
impl OPAC32_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC32_A> {
        match self.bits {
            0 => Some(OPAC32_A::TP0),
            1 => Some(OPAC32_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC32_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC32_A::TP1
    }
}
#[doc = "Field `OPAC32` writer - Off-platform Peripheral Access Control 32"]
pub type OPAC32_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR4_SPEC, u8, OPAC32_A, 4, O>;
impl<'a, const O: u8> OPAC32_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC32_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC32_A::TP1)
    }
}
impl R {
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 33"]
    #[inline(always)]
    pub fn opac33(&self) -> OPAC33_R {
        OPAC33_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 32"]
    #[inline(always)]
    pub fn opac32(&self) -> OPAC32_R {
        OPAC32_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 33"]
    #[inline(always)]
    #[must_use]
    pub fn opac33(&mut self) -> OPAC33_W<24> {
        OPAC33_W::new(self)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 32"]
    #[inline(always)]
    #[must_use]
    pub fn opac32(&mut self) -> OPAC32_W<28> {
        OPAC32_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr4](index.html) module"]
pub struct OPACR4_SPEC;
impl crate::RegisterSpec for OPACR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr4::R](R) reader structure"]
impl crate::Readable for OPACR4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr4::W](W) writer structure"]
impl crate::Writable for OPACR4_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPACR4 to value 0x4444_4444"]
impl crate::Resettable for OPACR4_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
