#[doc = "Register `FLSHCR2%s` reader"]
pub struct R(crate::R<FLSHCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<FLSHCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<FLSHCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<FLSHCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `FLSHCR2%s` writer"]
pub struct W(crate::W<FLSHCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<FLSHCR2_SPEC>;
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
impl From<crate::W<FLSHCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<FLSHCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ARDSEQID` reader - Sequence Index for AHB Read triggered Command in LUT."]
pub type ARDSEQID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARDSEQID` writer - Sequence Index for AHB Read triggered Command in LUT."]
pub type ARDSEQID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHCR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `ARDSEQNUM` reader - Sequence Number for AHB Read triggered Command in LUT."]
pub type ARDSEQNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARDSEQNUM` writer - Sequence Number for AHB Read triggered Command in LUT."]
pub type ARDSEQNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `AWRSEQID` reader - Sequence Index for AHB Write triggered Command."]
pub type AWRSEQID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWRSEQID` writer - Sequence Index for AHB Write triggered Command."]
pub type AWRSEQID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHCR2_SPEC, u8, u8, 4, O>;
#[doc = "Field `AWRSEQNUM` reader - Sequence Number for AHB Write triggered Command."]
pub type AWRSEQNUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AWRSEQNUM` writer - Sequence Number for AHB Write triggered Command."]
pub type AWRSEQNUM_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHCR2_SPEC, u8, u8, 3, O>;
#[doc = "Field `AWRWAIT` reader - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
pub type AWRWAIT_R = crate::FieldReader<u16, u16>;
#[doc = "Field `AWRWAIT` writer - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
pub type AWRWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, FLSHCR2_SPEC, u16, u16, 12, O>;
#[doc = "Field `AWRWAITUNIT` reader - AWRWAIT unit"]
pub type AWRWAITUNIT_R = crate::FieldReader<u8, AWRWAITUNIT_A>;
#[doc = "AWRWAIT unit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AWRWAITUNIT_A {
    #[doc = "0: The AWRWAIT unit is 2 ahb clock cycle"]
    AWRWAITUNIT_0 = 0,
    #[doc = "1: The AWRWAIT unit is 8 ahb clock cycle"]
    AWRWAITUNIT_1 = 1,
    #[doc = "2: The AWRWAIT unit is 32 ahb clock cycle"]
    AWRWAITUNIT_2 = 2,
    #[doc = "3: The AWRWAIT unit is 128 ahb clock cycle"]
    AWRWAITUNIT_3 = 3,
    #[doc = "4: The AWRWAIT unit is 512 ahb clock cycle"]
    AWRWAITUNIT_4 = 4,
    #[doc = "5: The AWRWAIT unit is 2048 ahb clock cycle"]
    AWRWAITUNIT_5 = 5,
    #[doc = "6: The AWRWAIT unit is 8192 ahb clock cycle"]
    AWRWAITUNIT_6 = 6,
    #[doc = "7: The AWRWAIT unit is 32768 ahb clock cycle"]
    AWRWAITUNIT_7 = 7,
}
impl From<AWRWAITUNIT_A> for u8 {
    #[inline(always)]
    fn from(variant: AWRWAITUNIT_A) -> Self {
        variant as _
    }
}
impl AWRWAITUNIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AWRWAITUNIT_A {
        match self.bits {
            0 => AWRWAITUNIT_A::AWRWAITUNIT_0,
            1 => AWRWAITUNIT_A::AWRWAITUNIT_1,
            2 => AWRWAITUNIT_A::AWRWAITUNIT_2,
            3 => AWRWAITUNIT_A::AWRWAITUNIT_3,
            4 => AWRWAITUNIT_A::AWRWAITUNIT_4,
            5 => AWRWAITUNIT_A::AWRWAITUNIT_5,
            6 => AWRWAITUNIT_A::AWRWAITUNIT_6,
            7 => AWRWAITUNIT_A::AWRWAITUNIT_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_0`"]
    #[inline(always)]
    pub fn is_awrwaitunit_0(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_0
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_1`"]
    #[inline(always)]
    pub fn is_awrwaitunit_1(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_1
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_2`"]
    #[inline(always)]
    pub fn is_awrwaitunit_2(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_2
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_3`"]
    #[inline(always)]
    pub fn is_awrwaitunit_3(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_3
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_4`"]
    #[inline(always)]
    pub fn is_awrwaitunit_4(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_4
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_5`"]
    #[inline(always)]
    pub fn is_awrwaitunit_5(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_5
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_6`"]
    #[inline(always)]
    pub fn is_awrwaitunit_6(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_6
    }
    #[doc = "Checks if the value of the field is `AWRWAITUNIT_7`"]
    #[inline(always)]
    pub fn is_awrwaitunit_7(&self) -> bool {
        *self == AWRWAITUNIT_A::AWRWAITUNIT_7
    }
}
#[doc = "Field `AWRWAITUNIT` writer - AWRWAIT unit"]
pub type AWRWAITUNIT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u32, FLSHCR2_SPEC, u8, AWRWAITUNIT_A, 3, O>;
impl<'a, const O: u8> AWRWAITUNIT_W<'a, O> {
    #[doc = "The AWRWAIT unit is 2 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_0(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_0)
    }
    #[doc = "The AWRWAIT unit is 8 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_1(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_1)
    }
    #[doc = "The AWRWAIT unit is 32 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_2(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_2)
    }
    #[doc = "The AWRWAIT unit is 128 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_3(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_3)
    }
    #[doc = "The AWRWAIT unit is 512 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_4(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_4)
    }
    #[doc = "The AWRWAIT unit is 2048 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_5(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_5)
    }
    #[doc = "The AWRWAIT unit is 8192 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_6(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_6)
    }
    #[doc = "The AWRWAIT unit is 32768 ahb clock cycle"]
    #[inline(always)]
    pub fn awrwaitunit_7(self) -> &'a mut W {
        self.variant(AWRWAITUNIT_A::AWRWAITUNIT_7)
    }
}
#[doc = "Field `CLRINSTRPTR` reader - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
pub type CLRINSTRPTR_R = crate::BitReader<bool>;
#[doc = "Field `CLRINSTRPTR` writer - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
pub type CLRINSTRPTR_W<'a, const O: u8> = crate::BitWriter<'a, u32, FLSHCR2_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:3 - Sequence Index for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn ardseqid(&self) -> ARDSEQID_R {
        ARDSEQID_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:7 - Sequence Number for AHB Read triggered Command in LUT."]
    #[inline(always)]
    pub fn ardseqnum(&self) -> ARDSEQNUM_R {
        ARDSEQNUM_R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Sequence Index for AHB Write triggered Command."]
    #[inline(always)]
    pub fn awrseqid(&self) -> AWRSEQID_R {
        AWRSEQID_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 13:15 - Sequence Number for AHB Write triggered Command."]
    #[inline(always)]
    pub fn awrseqnum(&self) -> AWRSEQNUM_R {
        AWRSEQNUM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:27 - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline(always)]
    pub fn awrwait(&self) -> AWRWAIT_R {
        AWRWAIT_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
    #[doc = "Bits 28:30 - AWRWAIT unit"]
    #[inline(always)]
    pub fn awrwaitunit(&self) -> AWRWAITUNIT_R {
        AWRWAITUNIT_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline(always)]
    pub fn clrinstrptr(&self) -> CLRINSTRPTR_R {
        CLRINSTRPTR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Sequence Index for AHB Read triggered Command in LUT."]
    #[inline(always)]
    #[must_use]
    pub fn ardseqid(&mut self) -> ARDSEQID_W<0> {
        ARDSEQID_W::new(self)
    }
    #[doc = "Bits 5:7 - Sequence Number for AHB Read triggered Command in LUT."]
    #[inline(always)]
    #[must_use]
    pub fn ardseqnum(&mut self) -> ARDSEQNUM_W<5> {
        ARDSEQNUM_W::new(self)
    }
    #[doc = "Bits 8:11 - Sequence Index for AHB Write triggered Command."]
    #[inline(always)]
    #[must_use]
    pub fn awrseqid(&mut self) -> AWRSEQID_W<8> {
        AWRSEQID_W::new(self)
    }
    #[doc = "Bits 13:15 - Sequence Number for AHB Write triggered Command."]
    #[inline(always)]
    #[must_use]
    pub fn awrseqnum(&mut self) -> AWRSEQNUM_W<13> {
        AWRSEQNUM_W::new(self)
    }
    #[doc = "Bits 16:27 - For certain devices (such as FPGA), it need some time to write data into internal memory after the command sequences finished on FlexSPI interface"]
    #[inline(always)]
    #[must_use]
    pub fn awrwait(&mut self) -> AWRWAIT_W<16> {
        AWRWAIT_W::new(self)
    }
    #[doc = "Bits 28:30 - AWRWAIT unit"]
    #[inline(always)]
    #[must_use]
    pub fn awrwaitunit(&mut self) -> AWRWAITUNIT_W<28> {
        AWRWAITUNIT_W::new(self)
    }
    #[doc = "Bit 31 - Clear the instruction pointer which is internally saved pointer by JMP_ON_CS. Refer Programmable Sequence Engine for details."]
    #[inline(always)]
    #[must_use]
    pub fn clrinstrptr(&mut self) -> CLRINSTRPTR_W<31> {
        CLRINSTRPTR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Flash Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [flshcr2](index.html) module"]
pub struct FLSHCR2_SPEC;
impl crate::RegisterSpec for FLSHCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [flshcr2::R](R) reader structure"]
impl crate::Readable for FLSHCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [flshcr2::W](W) writer structure"]
impl crate::Writable for FLSHCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLSHCR2%s to value 0"]
impl crate::Resettable for FLSHCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
