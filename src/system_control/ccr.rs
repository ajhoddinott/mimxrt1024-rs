#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NONBASETHRDENA` reader - Indicates how the processor enters Thread mode"]
pub type NONBASETHRDENA_R = crate::BitReader<NONBASETHRDENA_A>;
#[doc = "Indicates how the processor enters Thread mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NONBASETHRDENA_A {
    #[doc = "0: processor can enter Thread mode only when no exception is active"]
    NONBASETHRDENA_0 = 0,
    #[doc = "1: processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    NONBASETHRDENA_1 = 1,
}
impl From<NONBASETHRDENA_A> for bool {
    #[inline(always)]
    fn from(variant: NONBASETHRDENA_A) -> Self {
        variant as u8 != 0
    }
}
impl NONBASETHRDENA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NONBASETHRDENA_A {
        match self.bits {
            false => NONBASETHRDENA_A::NONBASETHRDENA_0,
            true => NONBASETHRDENA_A::NONBASETHRDENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `NONBASETHRDENA_0`"]
    #[inline(always)]
    pub fn is_nonbasethrdena_0(&self) -> bool {
        *self == NONBASETHRDENA_A::NONBASETHRDENA_0
    }
    #[doc = "Checks if the value of the field is `NONBASETHRDENA_1`"]
    #[inline(always)]
    pub fn is_nonbasethrdena_1(&self) -> bool {
        *self == NONBASETHRDENA_A::NONBASETHRDENA_1
    }
}
#[doc = "Field `NONBASETHRDENA` writer - Indicates how the processor enters Thread mode"]
pub type NONBASETHRDENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CCR_SPEC, NONBASETHRDENA_A, O>;
impl<'a, const O: u8> NONBASETHRDENA_W<'a, O> {
    #[doc = "processor can enter Thread mode only when no exception is active"]
    #[inline(always)]
    pub fn nonbasethrdena_0(self) -> &'a mut W {
        self.variant(NONBASETHRDENA_A::NONBASETHRDENA_0)
    }
    #[doc = "processor can enter Thread mode from any level under the control of an EXC_RETURN value"]
    #[inline(always)]
    pub fn nonbasethrdena_1(self) -> &'a mut W {
        self.variant(NONBASETHRDENA_A::NONBASETHRDENA_1)
    }
}
#[doc = "Field `USERSETMPEND` reader - Enables unprivileged software access to the STIR"]
pub type USERSETMPEND_R = crate::BitReader<USERSETMPEND_A>;
#[doc = "Enables unprivileged software access to the STIR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USERSETMPEND_A {
    #[doc = "0: disable"]
    USERSETMPEND_0 = 0,
    #[doc = "1: enable"]
    USERSETMPEND_1 = 1,
}
impl From<USERSETMPEND_A> for bool {
    #[inline(always)]
    fn from(variant: USERSETMPEND_A) -> Self {
        variant as u8 != 0
    }
}
impl USERSETMPEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USERSETMPEND_A {
        match self.bits {
            false => USERSETMPEND_A::USERSETMPEND_0,
            true => USERSETMPEND_A::USERSETMPEND_1,
        }
    }
    #[doc = "Checks if the value of the field is `USERSETMPEND_0`"]
    #[inline(always)]
    pub fn is_usersetmpend_0(&self) -> bool {
        *self == USERSETMPEND_A::USERSETMPEND_0
    }
    #[doc = "Checks if the value of the field is `USERSETMPEND_1`"]
    #[inline(always)]
    pub fn is_usersetmpend_1(&self) -> bool {
        *self == USERSETMPEND_A::USERSETMPEND_1
    }
}
#[doc = "Field `USERSETMPEND` writer - Enables unprivileged software access to the STIR"]
pub type USERSETMPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, USERSETMPEND_A, O>;
impl<'a, const O: u8> USERSETMPEND_W<'a, O> {
    #[doc = "disable"]
    #[inline(always)]
    pub fn usersetmpend_0(self) -> &'a mut W {
        self.variant(USERSETMPEND_A::USERSETMPEND_0)
    }
    #[doc = "enable"]
    #[inline(always)]
    pub fn usersetmpend_1(self) -> &'a mut W {
        self.variant(USERSETMPEND_A::USERSETMPEND_1)
    }
}
#[doc = "Field `UNALIGN_TRP` reader - Enables unaligned access traps"]
pub type UNALIGN_TRP_R = crate::BitReader<UNALIGN_TRP_A>;
#[doc = "Enables unaligned access traps\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UNALIGN_TRP_A {
    #[doc = "0: do not trap unaligned halfword and word accesses"]
    UNALIGN_TRP_0 = 0,
    #[doc = "1: trap unaligned halfword and word accesses"]
    UNALIGN_TRP_1 = 1,
}
impl From<UNALIGN_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: UNALIGN_TRP_A) -> Self {
        variant as u8 != 0
    }
}
impl UNALIGN_TRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UNALIGN_TRP_A {
        match self.bits {
            false => UNALIGN_TRP_A::UNALIGN_TRP_0,
            true => UNALIGN_TRP_A::UNALIGN_TRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `UNALIGN_TRP_0`"]
    #[inline(always)]
    pub fn is_unalign_trp_0(&self) -> bool {
        *self == UNALIGN_TRP_A::UNALIGN_TRP_0
    }
    #[doc = "Checks if the value of the field is `UNALIGN_TRP_1`"]
    #[inline(always)]
    pub fn is_unalign_trp_1(&self) -> bool {
        *self == UNALIGN_TRP_A::UNALIGN_TRP_1
    }
}
#[doc = "Field `UNALIGN_TRP` writer - Enables unaligned access traps"]
pub type UNALIGN_TRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, UNALIGN_TRP_A, O>;
impl<'a, const O: u8> UNALIGN_TRP_W<'a, O> {
    #[doc = "do not trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn unalign_trp_0(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::UNALIGN_TRP_0)
    }
    #[doc = "trap unaligned halfword and word accesses"]
    #[inline(always)]
    pub fn unalign_trp_1(self) -> &'a mut W {
        self.variant(UNALIGN_TRP_A::UNALIGN_TRP_1)
    }
}
#[doc = "Field `DIV_0_TRP` reader - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
pub type DIV_0_TRP_R = crate::BitReader<DIV_0_TRP_A>;
#[doc = "Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIV_0_TRP_A {
    #[doc = "0: do not trap divide by 0"]
    DIV_0_TRP_0 = 0,
    #[doc = "1: trap divide by 0"]
    DIV_0_TRP_1 = 1,
}
impl From<DIV_0_TRP_A> for bool {
    #[inline(always)]
    fn from(variant: DIV_0_TRP_A) -> Self {
        variant as u8 != 0
    }
}
impl DIV_0_TRP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIV_0_TRP_A {
        match self.bits {
            false => DIV_0_TRP_A::DIV_0_TRP_0,
            true => DIV_0_TRP_A::DIV_0_TRP_1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV_0_TRP_0`"]
    #[inline(always)]
    pub fn is_div_0_trp_0(&self) -> bool {
        *self == DIV_0_TRP_A::DIV_0_TRP_0
    }
    #[doc = "Checks if the value of the field is `DIV_0_TRP_1`"]
    #[inline(always)]
    pub fn is_div_0_trp_1(&self) -> bool {
        *self == DIV_0_TRP_A::DIV_0_TRP_1
    }
}
#[doc = "Field `DIV_0_TRP` writer - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
pub type DIV_0_TRP_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, DIV_0_TRP_A, O>;
impl<'a, const O: u8> DIV_0_TRP_W<'a, O> {
    #[doc = "do not trap divide by 0"]
    #[inline(always)]
    pub fn div_0_trp_0(self) -> &'a mut W {
        self.variant(DIV_0_TRP_A::DIV_0_TRP_0)
    }
    #[doc = "trap divide by 0"]
    #[inline(always)]
    pub fn div_0_trp_1(self) -> &'a mut W {
        self.variant(DIV_0_TRP_A::DIV_0_TRP_1)
    }
}
#[doc = "Field `BFHFNMIGN` reader - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
pub type BFHFNMIGN_R = crate::BitReader<BFHFNMIGN_A>;
#[doc = "Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BFHFNMIGN_A {
    #[doc = "0: data bus faults caused by load and store instructions cause a lock-up"]
    BFHFNMIGN_0 = 0,
    #[doc = "1: handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    BFHFNMIGN_1 = 1,
}
impl From<BFHFNMIGN_A> for bool {
    #[inline(always)]
    fn from(variant: BFHFNMIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl BFHFNMIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BFHFNMIGN_A {
        match self.bits {
            false => BFHFNMIGN_A::BFHFNMIGN_0,
            true => BFHFNMIGN_A::BFHFNMIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `BFHFNMIGN_0`"]
    #[inline(always)]
    pub fn is_bfhfnmign_0(&self) -> bool {
        *self == BFHFNMIGN_A::BFHFNMIGN_0
    }
    #[doc = "Checks if the value of the field is `BFHFNMIGN_1`"]
    #[inline(always)]
    pub fn is_bfhfnmign_1(&self) -> bool {
        *self == BFHFNMIGN_A::BFHFNMIGN_1
    }
}
#[doc = "Field `BFHFNMIGN` writer - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
pub type BFHFNMIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, BFHFNMIGN_A, O>;
impl<'a, const O: u8> BFHFNMIGN_W<'a, O> {
    #[doc = "data bus faults caused by load and store instructions cause a lock-up"]
    #[inline(always)]
    pub fn bfhfnmign_0(self) -> &'a mut W {
        self.variant(BFHFNMIGN_A::BFHFNMIGN_0)
    }
    #[doc = "handlers running at priority -1 and -2 ignore data bus faults caused by load and store instructions"]
    #[inline(always)]
    pub fn bfhfnmign_1(self) -> &'a mut W {
        self.variant(BFHFNMIGN_A::BFHFNMIGN_1)
    }
}
#[doc = "Field `STKALIGN` reader - Indicates stack alignment on exception entry"]
pub type STKALIGN_R = crate::BitReader<STKALIGN_A>;
#[doc = "Indicates stack alignment on exception entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STKALIGN_A {
    #[doc = "0: 4-byte aligned"]
    STKALIGN_0 = 0,
    #[doc = "1: 8-byte aligned"]
    STKALIGN_1 = 1,
}
impl From<STKALIGN_A> for bool {
    #[inline(always)]
    fn from(variant: STKALIGN_A) -> Self {
        variant as u8 != 0
    }
}
impl STKALIGN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STKALIGN_A {
        match self.bits {
            false => STKALIGN_A::STKALIGN_0,
            true => STKALIGN_A::STKALIGN_1,
        }
    }
    #[doc = "Checks if the value of the field is `STKALIGN_0`"]
    #[inline(always)]
    pub fn is_stkalign_0(&self) -> bool {
        *self == STKALIGN_A::STKALIGN_0
    }
    #[doc = "Checks if the value of the field is `STKALIGN_1`"]
    #[inline(always)]
    pub fn is_stkalign_1(&self) -> bool {
        *self == STKALIGN_A::STKALIGN_1
    }
}
#[doc = "Field `STKALIGN` writer - Indicates stack alignment on exception entry"]
pub type STKALIGN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, STKALIGN_A, O>;
impl<'a, const O: u8> STKALIGN_W<'a, O> {
    #[doc = "4-byte aligned"]
    #[inline(always)]
    pub fn stkalign_0(self) -> &'a mut W {
        self.variant(STKALIGN_A::STKALIGN_0)
    }
    #[doc = "8-byte aligned"]
    #[inline(always)]
    pub fn stkalign_1(self) -> &'a mut W {
        self.variant(STKALIGN_A::STKALIGN_1)
    }
}
#[doc = "Field `DC` reader - Enables L1 data cache."]
pub type DC_R = crate::BitReader<DC_A>;
#[doc = "Enables L1 data cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DC_A {
    #[doc = "0: L1 data cache disabled"]
    DC_0 = 0,
    #[doc = "1: L1 data cache enabled"]
    DC_1 = 1,
}
impl From<DC_A> for bool {
    #[inline(always)]
    fn from(variant: DC_A) -> Self {
        variant as u8 != 0
    }
}
impl DC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DC_A {
        match self.bits {
            false => DC_A::DC_0,
            true => DC_A::DC_1,
        }
    }
    #[doc = "Checks if the value of the field is `DC_0`"]
    #[inline(always)]
    pub fn is_dc_0(&self) -> bool {
        *self == DC_A::DC_0
    }
    #[doc = "Checks if the value of the field is `DC_1`"]
    #[inline(always)]
    pub fn is_dc_1(&self) -> bool {
        *self == DC_A::DC_1
    }
}
#[doc = "Field `DC` writer - Enables L1 data cache."]
pub type DC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, DC_A, O>;
impl<'a, const O: u8> DC_W<'a, O> {
    #[doc = "L1 data cache disabled"]
    #[inline(always)]
    pub fn dc_0(self) -> &'a mut W {
        self.variant(DC_A::DC_0)
    }
    #[doc = "L1 data cache enabled"]
    #[inline(always)]
    pub fn dc_1(self) -> &'a mut W {
        self.variant(DC_A::DC_1)
    }
}
#[doc = "Field `IC` reader - Enables L1 instruction cache."]
pub type IC_R = crate::BitReader<IC_A>;
#[doc = "Enables L1 instruction cache.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IC_A {
    #[doc = "0: L1 instruction cache disabled"]
    IC_0 = 0,
    #[doc = "1: L1 instruction cache enabled"]
    IC_1 = 1,
}
impl From<IC_A> for bool {
    #[inline(always)]
    fn from(variant: IC_A) -> Self {
        variant as u8 != 0
    }
}
impl IC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC_A {
        match self.bits {
            false => IC_A::IC_0,
            true => IC_A::IC_1,
        }
    }
    #[doc = "Checks if the value of the field is `IC_0`"]
    #[inline(always)]
    pub fn is_ic_0(&self) -> bool {
        *self == IC_A::IC_0
    }
    #[doc = "Checks if the value of the field is `IC_1`"]
    #[inline(always)]
    pub fn is_ic_1(&self) -> bool {
        *self == IC_A::IC_1
    }
}
#[doc = "Field `IC` writer - Enables L1 instruction cache."]
pub type IC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CCR_SPEC, IC_A, O>;
impl<'a, const O: u8> IC_W<'a, O> {
    #[doc = "L1 instruction cache disabled"]
    #[inline(always)]
    pub fn ic_0(self) -> &'a mut W {
        self.variant(IC_A::IC_0)
    }
    #[doc = "L1 instruction cache enabled"]
    #[inline(always)]
    pub fn ic_1(self) -> &'a mut W {
        self.variant(IC_A::IC_1)
    }
}
#[doc = "Field `BP` reader - Always reads-as-one. It indicates branch prediction is enabled."]
pub type BP_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Indicates how the processor enters Thread mode"]
    #[inline(always)]
    pub fn nonbasethrdena(&self) -> NONBASETHRDENA_R {
        NONBASETHRDENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline(always)]
    pub fn usersetmpend(&self) -> USERSETMPEND_R {
        USERSETMPEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    pub fn unalign_trp(&self) -> UNALIGN_TRP_R {
        UNALIGN_TRP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    pub fn div_0_trp(&self) -> DIV_0_TRP_R {
        DIV_0_TRP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    pub fn bfhfnmign(&self) -> BFHFNMIGN_R {
        BFHFNMIGN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    pub fn stkalign(&self) -> STKALIGN_R {
        STKALIGN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - Enables L1 data cache."]
    #[inline(always)]
    pub fn dc(&self) -> DC_R {
        DC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enables L1 instruction cache."]
    #[inline(always)]
    pub fn ic(&self) -> IC_R {
        IC_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Always reads-as-one. It indicates branch prediction is enabled."]
    #[inline(always)]
    pub fn bp(&self) -> BP_R {
        BP_R::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Indicates how the processor enters Thread mode"]
    #[inline(always)]
    #[must_use]
    pub fn nonbasethrdena(&mut self) -> NONBASETHRDENA_W<0> {
        NONBASETHRDENA_W::new(self)
    }
    #[doc = "Bit 1 - Enables unprivileged software access to the STIR"]
    #[inline(always)]
    #[must_use]
    pub fn usersetmpend(&mut self) -> USERSETMPEND_W<1> {
        USERSETMPEND_W::new(self)
    }
    #[doc = "Bit 3 - Enables unaligned access traps"]
    #[inline(always)]
    #[must_use]
    pub fn unalign_trp(&mut self) -> UNALIGN_TRP_W<3> {
        UNALIGN_TRP_W::new(self)
    }
    #[doc = "Bit 4 - Enables faulting or halting when the processor executes an SDIV or UDIV instruction with a divisor of 0"]
    #[inline(always)]
    #[must_use]
    pub fn div_0_trp(&mut self) -> DIV_0_TRP_W<4> {
        DIV_0_TRP_W::new(self)
    }
    #[doc = "Bit 8 - Enables handlers with priority -1 or -2 to ignore data BusFaults caused by load and store instructions."]
    #[inline(always)]
    #[must_use]
    pub fn bfhfnmign(&mut self) -> BFHFNMIGN_W<8> {
        BFHFNMIGN_W::new(self)
    }
    #[doc = "Bit 9 - Indicates stack alignment on exception entry"]
    #[inline(always)]
    #[must_use]
    pub fn stkalign(&mut self) -> STKALIGN_W<9> {
        STKALIGN_W::new(self)
    }
    #[doc = "Bit 16 - Enables L1 data cache."]
    #[inline(always)]
    #[must_use]
    pub fn dc(&mut self) -> DC_W<16> {
        DC_W::new(self)
    }
    #[doc = "Bit 17 - Enables L1 instruction cache."]
    #[inline(always)]
    #[must_use]
    pub fn ic(&mut self) -> IC_W<17> {
        IC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration and Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0x0004_0000"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0x0004_0000;
}
