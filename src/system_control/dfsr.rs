#[doc = "Register `DFSR` reader"]
pub struct R(crate::R<DFSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DFSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DFSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DFSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DFSR` writer"]
pub struct W(crate::W<DFSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DFSR_SPEC>;
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
impl From<crate::W<DFSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DFSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HALTED` reader - Indicates a debug event generated by either a C_HALT or C_STEP request, triggered by a write to the DHCSR or a step request triggered by setting DEMCR.MON_STEP to 1."]
pub type HALTED_R = crate::BitReader<HALTED_A>;
#[doc = "Indicates a debug event generated by either a C_HALT or C_STEP request, triggered by a write to the DHCSR or a step request triggered by setting DEMCR.MON_STEP to 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HALTED_A {
    #[doc = "0: No active halt request debug event"]
    HALTED_0 = 0,
    #[doc = "1: Halt request debug event active"]
    HALTED_1 = 1,
}
impl From<HALTED_A> for bool {
    #[inline(always)]
    fn from(variant: HALTED_A) -> Self {
        variant as u8 != 0
    }
}
impl HALTED_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HALTED_A {
        match self.bits {
            false => HALTED_A::HALTED_0,
            true => HALTED_A::HALTED_1,
        }
    }
    #[doc = "Checks if the value of the field is `HALTED_0`"]
    #[inline(always)]
    pub fn is_halted_0(&self) -> bool {
        *self == HALTED_A::HALTED_0
    }
    #[doc = "Checks if the value of the field is `HALTED_1`"]
    #[inline(always)]
    pub fn is_halted_1(&self) -> bool {
        *self == HALTED_A::HALTED_1
    }
}
#[doc = "Field `HALTED` writer - Indicates a debug event generated by either a C_HALT or C_STEP request, triggered by a write to the DHCSR or a step request triggered by setting DEMCR.MON_STEP to 1."]
pub type HALTED_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, HALTED_A, O>;
impl<'a, const O: u8> HALTED_W<'a, O> {
    #[doc = "No active halt request debug event"]
    #[inline(always)]
    pub fn halted_0(self) -> &'a mut W {
        self.variant(HALTED_A::HALTED_0)
    }
    #[doc = "Halt request debug event active"]
    #[inline(always)]
    pub fn halted_1(self) -> &'a mut W {
        self.variant(HALTED_A::HALTED_1)
    }
}
#[doc = "Field `BKPT` reader - Debug event generated by BKPT instruction execution or a breakpoint match in FPB"]
pub type BKPT_R = crate::BitReader<BKPT_A>;
#[doc = "Debug event generated by BKPT instruction execution or a breakpoint match in FPB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BKPT_A {
    #[doc = "0: No current breakpoint debug event"]
    BKPT_0 = 0,
    #[doc = "1: At least one current breakpoint debug event"]
    BKPT_1 = 1,
}
impl From<BKPT_A> for bool {
    #[inline(always)]
    fn from(variant: BKPT_A) -> Self {
        variant as u8 != 0
    }
}
impl BKPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BKPT_A {
        match self.bits {
            false => BKPT_A::BKPT_0,
            true => BKPT_A::BKPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `BKPT_0`"]
    #[inline(always)]
    pub fn is_bkpt_0(&self) -> bool {
        *self == BKPT_A::BKPT_0
    }
    #[doc = "Checks if the value of the field is `BKPT_1`"]
    #[inline(always)]
    pub fn is_bkpt_1(&self) -> bool {
        *self == BKPT_A::BKPT_1
    }
}
#[doc = "Field `BKPT` writer - Debug event generated by BKPT instruction execution or a breakpoint match in FPB"]
pub type BKPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, BKPT_A, O>;
impl<'a, const O: u8> BKPT_W<'a, O> {
    #[doc = "No current breakpoint debug event"]
    #[inline(always)]
    pub fn bkpt_0(self) -> &'a mut W {
        self.variant(BKPT_A::BKPT_0)
    }
    #[doc = "At least one current breakpoint debug event"]
    #[inline(always)]
    pub fn bkpt_1(self) -> &'a mut W {
        self.variant(BKPT_A::BKPT_1)
    }
}
#[doc = "Field `DWTTRAP` reader - Debug event generated by the DWT"]
pub type DWTTRAP_R = crate::BitReader<DWTTRAP_A>;
#[doc = "Debug event generated by the DWT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DWTTRAP_A {
    #[doc = "0: No current debug events generated by the DWT"]
    DWTTRAP_0 = 0,
    #[doc = "1: At least one current debug event generated by the DWT"]
    DWTTRAP_1 = 1,
}
impl From<DWTTRAP_A> for bool {
    #[inline(always)]
    fn from(variant: DWTTRAP_A) -> Self {
        variant as u8 != 0
    }
}
impl DWTTRAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DWTTRAP_A {
        match self.bits {
            false => DWTTRAP_A::DWTTRAP_0,
            true => DWTTRAP_A::DWTTRAP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DWTTRAP_0`"]
    #[inline(always)]
    pub fn is_dwttrap_0(&self) -> bool {
        *self == DWTTRAP_A::DWTTRAP_0
    }
    #[doc = "Checks if the value of the field is `DWTTRAP_1`"]
    #[inline(always)]
    pub fn is_dwttrap_1(&self) -> bool {
        *self == DWTTRAP_A::DWTTRAP_1
    }
}
#[doc = "Field `DWTTRAP` writer - Debug event generated by the DWT"]
pub type DWTTRAP_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, DWTTRAP_A, O>;
impl<'a, const O: u8> DWTTRAP_W<'a, O> {
    #[doc = "No current debug events generated by the DWT"]
    #[inline(always)]
    pub fn dwttrap_0(self) -> &'a mut W {
        self.variant(DWTTRAP_A::DWTTRAP_0)
    }
    #[doc = "At least one current debug event generated by the DWT"]
    #[inline(always)]
    pub fn dwttrap_1(self) -> &'a mut W {
        self.variant(DWTTRAP_A::DWTTRAP_1)
    }
}
#[doc = "Field `VCATCH` reader - Indicates triggering of a Vector catch"]
pub type VCATCH_R = crate::BitReader<VCATCH_A>;
#[doc = "Indicates triggering of a Vector catch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VCATCH_A {
    #[doc = "0: No Vector catch triggered"]
    VCATCH_0 = 0,
    #[doc = "1: Vector catch triggered"]
    VCATCH_1 = 1,
}
impl From<VCATCH_A> for bool {
    #[inline(always)]
    fn from(variant: VCATCH_A) -> Self {
        variant as u8 != 0
    }
}
impl VCATCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VCATCH_A {
        match self.bits {
            false => VCATCH_A::VCATCH_0,
            true => VCATCH_A::VCATCH_1,
        }
    }
    #[doc = "Checks if the value of the field is `VCATCH_0`"]
    #[inline(always)]
    pub fn is_vcatch_0(&self) -> bool {
        *self == VCATCH_A::VCATCH_0
    }
    #[doc = "Checks if the value of the field is `VCATCH_1`"]
    #[inline(always)]
    pub fn is_vcatch_1(&self) -> bool {
        *self == VCATCH_A::VCATCH_1
    }
}
#[doc = "Field `VCATCH` writer - Indicates triggering of a Vector catch"]
pub type VCATCH_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, VCATCH_A, O>;
impl<'a, const O: u8> VCATCH_W<'a, O> {
    #[doc = "No Vector catch triggered"]
    #[inline(always)]
    pub fn vcatch_0(self) -> &'a mut W {
        self.variant(VCATCH_A::VCATCH_0)
    }
    #[doc = "Vector catch triggered"]
    #[inline(always)]
    pub fn vcatch_1(self) -> &'a mut W {
        self.variant(VCATCH_A::VCATCH_1)
    }
}
#[doc = "Field `EXTERNAL` reader - Debug event generated because of the assertion of an external debug request"]
pub type EXTERNAL_R = crate::BitReader<EXTERNAL_A>;
#[doc = "Debug event generated because of the assertion of an external debug request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXTERNAL_A {
    #[doc = "0: No external debug request debug event"]
    EXTERNAL_0 = 0,
    #[doc = "1: External debug request debug event"]
    EXTERNAL_1 = 1,
}
impl From<EXTERNAL_A> for bool {
    #[inline(always)]
    fn from(variant: EXTERNAL_A) -> Self {
        variant as u8 != 0
    }
}
impl EXTERNAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXTERNAL_A {
        match self.bits {
            false => EXTERNAL_A::EXTERNAL_0,
            true => EXTERNAL_A::EXTERNAL_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_0`"]
    #[inline(always)]
    pub fn is_external_0(&self) -> bool {
        *self == EXTERNAL_A::EXTERNAL_0
    }
    #[doc = "Checks if the value of the field is `EXTERNAL_1`"]
    #[inline(always)]
    pub fn is_external_1(&self) -> bool {
        *self == EXTERNAL_A::EXTERNAL_1
    }
}
#[doc = "Field `EXTERNAL` writer - Debug event generated because of the assertion of an external debug request"]
pub type EXTERNAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, DFSR_SPEC, EXTERNAL_A, O>;
impl<'a, const O: u8> EXTERNAL_W<'a, O> {
    #[doc = "No external debug request debug event"]
    #[inline(always)]
    pub fn external_0(self) -> &'a mut W {
        self.variant(EXTERNAL_A::EXTERNAL_0)
    }
    #[doc = "External debug request debug event"]
    #[inline(always)]
    pub fn external_1(self) -> &'a mut W {
        self.variant(EXTERNAL_A::EXTERNAL_1)
    }
}
impl R {
    #[doc = "Bit 0 - Indicates a debug event generated by either a C_HALT or C_STEP request, triggered by a write to the DHCSR or a step request triggered by setting DEMCR.MON_STEP to 1."]
    #[inline(always)]
    pub fn halted(&self) -> HALTED_R {
        HALTED_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Debug event generated by BKPT instruction execution or a breakpoint match in FPB"]
    #[inline(always)]
    pub fn bkpt(&self) -> BKPT_R {
        BKPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Debug event generated by the DWT"]
    #[inline(always)]
    pub fn dwttrap(&self) -> DWTTRAP_R {
        DWTTRAP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates triggering of a Vector catch"]
    #[inline(always)]
    pub fn vcatch(&self) -> VCATCH_R {
        VCATCH_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Debug event generated because of the assertion of an external debug request"]
    #[inline(always)]
    pub fn external(&self) -> EXTERNAL_R {
        EXTERNAL_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates a debug event generated by either a C_HALT or C_STEP request, triggered by a write to the DHCSR or a step request triggered by setting DEMCR.MON_STEP to 1."]
    #[inline(always)]
    #[must_use]
    pub fn halted(&mut self) -> HALTED_W<0> {
        HALTED_W::new(self)
    }
    #[doc = "Bit 1 - Debug event generated by BKPT instruction execution or a breakpoint match in FPB"]
    #[inline(always)]
    #[must_use]
    pub fn bkpt(&mut self) -> BKPT_W<1> {
        BKPT_W::new(self)
    }
    #[doc = "Bit 2 - Debug event generated by the DWT"]
    #[inline(always)]
    #[must_use]
    pub fn dwttrap(&mut self) -> DWTTRAP_W<2> {
        DWTTRAP_W::new(self)
    }
    #[doc = "Bit 3 - Indicates triggering of a Vector catch"]
    #[inline(always)]
    #[must_use]
    pub fn vcatch(&mut self) -> VCATCH_W<3> {
        VCATCH_W::new(self)
    }
    #[doc = "Bit 4 - Debug event generated because of the assertion of an external debug request"]
    #[inline(always)]
    #[must_use]
    pub fn external(&mut self) -> EXTERNAL_W<4> {
        EXTERNAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Debug Fault Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dfsr](index.html) module"]
pub struct DFSR_SPEC;
impl crate::RegisterSpec for DFSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dfsr::R](R) reader structure"]
impl crate::Readable for DFSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dfsr::W](W) writer structure"]
impl crate::Writable for DFSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DFSR to value 0"]
impl crate::Resettable for DFSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
