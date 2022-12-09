#[doc = "Register `CACRR` reader"]
pub struct R(crate::R<CACRR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACRR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACRR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACRR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACRR` writer"]
pub struct W(crate::W<CACRR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACRR_SPEC>;
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
impl From<crate::W<CACRR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACRR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARM_PODF` reader - Divider for Arm clock root"]
pub type ARM_PODF_R = crate::FieldReader<u8, ARM_PODF_A>;
#[doc = "Divider for Arm clock root\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARM_PODF_A {
    #[doc = "0: divide by 1"]
    ARM_PODF_0 = 0,
    #[doc = "1: divide by 2"]
    ARM_PODF_1 = 1,
    #[doc = "2: divide by 3"]
    ARM_PODF_2 = 2,
    #[doc = "3: divide by 4"]
    ARM_PODF_3 = 3,
    #[doc = "4: divide by 5"]
    ARM_PODF_4 = 4,
    #[doc = "5: divide by 6"]
    ARM_PODF_5 = 5,
    #[doc = "6: divide by 7"]
    ARM_PODF_6 = 6,
    #[doc = "7: divide by 8"]
    ARM_PODF_7 = 7,
}
impl From<ARM_PODF_A> for u8 {
    #[inline(always)]
    fn from(variant: ARM_PODF_A) -> Self {
        variant as _
    }
}
impl ARM_PODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARM_PODF_A {
        match self.bits {
            0 => ARM_PODF_A::ARM_PODF_0,
            1 => ARM_PODF_A::ARM_PODF_1,
            2 => ARM_PODF_A::ARM_PODF_2,
            3 => ARM_PODF_A::ARM_PODF_3,
            4 => ARM_PODF_A::ARM_PODF_4,
            5 => ARM_PODF_A::ARM_PODF_5,
            6 => ARM_PODF_A::ARM_PODF_6,
            7 => ARM_PODF_A::ARM_PODF_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_0`"]
    #[inline(always)]
    pub fn is_arm_podf_0(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_0
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_1`"]
    #[inline(always)]
    pub fn is_arm_podf_1(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_1
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_2`"]
    #[inline(always)]
    pub fn is_arm_podf_2(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_2
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_3`"]
    #[inline(always)]
    pub fn is_arm_podf_3(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_3
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_4`"]
    #[inline(always)]
    pub fn is_arm_podf_4(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_4
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_5`"]
    #[inline(always)]
    pub fn is_arm_podf_5(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_5
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_6`"]
    #[inline(always)]
    pub fn is_arm_podf_6(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_6
    }
    #[doc = "Checks if the value of the field is `ARM_PODF_7`"]
    #[inline(always)]
    pub fn is_arm_podf_7(&self) -> bool {
        *self == ARM_PODF_A::ARM_PODF_7
    }
}
#[doc = "Field `ARM_PODF` writer - Divider for Arm clock root"]
pub type ARM_PODF_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, CACRR_SPEC, u8, ARM_PODF_A, 3, O>;
impl<'a, const O: u8> ARM_PODF_W<'a, O> {
    #[doc = "divide by 1"]
    #[inline(always)]
    pub fn arm_podf_0(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_0)
    }
    #[doc = "divide by 2"]
    #[inline(always)]
    pub fn arm_podf_1(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_1)
    }
    #[doc = "divide by 3"]
    #[inline(always)]
    pub fn arm_podf_2(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_2)
    }
    #[doc = "divide by 4"]
    #[inline(always)]
    pub fn arm_podf_3(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_3)
    }
    #[doc = "divide by 5"]
    #[inline(always)]
    pub fn arm_podf_4(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_4)
    }
    #[doc = "divide by 6"]
    #[inline(always)]
    pub fn arm_podf_5(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_5)
    }
    #[doc = "divide by 7"]
    #[inline(always)]
    pub fn arm_podf_6(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_6)
    }
    #[doc = "divide by 8"]
    #[inline(always)]
    pub fn arm_podf_7(self) -> &'a mut W {
        self.variant(ARM_PODF_A::ARM_PODF_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Divider for Arm clock root"]
    #[inline(always)]
    pub fn arm_podf(&self) -> ARM_PODF_R {
        ARM_PODF_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Divider for Arm clock root"]
    #[inline(always)]
    #[must_use]
    pub fn arm_podf(&mut self) -> ARM_PODF_W<0> {
        ARM_PODF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CCM Arm Clock Root Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cacrr](index.html) module"]
pub struct CACRR_SPEC;
impl crate::RegisterSpec for CACRR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cacrr::R](R) reader structure"]
impl crate::Readable for CACRR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cacrr::W](W) writer structure"]
impl crate::Writable for CACRR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACRR to value 0"]
impl crate::Resettable for CACRR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
