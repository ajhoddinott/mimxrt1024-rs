#[doc = "Register `USBMODE` reader"]
pub struct R(crate::R<USBMODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USBMODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USBMODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USBMODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USBMODE` writer"]
pub struct W(crate::W<USBMODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USBMODE_SPEC>;
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
impl From<crate::W<USBMODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USBMODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CM` reader - Controller Mode - R/WO"]
pub type CM_R = crate::FieldReader<u8, CM_A>;
#[doc = "Controller Mode - R/WO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CM_A {
    #[doc = "0: Idle \\[Default for combination host/device\\]"]
    CM_0 = 0,
    #[doc = "2: Device Controller \\[Default for device only controller\\]"]
    CM_2 = 2,
    #[doc = "3: Host Controller \\[Default for host only controller\\]"]
    CM_3 = 3,
}
impl From<CM_A> for u8 {
    #[inline(always)]
    fn from(variant: CM_A) -> Self {
        variant as _
    }
}
impl CM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CM_A> {
        match self.bits {
            0 => Some(CM_A::CM_0),
            2 => Some(CM_A::CM_2),
            3 => Some(CM_A::CM_3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CM_0`"]
    #[inline(always)]
    pub fn is_cm_0(&self) -> bool {
        *self == CM_A::CM_0
    }
    #[doc = "Checks if the value of the field is `CM_2`"]
    #[inline(always)]
    pub fn is_cm_2(&self) -> bool {
        *self == CM_A::CM_2
    }
    #[doc = "Checks if the value of the field is `CM_3`"]
    #[inline(always)]
    pub fn is_cm_3(&self) -> bool {
        *self == CM_A::CM_3
    }
}
#[doc = "Field `CM` writer - Controller Mode - R/WO"]
pub type CM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USBMODE_SPEC, u8, CM_A, 2, O>;
impl<'a, const O: u8> CM_W<'a, O> {
    #[doc = "Idle \\[Default for combination host/device\\]"]
    #[inline(always)]
    pub fn cm_0(self) -> &'a mut W {
        self.variant(CM_A::CM_0)
    }
    #[doc = "Device Controller \\[Default for device only controller\\]"]
    #[inline(always)]
    pub fn cm_2(self) -> &'a mut W {
        self.variant(CM_A::CM_2)
    }
    #[doc = "Host Controller \\[Default for host only controller\\]"]
    #[inline(always)]
    pub fn cm_3(self) -> &'a mut W {
        self.variant(CM_A::CM_3)
    }
}
#[doc = "Field `ES` reader - Endian Select - Read/Write"]
pub type ES_R = crate::BitReader<ES_A>;
#[doc = "Endian Select - Read/Write\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ES_A {
    #[doc = "0: Little Endian \\[Default\\]"]
    ES_0 = 0,
    #[doc = "1: Big Endian"]
    ES_1 = 1,
}
impl From<ES_A> for bool {
    #[inline(always)]
    fn from(variant: ES_A) -> Self {
        variant as u8 != 0
    }
}
impl ES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ES_A {
        match self.bits {
            false => ES_A::ES_0,
            true => ES_A::ES_1,
        }
    }
    #[doc = "Checks if the value of the field is `ES_0`"]
    #[inline(always)]
    pub fn is_es_0(&self) -> bool {
        *self == ES_A::ES_0
    }
    #[doc = "Checks if the value of the field is `ES_1`"]
    #[inline(always)]
    pub fn is_es_1(&self) -> bool {
        *self == ES_A::ES_1
    }
}
#[doc = "Field `ES` writer - Endian Select - Read/Write"]
pub type ES_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBMODE_SPEC, ES_A, O>;
impl<'a, const O: u8> ES_W<'a, O> {
    #[doc = "Little Endian \\[Default\\]"]
    #[inline(always)]
    pub fn es_0(self) -> &'a mut W {
        self.variant(ES_A::ES_0)
    }
    #[doc = "Big Endian"]
    #[inline(always)]
    pub fn es_1(self) -> &'a mut W {
        self.variant(ES_A::ES_1)
    }
}
#[doc = "Field `SLOM` reader - Setup Lockout Mode"]
pub type SLOM_R = crate::BitReader<SLOM_A>;
#[doc = "Setup Lockout Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SLOM_A {
    #[doc = "0: Setup Lockouts On (default);"]
    SLOM_0 = 0,
    #[doc = "1: Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register ."]
    SLOM_1 = 1,
}
impl From<SLOM_A> for bool {
    #[inline(always)]
    fn from(variant: SLOM_A) -> Self {
        variant as u8 != 0
    }
}
impl SLOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SLOM_A {
        match self.bits {
            false => SLOM_A::SLOM_0,
            true => SLOM_A::SLOM_1,
        }
    }
    #[doc = "Checks if the value of the field is `SLOM_0`"]
    #[inline(always)]
    pub fn is_slom_0(&self) -> bool {
        *self == SLOM_A::SLOM_0
    }
    #[doc = "Checks if the value of the field is `SLOM_1`"]
    #[inline(always)]
    pub fn is_slom_1(&self) -> bool {
        *self == SLOM_A::SLOM_1
    }
}
#[doc = "Field `SLOM` writer - Setup Lockout Mode"]
pub type SLOM_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBMODE_SPEC, SLOM_A, O>;
impl<'a, const O: u8> SLOM_W<'a, O> {
    #[doc = "Setup Lockouts On (default);"]
    #[inline(always)]
    pub fn slom_0(self) -> &'a mut W {
        self.variant(SLOM_A::SLOM_0)
    }
    #[doc = "Setup Lockouts Off (DCD requires use of Setup Data Buffer Tripwire in USBCMDUSB Command Register ."]
    #[inline(always)]
    pub fn slom_1(self) -> &'a mut W {
        self.variant(SLOM_A::SLOM_1)
    }
}
#[doc = "Field `SDIS` reader - Stream Disable Mode"]
pub type SDIS_R = crate::BitReader<bool>;
#[doc = "Field `SDIS` writer - Stream Disable Mode"]
pub type SDIS_W<'a, const O: u8> = crate::BitWriter<'a, u32, USBMODE_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:1 - Controller Mode - R/WO"]
    #[inline(always)]
    pub fn cm(&self) -> CM_R {
        CM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Endian Select - Read/Write"]
    #[inline(always)]
    pub fn es(&self) -> ES_R {
        ES_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Setup Lockout Mode"]
    #[inline(always)]
    pub fn slom(&self) -> SLOM_R {
        SLOM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Stream Disable Mode"]
    #[inline(always)]
    pub fn sdis(&self) -> SDIS_R {
        SDIS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controller Mode - R/WO"]
    #[inline(always)]
    #[must_use]
    pub fn cm(&mut self) -> CM_W<0> {
        CM_W::new(self)
    }
    #[doc = "Bit 2 - Endian Select - Read/Write"]
    #[inline(always)]
    #[must_use]
    pub fn es(&mut self) -> ES_W<2> {
        ES_W::new(self)
    }
    #[doc = "Bit 3 - Setup Lockout Mode"]
    #[inline(always)]
    #[must_use]
    pub fn slom(&mut self) -> SLOM_W<3> {
        SLOM_W::new(self)
    }
    #[doc = "Bit 4 - Stream Disable Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sdis(&mut self) -> SDIS_W<4> {
        SDIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Device Mode\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [usbmode](index.html) module"]
pub struct USBMODE_SPEC;
impl crate::RegisterSpec for USBMODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [usbmode::R](R) reader structure"]
impl crate::Readable for USBMODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [usbmode::W](W) writer structure"]
impl crate::Writable for USBMODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USBMODE to value 0x5000"]
impl crate::Resettable for USBMODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x5000;
}
