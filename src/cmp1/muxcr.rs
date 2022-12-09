#[doc = "Register `MUXCR` reader"]
pub struct R(crate::R<MUXCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MUXCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MUXCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MUXCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MUXCR` writer"]
pub struct W(crate::W<MUXCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MUXCR_SPEC>;
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
impl From<crate::W<MUXCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MUXCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSEL` reader - Minus Input Mux Control"]
pub type MSEL_R = crate::FieldReader<u8, MSEL_A>;
#[doc = "Minus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MSEL_A {
    #[doc = "0: IN0"]
    MSEL_0 = 0,
    #[doc = "1: IN1"]
    MSEL_1 = 1,
    #[doc = "2: IN2"]
    MSEL_2 = 2,
    #[doc = "3: IN3"]
    MSEL_3 = 3,
    #[doc = "4: IN4"]
    MSEL_4 = 4,
    #[doc = "5: IN5"]
    MSEL_5 = 5,
    #[doc = "6: IN6"]
    MSEL_6 = 6,
    #[doc = "7: IN7"]
    MSEL_7 = 7,
}
impl From<MSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: MSEL_A) -> Self {
        variant as _
    }
}
impl MSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSEL_A {
        match self.bits {
            0 => MSEL_A::MSEL_0,
            1 => MSEL_A::MSEL_1,
            2 => MSEL_A::MSEL_2,
            3 => MSEL_A::MSEL_3,
            4 => MSEL_A::MSEL_4,
            5 => MSEL_A::MSEL_5,
            6 => MSEL_A::MSEL_6,
            7 => MSEL_A::MSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MSEL_0`"]
    #[inline(always)]
    pub fn is_msel_0(&self) -> bool {
        *self == MSEL_A::MSEL_0
    }
    #[doc = "Checks if the value of the field is `MSEL_1`"]
    #[inline(always)]
    pub fn is_msel_1(&self) -> bool {
        *self == MSEL_A::MSEL_1
    }
    #[doc = "Checks if the value of the field is `MSEL_2`"]
    #[inline(always)]
    pub fn is_msel_2(&self) -> bool {
        *self == MSEL_A::MSEL_2
    }
    #[doc = "Checks if the value of the field is `MSEL_3`"]
    #[inline(always)]
    pub fn is_msel_3(&self) -> bool {
        *self == MSEL_A::MSEL_3
    }
    #[doc = "Checks if the value of the field is `MSEL_4`"]
    #[inline(always)]
    pub fn is_msel_4(&self) -> bool {
        *self == MSEL_A::MSEL_4
    }
    #[doc = "Checks if the value of the field is `MSEL_5`"]
    #[inline(always)]
    pub fn is_msel_5(&self) -> bool {
        *self == MSEL_A::MSEL_5
    }
    #[doc = "Checks if the value of the field is `MSEL_6`"]
    #[inline(always)]
    pub fn is_msel_6(&self) -> bool {
        *self == MSEL_A::MSEL_6
    }
    #[doc = "Checks if the value of the field is `MSEL_7`"]
    #[inline(always)]
    pub fn is_msel_7(&self) -> bool {
        *self == MSEL_A::MSEL_7
    }
}
#[doc = "Field `MSEL` writer - Minus Input Mux Control"]
pub type MSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MUXCR_SPEC, u8, MSEL_A, 3, O>;
impl<'a, const O: u8> MSEL_W<'a, O> {
    #[doc = "IN0"]
    #[inline(always)]
    pub fn msel_0(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_0)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn msel_1(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_1)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn msel_2(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_2)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn msel_3(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_3)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn msel_4(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_4)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn msel_5(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_5)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn msel_6(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_6)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn msel_7(self) -> &'a mut W {
        self.variant(MSEL_A::MSEL_7)
    }
}
#[doc = "Field `PSEL` reader - Plus Input Mux Control"]
pub type PSEL_R = crate::FieldReader<u8, PSEL_A>;
#[doc = "Plus Input Mux Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSEL_A {
    #[doc = "0: IN0"]
    PSEL_0 = 0,
    #[doc = "1: IN1"]
    PSEL_1 = 1,
    #[doc = "2: IN2"]
    PSEL_2 = 2,
    #[doc = "3: IN3"]
    PSEL_3 = 3,
    #[doc = "4: IN4"]
    PSEL_4 = 4,
    #[doc = "5: IN5"]
    PSEL_5 = 5,
    #[doc = "6: IN6"]
    PSEL_6 = 6,
    #[doc = "7: IN7"]
    PSEL_7 = 7,
}
impl From<PSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PSEL_A) -> Self {
        variant as _
    }
}
impl PSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PSEL_A {
        match self.bits {
            0 => PSEL_A::PSEL_0,
            1 => PSEL_A::PSEL_1,
            2 => PSEL_A::PSEL_2,
            3 => PSEL_A::PSEL_3,
            4 => PSEL_A::PSEL_4,
            5 => PSEL_A::PSEL_5,
            6 => PSEL_A::PSEL_6,
            7 => PSEL_A::PSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PSEL_0`"]
    #[inline(always)]
    pub fn is_psel_0(&self) -> bool {
        *self == PSEL_A::PSEL_0
    }
    #[doc = "Checks if the value of the field is `PSEL_1`"]
    #[inline(always)]
    pub fn is_psel_1(&self) -> bool {
        *self == PSEL_A::PSEL_1
    }
    #[doc = "Checks if the value of the field is `PSEL_2`"]
    #[inline(always)]
    pub fn is_psel_2(&self) -> bool {
        *self == PSEL_A::PSEL_2
    }
    #[doc = "Checks if the value of the field is `PSEL_3`"]
    #[inline(always)]
    pub fn is_psel_3(&self) -> bool {
        *self == PSEL_A::PSEL_3
    }
    #[doc = "Checks if the value of the field is `PSEL_4`"]
    #[inline(always)]
    pub fn is_psel_4(&self) -> bool {
        *self == PSEL_A::PSEL_4
    }
    #[doc = "Checks if the value of the field is `PSEL_5`"]
    #[inline(always)]
    pub fn is_psel_5(&self) -> bool {
        *self == PSEL_A::PSEL_5
    }
    #[doc = "Checks if the value of the field is `PSEL_6`"]
    #[inline(always)]
    pub fn is_psel_6(&self) -> bool {
        *self == PSEL_A::PSEL_6
    }
    #[doc = "Checks if the value of the field is `PSEL_7`"]
    #[inline(always)]
    pub fn is_psel_7(&self) -> bool {
        *self == PSEL_A::PSEL_7
    }
}
#[doc = "Field `PSEL` writer - Plus Input Mux Control"]
pub type PSEL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, MUXCR_SPEC, u8, PSEL_A, 3, O>;
impl<'a, const O: u8> PSEL_W<'a, O> {
    #[doc = "IN0"]
    #[inline(always)]
    pub fn psel_0(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_0)
    }
    #[doc = "IN1"]
    #[inline(always)]
    pub fn psel_1(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_1)
    }
    #[doc = "IN2"]
    #[inline(always)]
    pub fn psel_2(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_2)
    }
    #[doc = "IN3"]
    #[inline(always)]
    pub fn psel_3(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_3)
    }
    #[doc = "IN4"]
    #[inline(always)]
    pub fn psel_4(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_4)
    }
    #[doc = "IN5"]
    #[inline(always)]
    pub fn psel_5(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_5)
    }
    #[doc = "IN6"]
    #[inline(always)]
    pub fn psel_6(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_6)
    }
    #[doc = "IN7"]
    #[inline(always)]
    pub fn psel_7(self) -> &'a mut W {
        self.variant(PSEL_A::PSEL_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    pub fn msel(&self) -> MSEL_R {
        MSEL_R::new(self.bits & 7)
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    pub fn psel(&self) -> PSEL_R {
        PSEL_R::new((self.bits >> 3) & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Minus Input Mux Control"]
    #[inline(always)]
    #[must_use]
    pub fn msel(&mut self) -> MSEL_W<0> {
        MSEL_W::new(self)
    }
    #[doc = "Bits 3:5 - Plus Input Mux Control"]
    #[inline(always)]
    #[must_use]
    pub fn psel(&mut self) -> PSEL_W<3> {
        PSEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MUX Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [muxcr](index.html) module"]
pub struct MUXCR_SPEC;
impl crate::RegisterSpec for MUXCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [muxcr::R](R) reader structure"]
impl crate::Readable for MUXCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [muxcr::W](W) writer structure"]
impl crate::Writable for MUXCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MUXCR to value 0"]
impl crate::Resettable for MUXCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
