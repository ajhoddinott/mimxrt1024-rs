#[doc = "Register `CM7_AHBPCR` reader"]
pub struct R(crate::R<CM7_AHBPCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_AHBPCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_AHBPCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_AHBPCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM7_AHBPCR` writer"]
pub struct W(crate::W<CM7_AHBPCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM7_AHBPCR_SPEC>;
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
impl From<crate::W<CM7_AHBPCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM7_AHBPCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - AHBP enable."]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "AHBP enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: AHBP disabled. When disabled all accesses are made to the AXIM interface."]
    EN_0 = 0,
    #[doc = "1: AHBP enabled."]
    EN_1 = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::EN_0,
            true => EN_A::EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `EN_0`"]
    #[inline(always)]
    pub fn is_en_0(&self) -> bool {
        *self == EN_A::EN_0
    }
    #[doc = "Checks if the value of the field is `EN_1`"]
    #[inline(always)]
    pub fn is_en_1(&self) -> bool {
        *self == EN_A::EN_1
    }
}
#[doc = "Field `EN` writer - AHBP enable."]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_AHBPCR_SPEC, EN_A, O>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "AHBP disabled. When disabled all accesses are made to the AXIM interface."]
    #[inline(always)]
    pub fn en_0(self) -> &'a mut W {
        self.variant(EN_A::EN_0)
    }
    #[doc = "AHBP enabled."]
    #[inline(always)]
    pub fn en_1(self) -> &'a mut W {
        self.variant(EN_A::EN_1)
    }
}
#[doc = "Field `SZ` reader - AHBP size."]
pub type SZ_R = crate::FieldReader<u8, SZ_A>;
#[doc = "AHBP size.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SZ_A {
    #[doc = "0: 0MB. AHBP disabled."]
    SZ_0 = 0,
    #[doc = "1: 64MB."]
    SZ_1 = 1,
    #[doc = "2: 128MB."]
    SZ_2 = 2,
    #[doc = "3: 256MB."]
    SZ_3 = 3,
    #[doc = "4: 512MB."]
    SZ_4 = 4,
}
impl From<SZ_A> for u8 {
    #[inline(always)]
    fn from(variant: SZ_A) -> Self {
        variant as _
    }
}
impl SZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SZ_A> {
        match self.bits {
            0 => Some(SZ_A::SZ_0),
            1 => Some(SZ_A::SZ_1),
            2 => Some(SZ_A::SZ_2),
            3 => Some(SZ_A::SZ_3),
            4 => Some(SZ_A::SZ_4),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `SZ_0`"]
    #[inline(always)]
    pub fn is_sz_0(&self) -> bool {
        *self == SZ_A::SZ_0
    }
    #[doc = "Checks if the value of the field is `SZ_1`"]
    #[inline(always)]
    pub fn is_sz_1(&self) -> bool {
        *self == SZ_A::SZ_1
    }
    #[doc = "Checks if the value of the field is `SZ_2`"]
    #[inline(always)]
    pub fn is_sz_2(&self) -> bool {
        *self == SZ_A::SZ_2
    }
    #[doc = "Checks if the value of the field is `SZ_3`"]
    #[inline(always)]
    pub fn is_sz_3(&self) -> bool {
        *self == SZ_A::SZ_3
    }
    #[doc = "Checks if the value of the field is `SZ_4`"]
    #[inline(always)]
    pub fn is_sz_4(&self) -> bool {
        *self == SZ_A::SZ_4
    }
}
impl R {
    #[doc = "Bit 0 - AHBP enable."]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - AHBP size."]
    #[inline(always)]
    pub fn sz(&self) -> SZ_R {
        SZ_R::new(((self.bits >> 1) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - AHBP enable."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHBP Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_ahbpcr](index.html) module"]
pub struct CM7_AHBPCR_SPEC;
impl crate::RegisterSpec for CM7_AHBPCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_ahbpcr::R](R) reader structure"]
impl crate::Readable for CM7_AHBPCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm7_ahbpcr::W](W) writer structure"]
impl crate::Writable for CM7_AHBPCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM7_AHBPCR to value 0"]
impl crate::Resettable for CM7_AHBPCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
