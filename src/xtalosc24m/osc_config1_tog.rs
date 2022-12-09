#[doc = "Register `OSC_CONFIG1_TOG` reader"]
pub struct R(crate::R<OSC_CONFIG1_TOG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OSC_CONFIG1_TOG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OSC_CONFIG1_TOG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OSC_CONFIG1_TOG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `OSC_CONFIG1_TOG` writer"]
pub struct W(crate::W<OSC_CONFIG1_TOG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSC_CONFIG1_TOG_SPEC>;
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
impl From<crate::W<OSC_CONFIG1_TOG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSC_CONFIG1_TOG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COUNT_RC_TRG` reader - The target count used to tune the RC OSC frequency"]
pub type COUNT_RC_TRG_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT_RC_TRG` writer - The target count used to tune the RC OSC frequency"]
pub type COUNT_RC_TRG_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONFIG1_TOG_SPEC, u16, u16, 12, O>;
#[doc = "Field `COUNT_RC_CUR` reader - The current tuning value in use."]
pub type COUNT_RC_CUR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COUNT_RC_CUR` writer - The current tuning value in use."]
pub type COUNT_RC_CUR_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, OSC_CONFIG1_TOG_SPEC, u16, u16, 12, O>;
impl R {
    #[doc = "Bits 0:11 - The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    pub fn count_rc_trg(&self) -> COUNT_RC_TRG_R {
        COUNT_RC_TRG_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:31 - The current tuning value in use."]
    #[inline(always)]
    pub fn count_rc_cur(&self) -> COUNT_RC_CUR_R {
        COUNT_RC_CUR_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - The target count used to tune the RC OSC frequency"]
    #[inline(always)]
    #[must_use]
    pub fn count_rc_trg(&mut self) -> COUNT_RC_TRG_W<0> {
        COUNT_RC_TRG_W::new(self)
    }
    #[doc = "Bits 20:31 - The current tuning value in use."]
    #[inline(always)]
    #[must_use]
    pub fn count_rc_cur(&mut self) -> COUNT_RC_CUR_W<20> {
        COUNT_RC_CUR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XTAL OSC Configuration 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [osc_config1_tog](index.html) module"]
pub struct OSC_CONFIG1_TOG_SPEC;
impl crate::RegisterSpec for OSC_CONFIG1_TOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [osc_config1_tog::R](R) reader structure"]
impl crate::Readable for OSC_CONFIG1_TOG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [osc_config1_tog::W](W) writer structure"]
impl crate::Writable for OSC_CONFIG1_TOG_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets OSC_CONFIG1_TOG to value 0x02ee"]
impl crate::Resettable for OSC_CONFIG1_TOG_SPEC {
    const RESET_VALUE: Self::Ux = 0x02ee;
}
