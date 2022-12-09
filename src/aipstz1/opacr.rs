#[doc = "Register `OPACR` reader"]
pub struct R(crate::R<OPACR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR` writer"]
pub struct W(crate::W<OPACR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR_SPEC>;
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
impl From<crate::W<OPACR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAC7` reader - Off-platform Peripheral Access Control 7"]
pub type OPAC7_R = crate::FieldReader<u8, OPAC7_A>;
#[doc = "Off-platform Peripheral Access Control 7\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC7_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC7_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC7_A) -> Self {
        variant as _
    }
}
impl OPAC7_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC7_A> {
        match self.bits {
            0 => Some(OPAC7_A::TP0),
            1 => Some(OPAC7_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC7_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC7_A::TP1
    }
}
#[doc = "Field `OPAC7` writer - Off-platform Peripheral Access Control 7"]
pub type OPAC7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR_SPEC, u8, OPAC7_A, 4, O>;
impl<'a, const O: u8> OPAC7_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC7_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC7_A::TP1)
    }
}
#[doc = "Field `OPAC6` reader - Off-platform Peripheral Access Control 6"]
pub type OPAC6_R = crate::FieldReader<u8, OPAC6_A>;
#[doc = "Off-platform Peripheral Access Control 6\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC6_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC6_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC6_A) -> Self {
        variant as _
    }
}
impl OPAC6_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC6_A> {
        match self.bits {
            0 => Some(OPAC6_A::TP0),
            1 => Some(OPAC6_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC6_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC6_A::TP1
    }
}
#[doc = "Field `OPAC6` writer - Off-platform Peripheral Access Control 6"]
pub type OPAC6_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR_SPEC, u8, OPAC6_A, 4, O>;
impl<'a, const O: u8> OPAC6_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC6_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC6_A::TP1)
    }
}
#[doc = "Field `OPAC5` reader - Off-platform Peripheral Access Control 5"]
pub type OPAC5_R = crate::FieldReader<u8, OPAC5_A>;
#[doc = "Off-platform Peripheral Access Control 5\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC5_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC5_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC5_A) -> Self {
        variant as _
    }
}
impl OPAC5_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC5_A> {
        match self.bits {
            0 => Some(OPAC5_A::TP0),
            1 => Some(OPAC5_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC5_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC5_A::TP1
    }
}
#[doc = "Field `OPAC5` writer - Off-platform Peripheral Access Control 5"]
pub type OPAC5_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR_SPEC, u8, OPAC5_A, 4, O>;
impl<'a, const O: u8> OPAC5_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC5_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC5_A::TP1)
    }
}
#[doc = "Field `OPAC4` reader - Off-platform Peripheral Access Control 4"]
pub type OPAC4_R = crate::FieldReader<u8, OPAC4_A>;
#[doc = "Off-platform Peripheral Access Control 4\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC4_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC4_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC4_A) -> Self {
        variant as _
    }
}
impl OPAC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC4_A> {
        match self.bits {
            0 => Some(OPAC4_A::TP0),
            1 => Some(OPAC4_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC4_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC4_A::TP1
    }
}
#[doc = "Field `OPAC4` writer - Off-platform Peripheral Access Control 4"]
pub type OPAC4_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR_SPEC, u8, OPAC4_A, 4, O>;
impl<'a, const O: u8> OPAC4_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC4_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC4_A::TP1)
    }
}
#[doc = "Field `OPAC3` reader - Off-platform Peripheral Access Control 3"]
pub type OPAC3_R = crate::FieldReader<u8, OPAC3_A>;
#[doc = "Off-platform Peripheral Access Control 3\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC3_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC3_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC3_A) -> Self {
        variant as _
    }
}
impl OPAC3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC3_A> {
        match self.bits {
            0 => Some(OPAC3_A::TP0),
            1 => Some(OPAC3_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC3_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC3_A::TP1
    }
}
#[doc = "Field `OPAC3` writer - Off-platform Peripheral Access Control 3"]
pub type OPAC3_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR_SPEC, u8, OPAC3_A, 4, O>;
impl<'a, const O: u8> OPAC3_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC3_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC3_A::TP1)
    }
}
#[doc = "Field `OPAC2` reader - Off-platform Peripheral Access Control 2"]
pub type OPAC2_R = crate::FieldReader<u8, OPAC2_A>;
#[doc = "Off-platform Peripheral Access Control 2\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC2_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC2_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC2_A) -> Self {
        variant as _
    }
}
impl OPAC2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC2_A> {
        match self.bits {
            0 => Some(OPAC2_A::TP0),
            1 => Some(OPAC2_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC2_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC2_A::TP1
    }
}
#[doc = "Field `OPAC2` writer - Off-platform Peripheral Access Control 2"]
pub type OPAC2_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR_SPEC, u8, OPAC2_A, 4, O>;
impl<'a, const O: u8> OPAC2_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC2_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC2_A::TP1)
    }
}
#[doc = "Field `OPAC1` reader - Off-platform Peripheral Access Control 1"]
pub type OPAC1_R = crate::FieldReader<u8, OPAC1_A>;
#[doc = "Off-platform Peripheral Access Control 1\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC1_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC1_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC1_A) -> Self {
        variant as _
    }
}
impl OPAC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC1_A> {
        match self.bits {
            0 => Some(OPAC1_A::TP0),
            1 => Some(OPAC1_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC1_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC1_A::TP1
    }
}
#[doc = "Field `OPAC1` writer - Off-platform Peripheral Access Control 1"]
pub type OPAC1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR_SPEC, u8, OPAC1_A, 4, O>;
impl<'a, const O: u8> OPAC1_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC1_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC1_A::TP1)
    }
}
#[doc = "Field `OPAC0` reader - Off-platform Peripheral Access Control 0"]
pub type OPAC0_R = crate::FieldReader<u8, OPAC0_A>;
#[doc = "Off-platform Peripheral Access Control 0\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC0_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC0_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC0_A) -> Self {
        variant as _
    }
}
impl OPAC0_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC0_A> {
        match self.bits {
            0 => Some(OPAC0_A::TP0),
            1 => Some(OPAC0_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC0_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC0_A::TP1
    }
}
#[doc = "Field `OPAC0` writer - Off-platform Peripheral Access Control 0"]
pub type OPAC0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR_SPEC, u8, OPAC0_A, 4, O>;
impl<'a, const O: u8> OPAC0_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC0_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC0_A::TP1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 7"]
    #[inline(always)]
    pub fn opac7(&self) -> OPAC7_R {
        OPAC7_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 6"]
    #[inline(always)]
    pub fn opac6(&self) -> OPAC6_R {
        OPAC6_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 5"]
    #[inline(always)]
    pub fn opac5(&self) -> OPAC5_R {
        OPAC5_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 4"]
    #[inline(always)]
    pub fn opac4(&self) -> OPAC4_R {
        OPAC4_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 3"]
    #[inline(always)]
    pub fn opac3(&self) -> OPAC3_R {
        OPAC3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 2"]
    #[inline(always)]
    pub fn opac2(&self) -> OPAC2_R {
        OPAC2_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 1"]
    #[inline(always)]
    pub fn opac1(&self) -> OPAC1_R {
        OPAC1_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 0"]
    #[inline(always)]
    pub fn opac0(&self) -> OPAC0_R {
        OPAC0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 7"]
    #[inline(always)]
    #[must_use]
    pub fn opac7(&mut self) -> OPAC7_W<0> {
        OPAC7_W::new(self)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 6"]
    #[inline(always)]
    #[must_use]
    pub fn opac6(&mut self) -> OPAC6_W<4> {
        OPAC6_W::new(self)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 5"]
    #[inline(always)]
    #[must_use]
    pub fn opac5(&mut self) -> OPAC5_W<8> {
        OPAC5_W::new(self)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 4"]
    #[inline(always)]
    #[must_use]
    pub fn opac4(&mut self) -> OPAC4_W<12> {
        OPAC4_W::new(self)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 3"]
    #[inline(always)]
    #[must_use]
    pub fn opac3(&mut self) -> OPAC3_W<16> {
        OPAC3_W::new(self)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 2"]
    #[inline(always)]
    #[must_use]
    pub fn opac2(&mut self) -> OPAC2_W<20> {
        OPAC2_W::new(self)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 1"]
    #[inline(always)]
    #[must_use]
    pub fn opac1(&mut self) -> OPAC1_W<24> {
        OPAC1_W::new(self)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 0"]
    #[inline(always)]
    #[must_use]
    pub fn opac0(&mut self) -> OPAC0_W<28> {
        OPAC0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr](index.html) module"]
pub struct OPACR_SPEC;
impl crate::RegisterSpec for OPACR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr::R](R) reader structure"]
impl crate::Readable for OPACR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr::W](W) writer structure"]
impl crate::Writable for OPACR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPACR to value 0x4444_4444"]
impl crate::Resettable for OPACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
