#[doc = "Register `HS` reader"]
pub struct R(crate::R<HS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COCO0` reader - Conversion Complete Flag"]
pub type COCO0_R = crate::BitReader<bool>;
#[doc = "Field `COCO1` reader - Conversion Complete Flag"]
pub type COCO1_R = crate::BitReader<bool>;
#[doc = "Field `COCO2` reader - See description for COCO1."]
pub type COCO2_R = crate::BitReader<bool>;
#[doc = "Field `COCO3` reader - See description for COCO1."]
pub type COCO3_R = crate::BitReader<bool>;
#[doc = "Field `COCO4` reader - See description for COCO1."]
pub type COCO4_R = crate::BitReader<bool>;
#[doc = "Field `COCO5` reader - See description for COCO1."]
pub type COCO5_R = crate::BitReader<bool>;
#[doc = "Field `COCO6` reader - See description for COCO1."]
pub type COCO6_R = crate::BitReader<bool>;
#[doc = "Field `COCO7` reader - See description for COCO1."]
pub type COCO7_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - Conversion Complete Flag"]
    #[inline(always)]
    pub fn coco0(&self) -> COCO0_R {
        COCO0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Conversion Complete Flag"]
    #[inline(always)]
    pub fn coco1(&self) -> COCO1_R {
        COCO1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - See description for COCO1."]
    #[inline(always)]
    pub fn coco2(&self) -> COCO2_R {
        COCO2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - See description for COCO1."]
    #[inline(always)]
    pub fn coco3(&self) -> COCO3_R {
        COCO3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - See description for COCO1."]
    #[inline(always)]
    pub fn coco4(&self) -> COCO4_R {
        COCO4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - See description for COCO1."]
    #[inline(always)]
    pub fn coco5(&self) -> COCO5_R {
        COCO5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - See description for COCO1."]
    #[inline(always)]
    pub fn coco6(&self) -> COCO6_R {
        COCO6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - See description for COCO1."]
    #[inline(always)]
    pub fn coco7(&self) -> COCO7_R {
        COCO7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Status register for HW triggers\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hs](index.html) module"]
pub struct HS_SPEC;
impl crate::RegisterSpec for HS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hs::R](R) reader structure"]
impl crate::Readable for HS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HS to value 0"]
impl crate::Resettable for HS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
