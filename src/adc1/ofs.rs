#[doc = "Register `OFS` reader"]
pub struct R(crate::R<OFS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OFS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OFS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OFS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OFS` writer"]
pub struct W(crate::W<OFS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OFS_SPEC>;
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
impl From<crate::W<OFS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OFS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OFS` reader - Offset value"]
pub type OFS_R = crate::FieldReader<u16, u16>;
#[doc = "Field `OFS` writer - Offset value"]
pub type OFS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, OFS_SPEC, u16, u16, 12, O>;
#[doc = "Field `SIGN` reader - Sign bit"]
pub type SIGN_R = crate::BitReader<SIGN_A>;
#[doc = "Sign bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SIGN_A {
    #[doc = "0: The offset value is added with the raw result"]
    SIGN_0 = 0,
    #[doc = "1: The offset value is subtracted from the raw converted value"]
    SIGN_1 = 1,
}
impl From<SIGN_A> for bool {
    #[inline(always)]
    fn from(variant: SIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl SIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SIGN_A {
        match self.bits {
            false => SIGN_A::SIGN_0,
            true => SIGN_A::SIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `SIGN_0`"]
    #[inline(always)]
    pub fn is_sign_0(&self) -> bool {
        *self == SIGN_A::SIGN_0
    }
    #[doc = "Checks if the value of the field is `SIGN_1`"]
    #[inline(always)]
    pub fn is_sign_1(&self) -> bool {
        *self == SIGN_A::SIGN_1
    }
}
#[doc = "Field `SIGN` writer - Sign bit"]
pub type SIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, OFS_SPEC, SIGN_A, O>;
impl<'a, const O: u8> SIGN_W<'a, O> {
    #[doc = "The offset value is added with the raw result"]
    #[inline(always)]
    pub fn sign_0(self) -> &'a mut W {
        self.variant(SIGN_A::SIGN_0)
    }
    #[doc = "The offset value is subtracted from the raw converted value"]
    #[inline(always)]
    pub fn sign_1(self) -> &'a mut W {
        self.variant(SIGN_A::SIGN_1)
    }
}
impl R {
    #[doc = "Bits 0:11 - Offset value"]
    #[inline(always)]
    pub fn ofs(&self) -> OFS_R {
        OFS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 12 - Sign bit"]
    #[inline(always)]
    pub fn sign(&self) -> SIGN_R {
        SIGN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:11 - Offset value"]
    #[inline(always)]
    #[must_use]
    pub fn ofs(&mut self) -> OFS_W<0> {
        OFS_W::new(self)
    }
    #[doc = "Bit 12 - Sign bit"]
    #[inline(always)]
    #[must_use]
    pub fn sign(&mut self) -> SIGN_W<12> {
        SIGN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset correction value register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ofs](index.html) module"]
pub struct OFS_SPEC;
impl crate::RegisterSpec for OFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ofs::R](R) reader structure"]
impl crate::Readable for OFS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ofs::W](W) writer structure"]
impl crate::Writable for OFS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OFS to value 0"]
impl crate::Resettable for OFS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
