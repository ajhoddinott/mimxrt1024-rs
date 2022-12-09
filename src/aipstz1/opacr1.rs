#[doc = "Register `OPACR1` reader"]
pub struct R(crate::R<OPACR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR1` writer"]
pub struct W(crate::W<OPACR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR1_SPEC>;
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
impl From<crate::W<OPACR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAC15` reader - Off-platform Peripheral Access Control 15"]
pub type OPAC15_R = crate::FieldReader<u8, OPAC15_A>;
#[doc = "Off-platform Peripheral Access Control 15\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC15_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC15_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC15_A) -> Self {
        variant as _
    }
}
impl OPAC15_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC15_A> {
        match self.bits {
            0 => Some(OPAC15_A::TP0),
            1 => Some(OPAC15_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC15_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC15_A::TP1
    }
}
#[doc = "Field `OPAC15` writer - Off-platform Peripheral Access Control 15"]
pub type OPAC15_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR1_SPEC, u8, OPAC15_A, 4, O>;
impl<'a, const O: u8> OPAC15_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC15_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC15_A::TP1)
    }
}
#[doc = "Field `OPAC14` reader - Off-platform Peripheral Access Control 14"]
pub type OPAC14_R = crate::FieldReader<u8, OPAC14_A>;
#[doc = "Off-platform Peripheral Access Control 14\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC14_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC14_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC14_A) -> Self {
        variant as _
    }
}
impl OPAC14_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC14_A> {
        match self.bits {
            0 => Some(OPAC14_A::TP0),
            1 => Some(OPAC14_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC14_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC14_A::TP1
    }
}
#[doc = "Field `OPAC14` writer - Off-platform Peripheral Access Control 14"]
pub type OPAC14_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR1_SPEC, u8, OPAC14_A, 4, O>;
impl<'a, const O: u8> OPAC14_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC14_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC14_A::TP1)
    }
}
#[doc = "Field `OPAC13` reader - Off-platform Peripheral Access Control 13"]
pub type OPAC13_R = crate::FieldReader<u8, OPAC13_A>;
#[doc = "Off-platform Peripheral Access Control 13\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC13_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC13_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC13_A) -> Self {
        variant as _
    }
}
impl OPAC13_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC13_A> {
        match self.bits {
            0 => Some(OPAC13_A::TP0),
            1 => Some(OPAC13_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC13_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC13_A::TP1
    }
}
#[doc = "Field `OPAC13` writer - Off-platform Peripheral Access Control 13"]
pub type OPAC13_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR1_SPEC, u8, OPAC13_A, 4, O>;
impl<'a, const O: u8> OPAC13_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC13_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC13_A::TP1)
    }
}
#[doc = "Field `OPAC12` reader - Off-platform Peripheral Access Control 12"]
pub type OPAC12_R = crate::FieldReader<u8, OPAC12_A>;
#[doc = "Off-platform Peripheral Access Control 12\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC12_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC12_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC12_A) -> Self {
        variant as _
    }
}
impl OPAC12_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC12_A> {
        match self.bits {
            0 => Some(OPAC12_A::TP0),
            1 => Some(OPAC12_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC12_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC12_A::TP1
    }
}
#[doc = "Field `OPAC12` writer - Off-platform Peripheral Access Control 12"]
pub type OPAC12_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR1_SPEC, u8, OPAC12_A, 4, O>;
impl<'a, const O: u8> OPAC12_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC12_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC12_A::TP1)
    }
}
#[doc = "Field `OPAC11` reader - Off-platform Peripheral Access Control 11"]
pub type OPAC11_R = crate::FieldReader<u8, OPAC11_A>;
#[doc = "Off-platform Peripheral Access Control 11\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC11_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC11_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC11_A) -> Self {
        variant as _
    }
}
impl OPAC11_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC11_A> {
        match self.bits {
            0 => Some(OPAC11_A::TP0),
            1 => Some(OPAC11_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC11_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC11_A::TP1
    }
}
#[doc = "Field `OPAC11` writer - Off-platform Peripheral Access Control 11"]
pub type OPAC11_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR1_SPEC, u8, OPAC11_A, 4, O>;
impl<'a, const O: u8> OPAC11_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC11_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC11_A::TP1)
    }
}
#[doc = "Field `OPAC10` reader - Off-platform Peripheral Access Control 10"]
pub type OPAC10_R = crate::FieldReader<u8, OPAC10_A>;
#[doc = "Off-platform Peripheral Access Control 10\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC10_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC10_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC10_A) -> Self {
        variant as _
    }
}
impl OPAC10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC10_A> {
        match self.bits {
            0 => Some(OPAC10_A::TP0),
            1 => Some(OPAC10_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC10_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC10_A::TP1
    }
}
#[doc = "Field `OPAC10` writer - Off-platform Peripheral Access Control 10"]
pub type OPAC10_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR1_SPEC, u8, OPAC10_A, 4, O>;
impl<'a, const O: u8> OPAC10_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC10_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC10_A::TP1)
    }
}
#[doc = "Field `OPAC9` reader - Off-platform Peripheral Access Control 9"]
pub type OPAC9_R = crate::FieldReader<u8, OPAC9_A>;
#[doc = "Off-platform Peripheral Access Control 9\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC9_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC9_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC9_A) -> Self {
        variant as _
    }
}
impl OPAC9_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC9_A> {
        match self.bits {
            0 => Some(OPAC9_A::TP0),
            1 => Some(OPAC9_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC9_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC9_A::TP1
    }
}
#[doc = "Field `OPAC9` writer - Off-platform Peripheral Access Control 9"]
pub type OPAC9_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR1_SPEC, u8, OPAC9_A, 4, O>;
impl<'a, const O: u8> OPAC9_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC9_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC9_A::TP1)
    }
}
#[doc = "Field `OPAC8` reader - Off-platform Peripheral Access Control 8"]
pub type OPAC8_R = crate::FieldReader<u8, OPAC8_A>;
#[doc = "Off-platform Peripheral Access Control 8\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC8_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC8_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC8_A) -> Self {
        variant as _
    }
}
impl OPAC8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC8_A> {
        match self.bits {
            0 => Some(OPAC8_A::TP0),
            1 => Some(OPAC8_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC8_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC8_A::TP1
    }
}
#[doc = "Field `OPAC8` writer - Off-platform Peripheral Access Control 8"]
pub type OPAC8_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR1_SPEC, u8, OPAC8_A, 4, O>;
impl<'a, const O: u8> OPAC8_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC8_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC8_A::TP1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 15"]
    #[inline(always)]
    pub fn opac15(&self) -> OPAC15_R {
        OPAC15_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 14"]
    #[inline(always)]
    pub fn opac14(&self) -> OPAC14_R {
        OPAC14_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 13"]
    #[inline(always)]
    pub fn opac13(&self) -> OPAC13_R {
        OPAC13_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 12"]
    #[inline(always)]
    pub fn opac12(&self) -> OPAC12_R {
        OPAC12_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 11"]
    #[inline(always)]
    pub fn opac11(&self) -> OPAC11_R {
        OPAC11_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 10"]
    #[inline(always)]
    pub fn opac10(&self) -> OPAC10_R {
        OPAC10_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 9"]
    #[inline(always)]
    pub fn opac9(&self) -> OPAC9_R {
        OPAC9_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 8"]
    #[inline(always)]
    pub fn opac8(&self) -> OPAC8_R {
        OPAC8_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 15"]
    #[inline(always)]
    #[must_use]
    pub fn opac15(&mut self) -> OPAC15_W<0> {
        OPAC15_W::new(self)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 14"]
    #[inline(always)]
    #[must_use]
    pub fn opac14(&mut self) -> OPAC14_W<4> {
        OPAC14_W::new(self)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 13"]
    #[inline(always)]
    #[must_use]
    pub fn opac13(&mut self) -> OPAC13_W<8> {
        OPAC13_W::new(self)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 12"]
    #[inline(always)]
    #[must_use]
    pub fn opac12(&mut self) -> OPAC12_W<12> {
        OPAC12_W::new(self)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 11"]
    #[inline(always)]
    #[must_use]
    pub fn opac11(&mut self) -> OPAC11_W<16> {
        OPAC11_W::new(self)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 10"]
    #[inline(always)]
    #[must_use]
    pub fn opac10(&mut self) -> OPAC10_W<20> {
        OPAC10_W::new(self)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 9"]
    #[inline(always)]
    #[must_use]
    pub fn opac9(&mut self) -> OPAC9_W<24> {
        OPAC9_W::new(self)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 8"]
    #[inline(always)]
    #[must_use]
    pub fn opac8(&mut self) -> OPAC8_W<28> {
        OPAC8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr1](index.html) module"]
pub struct OPACR1_SPEC;
impl crate::RegisterSpec for OPACR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr1::R](R) reader structure"]
impl crate::Readable for OPACR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr1::W](W) writer structure"]
impl crate::Writable for OPACR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPACR1 to value 0x4444_4444"]
impl crate::Resettable for OPACR1_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
