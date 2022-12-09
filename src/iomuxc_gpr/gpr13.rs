#[doc = "Register `GPR13` reader"]
pub struct R(crate::R<GPR13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPR13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPR13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPR13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPR13` writer"]
pub struct W(crate::W<GPR13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPR13_SPEC>;
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
impl From<crate::W<GPR13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPR13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARCACHE_USDHC` reader - uSDHC block cacheable attribute value of AXI read transactions"]
pub type ARCACHE_USDHC_R = crate::BitReader<ARCACHE_USDHC_A>;
#[doc = "uSDHC block cacheable attribute value of AXI read transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARCACHE_USDHC_A {
    #[doc = "0: Cacheable attribute is off for read transactions"]
    ARCACHE_USDHC_0 = 0,
    #[doc = "1: Cacheable attribute is on for read transactions"]
    ARCACHE_USDHC_1 = 1,
}
impl From<ARCACHE_USDHC_A> for bool {
    #[inline(always)]
    fn from(variant: ARCACHE_USDHC_A) -> Self {
        variant as u8 != 0
    }
}
impl ARCACHE_USDHC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARCACHE_USDHC_A {
        match self.bits {
            false => ARCACHE_USDHC_A::ARCACHE_USDHC_0,
            true => ARCACHE_USDHC_A::ARCACHE_USDHC_1,
        }
    }
    #[doc = "Checks if the value of the field is `ARCACHE_USDHC_0`"]
    #[inline(always)]
    pub fn is_arcache_usdhc_0(&self) -> bool {
        *self == ARCACHE_USDHC_A::ARCACHE_USDHC_0
    }
    #[doc = "Checks if the value of the field is `ARCACHE_USDHC_1`"]
    #[inline(always)]
    pub fn is_arcache_usdhc_1(&self) -> bool {
        *self == ARCACHE_USDHC_A::ARCACHE_USDHC_1
    }
}
#[doc = "Field `ARCACHE_USDHC` writer - uSDHC block cacheable attribute value of AXI read transactions"]
pub type ARCACHE_USDHC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR13_SPEC, ARCACHE_USDHC_A, O>;
impl<'a, const O: u8> ARCACHE_USDHC_W<'a, O> {
    #[doc = "Cacheable attribute is off for read transactions"]
    #[inline(always)]
    pub fn arcache_usdhc_0(self) -> &'a mut W {
        self.variant(ARCACHE_USDHC_A::ARCACHE_USDHC_0)
    }
    #[doc = "Cacheable attribute is on for read transactions"]
    #[inline(always)]
    pub fn arcache_usdhc_1(self) -> &'a mut W {
        self.variant(ARCACHE_USDHC_A::ARCACHE_USDHC_1)
    }
}
#[doc = "Field `AWCACHE_USDHC` reader - uSDHC block cacheable attribute value of AXI write transactions"]
pub type AWCACHE_USDHC_R = crate::BitReader<AWCACHE_USDHC_A>;
#[doc = "uSDHC block cacheable attribute value of AXI write transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AWCACHE_USDHC_A {
    #[doc = "0: Cacheable attribute is off for write transactions"]
    AWCACHE_USDHC_0 = 0,
    #[doc = "1: Cacheable attribute is on for write transactions"]
    AWCACHE_USDHC_1 = 1,
}
impl From<AWCACHE_USDHC_A> for bool {
    #[inline(always)]
    fn from(variant: AWCACHE_USDHC_A) -> Self {
        variant as u8 != 0
    }
}
impl AWCACHE_USDHC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWCACHE_USDHC_A {
        match self.bits {
            false => AWCACHE_USDHC_A::AWCACHE_USDHC_0,
            true => AWCACHE_USDHC_A::AWCACHE_USDHC_1,
        }
    }
    #[doc = "Checks if the value of the field is `AWCACHE_USDHC_0`"]
    #[inline(always)]
    pub fn is_awcache_usdhc_0(&self) -> bool {
        *self == AWCACHE_USDHC_A::AWCACHE_USDHC_0
    }
    #[doc = "Checks if the value of the field is `AWCACHE_USDHC_1`"]
    #[inline(always)]
    pub fn is_awcache_usdhc_1(&self) -> bool {
        *self == AWCACHE_USDHC_A::AWCACHE_USDHC_1
    }
}
#[doc = "Field `AWCACHE_USDHC` writer - uSDHC block cacheable attribute value of AXI write transactions"]
pub type AWCACHE_USDHC_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, GPR13_SPEC, AWCACHE_USDHC_A, O>;
impl<'a, const O: u8> AWCACHE_USDHC_W<'a, O> {
    #[doc = "Cacheable attribute is off for write transactions"]
    #[inline(always)]
    pub fn awcache_usdhc_0(self) -> &'a mut W {
        self.variant(AWCACHE_USDHC_A::AWCACHE_USDHC_0)
    }
    #[doc = "Cacheable attribute is on for write transactions"]
    #[inline(always)]
    pub fn awcache_usdhc_1(self) -> &'a mut W {
        self.variant(AWCACHE_USDHC_A::AWCACHE_USDHC_1)
    }
}
#[doc = "Field `CACHE_ENET` reader - ENET block cacheable attribute value of AXI transactions"]
pub type CACHE_ENET_R = crate::BitReader<CACHE_ENET_A>;
#[doc = "ENET block cacheable attribute value of AXI transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHE_ENET_A {
    #[doc = "0: Cacheable attribute is off for read/write transactions"]
    CACHE_ENET_0 = 0,
    #[doc = "1: Cacheable attribute is on for read/write transactions"]
    CACHE_ENET_1 = 1,
}
impl From<CACHE_ENET_A> for bool {
    #[inline(always)]
    fn from(variant: CACHE_ENET_A) -> Self {
        variant as u8 != 0
    }
}
impl CACHE_ENET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_ENET_A {
        match self.bits {
            false => CACHE_ENET_A::CACHE_ENET_0,
            true => CACHE_ENET_A::CACHE_ENET_1,
        }
    }
    #[doc = "Checks if the value of the field is `CACHE_ENET_0`"]
    #[inline(always)]
    pub fn is_cache_enet_0(&self) -> bool {
        *self == CACHE_ENET_A::CACHE_ENET_0
    }
    #[doc = "Checks if the value of the field is `CACHE_ENET_1`"]
    #[inline(always)]
    pub fn is_cache_enet_1(&self) -> bool {
        *self == CACHE_ENET_A::CACHE_ENET_1
    }
}
#[doc = "Field `CACHE_ENET` writer - ENET block cacheable attribute value of AXI transactions"]
pub type CACHE_ENET_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR13_SPEC, CACHE_ENET_A, O>;
impl<'a, const O: u8> CACHE_ENET_W<'a, O> {
    #[doc = "Cacheable attribute is off for read/write transactions"]
    #[inline(always)]
    pub fn cache_enet_0(self) -> &'a mut W {
        self.variant(CACHE_ENET_A::CACHE_ENET_0)
    }
    #[doc = "Cacheable attribute is on for read/write transactions"]
    #[inline(always)]
    pub fn cache_enet_1(self) -> &'a mut W {
        self.variant(CACHE_ENET_A::CACHE_ENET_1)
    }
}
#[doc = "Field `CACHE_USB` reader - USB block cacheable attribute value of AXI transactions"]
pub type CACHE_USB_R = crate::BitReader<CACHE_USB_A>;
#[doc = "USB block cacheable attribute value of AXI transactions\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CACHE_USB_A {
    #[doc = "0: Cacheable attribute is off for read/write transactions"]
    CACHE_USB_0 = 0,
    #[doc = "1: Cacheable attribute is on for read/write transactions"]
    CACHE_USB_1 = 1,
}
impl From<CACHE_USB_A> for bool {
    #[inline(always)]
    fn from(variant: CACHE_USB_A) -> Self {
        variant as u8 != 0
    }
}
impl CACHE_USB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CACHE_USB_A {
        match self.bits {
            false => CACHE_USB_A::CACHE_USB_0,
            true => CACHE_USB_A::CACHE_USB_1,
        }
    }
    #[doc = "Checks if the value of the field is `CACHE_USB_0`"]
    #[inline(always)]
    pub fn is_cache_usb_0(&self) -> bool {
        *self == CACHE_USB_A::CACHE_USB_0
    }
    #[doc = "Checks if the value of the field is `CACHE_USB_1`"]
    #[inline(always)]
    pub fn is_cache_usb_1(&self) -> bool {
        *self == CACHE_USB_A::CACHE_USB_1
    }
}
#[doc = "Field `CACHE_USB` writer - USB block cacheable attribute value of AXI transactions"]
pub type CACHE_USB_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPR13_SPEC, CACHE_USB_A, O>;
impl<'a, const O: u8> CACHE_USB_W<'a, O> {
    #[doc = "Cacheable attribute is off for read/write transactions"]
    #[inline(always)]
    pub fn cache_usb_0(self) -> &'a mut W {
        self.variant(CACHE_USB_A::CACHE_USB_0)
    }
    #[doc = "Cacheable attribute is on for read/write transactions"]
    #[inline(always)]
    pub fn cache_usb_1(self) -> &'a mut W {
        self.variant(CACHE_USB_A::CACHE_USB_1)
    }
}
impl R {
    #[doc = "Bit 0 - uSDHC block cacheable attribute value of AXI read transactions"]
    #[inline(always)]
    pub fn arcache_usdhc(&self) -> ARCACHE_USDHC_R {
        ARCACHE_USDHC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - uSDHC block cacheable attribute value of AXI write transactions"]
    #[inline(always)]
    pub fn awcache_usdhc(&self) -> AWCACHE_USDHC_R {
        AWCACHE_USDHC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 7 - ENET block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub fn cache_enet(&self) -> CACHE_ENET_R {
        CACHE_ENET_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 13 - USB block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    pub fn cache_usb(&self) -> CACHE_USB_R {
        CACHE_USB_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - uSDHC block cacheable attribute value of AXI read transactions"]
    #[inline(always)]
    #[must_use]
    pub fn arcache_usdhc(&mut self) -> ARCACHE_USDHC_W<0> {
        ARCACHE_USDHC_W::new(self)
    }
    #[doc = "Bit 1 - uSDHC block cacheable attribute value of AXI write transactions"]
    #[inline(always)]
    #[must_use]
    pub fn awcache_usdhc(&mut self) -> AWCACHE_USDHC_W<1> {
        AWCACHE_USDHC_W::new(self)
    }
    #[doc = "Bit 7 - ENET block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    #[must_use]
    pub fn cache_enet(&mut self) -> CACHE_ENET_W<7> {
        CACHE_ENET_W::new(self)
    }
    #[doc = "Bit 13 - USB block cacheable attribute value of AXI transactions"]
    #[inline(always)]
    #[must_use]
    pub fn cache_usb(&mut self) -> CACHE_USB_W<13> {
        CACHE_USB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPR13 General Purpose Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gpr13](index.html) module"]
pub struct GPR13_SPEC;
impl crate::RegisterSpec for GPR13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gpr13::R](R) reader structure"]
impl crate::Readable for GPR13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gpr13::W](W) writer structure"]
impl crate::Writable for GPR13_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPR13 to value 0"]
impl crate::Resettable for GPR13_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
