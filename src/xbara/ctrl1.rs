#[doc = "Register `CTRL1` reader"]
pub struct R(crate::R<CTRL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CTRL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CTRL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CTRL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CTRL1` writer"]
pub struct W(crate::W<CTRL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CTRL1_SPEC>;
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
impl From<crate::W<CTRL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CTRL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DEN2` reader - DMA Enable for XBAR_OUT2"]
pub type DEN2_R = crate::BitReader<DEN2_A>;
#[doc = "DMA Enable for XBAR_OUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN2_A {
    #[doc = "0: DMA disabled"]
    DEN2_0 = 0,
    #[doc = "1: DMA enabled"]
    DEN2_1 = 1,
}
impl From<DEN2_A> for bool {
    #[inline(always)]
    fn from(variant: DEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl DEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN2_A {
        match self.bits {
            false => DEN2_A::DEN2_0,
            true => DEN2_A::DEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN2_0`"]
    #[inline(always)]
    pub fn is_den2_0(&self) -> bool {
        *self == DEN2_A::DEN2_0
    }
    #[doc = "Checks if the value of the field is `DEN2_1`"]
    #[inline(always)]
    pub fn is_den2_1(&self) -> bool {
        *self == DEN2_A::DEN2_1
    }
}
#[doc = "Field `DEN2` writer - DMA Enable for XBAR_OUT2"]
pub type DEN2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL1_SPEC, DEN2_A, O>;
impl<'a, const O: u8> DEN2_W<'a, O> {
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn den2_0(self) -> &'a mut W {
        self.variant(DEN2_A::DEN2_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn den2_1(self) -> &'a mut W {
        self.variant(DEN2_A::DEN2_1)
    }
}
#[doc = "Field `IEN2` reader - Interrupt Enable for XBAR_OUT2"]
pub type IEN2_R = crate::BitReader<IEN2_A>;
#[doc = "Interrupt Enable for XBAR_OUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN2_A {
    #[doc = "0: Interrupt disabled"]
    IEN2_0 = 0,
    #[doc = "1: Interrupt enabled"]
    IEN2_1 = 1,
}
impl From<IEN2_A> for bool {
    #[inline(always)]
    fn from(variant: IEN2_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN2_A {
        match self.bits {
            false => IEN2_A::IEN2_0,
            true => IEN2_A::IEN2_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN2_0`"]
    #[inline(always)]
    pub fn is_ien2_0(&self) -> bool {
        *self == IEN2_A::IEN2_0
    }
    #[doc = "Checks if the value of the field is `IEN2_1`"]
    #[inline(always)]
    pub fn is_ien2_1(&self) -> bool {
        *self == IEN2_A::IEN2_1
    }
}
#[doc = "Field `IEN2` writer - Interrupt Enable for XBAR_OUT2"]
pub type IEN2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL1_SPEC, IEN2_A, O>;
impl<'a, const O: u8> IEN2_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ien2_0(self) -> &'a mut W {
        self.variant(IEN2_A::IEN2_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ien2_1(self) -> &'a mut W {
        self.variant(IEN2_A::IEN2_1)
    }
}
#[doc = "Field `EDGE2` reader - Active edge for edge detection on XBAR_OUT2"]
pub type EDGE2_R = crate::FieldReader<u8, EDGE2_A>;
#[doc = "Active edge for edge detection on XBAR_OUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGE2_A {
    #[doc = "0: STS2 never asserts"]
    EDGE2_0 = 0,
    #[doc = "1: STS2 asserts on rising edges of XBAR_OUT2"]
    EDGE2_1 = 1,
    #[doc = "2: STS2 asserts on falling edges of XBAR_OUT2"]
    EDGE2_2 = 2,
    #[doc = "3: STS2 asserts on rising and falling edges of XBAR_OUT2"]
    EDGE2_3 = 3,
}
impl From<EDGE2_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE2_A) -> Self {
        variant as _
    }
}
impl EDGE2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE2_A {
        match self.bits {
            0 => EDGE2_A::EDGE2_0,
            1 => EDGE2_A::EDGE2_1,
            2 => EDGE2_A::EDGE2_2,
            3 => EDGE2_A::EDGE2_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE2_0`"]
    #[inline(always)]
    pub fn is_edge2_0(&self) -> bool {
        *self == EDGE2_A::EDGE2_0
    }
    #[doc = "Checks if the value of the field is `EDGE2_1`"]
    #[inline(always)]
    pub fn is_edge2_1(&self) -> bool {
        *self == EDGE2_A::EDGE2_1
    }
    #[doc = "Checks if the value of the field is `EDGE2_2`"]
    #[inline(always)]
    pub fn is_edge2_2(&self) -> bool {
        *self == EDGE2_A::EDGE2_2
    }
    #[doc = "Checks if the value of the field is `EDGE2_3`"]
    #[inline(always)]
    pub fn is_edge2_3(&self) -> bool {
        *self == EDGE2_A::EDGE2_3
    }
}
#[doc = "Field `EDGE2` writer - Active edge for edge detection on XBAR_OUT2"]
pub type EDGE2_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CTRL1_SPEC, u8, EDGE2_A, 2, O>;
impl<'a, const O: u8> EDGE2_W<'a, O> {
    #[doc = "STS2 never asserts"]
    #[inline(always)]
    pub fn edge2_0(self) -> &'a mut W {
        self.variant(EDGE2_A::EDGE2_0)
    }
    #[doc = "STS2 asserts on rising edges of XBAR_OUT2"]
    #[inline(always)]
    pub fn edge2_1(self) -> &'a mut W {
        self.variant(EDGE2_A::EDGE2_1)
    }
    #[doc = "STS2 asserts on falling edges of XBAR_OUT2"]
    #[inline(always)]
    pub fn edge2_2(self) -> &'a mut W {
        self.variant(EDGE2_A::EDGE2_2)
    }
    #[doc = "STS2 asserts on rising and falling edges of XBAR_OUT2"]
    #[inline(always)]
    pub fn edge2_3(self) -> &'a mut W {
        self.variant(EDGE2_A::EDGE2_3)
    }
}
#[doc = "Field `STS2` reader - Edge detection status for XBAR_OUT2"]
pub type STS2_R = crate::BitReader<STS2_A>;
#[doc = "Edge detection status for XBAR_OUT2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STS2_A {
    #[doc = "0: Active edge not yet detected on XBAR_OUT2"]
    STS2_0 = 0,
    #[doc = "1: Active edge detected on XBAR_OUT2"]
    STS2_1 = 1,
}
impl From<STS2_A> for bool {
    #[inline(always)]
    fn from(variant: STS2_A) -> Self {
        variant as u8 != 0
    }
}
impl STS2_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS2_A {
        match self.bits {
            false => STS2_A::STS2_0,
            true => STS2_A::STS2_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS2_0`"]
    #[inline(always)]
    pub fn is_sts2_0(&self) -> bool {
        *self == STS2_A::STS2_0
    }
    #[doc = "Checks if the value of the field is `STS2_1`"]
    #[inline(always)]
    pub fn is_sts2_1(&self) -> bool {
        *self == STS2_A::STS2_1
    }
}
#[doc = "Field `STS2` writer - Edge detection status for XBAR_OUT2"]
pub type STS2_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL1_SPEC, STS2_A, O>;
impl<'a, const O: u8> STS2_W<'a, O> {
    #[doc = "Active edge not yet detected on XBAR_OUT2"]
    #[inline(always)]
    pub fn sts2_0(self) -> &'a mut W {
        self.variant(STS2_A::STS2_0)
    }
    #[doc = "Active edge detected on XBAR_OUT2"]
    #[inline(always)]
    pub fn sts2_1(self) -> &'a mut W {
        self.variant(STS2_A::STS2_1)
    }
}
#[doc = "Field `DEN3` reader - DMA Enable for XBAR_OUT3"]
pub type DEN3_R = crate::BitReader<DEN3_A>;
#[doc = "DMA Enable for XBAR_OUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEN3_A {
    #[doc = "0: DMA disabled"]
    DEN3_0 = 0,
    #[doc = "1: DMA enabled"]
    DEN3_1 = 1,
}
impl From<DEN3_A> for bool {
    #[inline(always)]
    fn from(variant: DEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl DEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEN3_A {
        match self.bits {
            false => DEN3_A::DEN3_0,
            true => DEN3_A::DEN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `DEN3_0`"]
    #[inline(always)]
    pub fn is_den3_0(&self) -> bool {
        *self == DEN3_A::DEN3_0
    }
    #[doc = "Checks if the value of the field is `DEN3_1`"]
    #[inline(always)]
    pub fn is_den3_1(&self) -> bool {
        *self == DEN3_A::DEN3_1
    }
}
#[doc = "Field `DEN3` writer - DMA Enable for XBAR_OUT3"]
pub type DEN3_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL1_SPEC, DEN3_A, O>;
impl<'a, const O: u8> DEN3_W<'a, O> {
    #[doc = "DMA disabled"]
    #[inline(always)]
    pub fn den3_0(self) -> &'a mut W {
        self.variant(DEN3_A::DEN3_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn den3_1(self) -> &'a mut W {
        self.variant(DEN3_A::DEN3_1)
    }
}
#[doc = "Field `IEN3` reader - Interrupt Enable for XBAR_OUT3"]
pub type IEN3_R = crate::BitReader<IEN3_A>;
#[doc = "Interrupt Enable for XBAR_OUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IEN3_A {
    #[doc = "0: Interrupt disabled"]
    IEN3_0 = 0,
    #[doc = "1: Interrupt enabled"]
    IEN3_1 = 1,
}
impl From<IEN3_A> for bool {
    #[inline(always)]
    fn from(variant: IEN3_A) -> Self {
        variant as u8 != 0
    }
}
impl IEN3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IEN3_A {
        match self.bits {
            false => IEN3_A::IEN3_0,
            true => IEN3_A::IEN3_1,
        }
    }
    #[doc = "Checks if the value of the field is `IEN3_0`"]
    #[inline(always)]
    pub fn is_ien3_0(&self) -> bool {
        *self == IEN3_A::IEN3_0
    }
    #[doc = "Checks if the value of the field is `IEN3_1`"]
    #[inline(always)]
    pub fn is_ien3_1(&self) -> bool {
        *self == IEN3_A::IEN3_1
    }
}
#[doc = "Field `IEN3` writer - Interrupt Enable for XBAR_OUT3"]
pub type IEN3_W<'a, const O: u8> = crate::BitWriter<'a, u16, CTRL1_SPEC, IEN3_A, O>;
impl<'a, const O: u8> IEN3_W<'a, O> {
    #[doc = "Interrupt disabled"]
    #[inline(always)]
    pub fn ien3_0(self) -> &'a mut W {
        self.variant(IEN3_A::IEN3_0)
    }
    #[doc = "Interrupt enabled"]
    #[inline(always)]
    pub fn ien3_1(self) -> &'a mut W {
        self.variant(IEN3_A::IEN3_1)
    }
}
#[doc = "Field `EDGE3` reader - Active edge for edge detection on XBAR_OUT3"]
pub type EDGE3_R = crate::FieldReader<u8, EDGE3_A>;
#[doc = "Active edge for edge detection on XBAR_OUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EDGE3_A {
    #[doc = "0: STS3 never asserts"]
    EDGE3_0 = 0,
    #[doc = "1: STS3 asserts on rising edges of XBAR_OUT3"]
    EDGE3_1 = 1,
    #[doc = "2: STS3 asserts on falling edges of XBAR_OUT3"]
    EDGE3_2 = 2,
    #[doc = "3: STS3 asserts on rising and falling edges of XBAR_OUT3"]
    EDGE3_3 = 3,
}
impl From<EDGE3_A> for u8 {
    #[inline(always)]
    fn from(variant: EDGE3_A) -> Self {
        variant as _
    }
}
impl EDGE3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDGE3_A {
        match self.bits {
            0 => EDGE3_A::EDGE3_0,
            1 => EDGE3_A::EDGE3_1,
            2 => EDGE3_A::EDGE3_2,
            3 => EDGE3_A::EDGE3_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EDGE3_0`"]
    #[inline(always)]
    pub fn is_edge3_0(&self) -> bool {
        *self == EDGE3_A::EDGE3_0
    }
    #[doc = "Checks if the value of the field is `EDGE3_1`"]
    #[inline(always)]
    pub fn is_edge3_1(&self) -> bool {
        *self == EDGE3_A::EDGE3_1
    }
    #[doc = "Checks if the value of the field is `EDGE3_2`"]
    #[inline(always)]
    pub fn is_edge3_2(&self) -> bool {
        *self == EDGE3_A::EDGE3_2
    }
    #[doc = "Checks if the value of the field is `EDGE3_3`"]
    #[inline(always)]
    pub fn is_edge3_3(&self) -> bool {
        *self == EDGE3_A::EDGE3_3
    }
}
#[doc = "Field `EDGE3` writer - Active edge for edge detection on XBAR_OUT3"]
pub type EDGE3_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CTRL1_SPEC, u8, EDGE3_A, 2, O>;
impl<'a, const O: u8> EDGE3_W<'a, O> {
    #[doc = "STS3 never asserts"]
    #[inline(always)]
    pub fn edge3_0(self) -> &'a mut W {
        self.variant(EDGE3_A::EDGE3_0)
    }
    #[doc = "STS3 asserts on rising edges of XBAR_OUT3"]
    #[inline(always)]
    pub fn edge3_1(self) -> &'a mut W {
        self.variant(EDGE3_A::EDGE3_1)
    }
    #[doc = "STS3 asserts on falling edges of XBAR_OUT3"]
    #[inline(always)]
    pub fn edge3_2(self) -> &'a mut W {
        self.variant(EDGE3_A::EDGE3_2)
    }
    #[doc = "STS3 asserts on rising and falling edges of XBAR_OUT3"]
    #[inline(always)]
    pub fn edge3_3(self) -> &'a mut W {
        self.variant(EDGE3_A::EDGE3_3)
    }
}
#[doc = "Field `STS3` reader - Edge detection status for XBAR_OUT3"]
pub type STS3_R = crate::BitReader<STS3_A>;
#[doc = "Edge detection status for XBAR_OUT3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STS3_A {
    #[doc = "0: Active edge not yet detected on XBAR_OUT3"]
    STS3_0 = 0,
    #[doc = "1: Active edge detected on XBAR_OUT3"]
    STS3_1 = 1,
}
impl From<STS3_A> for bool {
    #[inline(always)]
    fn from(variant: STS3_A) -> Self {
        variant as u8 != 0
    }
}
impl STS3_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STS3_A {
        match self.bits {
            false => STS3_A::STS3_0,
            true => STS3_A::STS3_1,
        }
    }
    #[doc = "Checks if the value of the field is `STS3_0`"]
    #[inline(always)]
    pub fn is_sts3_0(&self) -> bool {
        *self == STS3_A::STS3_0
    }
    #[doc = "Checks if the value of the field is `STS3_1`"]
    #[inline(always)]
    pub fn is_sts3_1(&self) -> bool {
        *self == STS3_A::STS3_1
    }
}
#[doc = "Field `STS3` writer - Edge detection status for XBAR_OUT3"]
pub type STS3_W<'a, const O: u8> = crate::BitWriter1C<'a, u16, CTRL1_SPEC, STS3_A, O>;
impl<'a, const O: u8> STS3_W<'a, O> {
    #[doc = "Active edge not yet detected on XBAR_OUT3"]
    #[inline(always)]
    pub fn sts3_0(self) -> &'a mut W {
        self.variant(STS3_A::STS3_0)
    }
    #[doc = "Active edge detected on XBAR_OUT3"]
    #[inline(always)]
    pub fn sts3_1(self) -> &'a mut W {
        self.variant(STS3_A::STS3_1)
    }
}
impl R {
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT2"]
    #[inline(always)]
    pub fn den2(&self) -> DEN2_R {
        DEN2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT2"]
    #[inline(always)]
    pub fn ien2(&self) -> IEN2_R {
        IEN2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT2"]
    #[inline(always)]
    pub fn edge2(&self) -> EDGE2_R {
        EDGE2_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT2"]
    #[inline(always)]
    pub fn sts2(&self) -> STS2_R {
        STS2_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT3"]
    #[inline(always)]
    pub fn den3(&self) -> DEN3_R {
        DEN3_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT3"]
    #[inline(always)]
    pub fn ien3(&self) -> IEN3_R {
        IEN3_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT3"]
    #[inline(always)]
    pub fn edge3(&self) -> EDGE3_R {
        EDGE3_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT3"]
    #[inline(always)]
    pub fn sts3(&self) -> STS3_R {
        STS3_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Enable for XBAR_OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn den2(&mut self) -> DEN2_W<0> {
        DEN2_W::new(self)
    }
    #[doc = "Bit 1 - Interrupt Enable for XBAR_OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn ien2(&mut self) -> IEN2_W<1> {
        IEN2_W::new(self)
    }
    #[doc = "Bits 2:3 - Active edge for edge detection on XBAR_OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn edge2(&mut self) -> EDGE2_W<2> {
        EDGE2_W::new(self)
    }
    #[doc = "Bit 4 - Edge detection status for XBAR_OUT2"]
    #[inline(always)]
    #[must_use]
    pub fn sts2(&mut self) -> STS2_W<4> {
        STS2_W::new(self)
    }
    #[doc = "Bit 8 - DMA Enable for XBAR_OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn den3(&mut self) -> DEN3_W<8> {
        DEN3_W::new(self)
    }
    #[doc = "Bit 9 - Interrupt Enable for XBAR_OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn ien3(&mut self) -> IEN3_W<9> {
        IEN3_W::new(self)
    }
    #[doc = "Bits 10:11 - Active edge for edge detection on XBAR_OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn edge3(&mut self) -> EDGE3_W<10> {
        EDGE3_W::new(self)
    }
    #[doc = "Bit 12 - Edge detection status for XBAR_OUT3"]
    #[inline(always)]
    #[must_use]
    pub fn sts3(&mut self) -> STS3_W<12> {
        STS3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Crossbar A Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ctrl1](index.html) module"]
pub struct CTRL1_SPEC;
impl crate::RegisterSpec for CTRL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [ctrl1::R](R) reader structure"]
impl crate::Readable for CTRL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ctrl1::W](W) writer structure"]
impl crate::Writable for CTRL1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x1010;
}
#[doc = "`reset()` method sets CTRL1 to value 0"]
impl crate::Resettable for CTRL1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
