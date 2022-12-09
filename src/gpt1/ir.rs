#[doc = "Register `IR` reader"]
pub struct R(crate::R<IR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IR` writer"]
pub struct W(crate::W<IR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IR_SPEC>;
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
impl From<crate::W<IR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `OF1IE` reader - See OF3IE"]
pub type OF1IE_R = crate::BitReader<bool>;
#[doc = "Field `OF1IE` writer - See OF3IE"]
pub type OF1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `OF2IE` reader - See OF3IE"]
pub type OF2IE_R = crate::BitReader<bool>;
#[doc = "Field `OF2IE` writer - See OF3IE"]
pub type OF2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `OF3IE` reader - OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
pub type OF3IE_R = crate::BitReader<OF3IE_A>;
#[doc = "OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OF3IE_A {
    #[doc = "0: Output Compare Channel n interrupt is disabled."]
    OF3IE_0 = 0,
    #[doc = "1: Output Compare Channel n interrupt is enabled."]
    OF3IE_1 = 1,
}
impl From<OF3IE_A> for bool {
    #[inline(always)]
    fn from(variant: OF3IE_A) -> Self {
        variant as u8 != 0
    }
}
impl OF3IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OF3IE_A {
        match self.bits {
            false => OF3IE_A::OF3IE_0,
            true => OF3IE_A::OF3IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `OF3IE_0`"]
    #[inline(always)]
    pub fn is_of3ie_0(&self) -> bool {
        *self == OF3IE_A::OF3IE_0
    }
    #[doc = "Checks if the value of the field is `OF3IE_1`"]
    #[inline(always)]
    pub fn is_of3ie_1(&self) -> bool {
        *self == OF3IE_A::OF3IE_1
    }
}
#[doc = "Field `OF3IE` writer - OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
pub type OF3IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, OF3IE_A, O>;
impl<'a, const O: u8> OF3IE_W<'a, O> {
    #[doc = "Output Compare Channel n interrupt is disabled."]
    #[inline(always)]
    pub fn of3ie_0(self) -> &'a mut W {
        self.variant(OF3IE_A::OF3IE_0)
    }
    #[doc = "Output Compare Channel n interrupt is enabled."]
    #[inline(always)]
    pub fn of3ie_1(self) -> &'a mut W {
        self.variant(OF3IE_A::OF3IE_1)
    }
}
#[doc = "Field `IF1IE` reader - See IF2IE"]
pub type IF1IE_R = crate::BitReader<bool>;
#[doc = "Field `IF1IE` writer - See IF2IE"]
pub type IF1IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, bool, O>;
#[doc = "Field `IF2IE` reader - IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
pub type IF2IE_R = crate::BitReader<IF2IE_A>;
#[doc = "IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IF2IE_A {
    #[doc = "0: IF2IE Input Capture n Interrupt Enable is disabled."]
    IF2IE_0 = 0,
    #[doc = "1: IF2IE Input Capture n Interrupt Enable is enabled."]
    IF2IE_1 = 1,
}
impl From<IF2IE_A> for bool {
    #[inline(always)]
    fn from(variant: IF2IE_A) -> Self {
        variant as u8 != 0
    }
}
impl IF2IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IF2IE_A {
        match self.bits {
            false => IF2IE_A::IF2IE_0,
            true => IF2IE_A::IF2IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `IF2IE_0`"]
    #[inline(always)]
    pub fn is_if2ie_0(&self) -> bool {
        *self == IF2IE_A::IF2IE_0
    }
    #[doc = "Checks if the value of the field is `IF2IE_1`"]
    #[inline(always)]
    pub fn is_if2ie_1(&self) -> bool {
        *self == IF2IE_A::IF2IE_1
    }
}
#[doc = "Field `IF2IE` writer - IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
pub type IF2IE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, IF2IE_A, O>;
impl<'a, const O: u8> IF2IE_W<'a, O> {
    #[doc = "IF2IE Input Capture n Interrupt Enable is disabled."]
    #[inline(always)]
    pub fn if2ie_0(self) -> &'a mut W {
        self.variant(IF2IE_A::IF2IE_0)
    }
    #[doc = "IF2IE Input Capture n Interrupt Enable is enabled."]
    #[inline(always)]
    pub fn if2ie_1(self) -> &'a mut W {
        self.variant(IF2IE_A::IF2IE_1)
    }
}
#[doc = "Field `ROVIE` reader - Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
pub type ROVIE_R = crate::BitReader<ROVIE_A>;
#[doc = "Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ROVIE_A {
    #[doc = "0: Rollover interrupt is disabled."]
    ROVIE_0 = 0,
    #[doc = "1: Rollover interrupt enabled."]
    ROVIE_1 = 1,
}
impl From<ROVIE_A> for bool {
    #[inline(always)]
    fn from(variant: ROVIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ROVIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ROVIE_A {
        match self.bits {
            false => ROVIE_A::ROVIE_0,
            true => ROVIE_A::ROVIE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ROVIE_0`"]
    #[inline(always)]
    pub fn is_rovie_0(&self) -> bool {
        *self == ROVIE_A::ROVIE_0
    }
    #[doc = "Checks if the value of the field is `ROVIE_1`"]
    #[inline(always)]
    pub fn is_rovie_1(&self) -> bool {
        *self == ROVIE_A::ROVIE_1
    }
}
#[doc = "Field `ROVIE` writer - Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
pub type ROVIE_W<'a, const O: u8> = crate::BitWriter<'a, u32, IR_SPEC, ROVIE_A, O>;
impl<'a, const O: u8> ROVIE_W<'a, O> {
    #[doc = "Rollover interrupt is disabled."]
    #[inline(always)]
    pub fn rovie_0(self) -> &'a mut W {
        self.variant(ROVIE_A::ROVIE_0)
    }
    #[doc = "Rollover interrupt enabled."]
    #[inline(always)]
    pub fn rovie_1(self) -> &'a mut W {
        self.variant(ROVIE_A::ROVIE_1)
    }
}
impl R {
    #[doc = "Bit 0 - See OF3IE"]
    #[inline(always)]
    pub fn of1ie(&self) -> OF1IE_R {
        OF1IE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - See OF3IE"]
    #[inline(always)]
    pub fn of2ie(&self) -> OF2IE_R {
        OF2IE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[inline(always)]
    pub fn of3ie(&self) -> OF3IE_R {
        OF3IE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - See IF2IE"]
    #[inline(always)]
    pub fn if1ie(&self) -> IF1IE_R {
        IF1IE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[inline(always)]
    pub fn if2ie(&self) -> IF2IE_R {
        IF2IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[inline(always)]
    pub fn rovie(&self) -> ROVIE_R {
        ROVIE_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - See OF3IE"]
    #[inline(always)]
    #[must_use]
    pub fn of1ie(&mut self) -> OF1IE_W<0> {
        OF1IE_W::new(self)
    }
    #[doc = "Bit 1 - See OF3IE"]
    #[inline(always)]
    #[must_use]
    pub fn of2ie(&mut self) -> OF2IE_W<1> {
        OF2IE_W::new(self)
    }
    #[doc = "Bit 2 - OF3IE Output Compare 3 Interrupt Enable OF2IE Output Compare 2 Interrupt Enable OF1IE Output Compare 1 Interrupt Enable The OFnIE bit controls the Output Compare Channel n interrupt"]
    #[inline(always)]
    #[must_use]
    pub fn of3ie(&mut self) -> OF3IE_W<2> {
        OF3IE_W::new(self)
    }
    #[doc = "Bit 3 - See IF2IE"]
    #[inline(always)]
    #[must_use]
    pub fn if1ie(&mut self) -> IF1IE_W<3> {
        IF1IE_W::new(self)
    }
    #[doc = "Bit 4 - IF2IE Input capture 2 Interrupt Enable IF1IE Input capture 1 Interrupt Enable The IFnIE bit controls the IFnIE Input Capture n Interrupt Enable"]
    #[inline(always)]
    #[must_use]
    pub fn if2ie(&mut self) -> IF2IE_W<4> {
        IF2IE_W::new(self)
    }
    #[doc = "Bit 5 - Rollover Interrupt Enable. The ROVIE bit controls the Rollover interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rovie(&mut self) -> ROVIE_W<5> {
        ROVIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "GPT Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ir](index.html) module"]
pub struct IR_SPEC;
impl crate::RegisterSpec for IR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ir::R](R) reader structure"]
impl crate::Readable for IR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ir::W](W) writer structure"]
impl crate::Writable for IR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IR to value 0"]
impl crate::Resettable for IR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
