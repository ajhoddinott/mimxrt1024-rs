#[doc = "Register `MCFGR2` reader"]
pub struct R(crate::R<MCFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFGR2` writer"]
pub struct W(crate::W<MCFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFGR2_SPEC>;
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
impl From<crate::W<MCFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUSIDLE` reader - Bus Idle Timeout"]
pub type BUSIDLE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `BUSIDLE` writer - Bus Idle Timeout"]
pub type BUSIDLE_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCFGR2_SPEC, u16, u16, 12, O>;
#[doc = "Field `FILTSCL` reader - Glitch Filter SCL"]
pub type FILTSCL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTSCL` writer - Glitch Filter SCL"]
pub type FILTSCL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCFGR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `FILTSDA` reader - Glitch Filter SDA"]
pub type FILTSDA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FILTSDA` writer - Glitch Filter SDA"]
pub type FILTSDA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCFGR2_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:11 - Bus Idle Timeout"]
    #[inline(always)]
    pub fn busidle(&self) -> BUSIDLE_R {
        BUSIDLE_R::new((self.bits & 0x0fff) as u16)
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
    #[doc = "Bits 0:11 - Bus Idle Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn busidle(&mut self) -> BUSIDLE_W<0> {
        BUSIDLE_W::new(self)
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
#[doc = "Master Configuration 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr2](index.html) module"]
pub struct MCFGR2_SPEC;
impl crate::RegisterSpec for MCFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfgr2::R](R) reader structure"]
impl crate::Readable for MCFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfgr2::W](W) writer structure"]
impl crate::Writable for MCFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCFGR2 to value 0"]
impl crate::Resettable for MCFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
