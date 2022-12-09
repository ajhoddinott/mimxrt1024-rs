#[doc = "Register `DBICR1` reader"]
pub struct R(crate::R<DBICR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DBICR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DBICR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DBICR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DBICR1` writer"]
pub struct W(crate::W<DBICR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DBICR1_SPEC>;
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
impl From<crate::W<DBICR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DBICR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CES` reader - CSX Setup Time"]
pub type CES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CES` writer - CSX Setup Time"]
pub type CES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBICR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CEH` reader - CSX Hold Time"]
pub type CEH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CEH` writer - CSX Hold Time"]
pub type CEH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBICR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `WEL` reader - WRX Low Time"]
pub type WEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WEL` writer - WRX Low Time"]
pub type WEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBICR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `WEH` reader - WRX High Time"]
pub type WEH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WEH` writer - WRX High Time"]
pub type WEH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBICR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `REL` reader - RDX Low Time bit \\[3:0\\]"]
pub type REL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REL` writer - RDX Low Time bit \\[3:0\\]"]
pub type REL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBICR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `REH` reader - RDX High Time bit \\[3:0\\]"]
pub type REH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REH` writer - RDX High Time bit \\[3:0\\]"]
pub type REH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBICR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CEITV` reader - CSX interval time"]
pub type CEITV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CEITV` writer - CSX interval time"]
pub type CEITV_W<'a, const O: u8> = crate::FieldWriter<'a, u32, DBICR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - CSX Setup Time"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CSX Hold Time"]
    #[inline(always)]
    pub fn ceh(&self) -> CEH_R {
        CEH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - WRX Low Time"]
    #[inline(always)]
    pub fn wel(&self) -> WEL_R {
        WEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - WRX High Time"]
    #[inline(always)]
    pub fn weh(&self) -> WEH_R {
        WEH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - RDX Low Time bit \\[3:0\\]"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - RDX High Time bit \\[3:0\\]"]
    #[inline(always)]
    pub fn reh(&self) -> REH_R {
        REH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - CSX interval time"]
    #[inline(always)]
    pub fn ceitv(&self) -> CEITV_R {
        CEITV_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CSX Setup Time"]
    #[inline(always)]
    #[must_use]
    pub fn ces(&mut self) -> CES_W<0> {
        CES_W::new(self)
    }
    #[doc = "Bits 4:7 - CSX Hold Time"]
    #[inline(always)]
    #[must_use]
    pub fn ceh(&mut self) -> CEH_W<4> {
        CEH_W::new(self)
    }
    #[doc = "Bits 8:11 - WRX Low Time"]
    #[inline(always)]
    #[must_use]
    pub fn wel(&mut self) -> WEL_W<8> {
        WEL_W::new(self)
    }
    #[doc = "Bits 12:15 - WRX High Time"]
    #[inline(always)]
    #[must_use]
    pub fn weh(&mut self) -> WEH_W<12> {
        WEH_W::new(self)
    }
    #[doc = "Bits 16:19 - RDX Low Time bit \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn rel(&mut self) -> REL_W<16> {
        REL_W::new(self)
    }
    #[doc = "Bits 20:23 - RDX High Time bit \\[3:0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn reh(&mut self) -> REH_W<20> {
        REH_W::new(self)
    }
    #[doc = "Bits 24:27 - CSX interval time"]
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
#[doc = "DBI-B Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dbicr1](index.html) module"]
pub struct DBICR1_SPEC;
impl crate::RegisterSpec for DBICR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dbicr1::R](R) reader structure"]
impl crate::Readable for DBICR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dbicr1::W](W) writer structure"]
impl crate::Writable for DBICR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBICR1 to value 0"]
impl crate::Resettable for DBICR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
