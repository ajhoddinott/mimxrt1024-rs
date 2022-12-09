#[doc = "Register `KEY` reader"]
pub struct R(crate::R<KEY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY` writer"]
pub struct W(crate::W<KEY_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_SPEC>;
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
impl From<crate::W<KEY_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SUBWORD` reader - Key subword pointer"]
pub type SUBWORD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SUBWORD` writer - Key subword pointer"]
pub type SUBWORD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEY_SPEC, u8, u8, 2, O>;
#[doc = "Field `INDEX` reader - Key index pointer. The valid indices are 0-\\[number_keys\\]."]
pub type INDEX_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INDEX` writer - Key index pointer. The valid indices are 0-\\[number_keys\\]."]
pub type INDEX_W<'a, const O: u8> = crate::FieldWriter<'a, u32, KEY_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Key subword pointer"]
    #[inline(always)]
    pub fn subword(&self) -> SUBWORD_R {
        SUBWORD_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - Key index pointer. The valid indices are 0-\\[number_keys\\]."]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Key subword pointer"]
    #[inline(always)]
    #[must_use]
    pub fn subword(&mut self) -> SUBWORD_W<0> {
        SUBWORD_W::new(self)
    }
    #[doc = "Bits 4:5 - Key index pointer. The valid indices are 0-\\[number_keys\\]."]
    #[inline(always)]
    #[must_use]
    pub fn index(&mut self) -> INDEX_W<4> {
        INDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DCP key index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key](index.html) module"]
pub struct KEY_SPEC;
impl crate::RegisterSpec for KEY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key::R](R) reader structure"]
impl crate::Readable for KEY_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key::W](W) writer structure"]
impl crate::Writable for KEY_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY to value 0"]
impl crate::Resettable for KEY_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
