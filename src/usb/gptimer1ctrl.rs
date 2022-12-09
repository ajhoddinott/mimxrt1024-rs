#[doc = "Register `GPTIMER1CTRL` reader"]
pub struct R(crate::R<GPTIMER1CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GPTIMER1CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GPTIMER1CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GPTIMER1CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GPTIMER1CTRL` writer"]
pub struct W(crate::W<GPTIMER1CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GPTIMER1CTRL_SPEC>;
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
impl From<crate::W<GPTIMER1CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GPTIMER1CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `GPTCNT` reader - General Purpose Timer Counter. This field is the count value of the countdown timer."]
pub type GPTCNT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `GPTCNT` writer - General Purpose Timer Counter. This field is the count value of the countdown timer."]
pub type GPTCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, GPTIMER1CTRL_SPEC, u32, u32, 24, O>;
#[doc = "Field `GPTMODE` reader - General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software"]
pub type GPTMODE_R = crate::BitReader<GPTMODE_A>;
#[doc = "General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPTMODE_A {
    #[doc = "0: One Shot Mode"]
    GPTMODE_0 = 0,
    #[doc = "1: Repeat Mode"]
    GPTMODE_1 = 1,
}
impl From<GPTMODE_A> for bool {
    #[inline(always)]
    fn from(variant: GPTMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl GPTMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPTMODE_A {
        match self.bits {
            false => GPTMODE_A::GPTMODE_0,
            true => GPTMODE_A::GPTMODE_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPTMODE_0`"]
    #[inline(always)]
    pub fn is_gptmode_0(&self) -> bool {
        *self == GPTMODE_A::GPTMODE_0
    }
    #[doc = "Checks if the value of the field is `GPTMODE_1`"]
    #[inline(always)]
    pub fn is_gptmode_1(&self) -> bool {
        *self == GPTMODE_A::GPTMODE_1
    }
}
#[doc = "Field `GPTMODE` writer - General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software"]
pub type GPTMODE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTIMER1CTRL_SPEC, GPTMODE_A, O>;
impl<'a, const O: u8> GPTMODE_W<'a, O> {
    #[doc = "One Shot Mode"]
    #[inline(always)]
    pub fn gptmode_0(self) -> &'a mut W {
        self.variant(GPTMODE_A::GPTMODE_0)
    }
    #[doc = "Repeat Mode"]
    #[inline(always)]
    pub fn gptmode_1(self) -> &'a mut W {
        self.variant(GPTMODE_A::GPTMODE_1)
    }
}
#[doc = "Field `GPTRST` reader - General Purpose Timer Reset"]
pub type GPTRST_R = crate::BitReader<GPTRST_A>;
#[doc = "General Purpose Timer Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPTRST_A {
    #[doc = "0: No action"]
    GPTRST_0 = 0,
    #[doc = "1: Load counter value from GPTLD bits in USB_n_GPTIMER0LD"]
    GPTRST_1 = 1,
}
impl From<GPTRST_A> for bool {
    #[inline(always)]
    fn from(variant: GPTRST_A) -> Self {
        variant as u8 != 0
    }
}
impl GPTRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPTRST_A {
        match self.bits {
            false => GPTRST_A::GPTRST_0,
            true => GPTRST_A::GPTRST_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPTRST_0`"]
    #[inline(always)]
    pub fn is_gptrst_0(&self) -> bool {
        *self == GPTRST_A::GPTRST_0
    }
    #[doc = "Checks if the value of the field is `GPTRST_1`"]
    #[inline(always)]
    pub fn is_gptrst_1(&self) -> bool {
        *self == GPTRST_A::GPTRST_1
    }
}
#[doc = "Field `GPTRST` writer - General Purpose Timer Reset"]
pub type GPTRST_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTIMER1CTRL_SPEC, GPTRST_A, O>;
impl<'a, const O: u8> GPTRST_W<'a, O> {
    #[doc = "No action"]
    #[inline(always)]
    pub fn gptrst_0(self) -> &'a mut W {
        self.variant(GPTRST_A::GPTRST_0)
    }
    #[doc = "Load counter value from GPTLD bits in USB_n_GPTIMER0LD"]
    #[inline(always)]
    pub fn gptrst_1(self) -> &'a mut W {
        self.variant(GPTRST_A::GPTRST_1)
    }
}
#[doc = "Field `GPTRUN` reader - General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
pub type GPTRUN_R = crate::BitReader<GPTRUN_A>;
#[doc = "General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GPTRUN_A {
    #[doc = "0: Stop counting"]
    GPTRUN_0 = 0,
    #[doc = "1: Run"]
    GPTRUN_1 = 1,
}
impl From<GPTRUN_A> for bool {
    #[inline(always)]
    fn from(variant: GPTRUN_A) -> Self {
        variant as u8 != 0
    }
}
impl GPTRUN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GPTRUN_A {
        match self.bits {
            false => GPTRUN_A::GPTRUN_0,
            true => GPTRUN_A::GPTRUN_1,
        }
    }
    #[doc = "Checks if the value of the field is `GPTRUN_0`"]
    #[inline(always)]
    pub fn is_gptrun_0(&self) -> bool {
        *self == GPTRUN_A::GPTRUN_0
    }
    #[doc = "Checks if the value of the field is `GPTRUN_1`"]
    #[inline(always)]
    pub fn is_gptrun_1(&self) -> bool {
        *self == GPTRUN_A::GPTRUN_1
    }
}
#[doc = "Field `GPTRUN` writer - General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
pub type GPTRUN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GPTIMER1CTRL_SPEC, GPTRUN_A, O>;
impl<'a, const O: u8> GPTRUN_W<'a, O> {
    #[doc = "Stop counting"]
    #[inline(always)]
    pub fn gptrun_0(self) -> &'a mut W {
        self.variant(GPTRUN_A::GPTRUN_0)
    }
    #[doc = "Run"]
    #[inline(always)]
    pub fn gptrun_1(self) -> &'a mut W {
        self.variant(GPTRUN_A::GPTRUN_1)
    }
}
impl R {
    #[doc = "Bits 0:23 - General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline(always)]
    pub fn gptcnt(&self) -> GPTCNT_R {
        GPTCNT_R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software"]
    #[inline(always)]
    pub fn gptmode(&self) -> GPTMODE_R {
        GPTMODE_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 30 - General Purpose Timer Reset"]
    #[inline(always)]
    pub fn gptrst(&self) -> GPTRST_R {
        GPTRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[inline(always)]
    pub fn gptrun(&self) -> GPTRUN_R {
        GPTRUN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:23 - General Purpose Timer Counter. This field is the count value of the countdown timer."]
    #[inline(always)]
    #[must_use]
    pub fn gptcnt(&mut self) -> GPTCNT_W<0> {
        GPTCNT_W::new(self)
    }
    #[doc = "Bit 24 - General Purpose Timer Mode In one shot mode, the timer will count down to zero, generate an interrupt, and stop until the counter is reset by software"]
    #[inline(always)]
    #[must_use]
    pub fn gptmode(&mut self) -> GPTMODE_W<24> {
        GPTMODE_W::new(self)
    }
    #[doc = "Bit 30 - General Purpose Timer Reset"]
    #[inline(always)]
    #[must_use]
    pub fn gptrst(&mut self) -> GPTRST_W<30> {
        GPTRST_W::new(self)
    }
    #[doc = "Bit 31 - General Purpose Timer Run GPTCNT bits are not effected when setting or clearing this bit."]
    #[inline(always)]
    #[must_use]
    pub fn gptrun(&mut self) -> GPTRUN_W<31> {
        GPTRUN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General Purpose Timer #1 Controller\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gptimer1ctrl](index.html) module"]
pub struct GPTIMER1CTRL_SPEC;
impl crate::RegisterSpec for GPTIMER1CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gptimer1ctrl::R](R) reader structure"]
impl crate::Readable for GPTIMER1CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gptimer1ctrl::W](W) writer structure"]
impl crate::Writable for GPTIMER1CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GPTIMER1CTRL to value 0"]
impl crate::Resettable for GPTIMER1CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
