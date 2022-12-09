#[doc = "Register `INTEN` reader"]
pub struct R(crate::R<INTEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTEN` writer"]
pub struct W(crate::W<INTEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTEN_SPEC>;
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
impl From<crate::W<INTEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IPCMDDONEEN` reader - IP command done interrupt enable"]
pub type IPCMDDONEEN_R = crate::BitReader<IPCMDDONEEN_A>;
#[doc = "IP command done interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCMDDONEEN_A {
    #[doc = "0: Interrupt is disabled"]
    IPCMDDONEEN_0 = 0,
    #[doc = "1: Interrupt is enabled"]
    IPCMDDONEEN_1 = 1,
}
impl From<IPCMDDONEEN_A> for bool {
    #[inline(always)]
    fn from(variant: IPCMDDONEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPCMDDONEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCMDDONEEN_A {
        match self.bits {
            false => IPCMDDONEEN_A::IPCMDDONEEN_0,
            true => IPCMDDONEEN_A::IPCMDDONEEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPCMDDONEEN_0`"]
    #[inline(always)]
    pub fn is_ipcmddoneen_0(&self) -> bool {
        *self == IPCMDDONEEN_A::IPCMDDONEEN_0
    }
    #[doc = "Checks if the value of the field is `IPCMDDONEEN_1`"]
    #[inline(always)]
    pub fn is_ipcmddoneen_1(&self) -> bool {
        *self == IPCMDDONEEN_A::IPCMDDONEEN_1
    }
}
#[doc = "Field `IPCMDDONEEN` writer - IP command done interrupt enable"]
pub type IPCMDDONEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, IPCMDDONEEN_A, O>;
impl<'a, const O: u8> IPCMDDONEEN_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn ipcmddoneen_0(self) -> &'a mut W {
        self.variant(IPCMDDONEEN_A::IPCMDDONEEN_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn ipcmddoneen_1(self) -> &'a mut W {
        self.variant(IPCMDDONEEN_A::IPCMDDONEEN_1)
    }
}
#[doc = "Field `IPCMDERREN` reader - IP command error interrupt enable"]
pub type IPCMDERREN_R = crate::BitReader<IPCMDERREN_A>;
#[doc = "IP command error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IPCMDERREN_A {
    #[doc = "0: Interrupt is disabled"]
    IPCMDERREN_0 = 0,
    #[doc = "1: Interrupt is enabled"]
    IPCMDERREN_1 = 1,
}
impl From<IPCMDERREN_A> for bool {
    #[inline(always)]
    fn from(variant: IPCMDERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IPCMDERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IPCMDERREN_A {
        match self.bits {
            false => IPCMDERREN_A::IPCMDERREN_0,
            true => IPCMDERREN_A::IPCMDERREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `IPCMDERREN_0`"]
    #[inline(always)]
    pub fn is_ipcmderren_0(&self) -> bool {
        *self == IPCMDERREN_A::IPCMDERREN_0
    }
    #[doc = "Checks if the value of the field is `IPCMDERREN_1`"]
    #[inline(always)]
    pub fn is_ipcmderren_1(&self) -> bool {
        *self == IPCMDERREN_A::IPCMDERREN_1
    }
}
#[doc = "Field `IPCMDERREN` writer - IP command error interrupt enable"]
pub type IPCMDERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, IPCMDERREN_A, O>;
impl<'a, const O: u8> IPCMDERREN_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn ipcmderren_0(self) -> &'a mut W {
        self.variant(IPCMDERREN_A::IPCMDERREN_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn ipcmderren_1(self) -> &'a mut W {
        self.variant(IPCMDERREN_A::IPCMDERREN_1)
    }
}
#[doc = "Field `AXICMDERREN` reader - AXI command error interrupt enable"]
pub type AXICMDERREN_R = crate::BitReader<AXICMDERREN_A>;
#[doc = "AXI command error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXICMDERREN_A {
    #[doc = "0: Interrupt is disabled"]
    AXICMDERREN_0 = 0,
    #[doc = "1: Interrupt is enabled"]
    AXICMDERREN_1 = 1,
}
impl From<AXICMDERREN_A> for bool {
    #[inline(always)]
    fn from(variant: AXICMDERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl AXICMDERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXICMDERREN_A {
        match self.bits {
            false => AXICMDERREN_A::AXICMDERREN_0,
            true => AXICMDERREN_A::AXICMDERREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXICMDERREN_0`"]
    #[inline(always)]
    pub fn is_axicmderren_0(&self) -> bool {
        *self == AXICMDERREN_A::AXICMDERREN_0
    }
    #[doc = "Checks if the value of the field is `AXICMDERREN_1`"]
    #[inline(always)]
    pub fn is_axicmderren_1(&self) -> bool {
        *self == AXICMDERREN_A::AXICMDERREN_1
    }
}
#[doc = "Field `AXICMDERREN` writer - AXI command error interrupt enable"]
pub type AXICMDERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, AXICMDERREN_A, O>;
impl<'a, const O: u8> AXICMDERREN_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn axicmderren_0(self) -> &'a mut W {
        self.variant(AXICMDERREN_A::AXICMDERREN_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn axicmderren_1(self) -> &'a mut W {
        self.variant(AXICMDERREN_A::AXICMDERREN_1)
    }
}
#[doc = "Field `AXIBUSERREN` reader - AXI bus error interrupt enable"]
pub type AXIBUSERREN_R = crate::BitReader<AXIBUSERREN_A>;
#[doc = "AXI bus error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AXIBUSERREN_A {
    #[doc = "0: Interrupt is disabled"]
    AXIBUSERREN_0 = 0,
    #[doc = "1: Interrupt is enabled"]
    AXIBUSERREN_1 = 1,
}
impl From<AXIBUSERREN_A> for bool {
    #[inline(always)]
    fn from(variant: AXIBUSERREN_A) -> Self {
        variant as u8 != 0
    }
}
impl AXIBUSERREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AXIBUSERREN_A {
        match self.bits {
            false => AXIBUSERREN_A::AXIBUSERREN_0,
            true => AXIBUSERREN_A::AXIBUSERREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `AXIBUSERREN_0`"]
    #[inline(always)]
    pub fn is_axibuserren_0(&self) -> bool {
        *self == AXIBUSERREN_A::AXIBUSERREN_0
    }
    #[doc = "Checks if the value of the field is `AXIBUSERREN_1`"]
    #[inline(always)]
    pub fn is_axibuserren_1(&self) -> bool {
        *self == AXIBUSERREN_A::AXIBUSERREN_1
    }
}
#[doc = "Field `AXIBUSERREN` writer - AXI bus error interrupt enable"]
pub type AXIBUSERREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, AXIBUSERREN_A, O>;
impl<'a, const O: u8> AXIBUSERREN_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn axibuserren_0(self) -> &'a mut W {
        self.variant(AXIBUSERREN_A::AXIBUSERREN_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn axibuserren_1(self) -> &'a mut W {
        self.variant(AXIBUSERREN_A::AXIBUSERREN_1)
    }
}
#[doc = "Field `NDPAGEENDEN` reader - NAND page end interrupt enable"]
pub type NDPAGEENDEN_R = crate::BitReader<NDPAGEENDEN_A>;
#[doc = "NAND page end interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDPAGEENDEN_A {
    #[doc = "0: Interrupt is disabled"]
    NDPAGEENDEN_0 = 0,
    #[doc = "1: Interrupt is enabled"]
    NDPAGEENDEN_1 = 1,
}
impl From<NDPAGEENDEN_A> for bool {
    #[inline(always)]
    fn from(variant: NDPAGEENDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NDPAGEENDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDPAGEENDEN_A {
        match self.bits {
            false => NDPAGEENDEN_A::NDPAGEENDEN_0,
            true => NDPAGEENDEN_A::NDPAGEENDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDPAGEENDEN_0`"]
    #[inline(always)]
    pub fn is_ndpageenden_0(&self) -> bool {
        *self == NDPAGEENDEN_A::NDPAGEENDEN_0
    }
    #[doc = "Checks if the value of the field is `NDPAGEENDEN_1`"]
    #[inline(always)]
    pub fn is_ndpageenden_1(&self) -> bool {
        *self == NDPAGEENDEN_A::NDPAGEENDEN_1
    }
}
#[doc = "Field `NDPAGEENDEN` writer - NAND page end interrupt enable"]
pub type NDPAGEENDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, NDPAGEENDEN_A, O>;
impl<'a, const O: u8> NDPAGEENDEN_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn ndpageenden_0(self) -> &'a mut W {
        self.variant(NDPAGEENDEN_A::NDPAGEENDEN_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn ndpageenden_1(self) -> &'a mut W {
        self.variant(NDPAGEENDEN_A::NDPAGEENDEN_1)
    }
}
#[doc = "Field `NDNOPENDEN` reader - NAND no pending AXI access interrupt enable"]
pub type NDNOPENDEN_R = crate::BitReader<NDNOPENDEN_A>;
#[doc = "NAND no pending AXI access interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NDNOPENDEN_A {
    #[doc = "0: Interrupt is disabled"]
    NDNOPENDEN_0 = 0,
    #[doc = "1: Interrupt is enabled"]
    NDNOPENDEN_1 = 1,
}
impl From<NDNOPENDEN_A> for bool {
    #[inline(always)]
    fn from(variant: NDNOPENDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl NDNOPENDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NDNOPENDEN_A {
        match self.bits {
            false => NDNOPENDEN_A::NDNOPENDEN_0,
            true => NDNOPENDEN_A::NDNOPENDEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `NDNOPENDEN_0`"]
    #[inline(always)]
    pub fn is_ndnopenden_0(&self) -> bool {
        *self == NDNOPENDEN_A::NDNOPENDEN_0
    }
    #[doc = "Checks if the value of the field is `NDNOPENDEN_1`"]
    #[inline(always)]
    pub fn is_ndnopenden_1(&self) -> bool {
        *self == NDNOPENDEN_A::NDNOPENDEN_1
    }
}
#[doc = "Field `NDNOPENDEN` writer - NAND no pending AXI access interrupt enable"]
pub type NDNOPENDEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, INTEN_SPEC, NDNOPENDEN_A, O>;
impl<'a, const O: u8> NDNOPENDEN_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn ndnopenden_0(self) -> &'a mut W {
        self.variant(NDNOPENDEN_A::NDNOPENDEN_0)
    }
    #[doc = "Interrupt is enabled"]
    #[inline(always)]
    pub fn ndnopenden_1(self) -> &'a mut W {
        self.variant(NDNOPENDEN_A::NDNOPENDEN_1)
    }
}
impl R {
    #[doc = "Bit 0 - IP command done interrupt enable"]
    #[inline(always)]
    pub fn ipcmddoneen(&self) -> IPCMDDONEEN_R {
        IPCMDDONEEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IP command error interrupt enable"]
    #[inline(always)]
    pub fn ipcmderren(&self) -> IPCMDERREN_R {
        IPCMDERREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AXI command error interrupt enable"]
    #[inline(always)]
    pub fn axicmderren(&self) -> AXICMDERREN_R {
        AXICMDERREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AXI bus error interrupt enable"]
    #[inline(always)]
    pub fn axibuserren(&self) -> AXIBUSERREN_R {
        AXIBUSERREN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAND page end interrupt enable"]
    #[inline(always)]
    pub fn ndpageenden(&self) -> NDPAGEENDEN_R {
        NDPAGEENDEN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NAND no pending AXI access interrupt enable"]
    #[inline(always)]
    pub fn ndnopenden(&self) -> NDNOPENDEN_R {
        NDNOPENDEN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IP command done interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipcmddoneen(&mut self) -> IPCMDDONEEN_W<0> {
        IPCMDDONEEN_W::new(self)
    }
    #[doc = "Bit 1 - IP command error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ipcmderren(&mut self) -> IPCMDERREN_W<1> {
        IPCMDERREN_W::new(self)
    }
    #[doc = "Bit 2 - AXI command error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn axicmderren(&mut self) -> AXICMDERREN_W<2> {
        AXICMDERREN_W::new(self)
    }
    #[doc = "Bit 3 - AXI bus error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn axibuserren(&mut self) -> AXIBUSERREN_W<3> {
        AXIBUSERREN_W::new(self)
    }
    #[doc = "Bit 4 - NAND page end interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ndpageenden(&mut self) -> NDPAGEENDEN_W<4> {
        NDPAGEENDEN_W::new(self)
    }
    #[doc = "Bit 5 - NAND no pending AXI access interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ndnopenden(&mut self) -> NDNOPENDEN_W<5> {
        NDNOPENDEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Enable Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inten](index.html) module"]
pub struct INTEN_SPEC;
impl crate::RegisterSpec for INTEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inten::R](R) reader structure"]
impl crate::Readable for INTEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inten::W](W) writer structure"]
impl crate::Writable for INTEN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for INTEN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
