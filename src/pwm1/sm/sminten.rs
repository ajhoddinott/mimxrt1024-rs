#[doc = "Register `SMINTEN` reader"]
pub struct R(crate::R<SMINTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMINTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMINTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMINTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMINTEN` writer"]
pub struct W(crate::W<SMINTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMINTEN_SPEC>;
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
impl From<crate::W<SMINTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMINTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPIE` reader - Compare Interrupt Enables"]
pub type CMPIE_R = crate::FieldReader<u8, CMPIE_A>;
#[doc = "Compare Interrupt Enables\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPIE_A {
    #[doc = "0: The corresponding STS\\[CMPF\\]
bit will not cause an interrupt request."]
    DISABLED = 0,
    #[doc = "1: The corresponding STS\\[CMPF\\]
bit will cause an interrupt request."]
    ENABLED = 1,
}
impl From<CMPIE_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPIE_A) -> Self {
        variant as _
    }
}
impl CMPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPIE_A> {
        match self.bits {
            0 => Some(CMPIE_A::DISABLED),
            1 => Some(CMPIE_A::ENABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMPIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMPIE_A::ENABLED
    }
}
#[doc = "Field `CMPIE` writer - Compare Interrupt Enables"]
pub type CMPIE_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMINTEN_SPEC, u8, CMPIE_A, 6, O>;
impl<'a, const O: u8> CMPIE_W<'a, O> {
    #[doc = "The corresponding STS\\[CMPF\\]
bit will not cause an interrupt request."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMPIE_A::DISABLED)
    }
    #[doc = "The corresponding STS\\[CMPF\\]
bit will cause an interrupt request."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMPIE_A::ENABLED)
    }
}
#[doc = "Field `CX0IE` reader - Capture X 0 Interrupt Enable"]
pub type CX0IE_R = crate::BitReader<CX0IE_A>;
#[doc = "Capture X 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CX0IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFX0\\]."]
    DISABLED = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFX0\\]."]
    ENABLED = 1,
}
impl From<CX0IE_A> for bool {
    #[inline(always)]
    fn from(variant: CX0IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CX0IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CX0IE_A {
        match self.bits {
            false => CX0IE_A::DISABLED,
            true => CX0IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CX0IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CX0IE_A::ENABLED
    }
}
#[doc = "Field `CX0IE` writer - Capture X 0 Interrupt Enable"]
pub type CX0IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMINTEN_SPEC, CX0IE_A, O>;
impl<'a, const O: u8> CX0IE_W<'a, O> {
    #[doc = "Interrupt request disabled for STS\\[CFX0\\]."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CX0IE_A::DISABLED)
    }
    #[doc = "Interrupt request enabled for STS\\[CFX0\\]."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CX0IE_A::ENABLED)
    }
}
#[doc = "Field `CX1IE` reader - Capture X 1 Interrupt Enable"]
pub type CX1IE_R = crate::BitReader<CX1IE_A>;
#[doc = "Capture X 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CX1IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFX1\\]."]
    DISABLED = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFX1\\]."]
    ENABLED = 1,
}
impl From<CX1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CX1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CX1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CX1IE_A {
        match self.bits {
            false => CX1IE_A::DISABLED,
            true => CX1IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CX1IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CX1IE_A::ENABLED
    }
}
#[doc = "Field `CX1IE` writer - Capture X 1 Interrupt Enable"]
pub type CX1IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMINTEN_SPEC, CX1IE_A, O>;
impl<'a, const O: u8> CX1IE_W<'a, O> {
    #[doc = "Interrupt request disabled for STS\\[CFX1\\]."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CX1IE_A::DISABLED)
    }
    #[doc = "Interrupt request enabled for STS\\[CFX1\\]."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CX1IE_A::ENABLED)
    }
}
#[doc = "Field `CB0IE` reader - Capture B 0 Interrupt Enable"]
pub type CB0IE_R = crate::BitReader<CB0IE_A>;
#[doc = "Capture B 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CB0IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFB0\\]."]
    DISABLED = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFB0\\]."]
    ENABLED = 1,
}
impl From<CB0IE_A> for bool {
    #[inline(always)]
    fn from(variant: CB0IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CB0IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CB0IE_A {
        match self.bits {
            false => CB0IE_A::DISABLED,
            true => CB0IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CB0IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CB0IE_A::ENABLED
    }
}
#[doc = "Field `CB0IE` writer - Capture B 0 Interrupt Enable"]
pub type CB0IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMINTEN_SPEC, CB0IE_A, O>;
impl<'a, const O: u8> CB0IE_W<'a, O> {
    #[doc = "Interrupt request disabled for STS\\[CFB0\\]."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CB0IE_A::DISABLED)
    }
    #[doc = "Interrupt request enabled for STS\\[CFB0\\]."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CB0IE_A::ENABLED)
    }
}
#[doc = "Field `CB1IE` reader - Capture B 1 Interrupt Enable"]
pub type CB1IE_R = crate::BitReader<CB1IE_A>;
#[doc = "Capture B 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CB1IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFB1\\]."]
    DISABLED = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFB1\\]."]
    ENABLED = 1,
}
impl From<CB1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CB1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CB1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CB1IE_A {
        match self.bits {
            false => CB1IE_A::DISABLED,
            true => CB1IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CB1IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CB1IE_A::ENABLED
    }
}
#[doc = "Field `CB1IE` writer - Capture B 1 Interrupt Enable"]
pub type CB1IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMINTEN_SPEC, CB1IE_A, O>;
impl<'a, const O: u8> CB1IE_W<'a, O> {
    #[doc = "Interrupt request disabled for STS\\[CFB1\\]."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CB1IE_A::DISABLED)
    }
    #[doc = "Interrupt request enabled for STS\\[CFB1\\]."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CB1IE_A::ENABLED)
    }
}
#[doc = "Field `CA0IE` reader - Capture A 0 Interrupt Enable"]
pub type CA0IE_R = crate::BitReader<CA0IE_A>;
#[doc = "Capture A 0 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CA0IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFA0\\]."]
    DISABLED = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFA0\\]."]
    ENABLED = 1,
}
impl From<CA0IE_A> for bool {
    #[inline(always)]
    fn from(variant: CA0IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CA0IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CA0IE_A {
        match self.bits {
            false => CA0IE_A::DISABLED,
            true => CA0IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CA0IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CA0IE_A::ENABLED
    }
}
#[doc = "Field `CA0IE` writer - Capture A 0 Interrupt Enable"]
pub type CA0IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMINTEN_SPEC, CA0IE_A, O>;
impl<'a, const O: u8> CA0IE_W<'a, O> {
    #[doc = "Interrupt request disabled for STS\\[CFA0\\]."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CA0IE_A::DISABLED)
    }
    #[doc = "Interrupt request enabled for STS\\[CFA0\\]."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CA0IE_A::ENABLED)
    }
}
#[doc = "Field `CA1IE` reader - Capture A 1 Interrupt Enable"]
pub type CA1IE_R = crate::BitReader<CA1IE_A>;
#[doc = "Capture A 1 Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CA1IE_A {
    #[doc = "0: Interrupt request disabled for STS\\[CFA1\\]."]
    DISABLED = 0,
    #[doc = "1: Interrupt request enabled for STS\\[CFA1\\]."]
    ENABLED = 1,
}
impl From<CA1IE_A> for bool {
    #[inline(always)]
    fn from(variant: CA1IE_A) -> Self {
        variant as u8 != 0
    }
}
impl CA1IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CA1IE_A {
        match self.bits {
            false => CA1IE_A::DISABLED,
            true => CA1IE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CA1IE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CA1IE_A::ENABLED
    }
}
#[doc = "Field `CA1IE` writer - Capture A 1 Interrupt Enable"]
pub type CA1IE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMINTEN_SPEC, CA1IE_A, O>;
impl<'a, const O: u8> CA1IE_W<'a, O> {
    #[doc = "Interrupt request disabled for STS\\[CFA1\\]."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CA1IE_A::DISABLED)
    }
    #[doc = "Interrupt request enabled for STS\\[CFA1\\]."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CA1IE_A::ENABLED)
    }
}
#[doc = "Field `RIE` reader - Reload Interrupt Enable"]
pub type RIE_R = crate::BitReader<RIE_A>;
#[doc = "Reload Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RIE_A {
    #[doc = "0: STS\\[RF\\]
CPU interrupt requests disabled"]
    DISABLED = 0,
    #[doc = "1: STS\\[RF\\]
CPU interrupt requests enabled"]
    ENABLED = 1,
}
impl From<RIE_A> for bool {
    #[inline(always)]
    fn from(variant: RIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RIE_A {
        match self.bits {
            false => RIE_A::DISABLED,
            true => RIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RIE_A::ENABLED
    }
}
#[doc = "Field `RIE` writer - Reload Interrupt Enable"]
pub type RIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMINTEN_SPEC, RIE_A, O>;
impl<'a, const O: u8> RIE_W<'a, O> {
    #[doc = "STS\\[RF\\]
CPU interrupt requests disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RIE_A::DISABLED)
    }
    #[doc = "STS\\[RF\\]
CPU interrupt requests enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RIE_A::ENABLED)
    }
}
#[doc = "Field `REIE` reader - Reload Error Interrupt Enable"]
pub type REIE_R = crate::BitReader<REIE_A>;
#[doc = "Reload Error Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REIE_A {
    #[doc = "0: STS\\[REF\\]
CPU interrupt requests disabled"]
    DISABLED = 0,
    #[doc = "1: STS\\[REF\\]
CPU interrupt requests enabled"]
    ENABLED = 1,
}
impl From<REIE_A> for bool {
    #[inline(always)]
    fn from(variant: REIE_A) -> Self {
        variant as u8 != 0
    }
}
impl REIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REIE_A {
        match self.bits {
            false => REIE_A::DISABLED,
            true => REIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == REIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == REIE_A::ENABLED
    }
}
#[doc = "Field `REIE` writer - Reload Error Interrupt Enable"]
pub type REIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, SMINTEN_SPEC, REIE_A, O>;
impl<'a, const O: u8> REIE_W<'a, O> {
    #[doc = "STS\\[REF\\]
CPU interrupt requests disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(REIE_A::DISABLED)
    }
    #[doc = "STS\\[REF\\]
CPU interrupt requests enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(REIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:5 - Compare Interrupt Enables"]
    #[inline(always)]
    pub fn cmpie(&self) -> CMPIE_R {
        CMPIE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Capture X 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cx0ie(&self) -> CX0IE_R {
        CX0IE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture X 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cx1ie(&self) -> CX1IE_R {
        CX1IE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture B 0 Interrupt Enable"]
    #[inline(always)]
    pub fn cb0ie(&self) -> CB0IE_R {
        CB0IE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture B 1 Interrupt Enable"]
    #[inline(always)]
    pub fn cb1ie(&self) -> CB1IE_R {
        CB1IE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture A 0 Interrupt Enable"]
    #[inline(always)]
    pub fn ca0ie(&self) -> CA0IE_R {
        CA0IE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture A 1 Interrupt Enable"]
    #[inline(always)]
    pub fn ca1ie(&self) -> CA1IE_R {
        CA1IE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reload Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RIE_R {
        RIE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reload Error Interrupt Enable"]
    #[inline(always)]
    pub fn reie(&self) -> REIE_R {
        REIE_R::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Compare Interrupt Enables"]
    #[inline(always)]
    #[must_use]
    pub fn cmpie(&mut self) -> CMPIE_W<0> {
        CMPIE_W::new(self)
    }
    #[doc = "Bit 6 - Capture X 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cx0ie(&mut self) -> CX0IE_W<6> {
        CX0IE_W::new(self)
    }
    #[doc = "Bit 7 - Capture X 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cx1ie(&mut self) -> CX1IE_W<7> {
        CX1IE_W::new(self)
    }
    #[doc = "Bit 8 - Capture B 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cb0ie(&mut self) -> CB0IE_W<8> {
        CB0IE_W::new(self)
    }
    #[doc = "Bit 9 - Capture B 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cb1ie(&mut self) -> CB1IE_W<9> {
        CB1IE_W::new(self)
    }
    #[doc = "Bit 10 - Capture A 0 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ca0ie(&mut self) -> CA0IE_W<10> {
        CA0IE_W::new(self)
    }
    #[doc = "Bit 11 - Capture A 1 Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ca1ie(&mut self) -> CA1IE_W<11> {
        CA1IE_W::new(self)
    }
    #[doc = "Bit 12 - Reload Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn rie(&mut self) -> RIE_W<12> {
        RIE_W::new(self)
    }
    #[doc = "Bit 13 - Reload Error Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn reie(&mut self) -> REIE_W<13> {
        REIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sminten](index.html) module"]
pub struct SMINTEN_SPEC;
impl crate::RegisterSpec for SMINTEN_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [sminten::R](R) reader structure"]
impl crate::Readable for SMINTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sminten::W](W) writer structure"]
impl crate::Writable for SMINTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMINTEN to value 0"]
impl crate::Resettable for SMINTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
