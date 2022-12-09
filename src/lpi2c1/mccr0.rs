#[doc = "Register `MCCR0` reader"]
pub struct R(crate::R<MCCR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCCR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCCR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCCR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCCR0` writer"]
pub struct W(crate::W<MCCR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCCR0_SPEC>;
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
impl From<crate::W<MCCR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCCR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKLO` reader - Clock Low Period"]
pub type CLKLO_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKLO` writer - Clock Low Period"]
pub type CLKLO_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCCR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `CLKHI` reader - Clock High Period"]
pub type CLKHI_R = crate::FieldReader<u8, u8>;
#[doc = "Field `CLKHI` writer - Clock High Period"]
pub type CLKHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCCR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `SETHOLD` reader - Setup Hold Delay"]
pub type SETHOLD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SETHOLD` writer - Setup Hold Delay"]
pub type SETHOLD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCCR0_SPEC, u8, u8, 6, O>;
#[doc = "Field `DATAVD` reader - Data Valid Delay"]
pub type DATAVD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DATAVD` writer - Data Valid Delay"]
pub type DATAVD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCCR0_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Clock Low Period"]
    #[inline(always)]
    pub fn clklo(&self) -> CLKLO_R {
        CLKLO_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Clock High Period"]
    #[inline(always)]
    pub fn clkhi(&self) -> CLKHI_R {
        CLKHI_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Setup Hold Delay"]
    #[inline(always)]
    pub fn sethold(&self) -> SETHOLD_R {
        SETHOLD_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Data Valid Delay"]
    #[inline(always)]
    pub fn datavd(&self) -> DATAVD_R {
        DATAVD_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clock Low Period"]
    #[inline(always)]
    #[must_use]
    pub fn clklo(&mut self) -> CLKLO_W<0> {
        CLKLO_W::new(self)
    }
    #[doc = "Bits 8:13 - Clock High Period"]
    #[inline(always)]
    #[must_use]
    pub fn clkhi(&mut self) -> CLKHI_W<8> {
        CLKHI_W::new(self)
    }
    #[doc = "Bits 16:21 - Setup Hold Delay"]
    #[inline(always)]
    #[must_use]
    pub fn sethold(&mut self) -> SETHOLD_W<16> {
        SETHOLD_W::new(self)
    }
    #[doc = "Bits 24:29 - Data Valid Delay"]
    #[inline(always)]
    #[must_use]
    pub fn datavd(&mut self) -> DATAVD_W<24> {
        DATAVD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Clock Configuration 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mccr0](index.html) module"]
pub struct MCCR0_SPEC;
impl crate::RegisterSpec for MCCR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mccr0::R](R) reader structure"]
impl crate::Readable for MCCR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mccr0::W](W) writer structure"]
impl crate::Writable for MCCR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCCR0 to value 0"]
impl crate::Resettable for MCCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
