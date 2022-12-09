#[doc = "Register `CM7_ABFSR` reader"]
pub struct R(crate::R<CM7_ABFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CM7_ABFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CM7_ABFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CM7_ABFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CM7_ABFSR` writer"]
pub struct W(crate::W<CM7_ABFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CM7_ABFSR_SPEC>;
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
impl From<crate::W<CM7_ABFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CM7_ABFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ITCM` reader - Asynchronous fault on ITCM interface."]
pub type ITCM_R = crate::BitReader<bool>;
#[doc = "Field `ITCM` writer - Asynchronous fault on ITCM interface."]
pub type ITCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_ABFSR_SPEC, bool, O>;
#[doc = "Field `DTCM` reader - Asynchronous fault on DTCM interface."]
pub type DTCM_R = crate::BitReader<bool>;
#[doc = "Field `DTCM` writer - Asynchronous fault on DTCM interface."]
pub type DTCM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_ABFSR_SPEC, bool, O>;
#[doc = "Field `AHBP` reader - Asynchronous fault on AHBP interface."]
pub type AHBP_R = crate::BitReader<bool>;
#[doc = "Field `AHBP` writer - Asynchronous fault on AHBP interface."]
pub type AHBP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_ABFSR_SPEC, bool, O>;
#[doc = "Field `AXIM` reader - Asynchronous fault on AXIM interface."]
pub type AXIM_R = crate::BitReader<bool>;
#[doc = "Field `AXIM` writer - Asynchronous fault on AXIM interface."]
pub type AXIM_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_ABFSR_SPEC, bool, O>;
#[doc = "Field `EPPB` reader - Asynchronous fault on EPPB interface."]
pub type EPPB_R = crate::BitReader<bool>;
#[doc = "Field `EPPB` writer - Asynchronous fault on EPPB interface."]
pub type EPPB_W<'a, const O: u8> = crate::BitWriter<'a, u32, CM7_ABFSR_SPEC, bool, O>;
#[doc = "Field `AXIMTYPE` reader - Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
pub type AXIMTYPE_R = crate::FieldReader<u8, AXIMTYPE_A>;
#[doc = "Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AXIMTYPE_A {
    #[doc = "0: OKAY."]
    AXIMTYPE_0 = 0,
    #[doc = "1: EXOKAY."]
    AXIMTYPE_1 = 1,
    #[doc = "2: SLVERR."]
    AXIMTYPE_2 = 2,
    #[doc = "3: DECERR."]
    AXIMTYPE_3 = 3,
}
impl From<AXIMTYPE_A> for u8 {
    #[inline(always)]
    fn from(variant: AXIMTYPE_A) -> Self {
        variant as _
    }
}
impl AXIMTYPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIMTYPE_A {
        match self.bits {
            0 => AXIMTYPE_A::AXIMTYPE_0,
            1 => AXIMTYPE_A::AXIMTYPE_1,
            2 => AXIMTYPE_A::AXIMTYPE_2,
            3 => AXIMTYPE_A::AXIMTYPE_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_0`"]
    #[inline(always)]
    pub fn is_aximtype_0(&self) -> bool {
        *self == AXIMTYPE_A::AXIMTYPE_0
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_1`"]
    #[inline(always)]
    pub fn is_aximtype_1(&self) -> bool {
        *self == AXIMTYPE_A::AXIMTYPE_1
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_2`"]
    #[inline(always)]
    pub fn is_aximtype_2(&self) -> bool {
        *self == AXIMTYPE_A::AXIMTYPE_2
    }
    #[doc = "Checks if the value of the field is `AXIMTYPE_3`"]
    #[inline(always)]
    pub fn is_aximtype_3(&self) -> bool {
        *self == AXIMTYPE_A::AXIMTYPE_3
    }
}
#[doc = "Field `AXIMTYPE` writer - Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
pub type AXIMTYPE_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CM7_ABFSR_SPEC, u8, AXIMTYPE_A, 2, O>;
impl<'a, const O: u8> AXIMTYPE_W<'a, O> {
    #[doc = "OKAY."]
    #[inline(always)]
    pub fn aximtype_0(self) -> &'a mut W {
        self.variant(AXIMTYPE_A::AXIMTYPE_0)
    }
    #[doc = "EXOKAY."]
    #[inline(always)]
    pub fn aximtype_1(self) -> &'a mut W {
        self.variant(AXIMTYPE_A::AXIMTYPE_1)
    }
    #[doc = "SLVERR."]
    #[inline(always)]
    pub fn aximtype_2(self) -> &'a mut W {
        self.variant(AXIMTYPE_A::AXIMTYPE_2)
    }
    #[doc = "DECERR."]
    #[inline(always)]
    pub fn aximtype_3(self) -> &'a mut W {
        self.variant(AXIMTYPE_A::AXIMTYPE_3)
    }
}
impl R {
    #[doc = "Bit 0 - Asynchronous fault on ITCM interface."]
    #[inline(always)]
    pub fn itcm(&self) -> ITCM_R {
        ITCM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Asynchronous fault on DTCM interface."]
    #[inline(always)]
    pub fn dtcm(&self) -> DTCM_R {
        DTCM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Asynchronous fault on AHBP interface."]
    #[inline(always)]
    pub fn ahbp(&self) -> AHBP_R {
        AHBP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Asynchronous fault on AXIM interface."]
    #[inline(always)]
    pub fn axim(&self) -> AXIM_R {
        AXIM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Asynchronous fault on EPPB interface."]
    #[inline(always)]
    pub fn eppb(&self) -> EPPB_R {
        EPPB_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
    #[inline(always)]
    pub fn aximtype(&self) -> AXIMTYPE_R {
        AXIMTYPE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Asynchronous fault on ITCM interface."]
    #[inline(always)]
    #[must_use]
    pub fn itcm(&mut self) -> ITCM_W<0> {
        ITCM_W::new(self)
    }
    #[doc = "Bit 1 - Asynchronous fault on DTCM interface."]
    #[inline(always)]
    #[must_use]
    pub fn dtcm(&mut self) -> DTCM_W<1> {
        DTCM_W::new(self)
    }
    #[doc = "Bit 2 - Asynchronous fault on AHBP interface."]
    #[inline(always)]
    #[must_use]
    pub fn ahbp(&mut self) -> AHBP_W<2> {
        AHBP_W::new(self)
    }
    #[doc = "Bit 3 - Asynchronous fault on AXIM interface."]
    #[inline(always)]
    #[must_use]
    pub fn axim(&mut self) -> AXIM_W<3> {
        AXIM_W::new(self)
    }
    #[doc = "Bit 4 - Asynchronous fault on EPPB interface."]
    #[inline(always)]
    #[must_use]
    pub fn eppb(&mut self) -> EPPB_W<4> {
        EPPB_W::new(self)
    }
    #[doc = "Bits 8:9 - Indicates the type of fault on the AXIM interface. Only valid when AXIM is 1."]
    #[inline(always)]
    #[must_use]
    pub fn aximtype(&mut self) -> AXIMTYPE_W<8> {
        AXIMTYPE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Bus Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cm7_abfsr](index.html) module"]
pub struct CM7_ABFSR_SPEC;
impl crate::RegisterSpec for CM7_ABFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cm7_abfsr::R](R) reader structure"]
impl crate::Readable for CM7_ABFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cm7_abfsr::W](W) writer structure"]
impl crate::Writable for CM7_ABFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CM7_ABFSR to value 0"]
impl crate::Resettable for CM7_ABFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
