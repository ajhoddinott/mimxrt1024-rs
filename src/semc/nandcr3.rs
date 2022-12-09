#[doc = "Register `NANDCR3` reader"]
pub struct R(crate::R<NANDCR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NANDCR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NANDCR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NANDCR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NANDCR3` writer"]
pub struct W(crate::W<NANDCR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NANDCR3_SPEC>;
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
impl From<crate::W<NANDCR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NANDCR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NDOPT1` reader - NAND option bit 1"]
pub type NDOPT1_R = crate::BitReader<bool>;
#[doc = "Field `NDOPT1` writer - NAND option bit 1"]
pub type NDOPT1_W<'a, const O: u8> = crate::BitWriter<'a, u32, NANDCR3_SPEC, bool, O>;
#[doc = "Field `NDOPT2` reader - NAND option bit 2"]
pub type NDOPT2_R = crate::BitReader<bool>;
#[doc = "Field `NDOPT2` writer - NAND option bit 2"]
pub type NDOPT2_W<'a, const O: u8> = crate::BitWriter<'a, u32, NANDCR3_SPEC, bool, O>;
#[doc = "Field `NDOPT3` reader - NAND option bit 3"]
pub type NDOPT3_R = crate::BitReader<bool>;
#[doc = "Field `NDOPT3` writer - NAND option bit 3"]
pub type NDOPT3_W<'a, const O: u8> = crate::BitWriter<'a, u32, NANDCR3_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - NAND option bit 1"]
    #[inline(always)]
    pub fn ndopt1(&self) -> NDOPT1_R {
        NDOPT1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NAND option bit 2"]
    #[inline(always)]
    pub fn ndopt2(&self) -> NDOPT2_R {
        NDOPT2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NAND option bit 3"]
    #[inline(always)]
    pub fn ndopt3(&self) -> NDOPT3_R {
        NDOPT3_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NAND option bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn ndopt1(&mut self) -> NDOPT1_W<0> {
        NDOPT1_W::new(self)
    }
    #[doc = "Bit 1 - NAND option bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn ndopt2(&mut self) -> NDOPT2_W<1> {
        NDOPT2_W::new(self)
    }
    #[doc = "Bit 2 - NAND option bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn ndopt3(&mut self) -> NDOPT3_W<2> {
        NDOPT3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NAND Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [nandcr3](index.html) module"]
pub struct NANDCR3_SPEC;
impl crate::RegisterSpec for NANDCR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [nandcr3::R](R) reader structure"]
impl crate::Readable for NANDCR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [nandcr3::W](W) writer structure"]
impl crate::Writable for NANDCR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NANDCR3 to value 0"]
impl crate::Resettable for NANDCR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
