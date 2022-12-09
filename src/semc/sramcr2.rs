#[doc = "Register `SRAMCR2` reader"]
pub struct R(crate::R<SRAMCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SRAMCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SRAMCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SRAMCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SRAMCR2` writer"]
pub struct W(crate::W<SRAMCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SRAMCR2_SPEC>;
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
impl From<crate::W<SRAMCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SRAMCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TA` reader - Turnaround time"]
pub type TA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TA` writer - Turnaround time"]
pub type TA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAMCR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWDH` reader - Address to write data hold time"]
pub type AWDH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWDH` writer - Address to write data hold time"]
pub type AWDH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAMCR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `CEITV` reader - CE# interval time"]
pub type CEITV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CEITV` writer - CE# interval time"]
pub type CEITV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SRAMCR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 8:11 - Turnaround time"]
    #[inline(always)]
    pub fn ta(&self) -> TA_R {
        TA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Address to write data hold time"]
    #[inline(always)]
    pub fn awdh(&self) -> AWDH_R {
        AWDH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CE# interval time"]
    #[inline(always)]
    pub fn ceitv(&self) -> CEITV_R {
        CEITV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 8:11 - Turnaround time"]
    #[inline(always)]
    #[must_use]
    pub fn ta(&mut self) -> TA_W<8> {
        TA_W::new(self)
    }
    #[doc = "Bits 12:15 - Address to write data hold time"]
    #[inline(always)]
    #[must_use]
    pub fn awdh(&mut self) -> AWDH_W<12> {
        AWDH_W::new(self)
    }
    #[doc = "Bits 24:27 - CE# interval time"]
    #[inline(always)]
    #[must_use]
    pub fn ceitv(&mut self) -> CEITV_W<24> {
        CEITV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SRAM Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sramcr2](index.html) module"]
pub struct SRAMCR2_SPEC;
impl crate::RegisterSpec for SRAMCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sramcr2::R](R) reader structure"]
impl crate::Readable for SRAMCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sramcr2::W](W) writer structure"]
impl crate::Writable for SRAMCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SRAMCR2 to value 0"]
impl crate::Resettable for SRAMCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
