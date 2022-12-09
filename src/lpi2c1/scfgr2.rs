#[doc = "Register `SCFGR2` reader"]
pub struct R(crate::R<SCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SCFGR2` writer"]
pub struct W(crate::W<SCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SCFGR2_SPEC>;
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
impl From<crate::W<SCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKHOLD` reader - Clock Hold Time"]
pub type CLKHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKHOLD` writer - Clock Hold Time"]
pub type CLKHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFGR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `DATAVD` reader - Data Valid Delay"]
pub type DATAVD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAVD` writer - Data Valid Delay"]
pub type DATAVD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFGR2_SPEC, u8, u8, 6, O>;
#[doc = "Field `FILTSCL` reader - Glitch Filter SCL"]
pub type FILTSCL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTSCL` writer - Glitch Filter SCL"]
pub type FILTSCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFGR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `FILTSDA` reader - Glitch Filter SDA"]
pub type FILTSDA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTSDA` writer - Glitch Filter SDA"]
pub type FILTSDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SCFGR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Clock Hold Time"]
    #[inline(always)]
    pub fn clkhold(&self) -> CLKHOLD_R {
        CLKHOLD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&self) -> DATAVD_R {
        DATAVD_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:19 - Glitch Filter SCL"]
    #[inline(always)]
    pub fn filtscl(&self) -> FILTSCL_R {
        FILTSCL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Glitch Filter SDA"]
    #[inline(always)]
    pub fn filtsda(&self) -> FILTSDA_R {
        FILTSDA_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Clock Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn clkhold(&mut self) -> CLKHOLD_W<0> {
        CLKHOLD_W::new(self)
    }
    #[doc = "Bits 8:13 - Data Valid Delay"]
    #[inline(always)]
    #[must_use]
    pub fn datavd(&mut self) -> DATAVD_W<8> {
        DATAVD_W::new(self)
    }
    #[doc = "Bits 16:19 - Glitch Filter SCL"]
    #[inline(always)]
    #[must_use]
    pub fn filtscl(&mut self) -> FILTSCL_W<16> {
        FILTSCL_W::new(self)
    }
    #[doc = "Bits 24:27 - Glitch Filter SDA"]
    #[inline(always)]
    #[must_use]
    pub fn filtsda(&mut self) -> FILTSDA_W<24> {
        FILTSDA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [scfgr2](index.html) module"]
pub struct SCFGR2_SPEC;
impl crate::RegisterSpec for SCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [scfgr2::R](R) reader structure"]
impl crate::Readable for SCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [scfgr2::W](W) writer structure"]
impl crate::Writable for SCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SCFGR2 to value 0"]
impl crate::Resettable for SCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
