#[doc = "Register `CTRL` reader"]
pub struct R(crate::R<CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL` writer"]
pub struct W(crate::W<CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL_SPEC>;
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
impl From<crate::W<CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLEXEN` reader - FlexIO Enable"]
pub type FLEXEN_R = crate::BitReader<FLEXEN_A>;
#[doc = "FlexIO Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLEXEN_A {
    #[doc = "0: FlexIO module is disabled."]
    DISABLE = 0,
    #[doc = "1: FlexIO module is enabled."]
    ENABLE = 1,
}
impl From<FLEXEN_A> for bool {
    #[inline(always)]
    fn from(variant: FLEXEN_A) -> Self {
        variant as u8 != 0
    }
}
impl FLEXEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLEXEN_A {
        match self.bits {
            false => FLEXEN_A::DISABLE,
            true => FLEXEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == FLEXEN_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == FLEXEN_A::ENABLE
    }
}
#[doc = "Field `FLEXEN` writer - FlexIO Enable"]
pub type FLEXEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, FLEXEN_A, O>;
impl<'a, const O: u8> FLEXEN_W<'a, O> {
    #[doc = "FlexIO module is disabled."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(FLEXEN_A::DISABLE)
    }
    #[doc = "FlexIO module is enabled."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(FLEXEN_A::ENABLE)
    }
}
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SWRST_R = crate::BitReader<SWRST_A>;
#[doc = "Software Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SWRST_A {
    #[doc = "0: Software reset is disabled"]
    DISABLE = 0,
    #[doc = "1: Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    ENABLE = 1,
}
impl From<SWRST_A> for bool {
    #[inline(always)]
    fn from(variant: SWRST_A) -> Self {
        variant as u8 != 0
    }
}
impl SWRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SWRST_A {
        match self.bits {
            false => SWRST_A::DISABLE,
            true => SWRST_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == SWRST_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SWRST_A::ENABLE
    }
}
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, SWRST_A, O>;
impl<'a, const O: u8> SWRST_W<'a, O> {
    #[doc = "Software reset is disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(SWRST_A::DISABLE)
    }
    #[doc = "Software reset is enabled, all FlexIO registers except the Control Register are reset."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SWRST_A::ENABLE)
    }
}
#[doc = "Field `FASTACC` reader - Fast Access"]
pub type FASTACC_R = crate::BitReader<FASTACC_A>;
#[doc = "Fast Access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FASTACC_A {
    #[doc = "0: Configures for normal register accesses to FlexIO"]
    NORMAL = 0,
    #[doc = "1: Configures for fast register accesses to FlexIO"]
    FAST = 1,
}
impl From<FASTACC_A> for bool {
    #[inline(always)]
    fn from(variant: FASTACC_A) -> Self {
        variant as u8 != 0
    }
}
impl FASTACC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FASTACC_A {
        match self.bits {
            false => FASTACC_A::NORMAL,
            true => FASTACC_A::FAST,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == FASTACC_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `FAST`"]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == FASTACC_A::FAST
    }
}
#[doc = "Field `FASTACC` writer - Fast Access"]
pub type FASTACC_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, FASTACC_A, O>;
impl<'a, const O: u8> FASTACC_W<'a, O> {
    #[doc = "Configures for normal register accesses to FlexIO"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(FASTACC_A::NORMAL)
    }
    #[doc = "Configures for fast register accesses to FlexIO"]
    #[inline(always)]
    pub fn fast(self) -> &'a mut W {
        self.variant(FASTACC_A::FAST)
    }
}
#[doc = "Field `DBGE` reader - Debug Enable"]
pub type DBGE_R = crate::BitReader<DBGE_A>;
#[doc = "Debug Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DBGE_A {
    #[doc = "0: FlexIO is disabled in debug modes."]
    DISABLE = 0,
    #[doc = "1: FlexIO is enabled in debug modes"]
    EMABLE = 1,
}
impl From<DBGE_A> for bool {
    #[inline(always)]
    fn from(variant: DBGE_A) -> Self {
        variant as u8 != 0
    }
}
impl DBGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DBGE_A {
        match self.bits {
            false => DBGE_A::DISABLE,
            true => DBGE_A::EMABLE,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DBGE_A::DISABLE
    }
    #[doc = "Checks if the value of the field is `EMABLE`"]
    #[inline(always)]
    pub fn is_emable(&self) -> bool {
        *self == DBGE_A::EMABLE
    }
}
#[doc = "Field `DBGE` writer - Debug Enable"]
pub type DBGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DBGE_A, O>;
impl<'a, const O: u8> DBGE_W<'a, O> {
    #[doc = "FlexIO is disabled in debug modes."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DBGE_A::DISABLE)
    }
    #[doc = "FlexIO is enabled in debug modes"]
    #[inline(always)]
    pub fn emable(self) -> &'a mut W {
        self.variant(DBGE_A::EMABLE)
    }
}
#[doc = "Field `DOZEN` reader - Doze Enable"]
pub type DOZEN_R = crate::BitReader<DOZEN_A>;
#[doc = "Doze Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DOZEN_A {
    #[doc = "0: FlexIO enabled in Doze modes."]
    ENABLE = 0,
    #[doc = "1: FlexIO disabled in Doze modes."]
    DISABLE = 1,
}
impl From<DOZEN_A> for bool {
    #[inline(always)]
    fn from(variant: DOZEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DOZEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DOZEN_A {
        match self.bits {
            false => DOZEN_A::ENABLE,
            true => DOZEN_A::DISABLE,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DOZEN_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DOZEN_A::DISABLE
    }
}
#[doc = "Field `DOZEN` writer - Doze Enable"]
pub type DOZEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, CTRL_SPEC, DOZEN_A, O>;
impl<'a, const O: u8> DOZEN_W<'a, O> {
    #[doc = "FlexIO enabled in Doze modes."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(DOZEN_A::ENABLE)
    }
    #[doc = "FlexIO disabled in Doze modes."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(DOZEN_A::DISABLE)
    }
}
impl R {
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline(always)]
    pub fn flexen(&self) -> FLEXEN_R {
        FLEXEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SWRST_R {
        SWRST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline(always)]
    pub fn fastacc(&self) -> FASTACC_R {
        FASTACC_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline(always)]
    pub fn dbge(&self) -> DBGE_R {
        DBGE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline(always)]
    pub fn dozen(&self) -> DOZEN_R {
        DOZEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - FlexIO Enable"]
    #[inline(always)]
    #[must_use]
    pub fn flexen(&mut self) -> FLEXEN_W<0> {
        FLEXEN_W::new(self)
    }
    #[doc = "Bit 1 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<1> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 2 - Fast Access"]
    #[inline(always)]
    #[must_use]
    pub fn fastacc(&mut self) -> FASTACC_W<2> {
        FASTACC_W::new(self)
    }
    #[doc = "Bit 30 - Debug Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dbge(&mut self) -> DBGE_W<30> {
        DBGE_W::new(self)
    }
    #[doc = "Bit 31 - Doze Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dozen(&mut self) -> DOZEN_W<31> {
        DOZEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "FlexIO Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl](index.html) module"]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ctrl::R](R) reader structure"]
impl crate::Readable for CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl::W](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
