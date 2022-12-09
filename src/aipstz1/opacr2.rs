#[doc = "Register `OPACR2` reader"]
pub struct R(crate::R<OPACR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OPACR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OPACR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OPACR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OPACR2` writer"]
pub struct W(crate::W<OPACR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OPACR2_SPEC>;
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
impl From<crate::W<OPACR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OPACR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OPAC23` reader - Off-platform Peripheral Access Control 23"]
pub type OPAC23_R = crate::FieldReader<u8, OPAC23_A>;
#[doc = "Off-platform Peripheral Access Control 23\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC23_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC23_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC23_A) -> Self {
        variant as _
    }
}
impl OPAC23_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC23_A> {
        match self.bits {
            0 => Some(OPAC23_A::TP0),
            1 => Some(OPAC23_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC23_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC23_A::TP1
    }
}
#[doc = "Field `OPAC23` writer - Off-platform Peripheral Access Control 23"]
pub type OPAC23_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR2_SPEC, u8, OPAC23_A, 4, O>;
impl<'a, const O: u8> OPAC23_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC23_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC23_A::TP1)
    }
}
#[doc = "Field `OPAC22` reader - Off-platform Peripheral Access Control 22"]
pub type OPAC22_R = crate::FieldReader<u8, OPAC22_A>;
#[doc = "Off-platform Peripheral Access Control 22\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC22_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC22_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC22_A) -> Self {
        variant as _
    }
}
impl OPAC22_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC22_A> {
        match self.bits {
            0 => Some(OPAC22_A::TP0),
            1 => Some(OPAC22_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC22_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC22_A::TP1
    }
}
#[doc = "Field `OPAC22` writer - Off-platform Peripheral Access Control 22"]
pub type OPAC22_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR2_SPEC, u8, OPAC22_A, 4, O>;
impl<'a, const O: u8> OPAC22_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC22_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC22_A::TP1)
    }
}
#[doc = "Field `OPAC21` reader - Off-platform Peripheral Access Control 21"]
pub type OPAC21_R = crate::FieldReader<u8, OPAC21_A>;
#[doc = "Off-platform Peripheral Access Control 21\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC21_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC21_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC21_A) -> Self {
        variant as _
    }
}
impl OPAC21_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC21_A> {
        match self.bits {
            0 => Some(OPAC21_A::TP0),
            1 => Some(OPAC21_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC21_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC21_A::TP1
    }
}
#[doc = "Field `OPAC21` writer - Off-platform Peripheral Access Control 21"]
pub type OPAC21_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR2_SPEC, u8, OPAC21_A, 4, O>;
impl<'a, const O: u8> OPAC21_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC21_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC21_A::TP1)
    }
}
#[doc = "Field `OPAC20` reader - Off-platform Peripheral Access Control 20"]
pub type OPAC20_R = crate::FieldReader<u8, OPAC20_A>;
#[doc = "Off-platform Peripheral Access Control 20\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC20_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC20_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC20_A) -> Self {
        variant as _
    }
}
impl OPAC20_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC20_A> {
        match self.bits {
            0 => Some(OPAC20_A::TP0),
            1 => Some(OPAC20_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC20_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC20_A::TP1
    }
}
#[doc = "Field `OPAC20` writer - Off-platform Peripheral Access Control 20"]
pub type OPAC20_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR2_SPEC, u8, OPAC20_A, 4, O>;
impl<'a, const O: u8> OPAC20_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC20_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC20_A::TP1)
    }
}
#[doc = "Field `OPAC19` reader - Off-platform Peripheral Access Control 19"]
pub type OPAC19_R = crate::FieldReader<u8, OPAC19_A>;
#[doc = "Off-platform Peripheral Access Control 19\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC19_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC19_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC19_A) -> Self {
        variant as _
    }
}
impl OPAC19_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC19_A> {
        match self.bits {
            0 => Some(OPAC19_A::TP0),
            1 => Some(OPAC19_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC19_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC19_A::TP1
    }
}
#[doc = "Field `OPAC19` writer - Off-platform Peripheral Access Control 19"]
pub type OPAC19_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR2_SPEC, u8, OPAC19_A, 4, O>;
impl<'a, const O: u8> OPAC19_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC19_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC19_A::TP1)
    }
}
#[doc = "Field `OPAC18` reader - Off-platform Peripheral Access Control 18"]
pub type OPAC18_R = crate::FieldReader<u8, OPAC18_A>;
#[doc = "Off-platform Peripheral Access Control 18\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC18_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC18_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC18_A) -> Self {
        variant as _
    }
}
impl OPAC18_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC18_A> {
        match self.bits {
            0 => Some(OPAC18_A::TP0),
            1 => Some(OPAC18_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC18_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC18_A::TP1
    }
}
#[doc = "Field `OPAC18` writer - Off-platform Peripheral Access Control 18"]
pub type OPAC18_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR2_SPEC, u8, OPAC18_A, 4, O>;
impl<'a, const O: u8> OPAC18_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC18_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC18_A::TP1)
    }
}
#[doc = "Field `OPAC17` reader - Off-platform Peripheral Access Control 17"]
pub type OPAC17_R = crate::FieldReader<u8, OPAC17_A>;
#[doc = "Off-platform Peripheral Access Control 17\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC17_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC17_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC17_A) -> Self {
        variant as _
    }
}
impl OPAC17_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC17_A> {
        match self.bits {
            0 => Some(OPAC17_A::TP0),
            1 => Some(OPAC17_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC17_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC17_A::TP1
    }
}
#[doc = "Field `OPAC17` writer - Off-platform Peripheral Access Control 17"]
pub type OPAC17_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR2_SPEC, u8, OPAC17_A, 4, O>;
impl<'a, const O: u8> OPAC17_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC17_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC17_A::TP1)
    }
}
#[doc = "Field `OPAC16` reader - Off-platform Peripheral Access Control 16"]
pub type OPAC16_R = crate::FieldReader<u8, OPAC16_A>;
#[doc = "Off-platform Peripheral Access Control 16\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum OPAC16_A {
    #[doc = "0: Accesses from an untrusted master are allowed."]
    TP0 = 0,
    #[doc = "1: Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    TP1 = 1,
}
impl From<OPAC16_A> for u8 {
    #[inline(always)]
    fn from(variant: OPAC16_A) -> Self {
        variant as _
    }
}
impl OPAC16_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<OPAC16_A> {
        match self.bits {
            0 => Some(OPAC16_A::TP0),
            1 => Some(OPAC16_A::TP1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TP0`"]
    #[inline(always)]
    pub fn is_tp0(&self) -> bool {
        *self == OPAC16_A::TP0
    }
    #[doc = "Checks if the value of the field is `TP1`"]
    #[inline(always)]
    pub fn is_tp1(&self) -> bool {
        *self == OPAC16_A::TP1
    }
}
#[doc = "Field `OPAC16` writer - Off-platform Peripheral Access Control 16"]
pub type OPAC16_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OPACR2_SPEC, u8, OPAC16_A, 4, O>;
impl<'a, const O: u8> OPAC16_W<'a, O> {
    #[doc = "Accesses from an untrusted master are allowed."]
    #[inline(always)]
    pub fn tp0(self) -> &'a mut W {
        self.variant(OPAC16_A::TP0)
    }
    #[doc = "Accesses from an untrusted master are not allowed. If an access is attempted by an untrusted master, the access is terminated with an error response and no peripheral access is initiated on the IPS bus."]
    #[inline(always)]
    pub fn tp1(self) -> &'a mut W {
        self.variant(OPAC16_A::TP1)
    }
}
impl R {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 23"]
    #[inline(always)]
    pub fn opac23(&self) -> OPAC23_R {
        OPAC23_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 22"]
    #[inline(always)]
    pub fn opac22(&self) -> OPAC22_R {
        OPAC22_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 21"]
    #[inline(always)]
    pub fn opac21(&self) -> OPAC21_R {
        OPAC21_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 20"]
    #[inline(always)]
    pub fn opac20(&self) -> OPAC20_R {
        OPAC20_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 19"]
    #[inline(always)]
    pub fn opac19(&self) -> OPAC19_R {
        OPAC19_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 18"]
    #[inline(always)]
    pub fn opac18(&self) -> OPAC18_R {
        OPAC18_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 17"]
    #[inline(always)]
    pub fn opac17(&self) -> OPAC17_R {
        OPAC17_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 16"]
    #[inline(always)]
    pub fn opac16(&self) -> OPAC16_R {
        OPAC16_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Off-platform Peripheral Access Control 23"]
    #[inline(always)]
    #[must_use]
    pub fn opac23(&mut self) -> OPAC23_W<0> {
        OPAC23_W::new(self)
    }
    #[doc = "Bits 4:7 - Off-platform Peripheral Access Control 22"]
    #[inline(always)]
    #[must_use]
    pub fn opac22(&mut self) -> OPAC22_W<4> {
        OPAC22_W::new(self)
    }
    #[doc = "Bits 8:11 - Off-platform Peripheral Access Control 21"]
    #[inline(always)]
    #[must_use]
    pub fn opac21(&mut self) -> OPAC21_W<8> {
        OPAC21_W::new(self)
    }
    #[doc = "Bits 12:15 - Off-platform Peripheral Access Control 20"]
    #[inline(always)]
    #[must_use]
    pub fn opac20(&mut self) -> OPAC20_W<12> {
        OPAC20_W::new(self)
    }
    #[doc = "Bits 16:19 - Off-platform Peripheral Access Control 19"]
    #[inline(always)]
    #[must_use]
    pub fn opac19(&mut self) -> OPAC19_W<16> {
        OPAC19_W::new(self)
    }
    #[doc = "Bits 20:23 - Off-platform Peripheral Access Control 18"]
    #[inline(always)]
    #[must_use]
    pub fn opac18(&mut self) -> OPAC18_W<20> {
        OPAC18_W::new(self)
    }
    #[doc = "Bits 24:27 - Off-platform Peripheral Access Control 17"]
    #[inline(always)]
    #[must_use]
    pub fn opac17(&mut self) -> OPAC17_W<24> {
        OPAC17_W::new(self)
    }
    #[doc = "Bits 28:31 - Off-platform Peripheral Access Control 16"]
    #[inline(always)]
    #[must_use]
    pub fn opac16(&mut self) -> OPAC16_W<28> {
        OPAC16_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Off-Platform Peripheral Access Control Registers\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [opacr2](index.html) module"]
pub struct OPACR2_SPEC;
impl crate::RegisterSpec for OPACR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [opacr2::R](R) reader structure"]
impl crate::Readable for OPACR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [opacr2::W](W) writer structure"]
impl crate::Writable for OPACR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OPACR2 to value 0x4444_4444"]
impl crate::Resettable for OPACR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x4444_4444;
}
