#[doc = "Register `FRINDEX` reader"]
pub struct R(crate::R<FRINDEX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FRINDEX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FRINDEX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FRINDEX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FRINDEX` writer"]
pub struct W(crate::W<FRINDEX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FRINDEX_SPEC>;
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
impl From<crate::W<FRINDEX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FRINDEX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FRINDEX` reader - Frame Index"]
pub type FRINDEX_R = crate::FieldReader<u16, FRINDEX_A>;
#[doc = "Frame Index\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum FRINDEX_A {
    #[doc = "0: (1024) 12"]
    FRINDEX_0 = 0,
    #[doc = "1: (512) 11"]
    FRINDEX_1 = 1,
    #[doc = "2: (256) 10"]
    FRINDEX_2 = 2,
    #[doc = "3: (128) 9"]
    FRINDEX_3 = 3,
    #[doc = "4: (64) 8"]
    FRINDEX_4 = 4,
    #[doc = "5: (32) 7"]
    FRINDEX_5 = 5,
    #[doc = "6: (16) 6"]
    FRINDEX_6 = 6,
    #[doc = "7: (8) 5"]
    FRINDEX_7 = 7,
}
impl From<FRINDEX_A> for u16 {
    #[inline(always)]
    fn from(variant: FRINDEX_A) -> Self {
        variant as _
    }
}
impl FRINDEX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FRINDEX_A> {
        match self.bits {
            0 => Some(FRINDEX_A::FRINDEX_0),
            1 => Some(FRINDEX_A::FRINDEX_1),
            2 => Some(FRINDEX_A::FRINDEX_2),
            3 => Some(FRINDEX_A::FRINDEX_3),
            4 => Some(FRINDEX_A::FRINDEX_4),
            5 => Some(FRINDEX_A::FRINDEX_5),
            6 => Some(FRINDEX_A::FRINDEX_6),
            7 => Some(FRINDEX_A::FRINDEX_7),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FRINDEX_0`"]
    #[inline(always)]
    pub fn is_frindex_0(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_0
    }
    #[doc = "Checks if the value of the field is `FRINDEX_1`"]
    #[inline(always)]
    pub fn is_frindex_1(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_1
    }
    #[doc = "Checks if the value of the field is `FRINDEX_2`"]
    #[inline(always)]
    pub fn is_frindex_2(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_2
    }
    #[doc = "Checks if the value of the field is `FRINDEX_3`"]
    #[inline(always)]
    pub fn is_frindex_3(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_3
    }
    #[doc = "Checks if the value of the field is `FRINDEX_4`"]
    #[inline(always)]
    pub fn is_frindex_4(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_4
    }
    #[doc = "Checks if the value of the field is `FRINDEX_5`"]
    #[inline(always)]
    pub fn is_frindex_5(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_5
    }
    #[doc = "Checks if the value of the field is `FRINDEX_6`"]
    #[inline(always)]
    pub fn is_frindex_6(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_6
    }
    #[doc = "Checks if the value of the field is `FRINDEX_7`"]
    #[inline(always)]
    pub fn is_frindex_7(&self) -> bool {
        *self == FRINDEX_A::FRINDEX_7
    }
}
#[doc = "Field `FRINDEX` writer - Frame Index"]
pub type FRINDEX_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, FRINDEX_SPEC, u16, FRINDEX_A, 14, O>;
impl<'a, const O: u8> FRINDEX_W<'a, O> {
    #[doc = "(1024) 12"]
    #[inline(always)]
    pub fn frindex_0(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_0)
    }
    #[doc = "(512) 11"]
    #[inline(always)]
    pub fn frindex_1(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_1)
    }
    #[doc = "(256) 10"]
    #[inline(always)]
    pub fn frindex_2(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_2)
    }
    #[doc = "(128) 9"]
    #[inline(always)]
    pub fn frindex_3(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_3)
    }
    #[doc = "(64) 8"]
    #[inline(always)]
    pub fn frindex_4(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_4)
    }
    #[doc = "(32) 7"]
    #[inline(always)]
    pub fn frindex_5(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_5)
    }
    #[doc = "(16) 6"]
    #[inline(always)]
    pub fn frindex_6(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_6)
    }
    #[doc = "(8) 5"]
    #[inline(always)]
    pub fn frindex_7(self) -> &'a mut W {
        self.variant(FRINDEX_A::FRINDEX_7)
    }
}
impl R {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    pub fn frindex(&self) -> FRINDEX_R {
        FRINDEX_R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - Frame Index"]
    #[inline(always)]
    #[must_use]
    pub fn frindex(&mut self) -> FRINDEX_W<0> {
        FRINDEX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB Frame Index\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [frindex](index.html) module"]
pub struct FRINDEX_SPEC;
impl crate::RegisterSpec for FRINDEX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [frindex::R](R) reader structure"]
impl crate::Readable for FRINDEX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [frindex::W](W) writer structure"]
impl crate::Writable for FRINDEX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRINDEX to value 0"]
impl crate::Resettable for FRINDEX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
