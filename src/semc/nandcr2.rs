#[doc = "Register `NANDCR2` reader"]
pub struct R(crate::R<NANDCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NANDCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NANDCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NANDCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NANDCR2` writer"]
pub struct W(crate::W<NANDCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NANDCR2_SPEC>;
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
impl From<crate::W<NANDCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NANDCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TWHR` reader - WE# high to RE# low time"]
pub type TWHR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWHR` writer - WE# high to RE# low time"]
pub type TWHR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NANDCR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `TRHW` reader - RE# high to WE# low time"]
pub type TRHW_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRHW` writer - RE# high to WE# low time"]
pub type TRHW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NANDCR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `TADL` reader - Address cycle to data loading time"]
pub type TADL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TADL` writer - Address cycle to data loading time"]
pub type TADL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NANDCR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `TRR` reader - Ready to RE# low time"]
pub type TRR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TRR` writer - Ready to RE# low time"]
pub type TRR_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NANDCR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `TWB` reader - WE# high to busy time"]
pub type TWB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TWB` writer - WE# high to busy time"]
pub type TWB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NANDCR2_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - WE# high to RE# low time"]
    #[inline(always)]
    pub fn twhr(&self) -> TWHR_R {
        TWHR_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:11 - RE# high to WE# low time"]
    #[inline(always)]
    pub fn trhw(&self) -> TRHW_R {
        TRHW_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Address cycle to data loading time"]
    #[inline(always)]
    pub fn tadl(&self) -> TADL_R {
        TADL_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 18:23 - Ready to RE# low time"]
    #[inline(always)]
    pub fn trr(&self) -> TRR_R {
        TRR_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - WE# high to busy time"]
    #[inline(always)]
    pub fn twb(&self) -> TWB_R {
        TWB_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - WE# high to RE# low time"]
    #[inline(always)]
    #[must_use]
    pub fn twhr(&mut self) -> TWHR_W<0> {
        TWHR_W::new(self)
    }
    #[doc = "Bits 6:11 - RE# high to WE# low time"]
    #[inline(always)]
    #[must_use]
    pub fn trhw(&mut self) -> TRHW_W<6> {
        TRHW_W::new(self)
    }
    #[doc = "Bits 12:17 - Address cycle to data loading time"]
    #[inline(always)]
    #[must_use]
    pub fn tadl(&mut self) -> TADL_W<12> {
        TADL_W::new(self)
    }
    #[doc = "Bits 18:23 - Ready to RE# low time"]
    #[inline(always)]
    #[must_use]
    pub fn trr(&mut self) -> TRR_W<18> {
        TRR_W::new(self)
    }
    #[doc = "Bits 24:29 - WE# high to busy time"]
    #[inline(always)]
    #[must_use]
    pub fn twb(&mut self) -> TWB_W<24> {
        TWB_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nandcr2](index.html) module"]
pub struct NANDCR2_SPEC;
impl crate::RegisterSpec for NANDCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nandcr2::R](R) reader structure"]
impl crate::Readable for NANDCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nandcr2::W](W) writer structure"]
impl crate::Writable for NANDCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NANDCR2 to value 0x0001_0410"]
impl crate::Resettable for NANDCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0410;
}
