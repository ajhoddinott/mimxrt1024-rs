#[doc = "Register `IPCR1` reader"]
pub struct R(crate::R<IPCR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IPCR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IPCR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IPCR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IPCR1` writer"]
pub struct W(crate::W<IPCR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IPCR1_SPEC>;
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
impl From<crate::W<IPCR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IPCR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATSZ` reader - Data Size in Byte"]
pub type DATSZ_R = crate::FieldReader<u8, DATSZ_A>;
#[doc = "Data Size in Byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DATSZ_A {
    #[doc = "0: 4"]
    DATSZ_0 = 0,
    #[doc = "1: 1"]
    DATSZ_1 = 1,
    #[doc = "2: 2"]
    DATSZ_2 = 2,
    #[doc = "3: 3"]
    DATSZ_3 = 3,
    #[doc = "4: 4"]
    DATSZ_4 = 4,
    #[doc = "5: 4"]
    DATSZ_5 = 5,
    #[doc = "6: 4"]
    DATSZ_6 = 6,
    #[doc = "7: 4"]
    DATSZ_7 = 7,
}
impl From<DATSZ_A> for u8 {
    #[inline(always)]
    fn from(variant: DATSZ_A) -> Self {
        variant as _
    }
}
impl DATSZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DATSZ_A {
        match self.bits {
            0 => DATSZ_A::DATSZ_0,
            1 => DATSZ_A::DATSZ_1,
            2 => DATSZ_A::DATSZ_2,
            3 => DATSZ_A::DATSZ_3,
            4 => DATSZ_A::DATSZ_4,
            5 => DATSZ_A::DATSZ_5,
            6 => DATSZ_A::DATSZ_6,
            7 => DATSZ_A::DATSZ_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DATSZ_0`"]
    #[inline(always)]
    pub fn is_datsz_0(&self) -> bool {
        *self == DATSZ_A::DATSZ_0
    }
    #[doc = "Checks if the value of the field is `DATSZ_1`"]
    #[inline(always)]
    pub fn is_datsz_1(&self) -> bool {
        *self == DATSZ_A::DATSZ_1
    }
    #[doc = "Checks if the value of the field is `DATSZ_2`"]
    #[inline(always)]
    pub fn is_datsz_2(&self) -> bool {
        *self == DATSZ_A::DATSZ_2
    }
    #[doc = "Checks if the value of the field is `DATSZ_3`"]
    #[inline(always)]
    pub fn is_datsz_3(&self) -> bool {
        *self == DATSZ_A::DATSZ_3
    }
    #[doc = "Checks if the value of the field is `DATSZ_4`"]
    #[inline(always)]
    pub fn is_datsz_4(&self) -> bool {
        *self == DATSZ_A::DATSZ_4
    }
    #[doc = "Checks if the value of the field is `DATSZ_5`"]
    #[inline(always)]
    pub fn is_datsz_5(&self) -> bool {
        *self == DATSZ_A::DATSZ_5
    }
    #[doc = "Checks if the value of the field is `DATSZ_6`"]
    #[inline(always)]
    pub fn is_datsz_6(&self) -> bool {
        *self == DATSZ_A::DATSZ_6
    }
    #[doc = "Checks if the value of the field is `DATSZ_7`"]
    #[inline(always)]
    pub fn is_datsz_7(&self) -> bool {
        *self == DATSZ_A::DATSZ_7
    }
}
#[doc = "Field `DATSZ` writer - Data Size in Byte"]
pub type DATSZ_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, IPCR1_SPEC, u8, DATSZ_A, 3, O>;
impl<'a, const O: u8> DATSZ_W<'a, O> {
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_0(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_0)
    }
    #[doc = "1"]
    #[inline(always)]
    pub fn datsz_1(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_1)
    }
    #[doc = "2"]
    #[inline(always)]
    pub fn datsz_2(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_2)
    }
    #[doc = "3"]
    #[inline(always)]
    pub fn datsz_3(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_3)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_4(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_4)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_5(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_5)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_6(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_6)
    }
    #[doc = "4"]
    #[inline(always)]
    pub fn datsz_7(self) -> &'a mut W {
        self.variant(DATSZ_A::DATSZ_7)
    }
}
impl R {
    #[doc = "Bits 0:2 - Data Size in Byte"]
    #[inline(always)]
    pub fn datsz(&self) -> DATSZ_R {
        DATSZ_R::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Data Size in Byte"]
    #[inline(always)]
    #[must_use]
    pub fn datsz(&mut self) -> DATSZ_W<0> {
        DATSZ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IP Command Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ipcr1](index.html) module"]
pub struct IPCR1_SPEC;
impl crate::RegisterSpec for IPCR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ipcr1::R](R) reader structure"]
impl crate::Readable for IPCR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ipcr1::W](W) writer structure"]
impl crate::Writable for IPCR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IPCR1 to value 0"]
impl crate::Resettable for IPCR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
