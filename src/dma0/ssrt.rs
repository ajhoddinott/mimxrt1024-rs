#[doc = "Register `SSRT` reader"]
pub struct R(crate::R<SSRT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SSRT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SSRT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SSRT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SSRT` writer"]
pub struct W(crate::W<SSRT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SSRT_SPEC>;
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
impl From<crate::W<SSRT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SSRT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SSRT` reader - Set START field"]
pub type SSRT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SSRT` writer - Set START field"]
pub type SSRT_W<'a, const O: u8> = crate::FieldWriter<'a, u8, SSRT_SPEC, u8, u8, 5, O>;
#[doc = "Field `SAST` reader - Set All START fields (activates all channels)"]
pub type SAST_R = crate::BitReader<SAST_A>;
#[doc = "Set All START fields (activates all channels)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAST_A {
    #[doc = "0: Write 1 to only the TCDn_CSR\\[START\\]
field specified in the SSRT field"]
    SET_START = 0,
    #[doc = "1: Write 1 to all bits in TCDn_CSR\\[START\\]"]
    SET_ALL = 1,
}
impl From<SAST_A> for bool {
    #[inline(always)]
    fn from(variant: SAST_A) -> Self {
        variant as u8 != 0
    }
}
impl SAST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAST_A {
        match self.bits {
            false => SAST_A::SET_START,
            true => SAST_A::SET_ALL,
        }
    }
    #[doc = "Checks if the value of the field is `SET_START`"]
    #[inline(always)]
    pub fn is_set_start(&self) -> bool {
        *self == SAST_A::SET_START
    }
    #[doc = "Checks if the value of the field is `SET_ALL`"]
    #[inline(always)]
    pub fn is_set_all(&self) -> bool {
        *self == SAST_A::SET_ALL
    }
}
#[doc = "Field `SAST` writer - Set All START fields (activates all channels)"]
pub type SAST_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSRT_SPEC, SAST_A, O>;
impl<'a, const O: u8> SAST_W<'a, O> {
    #[doc = "Write 1 to only the TCDn_CSR\\[START\\]
field specified in the SSRT field"]
    #[inline(always)]
    pub fn set_start(self) -> &'a mut W {
        self.variant(SAST_A::SET_START)
    }
    #[doc = "Write 1 to all bits in TCDn_CSR\\[START\\]"]
    #[inline(always)]
    pub fn set_all(self) -> &'a mut W {
        self.variant(SAST_A::SET_ALL)
    }
}
#[doc = "Field `NOP` reader - No Op Enable"]
pub type NOP_R = crate::BitReader<NOP_A>;
#[doc = "No Op Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOP_A {
    #[doc = "0: Normal operation"]
    NORMAL_OPS = 0,
    #[doc = "1: No operation; all other fields in this register are ignored."]
    NO_OPS = 1,
}
impl From<NOP_A> for bool {
    #[inline(always)]
    fn from(variant: NOP_A) -> Self {
        variant as u8 != 0
    }
}
impl NOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOP_A {
        match self.bits {
            false => NOP_A::NORMAL_OPS,
            true => NOP_A::NO_OPS,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL_OPS`"]
    #[inline(always)]
    pub fn is_normal_ops(&self) -> bool {
        *self == NOP_A::NORMAL_OPS
    }
    #[doc = "Checks if the value of the field is `NO_OPS`"]
    #[inline(always)]
    pub fn is_no_ops(&self) -> bool {
        *self == NOP_A::NO_OPS
    }
}
#[doc = "Field `NOP` writer - No Op Enable"]
pub type NOP_W<'a, const O: u8> = crate::BitWriter<'a, u8, SSRT_SPEC, NOP_A, O>;
impl<'a, const O: u8> NOP_W<'a, O> {
    #[doc = "Normal operation"]
    #[inline(always)]
    pub fn normal_ops(self) -> &'a mut W {
        self.variant(NOP_A::NORMAL_OPS)
    }
    #[doc = "No operation; all other fields in this register are ignored."]
    #[inline(always)]
    pub fn no_ops(self) -> &'a mut W {
        self.variant(NOP_A::NO_OPS)
    }
}
impl R {
    #[doc = "Bits 0:4 - Set START field"]
    #[inline(always)]
    pub fn ssrt(&self) -> SSRT_R {
        SSRT_R::new(self.bits & 0x1f)
    }
    #[doc = "Bit 6 - Set All START fields (activates all channels)"]
    #[inline(always)]
    pub fn sast(&self) -> SAST_R {
        SAST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    pub fn nop(&self) -> NOP_R {
        NOP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Set START field"]
    #[inline(always)]
    #[must_use]
    pub fn ssrt(&mut self) -> SSRT_W<0> {
        SSRT_W::new(self)
    }
    #[doc = "Bit 6 - Set All START fields (activates all channels)"]
    #[inline(always)]
    #[must_use]
    pub fn sast(&mut self) -> SAST_W<6> {
        SAST_W::new(self)
    }
    #[doc = "Bit 7 - No Op Enable"]
    #[inline(always)]
    #[must_use]
    pub fn nop(&mut self) -> NOP_W<7> {
        NOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Set START Bit\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ssrt](index.html) module"]
pub struct SSRT_SPEC;
impl crate::RegisterSpec for SSRT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [ssrt::R](R) reader structure"]
impl crate::Readable for SSRT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ssrt::W](W) writer structure"]
impl crate::Writable for SSRT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SSRT to value 0"]
impl crate::Resettable for SSRT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
