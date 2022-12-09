#[doc = "Register `TST` reader"]
pub struct R(crate::R<TST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TST` writer"]
pub struct W(crate::W<TST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TST_SPEC>;
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
impl From<crate::W<TST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEST_COUNT` reader - TEST_COUNT"]
pub type TEST_COUNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST_COUNT` writer - TEST_COUNT"]
pub type TEST_COUNT_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TST_SPEC, u8, u8, 8, O>;
#[doc = "Field `TEST_PERIOD` reader - TEST_PERIOD"]
pub type TEST_PERIOD_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEST_PERIOD` writer - TEST_PERIOD"]
pub type TEST_PERIOD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, TST_SPEC, u8, u8, 5, O>;
#[doc = "Field `QDN` reader - Quadrature Decoder Negative Signal"]
pub type QDN_R = crate::BitReader<QDN_A>;
#[doc = "Quadrature Decoder Negative Signal\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum QDN_A {
    #[doc = "0: Generates a positive quadrature decoder signal"]
    QDN_0 = 0,
    #[doc = "1: Generates a negative quadrature decoder signal"]
    QDN_1 = 1,
}
impl From<QDN_A> for bool {
    #[inline(always)]
    fn from(variant: QDN_A) -> Self {
        variant as u8 != 0
    }
}
impl QDN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> QDN_A {
        match self.bits {
            false => QDN_A::QDN_0,
            true => QDN_A::QDN_1,
        }
    }
    #[doc = "Checks if the value of the field is `QDN_0`"]
    #[inline(always)]
    pub fn is_qdn_0(&self) -> bool {
        *self == QDN_A::QDN_0
    }
    #[doc = "Checks if the value of the field is `QDN_1`"]
    #[inline(always)]
    pub fn is_qdn_1(&self) -> bool {
        *self == QDN_A::QDN_1
    }
}
#[doc = "Field `QDN` writer - Quadrature Decoder Negative Signal"]
pub type QDN_W<'a, const O: u8> = crate::BitWriter<'a, u16, TST_SPEC, QDN_A, O>;
impl<'a, const O: u8> QDN_W<'a, O> {
    #[doc = "Generates a positive quadrature decoder signal"]
    #[inline(always)]
    pub fn qdn_0(self) -> &'a mut W {
        self.variant(QDN_A::QDN_0)
    }
    #[doc = "Generates a negative quadrature decoder signal"]
    #[inline(always)]
    pub fn qdn_1(self) -> &'a mut W {
        self.variant(QDN_A::QDN_1)
    }
}
#[doc = "Field `TCE` reader - Test Counter Enable"]
pub type TCE_R = crate::BitReader<TCE_A>;
#[doc = "Test Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCE_A {
    #[doc = "0: Disabled"]
    TCE_0 = 0,
    #[doc = "1: Enabled"]
    TCE_1 = 1,
}
impl From<TCE_A> for bool {
    #[inline(always)]
    fn from(variant: TCE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCE_A {
        match self.bits {
            false => TCE_A::TCE_0,
            true => TCE_A::TCE_1,
        }
    }
    #[doc = "Checks if the value of the field is `TCE_0`"]
    #[inline(always)]
    pub fn is_tce_0(&self) -> bool {
        *self == TCE_A::TCE_0
    }
    #[doc = "Checks if the value of the field is `TCE_1`"]
    #[inline(always)]
    pub fn is_tce_1(&self) -> bool {
        *self == TCE_A::TCE_1
    }
}
#[doc = "Field `TCE` writer - Test Counter Enable"]
pub type TCE_W<'a, const O: u8> = crate::BitWriter<'a, u16, TST_SPEC, TCE_A, O>;
impl<'a, const O: u8> TCE_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn tce_0(self) -> &'a mut W {
        self.variant(TCE_A::TCE_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn tce_1(self) -> &'a mut W {
        self.variant(TCE_A::TCE_1)
    }
}
#[doc = "Field `TEN` reader - Test Mode Enable"]
pub type TEN_R = crate::BitReader<TEN_A>;
#[doc = "Test Mode Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEN_A {
    #[doc = "0: Disabled"]
    TEN_0 = 0,
    #[doc = "1: Enabled"]
    TEN_1 = 1,
}
impl From<TEN_A> for bool {
    #[inline(always)]
    fn from(variant: TEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEN_A {
        match self.bits {
            false => TEN_A::TEN_0,
            true => TEN_A::TEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TEN_0`"]
    #[inline(always)]
    pub fn is_ten_0(&self) -> bool {
        *self == TEN_A::TEN_0
    }
    #[doc = "Checks if the value of the field is `TEN_1`"]
    #[inline(always)]
    pub fn is_ten_1(&self) -> bool {
        *self == TEN_A::TEN_1
    }
}
#[doc = "Field `TEN` writer - Test Mode Enable"]
pub type TEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, TST_SPEC, TEN_A, O>;
impl<'a, const O: u8> TEN_W<'a, O> {
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn ten_0(self) -> &'a mut W {
        self.variant(TEN_A::TEN_0)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn ten_1(self) -> &'a mut W {
        self.variant(TEN_A::TEN_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - TEST_COUNT"]
    #[inline(always)]
    pub fn test_count(&self) -> TEST_COUNT_R {
        TEST_COUNT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:12 - TEST_PERIOD"]
    #[inline(always)]
    pub fn test_period(&self) -> TEST_PERIOD_R {
        TEST_PERIOD_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 13 - Quadrature Decoder Negative Signal"]
    #[inline(always)]
    pub fn qdn(&self) -> QDN_R {
        QDN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Test Counter Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TCE_R {
        TCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Test Mode Enable"]
    #[inline(always)]
    pub fn ten(&self) -> TEN_R {
        TEN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - TEST_COUNT"]
    #[inline(always)]
    #[must_use]
    pub fn test_count(&mut self) -> TEST_COUNT_W<0> {
        TEST_COUNT_W::new(self)
    }
    #[doc = "Bits 8:12 - TEST_PERIOD"]
    #[inline(always)]
    #[must_use]
    pub fn test_period(&mut self) -> TEST_PERIOD_W<8> {
        TEST_PERIOD_W::new(self)
    }
    #[doc = "Bit 13 - Quadrature Decoder Negative Signal"]
    #[inline(always)]
    #[must_use]
    pub fn qdn(&mut self) -> QDN_W<13> {
        QDN_W::new(self)
    }
    #[doc = "Bit 14 - Test Counter Enable"]
    #[inline(always)]
    #[must_use]
    pub fn tce(&mut self) -> TCE_W<14> {
        TCE_W::new(self)
    }
    #[doc = "Bit 15 - Test Mode Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ten(&mut self) -> TEN_W<15> {
        TEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Test Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tst](index.html) module"]
pub struct TST_SPEC;
impl crate::RegisterSpec for TST_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [tst::R](R) reader structure"]
impl crate::Readable for TST_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tst::W](W) writer structure"]
impl crate::Writable for TST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TST to value 0"]
impl crate::Resettable for TST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
