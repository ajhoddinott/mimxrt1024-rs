#[doc = "Register `HPHACIVR` reader"]
pub struct R(crate::R<HPHACIVR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HPHACIVR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HPHACIVR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HPHACIVR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HPHACIVR` writer"]
pub struct W(crate::W<HPHACIVR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HPHACIVR_SPEC>;
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
impl From<crate::W<HPHACIVR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HPHACIVR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HAC_COUNTER_IV` reader - High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter"]
pub type HAC_COUNTER_IV_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HAC_COUNTER_IV` writer - High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter"]
pub type HAC_COUNTER_IV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, HPHACIVR_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter"]
    #[inline(always)]
    pub fn hac_counter_iv(&self) -> HAC_COUNTER_IV_R {
        HAC_COUNTER_IV_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - High Assurance Counter Initial Value This register is used to set the starting count value to the high assurance counter"]
    #[inline(always)]
    #[must_use]
    pub fn hac_counter_iv(&mut self) -> HAC_COUNTER_IV_W<0> {
        HAC_COUNTER_IV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SNVS_HP High Assurance Counter IV Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hphacivr](index.html) module"]
pub struct HPHACIVR_SPEC;
impl crate::RegisterSpec for HPHACIVR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hphacivr::R](R) reader structure"]
impl crate::Readable for HPHACIVR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hphacivr::W](W) writer structure"]
impl crate::Writable for HPHACIVR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HPHACIVR to value 0"]
impl crate::Resettable for HPHACIVR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
