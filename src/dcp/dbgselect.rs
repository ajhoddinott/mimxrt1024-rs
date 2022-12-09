#[doc = "Register `DBGSELECT` reader"]
pub struct R(crate::R<DBGSELECT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBGSELECT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBGSELECT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBGSELECT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBGSELECT` writer"]
pub struct W(crate::W<DBGSELECT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBGSELECT_SPEC>;
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
impl From<crate::W<DBGSELECT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBGSELECT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INDEX` reader - Selects a value to read via the debug data register."]
pub type INDEX_R = crate::FieldReader<u8, INDEX_A>;
#[doc = "Selects a value to read via the debug data register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum INDEX_A {
    #[doc = "1: CONTROL"]
    CONTROL = 1,
    #[doc = "16: OTPKEY0"]
    OTPKEY0 = 16,
    #[doc = "17: OTPKEY1"]
    OTPKEY1 = 17,
    #[doc = "18: OTPKEY2"]
    OTPKEY2 = 18,
    #[doc = "19: OTPKEY3"]
    OTPKEY3 = 19,
}
impl From<INDEX_A> for u8 {
    #[inline(always)]
    fn from(variant: INDEX_A) -> Self {
        variant as _
    }
}
impl INDEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<INDEX_A> {
        match self.bits {
            1 => Some(INDEX_A::CONTROL),
            16 => Some(INDEX_A::OTPKEY0),
            17 => Some(INDEX_A::OTPKEY1),
            18 => Some(INDEX_A::OTPKEY2),
            19 => Some(INDEX_A::OTPKEY3),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CONTROL`"]
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        *self == INDEX_A::CONTROL
    }
    #[doc = "Checks if the value of the field is `OTPKEY0`"]
    #[inline(always)]
    pub fn is_otpkey0(&self) -> bool {
        *self == INDEX_A::OTPKEY0
    }
    #[doc = "Checks if the value of the field is `OTPKEY1`"]
    #[inline(always)]
    pub fn is_otpkey1(&self) -> bool {
        *self == INDEX_A::OTPKEY1
    }
    #[doc = "Checks if the value of the field is `OTPKEY2`"]
    #[inline(always)]
    pub fn is_otpkey2(&self) -> bool {
        *self == INDEX_A::OTPKEY2
    }
    #[doc = "Checks if the value of the field is `OTPKEY3`"]
    #[inline(always)]
    pub fn is_otpkey3(&self) -> bool {
        *self == INDEX_A::OTPKEY3
    }
}
#[doc = "Field `INDEX` writer - Selects a value to read via the debug data register."]
pub type INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBGSELECT_SPEC, u8, INDEX_A, 8, O>;
impl<'a, const O: u8> INDEX_W<'a, O> {
    #[doc = "CONTROL"]
    #[inline(always)]
    pub fn control(self) -> &'a mut W {
        self.variant(INDEX_A::CONTROL)
    }
    #[doc = "OTPKEY0"]
    #[inline(always)]
    pub fn otpkey0(self) -> &'a mut W {
        self.variant(INDEX_A::OTPKEY0)
    }
    #[doc = "OTPKEY1"]
    #[inline(always)]
    pub fn otpkey1(self) -> &'a mut W {
        self.variant(INDEX_A::OTPKEY1)
    }
    #[doc = "OTPKEY2"]
    #[inline(always)]
    pub fn otpkey2(self) -> &'a mut W {
        self.variant(INDEX_A::OTPKEY2)
    }
    #[doc = "OTPKEY3"]
    #[inline(always)]
    pub fn otpkey3(self) -> &'a mut W {
        self.variant(INDEX_A::OTPKEY3)
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects a value to read via the debug data register."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects a value to read via the debug data register."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<0> {
        INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP debug select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbgselect](index.html) module"]
pub struct DBGSELECT_SPEC;
impl crate::RegisterSpec for DBGSELECT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbgselect::R](R) reader structure"]
impl crate::Readable for DBGSELECT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbgselect::W](W) writer structure"]
impl crate::Writable for DBGSELECT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBGSELECT to value 0"]
impl crate::Resettable for DBGSELECT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
