#[doc = "Register `NORCR1` reader"]
pub struct R(crate::R<NORCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<NORCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<NORCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<NORCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `NORCR1` writer"]
pub struct W(crate::W<NORCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<NORCR1_SPEC>;
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
impl From<crate::W<NORCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<NORCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CES` reader - CE setup time"]
pub type CES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CES` writer - CE setup time"]
pub type CES_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NORCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `CEH` reader - CE hold time"]
pub type CEH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CEH` writer - CE hold time"]
pub type CEH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NORCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `AS` reader - Address setup time"]
pub type AS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AS` writer - Address setup time"]
pub type AS_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NORCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `AH` reader - Address hold time"]
pub type AH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AH` writer - Address hold time"]
pub type AH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NORCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `WEL` reader - WE low time"]
pub type WEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WEL` writer - WE low time"]
pub type WEL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NORCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `WEH` reader - WE high time"]
pub type WEH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WEH` writer - WE high time"]
pub type WEH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NORCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `REL` reader - RE low time"]
pub type REL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REL` writer - RE low time"]
pub type REL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NORCR1_SPEC, u8, u8, 4, O>;
#[doc = "Field `REH` reader - RE high time"]
pub type REH_R = crate::FieldReader<u8, u8>;
#[doc = "Field `REH` writer - RE high time"]
pub type REH_W<'a, const O: u8> = crate::FieldWriter<'a, u32, NORCR1_SPEC, u8, u8, 4, O>;
impl R {
    #[doc = "Bits 0:3 - CE setup time"]
    #[inline(always)]
    pub fn ces(&self) -> CES_R {
        CES_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - CE hold time"]
    #[inline(always)]
    pub fn ceh(&self) -> CEH_R {
        CEH_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Address setup time"]
    #[inline(always)]
    pub fn as_(&self) -> AS_R {
        AS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Address hold time"]
    #[inline(always)]
    pub fn ah(&self) -> AH_R {
        AH_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - WE low time"]
    #[inline(always)]
    pub fn wel(&self) -> WEL_R {
        WEL_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - WE high time"]
    #[inline(always)]
    pub fn weh(&self) -> WEH_R {
        WEH_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - RE low time"]
    #[inline(always)]
    pub fn rel(&self) -> REL_R {
        REL_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - RE high time"]
    #[inline(always)]
    pub fn reh(&self) -> REH_R {
        REH_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - CE setup time"]
    #[inline(always)]
    #[must_use]
    pub fn ces(&mut self) -> CES_W<0> {
        CES_W::new(self)
    }
    #[doc = "Bits 4:7 - CE hold time"]
    #[inline(always)]
    #[must_use]
    pub fn ceh(&mut self) -> CEH_W<4> {
        CEH_W::new(self)
    }
    #[doc = "Bits 8:11 - Address setup time"]
    #[inline(always)]
    #[must_use]
    pub fn as_(&mut self) -> AS_W<8> {
        AS_W::new(self)
    }
    #[doc = "Bits 12:15 - Address hold time"]
    #[inline(always)]
    #[must_use]
    pub fn ah(&mut self) -> AH_W<12> {
        AH_W::new(self)
    }
    #[doc = "Bits 16:19 - WE low time"]
    #[inline(always)]
    #[must_use]
    pub fn wel(&mut self) -> WEL_W<16> {
        WEL_W::new(self)
    }
    #[doc = "Bits 20:23 - WE high time"]
    #[inline(always)]
    #[must_use]
    pub fn weh(&mut self) -> WEH_W<20> {
        WEH_W::new(self)
    }
    #[doc = "Bits 24:27 - RE low time"]
    #[inline(always)]
    #[must_use]
    pub fn rel(&mut self) -> REL_W<24> {
        REL_W::new(self)
    }
    #[doc = "Bits 28:31 - RE high time"]
    #[inline(always)]
    #[must_use]
    pub fn reh(&mut self) -> REH_W<28> {
        REH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "NOR Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [norcr1](index.html) module"]
pub struct NORCR1_SPEC;
impl crate::RegisterSpec for NORCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [norcr1::R](R) reader structure"]
impl crate::Readable for NORCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [norcr1::W](W) writer structure"]
impl crate::Writable for NORCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets NORCR1 to value 0"]
impl crate::Resettable for NORCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
