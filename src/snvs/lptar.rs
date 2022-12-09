#[doc = "Register `LPTAR` reader"]
pub struct R(crate::R<LPTAR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LPTAR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LPTAR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LPTAR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LPTAR` writer"]
pub struct W(crate::W<LPTAR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LPTAR_SPEC>;
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
impl From<crate::W<LPTAR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LPTAR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LPTA` reader - LP Time Alarm This register can be programmed only when the LP time alarm is disabled (LPTA_EN bit is not set)"]
pub type LPTA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LPTA` writer - LP Time Alarm This register can be programmed only when the LP time alarm is disabled (LPTA_EN bit is not set)"]
pub type LPTA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LPTAR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - LP Time Alarm This register can be programmed only when the LP time alarm is disabled (LPTA_EN bit is not set)"]
    #[inline(always)]
    pub fn lpta(&self) -> LPTA_R {
        LPTA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - LP Time Alarm This register can be programmed only when the LP time alarm is disabled (LPTA_EN bit is not set)"]
    #[inline(always)]
    #[must_use]
    pub fn lpta(&mut self) -> LPTA_W<0> {
        LPTA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_LP Time Alarm Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lptar](index.html) module"]
pub struct LPTAR_SPEC;
impl crate::RegisterSpec for LPTAR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lptar::R](R) reader structure"]
impl crate::Readable for LPTAR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lptar::W](W) writer structure"]
impl crate::Writable for LPTAR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LPTAR to value 0"]
impl crate::Resettable for LPTAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
