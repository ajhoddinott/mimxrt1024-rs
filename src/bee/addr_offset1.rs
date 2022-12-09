#[doc = "Register `ADDR_OFFSET1` reader"]
pub struct R(crate::R<ADDR_OFFSET1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDR_OFFSET1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDR_OFFSET1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDR_OFFSET1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDR_OFFSET1` writer"]
pub struct W(crate::W<ADDR_OFFSET1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDR_OFFSET1_SPEC>;
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
impl From<crate::W<ADDR_OFFSET1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDR_OFFSET1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR_OFFSET1` reader - Signed offset for BEE region 1"]
pub type ADDR_OFFSET1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR_OFFSET1` writer - Signed offset for BEE region 1"]
pub type ADDR_OFFSET1_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR_OFFSET1_SPEC, u16, u16, 16, O>;
#[doc = "Field `ADDR_OFFSET1_LOCK` reader - Lock bits for addr_offset1"]
pub type ADDR_OFFSET1_LOCK_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR_OFFSET1_LOCK` writer - Lock bits for addr_offset1"]
pub type ADDR_OFFSET1_LOCK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, ADDR_OFFSET1_SPEC, u16, u16, 16, O>;
impl R {
    #[doc = "Bits 0:15 - Signed offset for BEE region 1"]
    #[inline(always)]
    pub fn addr_offset1(&self) -> ADDR_OFFSET1_R {
        ADDR_OFFSET1_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Lock bits for addr_offset1"]
    #[inline(always)]
    pub fn addr_offset1_lock(&self) -> ADDR_OFFSET1_LOCK_R {
        ADDR_OFFSET1_LOCK_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Signed offset for BEE region 1"]
    #[inline(always)]
    #[must_use]
    pub fn addr_offset1(&mut self) -> ADDR_OFFSET1_W<0> {
        ADDR_OFFSET1_W::new(self)
    }
    #[doc = "Bits 16:31 - Lock bits for addr_offset1"]
    #[inline(always)]
    #[must_use]
    pub fn addr_offset1_lock(&mut self) -> ADDR_OFFSET1_LOCK_W<16> {
        ADDR_OFFSET1_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Offset region 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addr_offset1](index.html) module"]
pub struct ADDR_OFFSET1_SPEC;
impl crate::RegisterSpec for ADDR_OFFSET1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addr_offset1::R](R) reader structure"]
impl crate::Readable for ADDR_OFFSET1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addr_offset1::W](W) writer structure"]
impl crate::Writable for ADDR_OFFSET1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ADDR_OFFSET1 to value 0xf000"]
impl crate::Resettable for ADDR_OFFSET1_SPEC {
    const RESET_VALUE: Self::Ux = 0xf000;
}
