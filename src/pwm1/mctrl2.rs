#[doc = "Register `MCTRL2` reader"]
pub struct R(crate::R<MCTRL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCTRL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCTRL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCTRL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCTRL2` writer"]
pub struct W(crate::W<MCTRL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCTRL2_SPEC>;
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
impl From<crate::W<MCTRL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCTRL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MONPLL` reader - Monitor PLL State"]
pub type MONPLL_R = crate::FieldReader<u8, MONPLL_A>;
#[doc = "Monitor PLL State\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MONPLL_A {
    #[doc = "0: Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software."]
    NOTLOCKED_DO_NOT_MON_PLL = 0,
    #[doc = "1: Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems."]
    NOTLOCKED_MON_PLL = 1,
    #[doc = "2: Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset."]
    LOCKED_DO_NOT_MON_PLL = 2,
    #[doc = "3: Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset."]
    LOCKED_MON_PLL = 3,
}
impl From<MONPLL_A> for u8 {
    #[inline(always)]
    fn from(variant: MONPLL_A) -> Self {
        variant as _
    }
}
impl MONPLL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MONPLL_A {
        match self.bits {
            0 => MONPLL_A::NOTLOCKED_DO_NOT_MON_PLL,
            1 => MONPLL_A::NOTLOCKED_MON_PLL,
            2 => MONPLL_A::LOCKED_DO_NOT_MON_PLL,
            3 => MONPLL_A::LOCKED_MON_PLL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NOTLOCKED_DO_NOT_MON_PLL`"]
    #[inline(always)]
    pub fn is_notlocked_do_not_mon_pll(&self) -> bool {
        *self == MONPLL_A::NOTLOCKED_DO_NOT_MON_PLL
    }
    #[doc = "Checks if the value of the field is `NOTLOCKED_MON_PLL`"]
    #[inline(always)]
    pub fn is_notlocked_mon_pll(&self) -> bool {
        *self == MONPLL_A::NOTLOCKED_MON_PLL
    }
    #[doc = "Checks if the value of the field is `LOCKED_DO_NOT_MON_PLL`"]
    #[inline(always)]
    pub fn is_locked_do_not_mon_pll(&self) -> bool {
        *self == MONPLL_A::LOCKED_DO_NOT_MON_PLL
    }
    #[doc = "Checks if the value of the field is `LOCKED_MON_PLL`"]
    #[inline(always)]
    pub fn is_locked_mon_pll(&self) -> bool {
        *self == MONPLL_A::LOCKED_MON_PLL
    }
}
#[doc = "Field `MONPLL` writer - Monitor PLL State"]
pub type MONPLL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, MCTRL2_SPEC, u8, MONPLL_A, 2, O>;
impl<'a, const O: u8> MONPLL_W<'a, O> {
    #[doc = "Not locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software."]
    #[inline(always)]
    pub fn notlocked_do_not_mon_pll(self) -> &'a mut W {
        self.variant(MONPLL_A::NOTLOCKED_DO_NOT_MON_PLL)
    }
    #[doc = "Not locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems."]
    #[inline(always)]
    pub fn notlocked_mon_pll(self) -> &'a mut W {
        self.variant(MONPLL_A::NOTLOCKED_MON_PLL)
    }
    #[doc = "Locked. Do not monitor PLL operation. Resetting of the fractional delay block in case of PLL losing lock will be controlled by software. These bits are write protected until the next reset."]
    #[inline(always)]
    pub fn locked_do_not_mon_pll(self) -> &'a mut W {
        self.variant(MONPLL_A::LOCKED_DO_NOT_MON_PLL)
    }
    #[doc = "Locked. Monitor PLL operation to automatically disable the fractional delay block when the PLL encounters problems. These bits are write protected until the next reset."]
    #[inline(always)]
    pub fn locked_mon_pll(self) -> &'a mut W {
        self.variant(MONPLL_A::LOCKED_MON_PLL)
    }
}
impl R {
    #[doc = "Bits 0:1 - Monitor PLL State"]
    #[inline(always)]
    pub fn monpll(&self) -> MONPLL_R {
        MONPLL_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Monitor PLL State"]
    #[inline(always)]
    #[must_use]
    pub fn monpll(&mut self) -> MONPLL_W<0> {
        MONPLL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Control 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mctrl2](index.html) module"]
pub struct MCTRL2_SPEC;
impl crate::RegisterSpec for MCTRL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [mctrl2::R](R) reader structure"]
impl crate::Readable for MCTRL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mctrl2::W](W) writer structure"]
impl crate::Writable for MCTRL2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCTRL2 to value 0"]
impl crate::Resettable for MCTRL2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
