#[doc = "Register `BR[%s]` reader"]
pub struct R(crate::R<BR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BR[%s]` writer"]
pub struct W(crate::W<BR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BR_SPEC>;
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
impl From<crate::W<BR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VLD` reader - Valid"]
pub type VLD_R = crate::BitReader<VLD_A>;
#[doc = "Valid\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VLD_A {
    #[doc = "0: The memory is invalid, can not be accessed."]
    VLD_0 = 0,
    #[doc = "1: The memory is valid, can be accessed."]
    VLD_1 = 1,
}
impl From<VLD_A> for bool {
    #[inline(always)]
    fn from(variant: VLD_A) -> Self {
        variant as u8 != 0
    }
}
impl VLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VLD_A {
        match self.bits {
            false => VLD_A::VLD_0,
            true => VLD_A::VLD_1,
        }
    }
    #[doc = "Checks if the value of the field is `VLD_0`"]
    #[inline(always)]
    pub fn is_vld_0(&self) -> bool {
        *self == VLD_A::VLD_0
    }
    #[doc = "Checks if the value of the field is `VLD_1`"]
    #[inline(always)]
    pub fn is_vld_1(&self) -> bool {
        *self == VLD_A::VLD_1
    }
}
#[doc = "Field `VLD` writer - Valid"]
pub type VLD_W<'a, const O: u8> = crate::BitWriter<'a, u32, BR_SPEC, VLD_A, O>;
impl<'a, const O: u8> VLD_W<'a, O> {
    #[doc = "The memory is invalid, can not be accessed."]
    #[inline(always)]
    pub fn vld_0(self) -> &'a mut W {
        self.variant(VLD_A::VLD_0)
    }
    #[doc = "The memory is valid, can be accessed."]
    #[inline(always)]
    pub fn vld_1(self) -> &'a mut W {
        self.variant(VLD_A::VLD_1)
    }
}
#[doc = "Field `MS` reader - Memory size"]
pub type MS_R = crate::FieldReader<u8, MS_A>;
#[doc = "Memory size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MS_A {
    #[doc = "0: 4KB"]
    MS_0 = 0,
    #[doc = "1: 8KB"]
    MS_1 = 1,
    #[doc = "2: 16KB"]
    MS_2 = 2,
    #[doc = "3: 32KB"]
    MS_3 = 3,
    #[doc = "4: 64KB"]
    MS_4 = 4,
    #[doc = "5: 128KB"]
    MS_5 = 5,
    #[doc = "6: 256KB"]
    MS_6 = 6,
    #[doc = "7: 512KB"]
    MS_7 = 7,
    #[doc = "8: 1MB"]
    MS_8 = 8,
    #[doc = "9: 2MB"]
    MS_9 = 9,
    #[doc = "10: 4MB"]
    MS_10 = 10,
    #[doc = "11: 8MB"]
    MS_11 = 11,
    #[doc = "12: 16MB"]
    MS_12 = 12,
    #[doc = "13: 32MB"]
    MS_13 = 13,
    #[doc = "14: 64MB"]
    MS_14 = 14,
    #[doc = "15: 128MB"]
    MS_15 = 15,
    #[doc = "16: 256MB"]
    MS_16 = 16,
    #[doc = "17: 512MB"]
    MS_17 = 17,
    #[doc = "18: 1GB"]
    MS_18 = 18,
    #[doc = "19: 2GB"]
    MS_19 = 19,
    #[doc = "20: 4GB"]
    MS_20 = 20,
    #[doc = "21: 4GB"]
    MS_21 = 21,
    #[doc = "22: 4GB"]
    MS_22 = 22,
    #[doc = "23: 4GB"]
    MS_23 = 23,
    #[doc = "24: 4GB"]
    MS_24 = 24,
    #[doc = "25: 4GB"]
    MS_25 = 25,
    #[doc = "26: 4GB"]
    MS_26 = 26,
    #[doc = "27: 4GB"]
    MS_27 = 27,
    #[doc = "28: 4GB"]
    MS_28 = 28,
    #[doc = "29: 4GB"]
    MS_29 = 29,
    #[doc = "30: 4GB"]
    MS_30 = 30,
    #[doc = "31: 4GB"]
    MS_31 = 31,
}
impl From<MS_A> for u8 {
    #[inline(always)]
    fn from(variant: MS_A) -> Self {
        variant as _
    }
}
impl MS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MS_A {
        match self.bits {
            0 => MS_A::MS_0,
            1 => MS_A::MS_1,
            2 => MS_A::MS_2,
            3 => MS_A::MS_3,
            4 => MS_A::MS_4,
            5 => MS_A::MS_5,
            6 => MS_A::MS_6,
            7 => MS_A::MS_7,
            8 => MS_A::MS_8,
            9 => MS_A::MS_9,
            10 => MS_A::MS_10,
            11 => MS_A::MS_11,
            12 => MS_A::MS_12,
            13 => MS_A::MS_13,
            14 => MS_A::MS_14,
            15 => MS_A::MS_15,
            16 => MS_A::MS_16,
            17 => MS_A::MS_17,
            18 => MS_A::MS_18,
            19 => MS_A::MS_19,
            20 => MS_A::MS_20,
            21 => MS_A::MS_21,
            22 => MS_A::MS_22,
            23 => MS_A::MS_23,
            24 => MS_A::MS_24,
            25 => MS_A::MS_25,
            26 => MS_A::MS_26,
            27 => MS_A::MS_27,
            28 => MS_A::MS_28,
            29 => MS_A::MS_29,
            30 => MS_A::MS_30,
            31 => MS_A::MS_31,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MS_0`"]
    #[inline(always)]
    pub fn is_ms_0(&self) -> bool {
        *self == MS_A::MS_0
    }
    #[doc = "Checks if the value of the field is `MS_1`"]
    #[inline(always)]
    pub fn is_ms_1(&self) -> bool {
        *self == MS_A::MS_1
    }
    #[doc = "Checks if the value of the field is `MS_2`"]
    #[inline(always)]
    pub fn is_ms_2(&self) -> bool {
        *self == MS_A::MS_2
    }
    #[doc = "Checks if the value of the field is `MS_3`"]
    #[inline(always)]
    pub fn is_ms_3(&self) -> bool {
        *self == MS_A::MS_3
    }
    #[doc = "Checks if the value of the field is `MS_4`"]
    #[inline(always)]
    pub fn is_ms_4(&self) -> bool {
        *self == MS_A::MS_4
    }
    #[doc = "Checks if the value of the field is `MS_5`"]
    #[inline(always)]
    pub fn is_ms_5(&self) -> bool {
        *self == MS_A::MS_5
    }
    #[doc = "Checks if the value of the field is `MS_6`"]
    #[inline(always)]
    pub fn is_ms_6(&self) -> bool {
        *self == MS_A::MS_6
    }
    #[doc = "Checks if the value of the field is `MS_7`"]
    #[inline(always)]
    pub fn is_ms_7(&self) -> bool {
        *self == MS_A::MS_7
    }
    #[doc = "Checks if the value of the field is `MS_8`"]
    #[inline(always)]
    pub fn is_ms_8(&self) -> bool {
        *self == MS_A::MS_8
    }
    #[doc = "Checks if the value of the field is `MS_9`"]
    #[inline(always)]
    pub fn is_ms_9(&self) -> bool {
        *self == MS_A::MS_9
    }
    #[doc = "Checks if the value of the field is `MS_10`"]
    #[inline(always)]
    pub fn is_ms_10(&self) -> bool {
        *self == MS_A::MS_10
    }
    #[doc = "Checks if the value of the field is `MS_11`"]
    #[inline(always)]
    pub fn is_ms_11(&self) -> bool {
        *self == MS_A::MS_11
    }
    #[doc = "Checks if the value of the field is `MS_12`"]
    #[inline(always)]
    pub fn is_ms_12(&self) -> bool {
        *self == MS_A::MS_12
    }
    #[doc = "Checks if the value of the field is `MS_13`"]
    #[inline(always)]
    pub fn is_ms_13(&self) -> bool {
        *self == MS_A::MS_13
    }
    #[doc = "Checks if the value of the field is `MS_14`"]
    #[inline(always)]
    pub fn is_ms_14(&self) -> bool {
        *self == MS_A::MS_14
    }
    #[doc = "Checks if the value of the field is `MS_15`"]
    #[inline(always)]
    pub fn is_ms_15(&self) -> bool {
        *self == MS_A::MS_15
    }
    #[doc = "Checks if the value of the field is `MS_16`"]
    #[inline(always)]
    pub fn is_ms_16(&self) -> bool {
        *self == MS_A::MS_16
    }
    #[doc = "Checks if the value of the field is `MS_17`"]
    #[inline(always)]
    pub fn is_ms_17(&self) -> bool {
        *self == MS_A::MS_17
    }
    #[doc = "Checks if the value of the field is `MS_18`"]
    #[inline(always)]
    pub fn is_ms_18(&self) -> bool {
        *self == MS_A::MS_18
    }
    #[doc = "Checks if the value of the field is `MS_19`"]
    #[inline(always)]
    pub fn is_ms_19(&self) -> bool {
        *self == MS_A::MS_19
    }
    #[doc = "Checks if the value of the field is `MS_20`"]
    #[inline(always)]
    pub fn is_ms_20(&self) -> bool {
        *self == MS_A::MS_20
    }
    #[doc = "Checks if the value of the field is `MS_21`"]
    #[inline(always)]
    pub fn is_ms_21(&self) -> bool {
        *self == MS_A::MS_21
    }
    #[doc = "Checks if the value of the field is `MS_22`"]
    #[inline(always)]
    pub fn is_ms_22(&self) -> bool {
        *self == MS_A::MS_22
    }
    #[doc = "Checks if the value of the field is `MS_23`"]
    #[inline(always)]
    pub fn is_ms_23(&self) -> bool {
        *self == MS_A::MS_23
    }
    #[doc = "Checks if the value of the field is `MS_24`"]
    #[inline(always)]
    pub fn is_ms_24(&self) -> bool {
        *self == MS_A::MS_24
    }
    #[doc = "Checks if the value of the field is `MS_25`"]
    #[inline(always)]
    pub fn is_ms_25(&self) -> bool {
        *self == MS_A::MS_25
    }
    #[doc = "Checks if the value of the field is `MS_26`"]
    #[inline(always)]
    pub fn is_ms_26(&self) -> bool {
        *self == MS_A::MS_26
    }
    #[doc = "Checks if the value of the field is `MS_27`"]
    #[inline(always)]
    pub fn is_ms_27(&self) -> bool {
        *self == MS_A::MS_27
    }
    #[doc = "Checks if the value of the field is `MS_28`"]
    #[inline(always)]
    pub fn is_ms_28(&self) -> bool {
        *self == MS_A::MS_28
    }
    #[doc = "Checks if the value of the field is `MS_29`"]
    #[inline(always)]
    pub fn is_ms_29(&self) -> bool {
        *self == MS_A::MS_29
    }
    #[doc = "Checks if the value of the field is `MS_30`"]
    #[inline(always)]
    pub fn is_ms_30(&self) -> bool {
        *self == MS_A::MS_30
    }
    #[doc = "Checks if the value of the field is `MS_31`"]
    #[inline(always)]
    pub fn is_ms_31(&self) -> bool {
        *self == MS_A::MS_31
    }
}
#[doc = "Field `MS` writer - Memory size"]
pub type MS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, BR_SPEC, u8, MS_A, 5, O>;
impl<'a, const O: u8> MS_W<'a, O> {
    #[doc = "4KB"]
    #[inline(always)]
    pub fn ms_0(self) -> &'a mut W {
        self.variant(MS_A::MS_0)
    }
    #[doc = "8KB"]
    #[inline(always)]
    pub fn ms_1(self) -> &'a mut W {
        self.variant(MS_A::MS_1)
    }
    #[doc = "16KB"]
    #[inline(always)]
    pub fn ms_2(self) -> &'a mut W {
        self.variant(MS_A::MS_2)
    }
    #[doc = "32KB"]
    #[inline(always)]
    pub fn ms_3(self) -> &'a mut W {
        self.variant(MS_A::MS_3)
    }
    #[doc = "64KB"]
    #[inline(always)]
    pub fn ms_4(self) -> &'a mut W {
        self.variant(MS_A::MS_4)
    }
    #[doc = "128KB"]
    #[inline(always)]
    pub fn ms_5(self) -> &'a mut W {
        self.variant(MS_A::MS_5)
    }
    #[doc = "256KB"]
    #[inline(always)]
    pub fn ms_6(self) -> &'a mut W {
        self.variant(MS_A::MS_6)
    }
    #[doc = "512KB"]
    #[inline(always)]
    pub fn ms_7(self) -> &'a mut W {
        self.variant(MS_A::MS_7)
    }
    #[doc = "1MB"]
    #[inline(always)]
    pub fn ms_8(self) -> &'a mut W {
        self.variant(MS_A::MS_8)
    }
    #[doc = "2MB"]
    #[inline(always)]
    pub fn ms_9(self) -> &'a mut W {
        self.variant(MS_A::MS_9)
    }
    #[doc = "4MB"]
    #[inline(always)]
    pub fn ms_10(self) -> &'a mut W {
        self.variant(MS_A::MS_10)
    }
    #[doc = "8MB"]
    #[inline(always)]
    pub fn ms_11(self) -> &'a mut W {
        self.variant(MS_A::MS_11)
    }
    #[doc = "16MB"]
    #[inline(always)]
    pub fn ms_12(self) -> &'a mut W {
        self.variant(MS_A::MS_12)
    }
    #[doc = "32MB"]
    #[inline(always)]
    pub fn ms_13(self) -> &'a mut W {
        self.variant(MS_A::MS_13)
    }
    #[doc = "64MB"]
    #[inline(always)]
    pub fn ms_14(self) -> &'a mut W {
        self.variant(MS_A::MS_14)
    }
    #[doc = "128MB"]
    #[inline(always)]
    pub fn ms_15(self) -> &'a mut W {
        self.variant(MS_A::MS_15)
    }
    #[doc = "256MB"]
    #[inline(always)]
    pub fn ms_16(self) -> &'a mut W {
        self.variant(MS_A::MS_16)
    }
    #[doc = "512MB"]
    #[inline(always)]
    pub fn ms_17(self) -> &'a mut W {
        self.variant(MS_A::MS_17)
    }
    #[doc = "1GB"]
    #[inline(always)]
    pub fn ms_18(self) -> &'a mut W {
        self.variant(MS_A::MS_18)
    }
    #[doc = "2GB"]
    #[inline(always)]
    pub fn ms_19(self) -> &'a mut W {
        self.variant(MS_A::MS_19)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_20(self) -> &'a mut W {
        self.variant(MS_A::MS_20)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_21(self) -> &'a mut W {
        self.variant(MS_A::MS_21)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_22(self) -> &'a mut W {
        self.variant(MS_A::MS_22)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_23(self) -> &'a mut W {
        self.variant(MS_A::MS_23)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_24(self) -> &'a mut W {
        self.variant(MS_A::MS_24)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_25(self) -> &'a mut W {
        self.variant(MS_A::MS_25)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_26(self) -> &'a mut W {
        self.variant(MS_A::MS_26)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_27(self) -> &'a mut W {
        self.variant(MS_A::MS_27)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_28(self) -> &'a mut W {
        self.variant(MS_A::MS_28)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_29(self) -> &'a mut W {
        self.variant(MS_A::MS_29)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_30(self) -> &'a mut W {
        self.variant(MS_A::MS_30)
    }
    #[doc = "4GB"]
    #[inline(always)]
    pub fn ms_31(self) -> &'a mut W {
        self.variant(MS_A::MS_31)
    }
}
#[doc = "Field `BA` reader - Base Address"]
pub type BA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `BA` writer - Base Address"]
pub type BA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, BR_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    pub fn vld(&self) -> VLD_R {
        VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Memory size"]
    #[inline(always)]
    pub fn ms(&self) -> MS_R {
        MS_R::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 12:31 - Base Address"]
    #[inline(always)]
    pub fn ba(&self) -> BA_R {
        BA_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Valid"]
    #[inline(always)]
    #[must_use]
    pub fn vld(&mut self) -> VLD_W<0> {
        VLD_W::new(self)
    }
    #[doc = "Bits 1:5 - Memory size"]
    #[inline(always)]
    #[must_use]
    pub fn ms(&mut self) -> MS_W<1> {
        MS_W::new(self)
    }
    #[doc = "Bits 12:31 - Base Address"]
    #[inline(always)]
    #[must_use]
    pub fn ba(&mut self) -> BA_W<12> {
        BA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Base Register n\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [br](index.html) module"]
pub struct BR_SPEC;
impl crate::RegisterSpec for BR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [br::R](R) reader structure"]
impl crate::Readable for BR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [br::W](W) writer structure"]
impl crate::Writable for BR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BR[%s]
to value 0"]
impl crate::Resettable for BR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
