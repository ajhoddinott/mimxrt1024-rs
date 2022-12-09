#[doc = "Register `IFLAG1` reader"]
pub struct R(crate::R<IFLAG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IFLAG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IFLAG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IFLAG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IFLAG1` writer"]
pub struct W(crate::W<IFLAG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IFLAG1_SPEC>;
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
impl From<crate::W<IFLAG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IFLAG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF4TO0I` reader - If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
pub type BUF4TO0I_R = crate::FieldReader<u8, BUF4TO0I_A>;
#[doc = "If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BUF4TO0I_A {
    #[doc = "0: No such occurrence"]
    BUF4TO0I_0 = 0,
    #[doc = "1: Corresponding MB completed transmission/reception"]
    BUF4TO0I_1 = 1,
}
impl From<BUF4TO0I_A> for u8 {
    #[inline(always)]
    fn from(variant: BUF4TO0I_A) -> Self {
        variant as _
    }
}
impl BUF4TO0I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUF4TO0I_A> {
        match self.bits {
            0 => Some(BUF4TO0I_A::BUF4TO0I_0),
            1 => Some(BUF4TO0I_A::BUF4TO0I_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUF4TO0I_0`"]
    #[inline(always)]
    pub fn is_buf4to0i_0(&self) -> bool {
        *self == BUF4TO0I_A::BUF4TO0I_0
    }
    #[doc = "Checks if the value of the field is `BUF4TO0I_1`"]
    #[inline(always)]
    pub fn is_buf4to0i_1(&self) -> bool {
        *self == BUF4TO0I_A::BUF4TO0I_1
    }
}
#[doc = "Field `BUF4TO0I` writer - If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
pub type BUF4TO0I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IFLAG1_SPEC, u8, BUF4TO0I_A, 5, O>;
impl<'a, const O: u8> BUF4TO0I_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf4to0i_0(self) -> &'a mut W {
        self.variant(BUF4TO0I_A::BUF4TO0I_0)
    }
    #[doc = "Corresponding MB completed transmission/reception"]
    #[inline(always)]
    pub fn buf4to0i_1(self) -> &'a mut W {
        self.variant(BUF4TO0I_A::BUF4TO0I_1)
    }
}
#[doc = "Field `BUF5I` reader - If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
pub type BUF5I_R = crate::BitReader<BUF5I_A>;
#[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF5I_A {
    #[doc = "0: No such occurrence"]
    BUF5I_0 = 0,
    #[doc = "1: MB5 completed transmission/reception or frames available in the FIFO"]
    BUF5I_1 = 1,
}
impl From<BUF5I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF5I_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF5I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF5I_A {
        match self.bits {
            false => BUF5I_A::BUF5I_0,
            true => BUF5I_A::BUF5I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF5I_0`"]
    #[inline(always)]
    pub fn is_buf5i_0(&self) -> bool {
        *self == BUF5I_A::BUF5I_0
    }
    #[doc = "Checks if the value of the field is `BUF5I_1`"]
    #[inline(always)]
    pub fn is_buf5i_1(&self) -> bool {
        *self == BUF5I_A::BUF5I_1
    }
}
#[doc = "Field `BUF5I` writer - If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
pub type BUF5I_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF5I_A, O>;
impl<'a, const O: u8> BUF5I_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf5i_0(self) -> &'a mut W {
        self.variant(BUF5I_A::BUF5I_0)
    }
    #[doc = "MB5 completed transmission/reception or frames available in the FIFO"]
    #[inline(always)]
    pub fn buf5i_1(self) -> &'a mut W {
        self.variant(BUF5I_A::BUF5I_1)
    }
}
#[doc = "Field `BUF6I` reader - If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
pub type BUF6I_R = crate::BitReader<BUF6I_A>;
#[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF6I_A {
    #[doc = "0: No such occurrence"]
    BUF6I_0 = 0,
    #[doc = "1: MB6 completed transmission/reception or FIFO almost full"]
    BUF6I_1 = 1,
}
impl From<BUF6I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF6I_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF6I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF6I_A {
        match self.bits {
            false => BUF6I_A::BUF6I_0,
            true => BUF6I_A::BUF6I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF6I_0`"]
    #[inline(always)]
    pub fn is_buf6i_0(&self) -> bool {
        *self == BUF6I_A::BUF6I_0
    }
    #[doc = "Checks if the value of the field is `BUF6I_1`"]
    #[inline(always)]
    pub fn is_buf6i_1(&self) -> bool {
        *self == BUF6I_A::BUF6I_1
    }
}
#[doc = "Field `BUF6I` writer - If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
pub type BUF6I_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF6I_A, O>;
impl<'a, const O: u8> BUF6I_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf6i_0(self) -> &'a mut W {
        self.variant(BUF6I_A::BUF6I_0)
    }
    #[doc = "MB6 completed transmission/reception or FIFO almost full"]
    #[inline(always)]
    pub fn buf6i_1(self) -> &'a mut W {
        self.variant(BUF6I_A::BUF6I_1)
    }
}
#[doc = "Field `BUF7I` reader - If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
pub type BUF7I_R = crate::BitReader<BUF7I_A>;
#[doc = "If the Rx FIFO is not enabled, this bit flags the interrupt for MB7\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUF7I_A {
    #[doc = "0: No such occurrence"]
    BUF7I_0 = 0,
    #[doc = "1: MB7 completed transmission/reception or FIFO overflow"]
    BUF7I_1 = 1,
}
impl From<BUF7I_A> for bool {
    #[inline(always)]
    fn from(variant: BUF7I_A) -> Self {
        variant as u8 != 0
    }
}
impl BUF7I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUF7I_A {
        match self.bits {
            false => BUF7I_A::BUF7I_0,
            true => BUF7I_A::BUF7I_1,
        }
    }
    #[doc = "Checks if the value of the field is `BUF7I_0`"]
    #[inline(always)]
    pub fn is_buf7i_0(&self) -> bool {
        *self == BUF7I_A::BUF7I_0
    }
    #[doc = "Checks if the value of the field is `BUF7I_1`"]
    #[inline(always)]
    pub fn is_buf7i_1(&self) -> bool {
        *self == BUF7I_A::BUF7I_1
    }
}
#[doc = "Field `BUF7I` writer - If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
pub type BUF7I_W<'a, const O: u8> = crate::BitWriter<'a, u32, IFLAG1_SPEC, BUF7I_A, O>;
impl<'a, const O: u8> BUF7I_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf7i_0(self) -> &'a mut W {
        self.variant(BUF7I_A::BUF7I_0)
    }
    #[doc = "MB7 completed transmission/reception or FIFO overflow"]
    #[inline(always)]
    pub fn buf7i_1(self) -> &'a mut W {
        self.variant(BUF7I_A::BUF7I_1)
    }
}
#[doc = "Field `BUF31TO8I` reader - Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
pub type BUF31TO8I_R = crate::FieldReader<u32, BUF31TO8I_A>;
#[doc = "Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum BUF31TO8I_A {
    #[doc = "0: No such occurrence"]
    BUF31TO8I_0 = 0,
    #[doc = "1: The corresponding MB has successfully completed transmission or reception"]
    BUF31TO8I_1 = 1,
}
impl From<BUF31TO8I_A> for u32 {
    #[inline(always)]
    fn from(variant: BUF31TO8I_A) -> Self {
        variant as _
    }
}
impl BUF31TO8I_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BUF31TO8I_A> {
        match self.bits {
            0 => Some(BUF31TO8I_A::BUF31TO8I_0),
            1 => Some(BUF31TO8I_A::BUF31TO8I_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BUF31TO8I_0`"]
    #[inline(always)]
    pub fn is_buf31to8i_0(&self) -> bool {
        *self == BUF31TO8I_A::BUF31TO8I_0
    }
    #[doc = "Checks if the value of the field is `BUF31TO8I_1`"]
    #[inline(always)]
    pub fn is_buf31to8i_1(&self) -> bool {
        *self == BUF31TO8I_A::BUF31TO8I_1
    }
}
#[doc = "Field `BUF31TO8I` writer - Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
pub type BUF31TO8I_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, IFLAG1_SPEC, u32, BUF31TO8I_A, 24, O>;
impl<'a, const O: u8> BUF31TO8I_W<'a, O> {
    #[doc = "No such occurrence"]
    #[inline(always)]
    pub fn buf31to8i_0(self) -> &'a mut W {
        self.variant(BUF31TO8I_A::BUF31TO8I_0)
    }
    #[doc = "The corresponding MB has successfully completed transmission or reception"]
    #[inline(always)]
    pub fn buf31to8i_1(self) -> &'a mut W {
        self.variant(BUF31TO8I_A::BUF31TO8I_1)
    }
}
impl R {
    #[doc = "Bits 0:4 - If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
    #[inline(always)]
    pub fn buf4to0i(&self) -> BUF4TO0I_R {
        BUF4TO0I_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
    #[inline(always)]
    pub fn buf5i(&self) -> BUF5I_R {
        BUF5I_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
    #[inline(always)]
    pub fn buf6i(&self) -> BUF6I_R {
        BUF6I_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
    #[inline(always)]
    pub fn buf7i(&self) -> BUF7I_R {
        BUF7I_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
    #[inline(always)]
    pub fn buf31to8i(&self) -> BUF31TO8I_R {
        BUF31TO8I_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:4 - If the Rx FIFO is not enabled, these bits flag the interrupts for MB0 to MB4"]
    #[inline(always)]
    #[must_use]
    pub fn buf4to0i(&mut self) -> BUF4TO0I_W<0> {
        BUF4TO0I_W::new(self)
    }
    #[doc = "Bit 5 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB5"]
    #[inline(always)]
    #[must_use]
    pub fn buf5i(&mut self) -> BUF5I_W<5> {
        BUF5I_W::new(self)
    }
    #[doc = "Bit 6 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB6"]
    #[inline(always)]
    #[must_use]
    pub fn buf6i(&mut self) -> BUF6I_W<6> {
        BUF6I_W::new(self)
    }
    #[doc = "Bit 7 - If the Rx FIFO is not enabled, this bit flags the interrupt for MB7"]
    #[inline(always)]
    #[must_use]
    pub fn buf7i(&mut self) -> BUF7I_W<7> {
        BUF7I_W::new(self)
    }
    #[doc = "Bits 8:31 - Each bit flags the respective FLEXCAN Message Buffer (MB8 to MB31) interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn buf31to8i(&mut self) -> BUF31TO8I_W<8> {
        BUF31TO8I_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt Flags 1 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [iflag1](index.html) module"]
pub struct IFLAG1_SPEC;
impl crate::RegisterSpec for IFLAG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [iflag1::R](R) reader structure"]
impl crate::Readable for IFLAG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [iflag1::W](W) writer structure"]
impl crate::Writable for IFLAG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IFLAG1 to value 0"]
impl crate::Resettable for IFLAG1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
