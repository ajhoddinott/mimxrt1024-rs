#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OF1` reader - See OF3"]
pub type OF1_R = crate::BitReader<bool>;
#[doc = "Field `OF1` writer - See OF3"]
pub type OF1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OF2` reader - See OF3"]
pub type OF2_R = crate::BitReader<bool>;
#[doc = "Field `OF2` writer - See OF3"]
pub type OF2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `OF3` reader - OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
pub type OF3_R = crate::BitReader<OF3_A>;
#[doc = "OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OF3_A {
    #[doc = "0: Compare event has not occurred."]
    OF3_0 = 0,
    #[doc = "1: Compare event has occurred."]
    OF3_1 = 1,
}
impl From<OF3_A> for bool {
    #[inline(always)]
    fn from(variant: OF3_A) -> Self {
        variant as u8 != 0
    }
}
impl OF3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OF3_A {
        match self.bits {
            false => OF3_A::OF3_0,
            true => OF3_A::OF3_1,
        }
    }
    #[doc = "Checks if the value of the field is `OF3_0`"]
    #[inline(always)]
    pub fn is_of3_0(&self) -> bool {
        *self == OF3_A::OF3_0
    }
    #[doc = "Checks if the value of the field is `OF3_1`"]
    #[inline(always)]
    pub fn is_of3_1(&self) -> bool {
        *self == OF3_A::OF3_1
    }
}
#[doc = "Field `OF3` writer - OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
pub type OF3_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, OF3_A, O>;
impl<'a, const O: u8> OF3_W<'a, O> {
    #[doc = "Compare event has not occurred."]
    #[inline(always)]
    pub fn of3_0(self) -> &'a mut W {
        self.variant(OF3_A::OF3_0)
    }
    #[doc = "Compare event has occurred."]
    #[inline(always)]
    pub fn of3_1(self) -> &'a mut W {
        self.variant(OF3_A::OF3_1)
    }
}
#[doc = "Field `IF1` reader - See IF2"]
pub type IF1_R = crate::BitReader<bool>;
#[doc = "Field `IF1` writer - See IF2"]
pub type IF1_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, bool, O>;
#[doc = "Field `IF2` reader - IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
pub type IF2_R = crate::BitReader<IF2_A>;
#[doc = "IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IF2_A {
    #[doc = "0: Capture event has not occurred."]
    IF2_0 = 0,
    #[doc = "1: Capture event has occurred."]
    IF2_1 = 1,
}
impl From<IF2_A> for bool {
    #[inline(always)]
    fn from(variant: IF2_A) -> Self {
        variant as u8 != 0
    }
}
impl IF2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF2_A {
        match self.bits {
            false => IF2_A::IF2_0,
            true => IF2_A::IF2_1,
        }
    }
    #[doc = "Checks if the value of the field is `IF2_0`"]
    #[inline(always)]
    pub fn is_if2_0(&self) -> bool {
        *self == IF2_A::IF2_0
    }
    #[doc = "Checks if the value of the field is `IF2_1`"]
    #[inline(always)]
    pub fn is_if2_1(&self) -> bool {
        *self == IF2_A::IF2_1
    }
}
#[doc = "Field `IF2` writer - IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
pub type IF2_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, IF2_A, O>;
impl<'a, const O: u8> IF2_W<'a, O> {
    #[doc = "Capture event has not occurred."]
    #[inline(always)]
    pub fn if2_0(self) -> &'a mut W {
        self.variant(IF2_A::IF2_0)
    }
    #[doc = "Capture event has occurred."]
    #[inline(always)]
    pub fn if2_1(self) -> &'a mut W {
        self.variant(IF2_A::IF2_1)
    }
}
#[doc = "Field `ROV` reader - Rollover Flag"]
pub type ROV_R = crate::BitReader<ROV_A>;
#[doc = "Rollover Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROV_A {
    #[doc = "0: Rollover has not occurred."]
    ROV_0 = 0,
    #[doc = "1: Rollover has occurred."]
    ROV_1 = 1,
}
impl From<ROV_A> for bool {
    #[inline(always)]
    fn from(variant: ROV_A) -> Self {
        variant as u8 != 0
    }
}
impl ROV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROV_A {
        match self.bits {
            false => ROV_A::ROV_0,
            true => ROV_A::ROV_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROV_0`"]
    #[inline(always)]
    pub fn is_rov_0(&self) -> bool {
        *self == ROV_A::ROV_0
    }
    #[doc = "Checks if the value of the field is `ROV_1`"]
    #[inline(always)]
    pub fn is_rov_1(&self) -> bool {
        *self == ROV_A::ROV_1
    }
}
#[doc = "Field `ROV` writer - Rollover Flag"]
pub type ROV_W<'a, const O: u8> = crate::BitWriter1C<'a, u32, SR_SPEC, ROV_A, O>;
impl<'a, const O: u8> ROV_W<'a, O> {
    #[doc = "Rollover has not occurred."]
    #[inline(always)]
    pub fn rov_0(self) -> &'a mut W {
        self.variant(ROV_A::ROV_0)
    }
    #[doc = "Rollover has occurred."]
    #[inline(always)]
    pub fn rov_1(self) -> &'a mut W {
        self.variant(ROV_A::ROV_1)
    }
}
impl R {
    #[doc = "Bit 0 - See OF3"]
    #[inline(always)]
    pub fn of1(&self) -> OF1_R {
        OF1_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See OF3"]
    #[inline(always)]
    pub fn of2(&self) -> OF2_R {
        OF2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[inline(always)]
    pub fn of3(&self) -> OF3_R {
        OF3_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - See IF2"]
    #[inline(always)]
    pub fn if1(&self) -> IF1_R {
        IF1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[inline(always)]
    pub fn if2(&self) -> IF2_R {
        IF2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rollover Flag"]
    #[inline(always)]
    pub fn rov(&self) -> ROV_R {
        ROV_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See OF3"]
    #[inline(always)]
    #[must_use]
    pub fn of1(&mut self) -> OF1_W<0> {
        OF1_W::new(self)
    }
    #[doc = "Bit 1 - See OF3"]
    #[inline(always)]
    #[must_use]
    pub fn of2(&mut self) -> OF2_W<1> {
        OF2_W::new(self)
    }
    #[doc = "Bit 2 - OF3 Output Compare 3 Flag OF2 Output Compare 2 Flag OF1 Output Compare 1 Flag The OFn bit indicates that a compare event has occurred on Output Compare channel n"]
    #[inline(always)]
    #[must_use]
    pub fn of3(&mut self) -> OF3_W<2> {
        OF3_W::new(self)
    }
    #[doc = "Bit 3 - See IF2"]
    #[inline(always)]
    #[must_use]
    pub fn if1(&mut self) -> IF1_W<3> {
        IF1_W::new(self)
    }
    #[doc = "Bit 4 - IF2 Input capture 2 Flag IF1 Input capture 1 Flag The IFn bit indicates that a capture event has occurred on Input Capture channel n"]
    #[inline(always)]
    #[must_use]
    pub fn if2(&mut self) -> IF2_W<4> {
        IF2_W::new(self)
    }
    #[doc = "Bit 5 - Rollover Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rov(&mut self) -> ROV_W<5> {
        ROV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x3f;
}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
