#[doc = "Register `MCFGR3` reader"]
pub struct R(crate::R<MCFGR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCFGR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCFGR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCFGR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCFGR3` writer"]
pub struct W(crate::W<MCFGR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCFGR3_SPEC>;
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
impl From<crate::W<MCFGR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCFGR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINLOW` reader - Pin Low Timeout"]
pub type PINLOW_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PINLOW` writer - Pin Low Timeout"]
pub type PINLOW_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCFGR3_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 8:19 - Pin Low Timeout"]
    #[inline(always)]
    pub fn pinlow(&self) -> PINLOW_R {
        PINLOW_R::new(((self.bits >> 8) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 8:19 - Pin Low Timeout"]
    #[inline(always)]
    #[must_use]
    pub fn pinlow(&mut self) -> PINLOW_W<8> {
        PINLOW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Configuration 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcfgr3](index.html) module"]
pub struct MCFGR3_SPEC;
impl crate::RegisterSpec for MCFGR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcfgr3::R](R) reader structure"]
impl crate::Readable for MCFGR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcfgr3::W](W) writer structure"]
impl crate::Writable for MCFGR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCFGR3 to value 0"]
impl crate::Resettable for MCFGR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
