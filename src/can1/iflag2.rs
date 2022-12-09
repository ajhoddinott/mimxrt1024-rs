#[doc = "Register `IFLAG2` reader"]
pub struct R(crate::R<IFLAG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLAG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLAG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLAG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLAG2` writer"]
pub struct W(crate::W<IFLAG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLAG2_SPEC>;
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
impl From<crate::W<IFLAG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLAG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUFHI` reader - Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt."]
pub type BUFHI_R = crate::FieldReader<u32, BUFHI_A>;
#[doc = "Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BUFHI_A {
    #[doc = "0: No such occurrence"]
    BUFHI_0 = 0,
    #[doc = "1: The corresponding buffer has successfully completed transmission or reception"]
    BUFHI_1 = 1,
}
impl From<BUFHI_A> for u32 {
    #[inline(always)]
    fn from(variant: BUFHI_A) -> Self {
        variant as _
    }
}
impl BUFHI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUFHI_A> {
        match self.bits {
            0 => Some(BUFHI_A::BUFHI_0),
            1 => Some(BUFHI_A::BUFHI_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUFHI_0`"]
    #[inline(always)]
    pub fn is_bufhi_0(&self) -> bool {
        *self == BUFHI_A::BUFHI_0
    }
    #[doc = "Checks if the value of the field is `BUFHI_1`"]
    #[inline(always)]
    pub fn is_bufhi_1(&self) -> bool {
        *self == BUFHI_A::BUFHI_1
    }
}
#[doc = "Field `BUFHI` writer - Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt."]
pub type BUFHI_W<'a, const O: u8> = crate::FieldWriter<'a, u32, IFLAG2_SPEC, u32, BUFHI_A, 32, O>;
impl<'a, const O: u8> BUFHI_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn bufhi_0(self) -> &'a mut W {
        self.variant(BUFHI_A::BUFHI_0)
    }
    #[doc = "The corresponding buffer has successfully completed transmission or reception"]
    #[inline(always)]
    pub fn bufhi_1(self) -> &'a mut W {
        self.variant(BUFHI_A::BUFHI_1)
    }
}
impl R {
    #[doc = "Bits 0:31 - Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt."]
    #[inline(always)]
    pub fn bufhi(&self) -> BUFHI_R {
        BUFHI_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Each bit flags the respective FLEXCAN Message Buffer (MB32 to MB63) interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn bufhi(&mut self) -> BUFHI_W<0> {
        BUFHI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iflag2](index.html) module"]
pub struct IFLAG2_SPEC;
impl crate::RegisterSpec for IFLAG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iflag2::R](R) reader structure"]
impl crate::Readable for IFLAG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iflag2::W](W) writer structure"]
impl crate::Writable for IFLAG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFLAG2 to value 0"]
impl crate::Resettable for IFLAG2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
