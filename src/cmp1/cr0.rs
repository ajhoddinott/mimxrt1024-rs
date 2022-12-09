#[doc = "Register `CR0` reader"]
pub struct R(crate::R<CR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR0` writer"]
pub struct W(crate::W<CR0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR0_SPEC>;
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
impl From<crate::W<CR0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HYSTCTR` reader - Comparator hard block hysteresis control"]
pub type HYSTCTR_R = crate::FieldReader<u8, HYSTCTR_A>;
#[doc = "Comparator hard block hysteresis control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HYSTCTR_A {
    #[doc = "0: Level 0"]
    HYSTCTR_0 = 0,
    #[doc = "1: Level 1"]
    HYSTCTR_1 = 1,
    #[doc = "2: Level 2"]
    HYSTCTR_2 = 2,
    #[doc = "3: Level 3"]
    HYSTCTR_3 = 3,
}
impl From<HYSTCTR_A> for u8 {
    #[inline(always)]
    fn from(variant: HYSTCTR_A) -> Self {
        variant as _
    }
}
impl HYSTCTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HYSTCTR_A {
        match self.bits {
            0 => HYSTCTR_A::HYSTCTR_0,
            1 => HYSTCTR_A::HYSTCTR_1,
            2 => HYSTCTR_A::HYSTCTR_2,
            3 => HYSTCTR_A::HYSTCTR_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_0`"]
    #[inline(always)]
    pub fn is_hystctr_0(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_0
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_1`"]
    #[inline(always)]
    pub fn is_hystctr_1(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_1
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_2`"]
    #[inline(always)]
    pub fn is_hystctr_2(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_2
    }
    #[doc = "Checks if the value of the field is `HYSTCTR_3`"]
    #[inline(always)]
    pub fn is_hystctr_3(&self) -> bool {
        *self == HYSTCTR_A::HYSTCTR_3
    }
}
#[doc = "Field `HYSTCTR` writer - Comparator hard block hysteresis control"]
pub type HYSTCTR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, CR0_SPEC, u8, HYSTCTR_A, 2, O>;
impl<'a, const O: u8> HYSTCTR_W<'a, O> {
    #[doc = "Level 0"]
    #[inline(always)]
    pub fn hystctr_0(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_0)
    }
    #[doc = "Level 1"]
    #[inline(always)]
    pub fn hystctr_1(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_1)
    }
    #[doc = "Level 2"]
    #[inline(always)]
    pub fn hystctr_2(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_2)
    }
    #[doc = "Level 3"]
    #[inline(always)]
    pub fn hystctr_3(self) -> &'a mut W {
        self.variant(HYSTCTR_A::HYSTCTR_3)
    }
}
#[doc = "Field `FILTER_CNT` reader - Filter Sample Count"]
pub type FILTER_CNT_R = crate::FieldReader<u8, FILTER_CNT_A>;
#[doc = "Filter Sample Count\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FILTER_CNT_A {
    #[doc = "0: Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    FILTER_CNT_0 = 0,
    #[doc = "1: One sample must agree. The comparator output is simply sampled."]
    FILTER_CNT_1 = 1,
    #[doc = "2: 2 consecutive samples must agree."]
    FILTER_CNT_2 = 2,
    #[doc = "3: 3 consecutive samples must agree."]
    FILTER_CNT_3 = 3,
    #[doc = "4: 4 consecutive samples must agree."]
    FILTER_CNT_4 = 4,
    #[doc = "5: 5 consecutive samples must agree."]
    FILTER_CNT_5 = 5,
    #[doc = "6: 6 consecutive samples must agree."]
    FILTER_CNT_6 = 6,
    #[doc = "7: 7 consecutive samples must agree."]
    FILTER_CNT_7 = 7,
}
impl From<FILTER_CNT_A> for u8 {
    #[inline(always)]
    fn from(variant: FILTER_CNT_A) -> Self {
        variant as _
    }
}
impl FILTER_CNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FILTER_CNT_A {
        match self.bits {
            0 => FILTER_CNT_A::FILTER_CNT_0,
            1 => FILTER_CNT_A::FILTER_CNT_1,
            2 => FILTER_CNT_A::FILTER_CNT_2,
            3 => FILTER_CNT_A::FILTER_CNT_3,
            4 => FILTER_CNT_A::FILTER_CNT_4,
            5 => FILTER_CNT_A::FILTER_CNT_5,
            6 => FILTER_CNT_A::FILTER_CNT_6,
            7 => FILTER_CNT_A::FILTER_CNT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_0`"]
    #[inline(always)]
    pub fn is_filter_cnt_0(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_0
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_1`"]
    #[inline(always)]
    pub fn is_filter_cnt_1(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_1
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_2`"]
    #[inline(always)]
    pub fn is_filter_cnt_2(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_2
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_3`"]
    #[inline(always)]
    pub fn is_filter_cnt_3(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_3
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_4`"]
    #[inline(always)]
    pub fn is_filter_cnt_4(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_4
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_5`"]
    #[inline(always)]
    pub fn is_filter_cnt_5(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_5
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_6`"]
    #[inline(always)]
    pub fn is_filter_cnt_6(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_6
    }
    #[doc = "Checks if the value of the field is `FILTER_CNT_7`"]
    #[inline(always)]
    pub fn is_filter_cnt_7(&self) -> bool {
        *self == FILTER_CNT_A::FILTER_CNT_7
    }
}
#[doc = "Field `FILTER_CNT` writer - Filter Sample Count"]
pub type FILTER_CNT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u8, CR0_SPEC, u8, FILTER_CNT_A, 3, O>;
impl<'a, const O: u8> FILTER_CNT_W<'a, O> {
    #[doc = "Filter is disabled. If SE = 1, then COUT is a logic 0. This is not a legal state, and is not recommended. If SE = 0, COUT = COUTA."]
    #[inline(always)]
    pub fn filter_cnt_0(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_0)
    }
    #[doc = "One sample must agree. The comparator output is simply sampled."]
    #[inline(always)]
    pub fn filter_cnt_1(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_1)
    }
    #[doc = "2 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_2(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_2)
    }
    #[doc = "3 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_3(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_3)
    }
    #[doc = "4 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_4(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_4)
    }
    #[doc = "5 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_5(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_5)
    }
    #[doc = "6 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_6(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_6)
    }
    #[doc = "7 consecutive samples must agree."]
    #[inline(always)]
    pub fn filter_cnt_7(self) -> &'a mut W {
        self.variant(FILTER_CNT_A::FILTER_CNT_7)
    }
}
impl R {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline(always)]
    pub fn hystctr(&self) -> HYSTCTR_R {
        HYSTCTR_R::new(self.bits & 3)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    pub fn filter_cnt(&self) -> FILTER_CNT_R {
        FILTER_CNT_R::new((self.bits >> 4) & 7)
    }
}
impl W {
    #[doc = "Bits 0:1 - Comparator hard block hysteresis control"]
    #[inline(always)]
    #[must_use]
    pub fn hystctr(&mut self) -> HYSTCTR_W<0> {
        HYSTCTR_W::new(self)
    }
    #[doc = "Bits 4:6 - Filter Sample Count"]
    #[inline(always)]
    #[must_use]
    pub fn filter_cnt(&mut self) -> FILTER_CNT_W<4> {
        FILTER_CNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMP Control Register 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr0](index.html) module"]
pub struct CR0_SPEC;
impl crate::RegisterSpec for CR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [cr0::R](R) reader structure"]
impl crate::Readable for CR0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr0::W](W) writer structure"]
impl crate::Writable for CR0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for CR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
