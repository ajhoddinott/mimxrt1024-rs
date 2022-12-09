#[doc = "Register `MASK` reader"]
pub struct R(crate::R<MASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MASK` writer"]
pub struct W(crate::W<MASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MASK_SPEC>;
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
impl From<crate::W<MASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASKX` reader - PWM_X Masks"]
pub type MASKX_R = crate::FieldReader<u8, MASKX_A>;
#[doc = "PWM_X Masks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKX_A {
    #[doc = "0: PWM_X output normal."]
    NORMAL = 0,
    #[doc = "1: PWM_X output masked."]
    MASKED = 1,
}
impl From<MASKX_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKX_A) -> Self {
        variant as _
    }
}
impl MASKX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASKX_A> {
        match self.bits {
            0 => Some(MASKX_A::NORMAL),
            1 => Some(MASKX_A::MASKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MASKX_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MASKX_A::MASKED
    }
}
#[doc = "Field `MASKX` writer - PWM_X Masks"]
pub type MASKX_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MASK_SPEC, u8, MASKX_A, 4, O>;
impl<'a, const O: u8> MASKX_W<'a, O> {
    #[doc = "PWM_X output normal."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MASKX_A::NORMAL)
    }
    #[doc = "PWM_X output masked."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MASKX_A::MASKED)
    }
}
#[doc = "Field `MASKB` reader - PWM_B Masks"]
pub type MASKB_R = crate::FieldReader<u8, MASKB_A>;
#[doc = "PWM_B Masks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKB_A {
    #[doc = "0: PWM_B output normal."]
    NORMAL = 0,
    #[doc = "1: PWM_B output masked."]
    MASKED = 1,
}
impl From<MASKB_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKB_A) -> Self {
        variant as _
    }
}
impl MASKB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASKB_A> {
        match self.bits {
            0 => Some(MASKB_A::NORMAL),
            1 => Some(MASKB_A::MASKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MASKB_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MASKB_A::MASKED
    }
}
#[doc = "Field `MASKB` writer - PWM_B Masks"]
pub type MASKB_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MASK_SPEC, u8, MASKB_A, 4, O>;
impl<'a, const O: u8> MASKB_W<'a, O> {
    #[doc = "PWM_B output normal."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MASKB_A::NORMAL)
    }
    #[doc = "PWM_B output masked."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MASKB_A::MASKED)
    }
}
#[doc = "Field `MASKA` reader - PWM_A Masks"]
pub type MASKA_R = crate::FieldReader<u8, MASKA_A>;
#[doc = "PWM_A Masks\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKA_A {
    #[doc = "0: PWM_A output normal."]
    NORMAL = 0,
    #[doc = "1: PWM_A output masked."]
    MASKED = 1,
}
impl From<MASKA_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKA_A) -> Self {
        variant as _
    }
}
impl MASKA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MASKA_A> {
        match self.bits {
            0 => Some(MASKA_A::NORMAL),
            1 => Some(MASKA_A::MASKED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == MASKA_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == MASKA_A::MASKED
    }
}
#[doc = "Field `MASKA` writer - PWM_A Masks"]
pub type MASKA_W<'a, const O: u8> = crate::FieldWriter<'a, u16, MASK_SPEC, u8, MASKA_A, 4, O>;
impl<'a, const O: u8> MASKA_W<'a, O> {
    #[doc = "PWM_A output normal."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(MASKA_A::NORMAL)
    }
    #[doc = "PWM_A output masked."]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(MASKA_A::MASKED)
    }
}
#[doc = "Update Mask Bits Immediately\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum UPDATE_MASK_AW {
    #[doc = "0: Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule."]
    NORMAL = 0,
    #[doc = "1: Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit."]
    MASK_IMMEDIATE = 1,
}
impl From<UPDATE_MASK_AW> for u8 {
    #[inline(always)]
    fn from(variant: UPDATE_MASK_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `UPDATE_MASK` writer - Update Mask Bits Immediately"]
pub type UPDATE_MASK_W<'a, const O: u8> =
    crate::FieldWriter<'a, u16, MASK_SPEC, u8, UPDATE_MASK_AW, 4, O>;
impl<'a, const O: u8> UPDATE_MASK_W<'a, O> {
    #[doc = "Normal operation. MASK* bits within the corresponding submodule are not updated until a FORCE_OUT event occurs within the submodule."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(UPDATE_MASK_AW::NORMAL)
    }
    #[doc = "Immediate operation. MASK* bits within the corresponding submodule are updated on the following clock edge after setting this bit."]
    #[inline(always)]
    pub fn mask_immediate(self) -> &'a mut W {
        self.variant(UPDATE_MASK_AW::MASK_IMMEDIATE)
    }
}
impl R {
    #[doc = "Bits 0:3 - PWM_X Masks"]
    #[inline(always)]
    pub fn maskx(&self) -> MASKX_R {
        MASKX_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - PWM_B Masks"]
    #[inline(always)]
    pub fn maskb(&self) -> MASKB_R {
        MASKB_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - PWM_A Masks"]
    #[inline(always)]
    pub fn maska(&self) -> MASKA_R {
        MASKA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - PWM_X Masks"]
    #[inline(always)]
    #[must_use]
    pub fn maskx(&mut self) -> MASKX_W<0> {
        MASKX_W::new(self)
    }
    #[doc = "Bits 4:7 - PWM_B Masks"]
    #[inline(always)]
    #[must_use]
    pub fn maskb(&mut self) -> MASKB_W<4> {
        MASKB_W::new(self)
    }
    #[doc = "Bits 8:11 - PWM_A Masks"]
    #[inline(always)]
    #[must_use]
    pub fn maska(&mut self) -> MASKA_W<8> {
        MASKA_W::new(self)
    }
    #[doc = "Bits 12:15 - Update Mask Bits Immediately"]
    #[inline(always)]
    #[must_use]
    pub fn update_mask(&mut self) -> UPDATE_MASK_W<12> {
        UPDATE_MASK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mask](index.html) module"]
pub struct MASK_SPEC;
impl crate::RegisterSpec for MASK_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mask::R](R) reader structure"]
impl crate::Readable for MASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mask::W](W) writer structure"]
impl crate::Writable for MASK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MASK to value 0"]
impl crate::Resettable for MASK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
