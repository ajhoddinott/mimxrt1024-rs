#[doc = "Register `KPCR` reader"]
pub struct R(crate::R<KPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KPCR` writer"]
pub struct W(crate::W<KPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KPCR_SPEC>;
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
impl From<crate::W<KPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KRE` reader - Keypad Row Enable"]
pub type KRE_R = crate::FieldReader<u8, KRE_A>;
#[doc = "Keypad Row Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KRE_A {
    #[doc = "0: Row is not included in the keypad key press detect."]
    KRE_0 = 0,
    #[doc = "1: Row is included in the keypad key press detect."]
    KRE_1 = 1,
}
impl From<KRE_A> for u8 {
    #[inline(always)]
    fn from(variant: KRE_A) -> Self {
        variant as _
    }
}
impl KRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KRE_A> {
        match self.bits {
            0 => Some(KRE_A::KRE_0),
            1 => Some(KRE_A::KRE_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `KRE_0`"]
    #[inline(always)]
    pub fn is_kre_0(&self) -> bool {
        *self == KRE_A::KRE_0
    }
    #[doc = "Checks if the value of the field is `KRE_1`"]
    #[inline(always)]
    pub fn is_kre_1(&self) -> bool {
        *self == KRE_A::KRE_1
    }
}
#[doc = "Field `KRE` writer - Keypad Row Enable"]
pub type KRE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, KPCR_SPEC, u8, KRE_A, 8, O>;
impl<'a, const O: u8> KRE_W<'a, O> {
    #[doc = "Row is not included in the keypad key press detect."]
    #[inline(always)]
    pub fn kre_0(self) -> &'a mut W {
        self.variant(KRE_A::KRE_0)
    }
    #[doc = "Row is included in the keypad key press detect."]
    #[inline(always)]
    pub fn kre_1(self) -> &'a mut W {
        self.variant(KRE_A::KRE_1)
    }
}
#[doc = "Field `KCO` reader - Keypad Column Strobe Open-Drain Enable"]
pub type KCO_R = crate::FieldReader<u8, KCO_A>;
#[doc = "Keypad Column Strobe Open-Drain Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum KCO_A {
    #[doc = "0: Column strobe output is totem pole drive."]
    TOTEM_POLE = 0,
    #[doc = "1: Column strobe output is open drain."]
    OPEN_DRAIN = 1,
}
impl From<KCO_A> for u8 {
    #[inline(always)]
    fn from(variant: KCO_A) -> Self {
        variant as _
    }
}
impl KCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<KCO_A> {
        match self.bits {
            0 => Some(KCO_A::TOTEM_POLE),
            1 => Some(KCO_A::OPEN_DRAIN),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TOTEM_POLE`"]
    #[inline(always)]
    pub fn is_totem_pole(&self) -> bool {
        *self == KCO_A::TOTEM_POLE
    }
    #[doc = "Checks if the value of the field is `OPEN_DRAIN`"]
    #[inline(always)]
    pub fn is_open_drain(&self) -> bool {
        *self == KCO_A::OPEN_DRAIN
    }
}
#[doc = "Field `KCO` writer - Keypad Column Strobe Open-Drain Enable"]
pub type KCO_W<'a, const O: u8> = crate::FieldWriter<'a, u16, KPCR_SPEC, u8, KCO_A, 8, O>;
impl<'a, const O: u8> KCO_W<'a, O> {
    #[doc = "Column strobe output is totem pole drive."]
    #[inline(always)]
    pub fn totem_pole(self) -> &'a mut W {
        self.variant(KCO_A::TOTEM_POLE)
    }
    #[doc = "Column strobe output is open drain."]
    #[inline(always)]
    pub fn open_drain(self) -> &'a mut W {
        self.variant(KCO_A::OPEN_DRAIN)
    }
}
impl R {
    #[doc = "Bits 0:7 - Keypad Row Enable"]
    #[inline(always)]
    pub fn kre(&self) -> KRE_R {
        KRE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Keypad Column Strobe Open-Drain Enable"]
    #[inline(always)]
    pub fn kco(&self) -> KCO_R {
        KCO_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Keypad Row Enable"]
    #[inline(always)]
    #[must_use]
    pub fn kre(&mut self) -> KRE_W<0> {
        KRE_W::new(self)
    }
    #[doc = "Bits 8:15 - Keypad Column Strobe Open-Drain Enable"]
    #[inline(always)]
    #[must_use]
    pub fn kco(&mut self) -> KCO_W<8> {
        KCO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Keypad Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kpcr](index.html) module"]
pub struct KPCR_SPEC;
impl crate::RegisterSpec for KPCR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [kpcr::R](R) reader structure"]
impl crate::Readable for KPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kpcr::W](W) writer structure"]
impl crate::Writable for KPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KPCR to value 0"]
impl crate::Resettable for KPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
