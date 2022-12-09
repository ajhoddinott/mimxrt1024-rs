#[doc = "Register `OPACR3` reader"]
pub struct R(crate::R<OPACR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR3` writer"]
pub struct W(crate::W<OPACR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR3_SPEC>;
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
impl From<crate::W<OPACR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAC31` reader - Off-platform Peripheral Access Control 31"]
pub type OPAC31_R = crate::FieldReader<u8, OPAC31_A>;
#[doc = "Off-platform Peripheral Access Control 31\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC31_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC31_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC31_A) -> Self {
        variant as _
    }
}
impl OPAC31_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC31_A> {
        match self.bits {
            0 => Some(OPAC31_A::TP0),
            1 => Some(OPAC31_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC31_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC31_A::TP1
    }
}
#[doc = "Field `OPAC31` writer - Off-platform Peripheral Access Control 31"]
pub type OPAC31_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR3_SPEC, u8, OPAC31_A, 4, O>;
impl<'a, const O: u8> OPAC31_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC31_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC31_A::TP1)
    }
}
#[doc = "Field `OPAC30` reader - Off-platform Peripheral Access Control 30"]
pub type OPAC30_R = crate::FieldReader<u8, OPAC30_A>;
#[doc = "Off-platform Peripheral Access Control 30\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC30_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC30_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC30_A) -> Self {
        variant as _
    }
}
impl OPAC30_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC30_A> {
        match self.bits {
            0 => Some(OPAC30_A::TP0),
            1 => Some(OPAC30_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC30_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC30_A::TP1
    }
}
#[doc = "Field `OPAC30` writer - Off-platform Peripheral Access Control 30"]
pub type OPAC30_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR3_SPEC, u8, OPAC30_A, 4, O>;
impl<'a, const O: u8> OPAC30_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC30_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC30_A::TP1)
    }
}
#[doc = "Field `OPAC29` reader - Off-platform Peripheral Access Control 29"]
pub type OPAC29_R = crate::FieldReader<u8, OPAC29_A>;
#[doc = "Off-platform Peripheral Access Control 29\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC29_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC29_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC29_A) -> Self {
        variant as _
    }
}
impl OPAC29_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC29_A> {
        match self.bits {
            0 => Some(OPAC29_A::TP0),
            1 => Some(OPAC29_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC29_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC29_A::TP1
    }
}
#[doc = "Field `OPAC29` writer - Off-platform Peripheral Access Control 29"]
pub type OPAC29_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR3_SPEC, u8, OPAC29_A, 4, O>;
impl<'a, const O: u8> OPAC29_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC29_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC29_A::TP1)
    }
}
#[doc = "Field `OPAC28` reader - Off-platform Peripheral Access Control 28"]
pub type OPAC28_R = crate::FieldReader<u8, OPAC28_A>;
#[doc = "Off-platform Peripheral Access Control 28\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC28_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC28_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC28_A) -> Self {
        variant as _
    }
}
impl OPAC28_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC28_A> {
        match self.bits {
            0 => Some(OPAC28_A::TP0),
            1 => Some(OPAC28_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC28_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC28_A::TP1
    }
}
#[doc = "Field `OPAC28` writer - Off-platform Peripheral Access Control 28"]
pub type OPAC28_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR3_SPEC, u8, OPAC28_A, 4, O>;
impl<'a, const O: u8> OPAC28_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC28_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC28_A::TP1)
    }
}
#[doc = "Field `OPAC27` reader - Off-platform Peripheral Access Control 27"]
pub type OPAC27_R = crate::FieldReader<u8, OPAC27_A>;
#[doc = "Off-platform Peripheral Access Control 27\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC27_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC27_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC27_A) -> Self {
        variant as _
    }
}
impl OPAC27_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC27_A> {
        match self.bits {
            0 => Some(OPAC27_A::TP0),
            1 => Some(OPAC27_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC27_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC27_A::TP1
    }
}
#[doc = "Field `OPAC27` writer - Off-platform Peripheral Access Control 27"]
pub type OPAC27_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR3_SPEC, u8, OPAC27_A, 4, O>;
impl<'a, const O: u8> OPAC27_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC27_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC27_A::TP1)
    }
}
#[doc = "Field `OPAC26` reader - Off-platform Peripheral Access Control 26"]
pub type OPAC26_R = crate::FieldReader<u8, OPAC26_A>;
#[doc = "Off-platform Peripheral Access Control 26\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC26_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC26_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC26_A) -> Self {
        variant as _
    }
}
impl OPAC26_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC26_A> {
        match self.bits {
            0 => Some(OPAC26_A::TP0),
            1 => Some(OPAC26_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC26_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC26_A::TP1
    }
}
#[doc = "Field `OPAC26` writer - Off-platform Peripheral Access Control 26"]
pub type OPAC26_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR3_SPEC, u8, OPAC26_A, 4, O>;
impl<'a, const O: u8> OPAC26_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC26_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC26_A::TP1)
    }
}
#[doc = "Field `OPAC25` reader - Off-platform Peripheral Access Control 25"]
pub type OPAC25_R = crate::FieldReader<u8, OPAC25_A>;
#[doc = "Off-platform Peripheral Access Control 25\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC25_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC25_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC25_A) -> Self {
        variant as _
    }
}
impl OPAC25_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC25_A> {
        match self.bits {
            0 => Some(OPAC25_A::TP0),
            1 => Some(OPAC25_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC25_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC25_A::TP1
    }
}
#[doc = "Field `OPAC25` writer - Off-platform Peripheral Access Control 25"]
pub type OPAC25_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR3_SPEC, u8, OPAC25_A, 4, O>;
impl<'a, const O: u8> OPAC25_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC25_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC25_A::TP1)
    }
}
#[doc = "Field `OPAC24` reader - Off-platform Peripheral Access Control 24"]
pub type OPAC24_R = crate::FieldReader<u8, OPAC24_A>;
#[doc = "Off-platform Peripheral Access Control 24\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC24_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC24_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC24_A) -> Self {
        variant as _
    }
}
impl OPAC24_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC24_A> {
        match self.bits {
            0 => Some(OPAC24_A::TP0),
            1 => Some(OPAC24_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC24_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC24_A::TP1
    }
}
#[doc = "Field `OPAC24` writer - Off-platform Peripheral Access Control 24"]
pub type OPAC24_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR3_SPEC, u8, OPAC24_A, 4, O>;
impl<'a, const O: u8> OPAC24_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC24_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC24_A::TP1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 31"]
    #[inline(always)]
    pub fn opac31(&self) -> OPAC31_R {
        OPAC31_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 30"]
    #[inline(always)]
    pub fn opac30(&self) -> OPAC30_R {
        OPAC30_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 29"]
    #[inline(always)]
    pub fn opac29(&self) -> OPAC29_R {
        OPAC29_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 28"]
    #[inline(always)]
    pub fn opac28(&self) -> OPAC28_R {
        OPAC28_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 27"]
    #[inline(always)]
    pub fn opac27(&self) -> OPAC27_R {
        OPAC27_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 26"]
    #[inline(always)]
    pub fn opac26(&self) -> OPAC26_R {
        OPAC26_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 25"]
    #[inline(always)]
    pub fn opac25(&self) -> OPAC25_R {
        OPAC25_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 24"]
    #[inline(always)]
    pub fn opac24(&self) -> OPAC24_R {
        OPAC24_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 31"]
    #[inline(always)]
    #[must_use]
    pub fn opac31(&mut self) -> OPAC31_W<0> {
        OPAC31_W::new(self)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 30"]
    #[inline(always)]
    #[must_use]
    pub fn opac30(&mut self) -> OPAC30_W<4> {
        OPAC30_W::new(self)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 29"]
    #[inline(always)]
    #[must_use]
    pub fn opac29(&mut self) -> OPAC29_W<8> {
        OPAC29_W::new(self)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 28"]
    #[inline(always)]
    #[must_use]
    pub fn opac28(&mut self) -> OPAC28_W<12> {
        OPAC28_W::new(self)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 27"]
    #[inline(always)]
    #[must_use]
    pub fn opac27(&mut self) -> OPAC27_W<16> {
        OPAC27_W::new(self)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 26"]
    #[inline(always)]
    #[must_use]
    pub fn opac26(&mut self) -> OPAC26_W<20> {
        OPAC26_W::new(self)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 25"]
    #[inline(always)]
    #[must_use]
    pub fn opac25(&mut self) -> OPAC25_W<24> {
        OPAC25_W::new(self)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 24"]
    #[inline(always)]
    #[must_use]
    pub fn opac24(&mut self) -> OPAC24_W<28> {
        OPAC24_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr3](index.html) module"]
pub struct OPACR3_SPEC;
impl crate::RegisterSpec for OPACR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr3::R](R) reader structure"]
impl crate::Readable for OPACR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr3::W](W) writer structure"]
impl crate::Writable for OPACR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPACR3 to value 0x4444_4444"]
impl crate::Resettable for OPACR3_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
