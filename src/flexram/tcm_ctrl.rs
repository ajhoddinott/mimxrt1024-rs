#[doc = "Register `TCM_CTRL` reader"]
pub struct R(crate::R<TCM_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TCM_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TCM_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TCM_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TCM_CTRL` writer"]
pub struct W(crate::W<TCM_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TCM_CTRL_SPEC>;
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
impl From<crate::W<TCM_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TCM_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TCM_WWAIT_EN` reader - TCM Write Wait Mode Enable"]
pub type TCM_WWAIT_EN_R = crate::BitReader<TCM_WWAIT_EN_A>;
#[doc = "TCM Write Wait Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCM_WWAIT_EN_A {
    #[doc = "0: TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
    TCM_WWAIT_EN_0 = 0,
    #[doc = "1: TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
    TCM_WWAIT_EN_1 = 1,
}
impl From<TCM_WWAIT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TCM_WWAIT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TCM_WWAIT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCM_WWAIT_EN_A {
        match self.bits {
            false => TCM_WWAIT_EN_A::TCM_WWAIT_EN_0,
            true => TCM_WWAIT_EN_A::TCM_WWAIT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCM_WWAIT_EN_0`"]
    #[inline(always)]
    pub fn is_tcm_wwait_en_0(&self) -> bool {
        *self == TCM_WWAIT_EN_A::TCM_WWAIT_EN_0
    }
    #[doc = "Checks if the value of the field is `TCM_WWAIT_EN_1`"]
    #[inline(always)]
    pub fn is_tcm_wwait_en_1(&self) -> bool {
        *self == TCM_WWAIT_EN_A::TCM_WWAIT_EN_1
    }
}
#[doc = "Field `TCM_WWAIT_EN` writer - TCM Write Wait Mode Enable"]
pub type TCM_WWAIT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TCM_CTRL_SPEC, TCM_WWAIT_EN_A, O>;
impl<'a, const O: u8> TCM_WWAIT_EN_W<'a, O> {
    #[doc = "TCM write fast mode: Write RAM accesses are expected to be finished in 1-cycle."]
    #[inline(always)]
    pub fn tcm_wwait_en_0(self) -> &'a mut W {
        self.variant(TCM_WWAIT_EN_A::TCM_WWAIT_EN_0)
    }
    #[doc = "TCM write wait mode: Write RAM accesses are expected to be finished in 2-cycles."]
    #[inline(always)]
    pub fn tcm_wwait_en_1(self) -> &'a mut W {
        self.variant(TCM_WWAIT_EN_A::TCM_WWAIT_EN_1)
    }
}
#[doc = "Field `TCM_RWAIT_EN` reader - TCM Read Wait Mode Enable"]
pub type TCM_RWAIT_EN_R = crate::BitReader<TCM_RWAIT_EN_A>;
#[doc = "TCM Read Wait Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCM_RWAIT_EN_A {
    #[doc = "0: TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
    TCM_RWAIT_EN_0 = 0,
    #[doc = "1: TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
    TCM_RWAIT_EN_1 = 1,
}
impl From<TCM_RWAIT_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TCM_RWAIT_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TCM_RWAIT_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCM_RWAIT_EN_A {
        match self.bits {
            false => TCM_RWAIT_EN_A::TCM_RWAIT_EN_0,
            true => TCM_RWAIT_EN_A::TCM_RWAIT_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCM_RWAIT_EN_0`"]
    #[inline(always)]
    pub fn is_tcm_rwait_en_0(&self) -> bool {
        *self == TCM_RWAIT_EN_A::TCM_RWAIT_EN_0
    }
    #[doc = "Checks if the value of the field is `TCM_RWAIT_EN_1`"]
    #[inline(always)]
    pub fn is_tcm_rwait_en_1(&self) -> bool {
        *self == TCM_RWAIT_EN_A::TCM_RWAIT_EN_1
    }
}
#[doc = "Field `TCM_RWAIT_EN` writer - TCM Read Wait Mode Enable"]
pub type TCM_RWAIT_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, TCM_CTRL_SPEC, TCM_RWAIT_EN_A, O>;
impl<'a, const O: u8> TCM_RWAIT_EN_W<'a, O> {
    #[doc = "TCM read fast mode: Read RAM accesses are expected to be finished in 1-cycle."]
    #[inline(always)]
    pub fn tcm_rwait_en_0(self) -> &'a mut W {
        self.variant(TCM_RWAIT_EN_A::TCM_RWAIT_EN_0)
    }
    #[doc = "TCM read wait mode: Read RAM accesses are expected to be finished in 2-cycles."]
    #[inline(always)]
    pub fn tcm_rwait_en_1(self) -> &'a mut W {
        self.variant(TCM_RWAIT_EN_A::TCM_RWAIT_EN_1)
    }
}
#[doc = "Field `FORCE_CLK_ON` reader - Force RAM Clock Always On"]
pub type FORCE_CLK_ON_R = crate::BitReader<bool>;
#[doc = "Field `FORCE_CLK_ON` writer - Force RAM Clock Always On"]
pub type FORCE_CLK_ON_W<'a, const O: u8> = crate::BitWriter<'a, u32, TCM_CTRL_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - TCM Write Wait Mode Enable"]
    #[inline(always)]
    pub fn tcm_wwait_en(&self) -> TCM_WWAIT_EN_R {
        TCM_WWAIT_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TCM Read Wait Mode Enable"]
    #[inline(always)]
    pub fn tcm_rwait_en(&self) -> TCM_RWAIT_EN_R {
        TCM_RWAIT_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Force RAM Clock Always On"]
    #[inline(always)]
    pub fn force_clk_on(&self) -> FORCE_CLK_ON_R {
        FORCE_CLK_ON_R::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TCM Write Wait Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcm_wwait_en(&mut self) -> TCM_WWAIT_EN_W<0> {
        TCM_WWAIT_EN_W::new(self)
    }
    #[doc = "Bit 1 - TCM Read Wait Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcm_rwait_en(&mut self) -> TCM_RWAIT_EN_W<1> {
        TCM_RWAIT_EN_W::new(self)
    }
    #[doc = "Bit 2 - Force RAM Clock Always On"]
    #[inline(always)]
    #[must_use]
    pub fn force_clk_on(&mut self) -> FORCE_CLK_ON_W<2> {
        FORCE_CLK_ON_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TCM CRTL Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tcm_ctrl](index.html) module"]
pub struct TCM_CTRL_SPEC;
impl crate::RegisterSpec for TCM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tcm_ctrl::R](R) reader structure"]
impl crate::Readable for TCM_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tcm_ctrl::W](W) writer structure"]
impl crate::Writable for TCM_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TCM_CTRL to value 0"]
impl crate::Resettable for TCM_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
