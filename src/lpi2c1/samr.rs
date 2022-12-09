#[doc = "Register `SAMR` reader"]
pub struct R(crate::R<SAMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAMR` writer"]
pub struct W(crate::W<SAMR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAMR_SPEC>;
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
impl From<crate::W<SAMR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAMR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDR0` reader - Address 0 Value"]
pub type ADDR0_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR0` writer - Address 0 Value"]
pub type ADDR0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMR_SPEC, u16, u16, 10, O>;
#[doc = "Field `ADDR1` reader - Address 1 Value"]
pub type ADDR1_R = crate::FieldReader<u16, u16>;
#[doc = "Field `ADDR1` writer - Address 1 Value"]
pub type ADDR1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SAMR_SPEC, u16, u16, 10, O>;
impl R {
    #[doc = "Bits 1:10 - Address 0 Value"]
    #[inline(always)]
    pub fn addr0(&self) -> ADDR0_R {
        ADDR0_R::new(((self.bits >> 1) & 0x03ff) as u16)
    }
    #[doc = "Bits 17:26 - Address 1 Value"]
    #[inline(always)]
    pub fn addr1(&self) -> ADDR1_R {
        ADDR1_R::new(((self.bits >> 17) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 1:10 - Address 0 Value"]
    #[inline(always)]
    #[must_use]
    pub fn addr0(&mut self) -> ADDR0_W<1> {
        ADDR0_W::new(self)
    }
    #[doc = "Bits 17:26 - Address 1 Value"]
    #[inline(always)]
    #[must_use]
    pub fn addr1(&mut self) -> ADDR1_W<17> {
        ADDR1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Match\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [samr](index.html) module"]
pub struct SAMR_SPEC;
impl crate::RegisterSpec for SAMR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [samr::R](R) reader structure"]
impl crate::Readable for SAMR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [samr::W](W) writer structure"]
impl crate::Writable for SAMR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAMR to value 0"]
impl crate::Resettable for SAMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
