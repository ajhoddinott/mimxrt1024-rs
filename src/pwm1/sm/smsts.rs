#[doc = "Register `SMSTS` reader"]
pub struct R(crate::R<SMSTS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMSTS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMSTS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMSTS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMSTS` writer"]
pub struct W(crate::W<SMSTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMSTS_SPEC>;
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
impl From<crate::W<SMSTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMSTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPF` reader - Compare Flags"]
pub type CMPF_R = crate::FieldReader<u8, CMPF_A>;
#[doc = "Compare Flags\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMPF_A {
    #[doc = "0: No compare event has occurred for a particular VALx value."]
    NO_EVENT = 0,
    #[doc = "1: A compare event has occurred for a particular VALx value."]
    EVENT = 1,
}
impl From<CMPF_A> for u8 {
    #[inline(always)]
    fn from(variant: CMPF_A) -> Self {
        variant as _
    }
}
impl CMPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CMPF_A> {
        match self.bits {
            0 => Some(CMPF_A::NO_EVENT),
            1 => Some(CMPF_A::EVENT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_EVENT`"]
    #[inline(always)]
    pub fn is_no_event(&self) -> bool {
        *self == CMPF_A::NO_EVENT
    }
    #[doc = "Checks if the value of the field is `EVENT`"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == CMPF_A::EVENT
    }
}
#[doc = "Field `CMPF` writer - Compare Flags"]
pub type CMPF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, SMSTS_SPEC, u8, CMPF_A, 6, O>;
impl<'a, const O: u8> CMPF_W<'a, O> {
    #[doc = "No compare event has occurred for a particular VALx value."]
    #[inline(always)]
    pub fn no_event(self) -> &'a mut W {
        self.variant(CMPF_A::NO_EVENT)
    }
    #[doc = "A compare event has occurred for a particular VALx value."]
    #[inline(always)]
    pub fn event(self) -> &'a mut W {
        self.variant(CMPF_A::EVENT)
    }
}
#[doc = "Field `CFX0` reader - Capture Flag X0"]
pub type CFX0_R = crate::BitReader<bool>;
#[doc = "Field `CFX0` writer - Capture Flag X0"]
pub type CFX0_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, SMSTS_SPEC, bool, O>;
#[doc = "Field `CFX1` reader - Capture Flag X1"]
pub type CFX1_R = crate::BitReader<bool>;
#[doc = "Field `CFX1` writer - Capture Flag X1"]
pub type CFX1_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, SMSTS_SPEC, bool, O>;
#[doc = "Field `CFB0` reader - Capture Flag B0"]
pub type CFB0_R = crate::BitReader<bool>;
#[doc = "Field `CFB0` writer - Capture Flag B0"]
pub type CFB0_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, SMSTS_SPEC, bool, O>;
#[doc = "Field `CFB1` reader - Capture Flag B1"]
pub type CFB1_R = crate::BitReader<bool>;
#[doc = "Field `CFB1` writer - Capture Flag B1"]
pub type CFB1_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, SMSTS_SPEC, bool, O>;
#[doc = "Field `CFA0` reader - Capture Flag A0"]
pub type CFA0_R = crate::BitReader<bool>;
#[doc = "Field `CFA0` writer - Capture Flag A0"]
pub type CFA0_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, SMSTS_SPEC, bool, O>;
#[doc = "Field `CFA1` reader - Capture Flag A1"]
pub type CFA1_R = crate::BitReader<bool>;
#[doc = "Field `CFA1` writer - Capture Flag A1"]
pub type CFA1_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, SMSTS_SPEC, bool, O>;
#[doc = "Field `RF` reader - Reload Flag"]
pub type RF_R = crate::BitReader<RF_A>;
#[doc = "Reload Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RF_A {
    #[doc = "0: No new reload cycle since last STS\\[RF\\]
clearing"]
    NO_FLAG = 0,
    #[doc = "1: New reload cycle since last STS\\[RF\\]
clearing"]
    FLAG = 1,
}
impl From<RF_A> for bool {
    #[inline(always)]
    fn from(variant: RF_A) -> Self {
        variant as u8 != 0
    }
}
impl RF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RF_A {
        match self.bits {
            false => RF_A::NO_FLAG,
            true => RF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == RF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == RF_A::FLAG
    }
}
#[doc = "Field `RF` writer - Reload Flag"]
pub type RF_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, SMSTS_SPEC, RF_A, O>;
impl<'a, const O: u8> RF_W<'a, O> {
    #[doc = "No new reload cycle since last STS\\[RF\\]
clearing"]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(RF_A::NO_FLAG)
    }
    #[doc = "New reload cycle since last STS\\[RF\\]
clearing"]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(RF_A::FLAG)
    }
}
#[doc = "Field `REF` reader - Reload Error Flag"]
pub type REF_R = crate::BitReader<REF_A>;
#[doc = "Reload Error Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REF_A {
    #[doc = "0: No reload error occurred."]
    NO_FLAG = 0,
    #[doc = "1: Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\]
= 0."]
    FLAG = 1,
}
impl From<REF_A> for bool {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as u8 != 0
    }
}
impl REF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REF_A {
        match self.bits {
            false => REF_A::NO_FLAG,
            true => REF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == REF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == REF_A::FLAG
    }
}
#[doc = "Field `REF` writer - Reload Error Flag"]
pub type REF_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, SMSTS_SPEC, REF_A, O>;
impl<'a, const O: u8> REF_W<'a, O> {
    #[doc = "No reload error occurred."]
    #[inline(always)]
    pub fn no_flag(self) -> &'a mut W {
        self.variant(REF_A::NO_FLAG)
    }
    #[doc = "Reload signal occurred with non-coherent data and MCTRL\\[LDOK\\]
= 0."]
    #[inline(always)]
    pub fn flag(self) -> &'a mut W {
        self.variant(REF_A::FLAG)
    }
}
#[doc = "Field `RUF` reader - Registers Updated Flag"]
pub type RUF_R = crate::BitReader<RUF_A>;
#[doc = "Registers Updated Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RUF_A {
    #[doc = "0: No register update has occurred since last reload."]
    NO_FLAG = 0,
    #[doc = "1: At least one of the double buffered registers has been updated since the last reload."]
    FLAG = 1,
}
impl From<RUF_A> for bool {
    #[inline(always)]
    fn from(variant: RUF_A) -> Self {
        variant as u8 != 0
    }
}
impl RUF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUF_A {
        match self.bits {
            false => RUF_A::NO_FLAG,
            true => RUF_A::FLAG,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FLAG`"]
    #[inline(always)]
    pub fn is_no_flag(&self) -> bool {
        *self == RUF_A::NO_FLAG
    }
    #[doc = "Checks if the value of the field is `FLAG`"]
    #[inline(always)]
    pub fn is_flag(&self) -> bool {
        *self == RUF_A::FLAG
    }
}
impl R {
    #[doc = "Bits 0:5 - Compare Flags"]
    #[inline(always)]
    pub fn cmpf(&self) -> CMPF_R {
        CMPF_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - Capture Flag X0"]
    #[inline(always)]
    pub fn cfx0(&self) -> CFX0_R {
        CFX0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Capture Flag X1"]
    #[inline(always)]
    pub fn cfx1(&self) -> CFX1_R {
        CFX1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Capture Flag B0"]
    #[inline(always)]
    pub fn cfb0(&self) -> CFB0_R {
        CFB0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Capture Flag B1"]
    #[inline(always)]
    pub fn cfb1(&self) -> CFB1_R {
        CFB1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture Flag A0"]
    #[inline(always)]
    pub fn cfa0(&self) -> CFA0_R {
        CFA0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Capture Flag A1"]
    #[inline(always)]
    pub fn cfa1(&self) -> CFA1_R {
        CFA1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reload Flag"]
    #[inline(always)]
    pub fn rf(&self) -> RF_R {
        RF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reload Error Flag"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Registers Updated Flag"]
    #[inline(always)]
    pub fn ruf(&self) -> RUF_R {
        RUF_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Compare Flags"]
    #[inline(always)]
    #[must_use]
    pub fn cmpf(&mut self) -> CMPF_W<0> {
        CMPF_W::new(self)
    }
    #[doc = "Bit 6 - Capture Flag X0"]
    #[inline(always)]
    #[must_use]
    pub fn cfx0(&mut self) -> CFX0_W<6> {
        CFX0_W::new(self)
    }
    #[doc = "Bit 7 - Capture Flag X1"]
    #[inline(always)]
    #[must_use]
    pub fn cfx1(&mut self) -> CFX1_W<7> {
        CFX1_W::new(self)
    }
    #[doc = "Bit 8 - Capture Flag B0"]
    #[inline(always)]
    #[must_use]
    pub fn cfb0(&mut self) -> CFB0_W<8> {
        CFB0_W::new(self)
    }
    #[doc = "Bit 9 - Capture Flag B1"]
    #[inline(always)]
    #[must_use]
    pub fn cfb1(&mut self) -> CFB1_W<9> {
        CFB1_W::new(self)
    }
    #[doc = "Bit 10 - Capture Flag A0"]
    #[inline(always)]
    #[must_use]
    pub fn cfa0(&mut self) -> CFA0_W<10> {
        CFA0_W::new(self)
    }
    #[doc = "Bit 11 - Capture Flag A1"]
    #[inline(always)]
    #[must_use]
    pub fn cfa1(&mut self) -> CFA1_W<11> {
        CFA1_W::new(self)
    }
    #[doc = "Bit 12 - Reload Flag"]
    #[inline(always)]
    #[must_use]
    pub fn rf(&mut self) -> RF_W<12> {
        RF_W::new(self)
    }
    #[doc = "Bit 13 - Reload Error Flag"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> REF_W<13> {
        REF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smsts](index.html) module"]
pub struct SMSTS_SPEC;
impl crate::RegisterSpec for SMSTS_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [smsts::R](R) reader structure"]
impl crate::Readable for SMSTS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smsts::W](W) writer structure"]
impl crate::Writable for SMSTS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x3fff;
}
#[doc = "`reset()` method sets SMSTS to value 0"]
impl crate::Resettable for SMSTS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
