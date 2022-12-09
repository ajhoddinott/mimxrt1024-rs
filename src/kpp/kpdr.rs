#[doc = "Register `KPDR` reader"]
pub struct R(crate::R<KPDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KPDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KPDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KPDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KPDR` writer"]
pub struct W(crate::W<KPDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KPDR_SPEC>;
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
impl From<crate::W<KPDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KPDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KRD` reader - Keypad Row Data"]
pub type KRD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KRD` writer - Keypad Row Data"]
pub type KRD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, KPDR_SPEC, u8, u8, 8, O>;
#[doc = "Field `KCD` reader - Keypad Column Data"]
pub type KCD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KCD` writer - Keypad Column Data"]
pub type KCD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, KPDR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Keypad Row Data"]
    #[inline(always)]
    pub fn krd(&self) -> KRD_R {
        KRD_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Keypad Column Data"]
    #[inline(always)]
    pub fn kcd(&self) -> KCD_R {
        KCD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Keypad Row Data"]
    #[inline(always)]
    #[must_use]
    pub fn krd(&mut self) -> KRD_W<0> {
        KRD_W::new(self)
    }
    #[doc = "Bits 8:15 - Keypad Column Data"]
    #[inline(always)]
    #[must_use]
    pub fn kcd(&mut self) -> KCD_W<8> {
        KCD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Keypad Data Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [kpdr](index.html) module"]
pub struct KPDR_SPEC;
impl crate::RegisterSpec for KPDR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [kpdr::R](R) reader structure"]
impl crate::Readable for KPDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [kpdr::W](W) writer structure"]
impl crate::Writable for KPDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KPDR to value 0"]
impl crate::Resettable for KPDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
