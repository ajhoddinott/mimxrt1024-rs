#[doc = "Register `BLK_ATT` reader"]
pub struct R(crate::R<BLK_ATT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK_ATT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK_ATT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK_ATT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BLK_ATT` writer"]
pub struct W(crate::W<BLK_ATT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BLK_ATT_SPEC>;
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
impl From<crate::W<BLK_ATT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BLK_ATT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BLKSIZE` reader - Transfer block size"]
pub type BLKSIZE_R = crate::FieldReader<u16, BLKSIZE_A>;
#[doc = "Transfer block size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLKSIZE_A {
    #[doc = "0: No data transfer"]
    BLKSIZE_0 = 0,
    #[doc = "1: 1 byte"]
    BLKSIZE_1 = 1,
    #[doc = "2: 2 bytes"]
    BLKSIZE_2 = 2,
    #[doc = "3: 3 bytes"]
    BLKSIZE_3 = 3,
    #[doc = "4: 4 bytes"]
    BLKSIZE_4 = 4,
    #[doc = "511: 511 bytes"]
    BLKSIZE_511 = 511,
    #[doc = "512: 512 bytes"]
    BLKSIZE_512 = 512,
    #[doc = "2048: 2048 bytes"]
    BLKSIZE_2048 = 2048,
    #[doc = "4096: 4096 bytes"]
    BLKSIZE_4096 = 4096,
}
impl From<BLKSIZE_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKSIZE_A) -> Self {
        variant as _
    }
}
impl BLKSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLKSIZE_A> {
        match self.bits {
            0 => Some(BLKSIZE_A::BLKSIZE_0),
            1 => Some(BLKSIZE_A::BLKSIZE_1),
            2 => Some(BLKSIZE_A::BLKSIZE_2),
            3 => Some(BLKSIZE_A::BLKSIZE_3),
            4 => Some(BLKSIZE_A::BLKSIZE_4),
            511 => Some(BLKSIZE_A::BLKSIZE_511),
            512 => Some(BLKSIZE_A::BLKSIZE_512),
            2048 => Some(BLKSIZE_A::BLKSIZE_2048),
            4096 => Some(BLKSIZE_A::BLKSIZE_4096),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_0`"]
    #[inline(always)]
    pub fn is_blksize_0(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_0
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_1`"]
    #[inline(always)]
    pub fn is_blksize_1(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_1
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_2`"]
    #[inline(always)]
    pub fn is_blksize_2(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_2
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_3`"]
    #[inline(always)]
    pub fn is_blksize_3(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_3
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_4`"]
    #[inline(always)]
    pub fn is_blksize_4(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_4
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_511`"]
    #[inline(always)]
    pub fn is_blksize_511(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_511
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_512`"]
    #[inline(always)]
    pub fn is_blksize_512(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_512
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_2048`"]
    #[inline(always)]
    pub fn is_blksize_2048(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_2048
    }
    #[doc = "Checks if the value of the field is `BLKSIZE_4096`"]
    #[inline(always)]
    pub fn is_blksize_4096(&self) -> bool {
        *self == BLKSIZE_A::BLKSIZE_4096
    }
}
#[doc = "Field `BLKSIZE` writer - Transfer block size"]
pub type BLKSIZE_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLK_ATT_SPEC, u16, BLKSIZE_A, 13, O>;
impl<'a, const O: u8> BLKSIZE_W<'a, O> {
    #[doc = "No data transfer"]
    #[inline(always)]
    pub fn blksize_0(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_0)
    }
    #[doc = "1 byte"]
    #[inline(always)]
    pub fn blksize_1(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_1)
    }
    #[doc = "2 bytes"]
    #[inline(always)]
    pub fn blksize_2(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_2)
    }
    #[doc = "3 bytes"]
    #[inline(always)]
    pub fn blksize_3(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_3)
    }
    #[doc = "4 bytes"]
    #[inline(always)]
    pub fn blksize_4(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_4)
    }
    #[doc = "511 bytes"]
    #[inline(always)]
    pub fn blksize_511(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_511)
    }
    #[doc = "512 bytes"]
    #[inline(always)]
    pub fn blksize_512(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_512)
    }
    #[doc = "2048 bytes"]
    #[inline(always)]
    pub fn blksize_2048(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_2048)
    }
    #[doc = "4096 bytes"]
    #[inline(always)]
    pub fn blksize_4096(self) -> &'a mut W {
        self.variant(BLKSIZE_A::BLKSIZE_4096)
    }
}
#[doc = "Field `BLKCNT` reader - Blocks count for current transfer"]
pub type BLKCNT_R = crate::FieldReader<u16, BLKCNT_A>;
#[doc = "Blocks count for current transfer\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BLKCNT_A {
    #[doc = "0: Stop count"]
    BLKCNT_0 = 0,
    #[doc = "1: 1 block"]
    BLKCNT_1 = 1,
    #[doc = "2: 2 blocks"]
    BLKCNT_2 = 2,
    #[doc = "65535: 65535 blocks"]
    BLKCNT_65535 = 65535,
}
impl From<BLKCNT_A> for u16 {
    #[inline(always)]
    fn from(variant: BLKCNT_A) -> Self {
        variant as _
    }
}
impl BLKCNT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<BLKCNT_A> {
        match self.bits {
            0 => Some(BLKCNT_A::BLKCNT_0),
            1 => Some(BLKCNT_A::BLKCNT_1),
            2 => Some(BLKCNT_A::BLKCNT_2),
            65535 => Some(BLKCNT_A::BLKCNT_65535),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BLKCNT_0`"]
    #[inline(always)]
    pub fn is_blkcnt_0(&self) -> bool {
        *self == BLKCNT_A::BLKCNT_0
    }
    #[doc = "Checks if the value of the field is `BLKCNT_1`"]
    #[inline(always)]
    pub fn is_blkcnt_1(&self) -> bool {
        *self == BLKCNT_A::BLKCNT_1
    }
    #[doc = "Checks if the value of the field is `BLKCNT_2`"]
    #[inline(always)]
    pub fn is_blkcnt_2(&self) -> bool {
        *self == BLKCNT_A::BLKCNT_2
    }
    #[doc = "Checks if the value of the field is `BLKCNT_65535`"]
    #[inline(always)]
    pub fn is_blkcnt_65535(&self) -> bool {
        *self == BLKCNT_A::BLKCNT_65535
    }
}
#[doc = "Field `BLKCNT` writer - Blocks count for current transfer"]
pub type BLKCNT_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, BLK_ATT_SPEC, u16, BLKCNT_A, 16, O>;
impl<'a, const O: u8> BLKCNT_W<'a, O> {
    #[doc = "Stop count"]
    #[inline(always)]
    pub fn blkcnt_0(self) -> &'a mut W {
        self.variant(BLKCNT_A::BLKCNT_0)
    }
    #[doc = "1 block"]
    #[inline(always)]
    pub fn blkcnt_1(self) -> &'a mut W {
        self.variant(BLKCNT_A::BLKCNT_1)
    }
    #[doc = "2 blocks"]
    #[inline(always)]
    pub fn blkcnt_2(self) -> &'a mut W {
        self.variant(BLKCNT_A::BLKCNT_2)
    }
    #[doc = "65535 blocks"]
    #[inline(always)]
    pub fn blkcnt_65535(self) -> &'a mut W {
        self.variant(BLKCNT_A::BLKCNT_65535)
    }
}
impl R {
    #[doc = "Bits 0:12 - Transfer block size"]
    #[inline(always)]
    pub fn blksize(&self) -> BLKSIZE_R {
        BLKSIZE_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:31 - Blocks count for current transfer"]
    #[inline(always)]
    pub fn blkcnt(&self) -> BLKCNT_R {
        BLKCNT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - Transfer block size"]
    #[inline(always)]
    #[must_use]
    pub fn blksize(&mut self) -> BLKSIZE_W<0> {
        BLKSIZE_W::new(self)
    }
    #[doc = "Bits 16:31 - Blocks count for current transfer"]
    #[inline(always)]
    #[must_use]
    pub fn blkcnt(&mut self) -> BLKCNT_W<16> {
        BLKCNT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Block Attributes\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk_att](index.html) module"]
pub struct BLK_ATT_SPEC;
impl crate::RegisterSpec for BLK_ATT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk_att::R](R) reader structure"]
impl crate::Readable for BLK_ATT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [blk_att::W](W) writer structure"]
impl crate::Writable for BLK_ATT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BLK_ATT to value 0x0001_0000"]
impl crate::Resettable for BLK_ATT_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
