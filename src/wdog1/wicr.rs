#[doc = "Register `WICR` reader"]
pub struct R(crate::R<WICR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WICR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WICR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WICR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WICR` writer"]
pub struct W(crate::W<WICR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WICR_SPEC>;
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
impl From<crate::W<WICR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WICR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WICT` reader - WICT"]
pub type WICT_R = crate::FieldReader<u8, WICT_A>;
#[doc = "WICT\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WICT_A {
    #[doc = "0: WICT\\[7:0\\]
= Time duration between interrupt and time-out is 0 seconds."]
    WICT_0 = 0,
    #[doc = "1: WICT\\[7:0\\]
= Time duration between interrupt and time-out is 0.5 seconds."]
    WICT_1 = 1,
    #[doc = "4: WICT\\[7:0\\]
= Time duration between interrupt and time-out is 2 seconds (Default)."]
    WICT_4 = 4,
    #[doc = "255: WICT\\[7:0\\]
= Time duration between interrupt and time-out is 127.5 seconds."]
    WICT_255 = 255,
}
impl From<WICT_A> for u8 {
    #[inline(always)]
    fn from(variant: WICT_A) -> Self {
        variant as _
    }
}
impl WICT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WICT_A> {
        match self.bits {
            0 => Some(WICT_A::WICT_0),
            1 => Some(WICT_A::WICT_1),
            4 => Some(WICT_A::WICT_4),
            255 => Some(WICT_A::WICT_255),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `WICT_0`"]
    #[inline(always)]
    pub fn is_wict_0(&self) -> bool {
        *self == WICT_A::WICT_0
    }
    #[doc = "Checks if the value of the field is `WICT_1`"]
    #[inline(always)]
    pub fn is_wict_1(&self) -> bool {
        *self == WICT_A::WICT_1
    }
    #[doc = "Checks if the value of the field is `WICT_4`"]
    #[inline(always)]
    pub fn is_wict_4(&self) -> bool {
        *self == WICT_A::WICT_4
    }
    #[doc = "Checks if the value of the field is `WICT_255`"]
    #[inline(always)]
    pub fn is_wict_255(&self) -> bool {
        *self == WICT_A::WICT_255
    }
}
#[doc = "Field `WICT` writer - WICT"]
pub type WICT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WICR_SPEC, u8, WICT_A, 8, O>;
impl<'a, const O: u8> WICT_W<'a, O> {
    #[doc = "WICT\\[7:0\\]
= Time duration between interrupt and time-out is 0 seconds."]
    #[inline(always)]
    pub fn wict_0(self) -> &'a mut W {
        self.variant(WICT_A::WICT_0)
    }
    #[doc = "WICT\\[7:0\\]
= Time duration between interrupt and time-out is 0.5 seconds."]
    #[inline(always)]
    pub fn wict_1(self) -> &'a mut W {
        self.variant(WICT_A::WICT_1)
    }
    #[doc = "WICT\\[7:0\\]
= Time duration between interrupt and time-out is 2 seconds (Default)."]
    #[inline(always)]
    pub fn wict_4(self) -> &'a mut W {
        self.variant(WICT_A::WICT_4)
    }
    #[doc = "WICT\\[7:0\\]
= Time duration between interrupt and time-out is 127.5 seconds."]
    #[inline(always)]
    pub fn wict_255(self) -> &'a mut W {
        self.variant(WICT_A::WICT_255)
    }
}
#[doc = "Field `WTIS` reader - WTIS"]
pub type WTIS_R = crate::BitReader<WTIS_A>;
#[doc = "WTIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WTIS_A {
    #[doc = "0: No interrupt has occurred (Default)."]
    WTIS_0 = 0,
    #[doc = "1: Interrupt has occurred"]
    WTIS_1 = 1,
}
impl From<WTIS_A> for bool {
    #[inline(always)]
    fn from(variant: WTIS_A) -> Self {
        variant as u8 != 0
    }
}
impl WTIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WTIS_A {
        match self.bits {
            false => WTIS_A::WTIS_0,
            true => WTIS_A::WTIS_1,
        }
    }
    #[doc = "Checks if the value of the field is `WTIS_0`"]
    #[inline(always)]
    pub fn is_wtis_0(&self) -> bool {
        *self == WTIS_A::WTIS_0
    }
    #[doc = "Checks if the value of the field is `WTIS_1`"]
    #[inline(always)]
    pub fn is_wtis_1(&self) -> bool {
        *self == WTIS_A::WTIS_1
    }
}
#[doc = "Field `WTIS` writer - WTIS"]
pub type WTIS_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, WICR_SPEC, WTIS_A, O>;
impl<'a, const O: u8> WTIS_W<'a, O> {
    #[doc = "No interrupt has occurred (Default)."]
    #[inline(always)]
    pub fn wtis_0(self) -> &'a mut W {
        self.variant(WTIS_A::WTIS_0)
    }
    #[doc = "Interrupt has occurred"]
    #[inline(always)]
    pub fn wtis_1(self) -> &'a mut W {
        self.variant(WTIS_A::WTIS_1)
    }
}
#[doc = "Field `WIE` reader - WIE"]
pub type WIE_R = crate::BitReader<WIE_A>;
#[doc = "WIE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WIE_A {
    #[doc = "0: Disable Interrupt (Default)."]
    WIE_0 = 0,
    #[doc = "1: Enable Interrupt."]
    WIE_1 = 1,
}
impl From<WIE_A> for bool {
    #[inline(always)]
    fn from(variant: WIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WIE_A {
        match self.bits {
            false => WIE_A::WIE_0,
            true => WIE_A::WIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `WIE_0`"]
    #[inline(always)]
    pub fn is_wie_0(&self) -> bool {
        *self == WIE_A::WIE_0
    }
    #[doc = "Checks if the value of the field is `WIE_1`"]
    #[inline(always)]
    pub fn is_wie_1(&self) -> bool {
        *self == WIE_A::WIE_1
    }
}
#[doc = "Field `WIE` writer - WIE"]
pub type WIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, WICR_SPEC, WIE_A, O>;
impl<'a, const O: u8> WIE_W<'a, O> {
    #[doc = "Disable Interrupt (Default)."]
    #[inline(always)]
    pub fn wie_0(self) -> &'a mut W {
        self.variant(WIE_A::WIE_0)
    }
    #[doc = "Enable Interrupt."]
    #[inline(always)]
    pub fn wie_1(self) -> &'a mut W {
        self.variant(WIE_A::WIE_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - WICT"]
    #[inline(always)]
    pub fn wict(&self) -> WICT_R {
        WICT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 14 - WTIS"]
    #[inline(always)]
    pub fn wtis(&self) -> WTIS_R {
        WTIS_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - WIE"]
    #[inline(always)]
    pub fn wie(&self) -> WIE_R {
        WIE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - WICT"]
    #[inline(always)]
    #[must_use]
    pub fn wict(&mut self) -> WICT_W<0> {
        WICT_W::new(self)
    }
    #[doc = "Bit 14 - WTIS"]
    #[inline(always)]
    #[must_use]
    pub fn wtis(&mut self) -> WTIS_W<14> {
        WTIS_W::new(self)
    }
    #[doc = "Bit 15 - WIE"]
    #[inline(always)]
    #[must_use]
    pub fn wie(&mut self) -> WIE_W<15> {
        WIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Interrupt Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wicr](index.html) module"]
pub struct WICR_SPEC;
impl crate::RegisterSpec for WICR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wicr::R](R) reader structure"]
impl crate::Readable for WICR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wicr::W](W) writer structure"]
impl crate::Writable for WICR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x4000;
}
#[doc = "`reset()` method sets WICR to value 0x04"]
impl crate::Resettable for WICR_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
