#[doc = "Register `CM7_AHBSCR` reader"]
pub struct R(crate::R<CM7_AHBSCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_AHBSCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_AHBSCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_AHBSCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM7_AHBSCR` writer"]
pub struct W(crate::W<CM7_AHBSCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM7_AHBSCR_SPEC>;
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
impl From<crate::W<CM7_AHBSCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM7_AHBSCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CTL` reader - AHBS prioritization control."]
pub type CTL_R = crate::FieldReader<u8, CTL_A>;
#[doc = "AHBS prioritization control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTL_A {
    #[doc = "0: AHBS access priority demoted. This is the reset value."]
    CTL_0 = 0,
    #[doc = "1: Software access priority demoted."]
    CTL_1 = 1,
    #[doc = "2: AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR\\[INITCOUNT\\]
value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR\\[TPRI\\]."]
    CTL_2 = 2,
    #[doc = "3: AHBSPRI signal has control of access priority."]
    CTL_3 = 3,
}
impl From<CTL_A> for u8 {
    #[inline(always)]
    fn from(variant: CTL_A) -> Self {
        variant as _
    }
}
impl CTL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTL_A {
        match self.bits {
            0 => CTL_A::CTL_0,
            1 => CTL_A::CTL_1,
            2 => CTL_A::CTL_2,
            3 => CTL_A::CTL_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CTL_0`"]
    #[inline(always)]
    pub fn is_ctl_0(&self) -> bool {
        *self == CTL_A::CTL_0
    }
    #[doc = "Checks if the value of the field is `CTL_1`"]
    #[inline(always)]
    pub fn is_ctl_1(&self) -> bool {
        *self == CTL_A::CTL_1
    }
    #[doc = "Checks if the value of the field is `CTL_2`"]
    #[inline(always)]
    pub fn is_ctl_2(&self) -> bool {
        *self == CTL_A::CTL_2
    }
    #[doc = "Checks if the value of the field is `CTL_3`"]
    #[inline(always)]
    pub fn is_ctl_3(&self) -> bool {
        *self == CTL_A::CTL_3
    }
}
#[doc = "Field `CTL` writer - AHBS prioritization control."]
pub type CTL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, CM7_AHBSCR_SPEC, u8, CTL_A, 2, O>;
impl<'a, const O: u8> CTL_W<'a, O> {
    #[doc = "AHBS access priority demoted. This is the reset value."]
    #[inline(always)]
    pub fn ctl_0(self) -> &'a mut W {
        self.variant(CTL_A::CTL_0)
    }
    #[doc = "Software access priority demoted."]
    #[inline(always)]
    pub fn ctl_1(self) -> &'a mut W {
        self.variant(CTL_A::CTL_1)
    }
    #[doc = "AHBS access priority demoted by initializing the fairness counter to the CM7_AHBSCR\\[INITCOUNT\\]
value when the software execution priority is higher than or equal to the threshold level programed in CM7_AHBSCR\\[TPRI\\]."]
    #[inline(always)]
    pub fn ctl_2(self) -> &'a mut W {
        self.variant(CTL_A::CTL_2)
    }
    #[doc = "AHBSPRI signal has control of access priority."]
    #[inline(always)]
    pub fn ctl_3(self) -> &'a mut W {
        self.variant(CTL_A::CTL_3)
    }
}
#[doc = "Field `TPRI` reader - Threshold execution priority for AHBS traffic demotion."]
pub type TPRI_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TPRI` writer - Threshold execution priority for AHBS traffic demotion."]
pub type TPRI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM7_AHBSCR_SPEC, u16, u16, 9, O>;
#[doc = "Field `INITCOUNT` reader - Fairness counter initialization value."]
pub type INITCOUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `INITCOUNT` writer - Fairness counter initialization value."]
pub type INITCOUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, CM7_AHBSCR_SPEC, u8, u8, 5, O>;
impl R {
    #[doc = "Bits 0:1 - AHBS prioritization control."]
    #[inline(always)]
    pub fn ctl(&self) -> CTL_R {
        CTL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:10 - Threshold execution priority for AHBS traffic demotion."]
    #[inline(always)]
    pub fn tpri(&self) -> TPRI_R {
        TPRI_R::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bits 11:15 - Fairness counter initialization value."]
    #[inline(always)]
    pub fn initcount(&self) -> INITCOUNT_R {
        INITCOUNT_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - AHBS prioritization control."]
    #[inline(always)]
    #[must_use]
    pub fn ctl(&mut self) -> CTL_W<0> {
        CTL_W::new(self)
    }
    #[doc = "Bits 2:10 - Threshold execution priority for AHBS traffic demotion."]
    #[inline(always)]
    #[must_use]
    pub fn tpri(&mut self) -> TPRI_W<2> {
        TPRI_W::new(self)
    }
    #[doc = "Bits 11:15 - Fairness counter initialization value."]
    #[inline(always)]
    #[must_use]
    pub fn initcount(&mut self) -> INITCOUNT_W<11> {
        INITCOUNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB Slave Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_ahbscr](index.html) module"]
pub struct CM7_AHBSCR_SPEC;
impl crate::RegisterSpec for CM7_AHBSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_ahbscr::R](R) reader structure"]
impl crate::Readable for CM7_AHBSCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm7_ahbscr::W](W) writer structure"]
impl crate::Writable for CM7_AHBSCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM7_AHBSCR to value 0"]
impl crate::Resettable for CM7_AHBSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
