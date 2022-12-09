#[doc = "Register `IMR` reader"]
pub struct R(crate::R<IMR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IMR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IMR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IMR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HOME` reader - HOME"]
pub type HOME_R = crate::BitReader<bool>;
#[doc = "Field `INDEX` reader - INDEX"]
pub type INDEX_R = crate::BitReader<bool>;
#[doc = "Field `PHB` reader - PHB"]
pub type PHB_R = crate::BitReader<bool>;
#[doc = "Field `PHA` reader - PHA"]
pub type PHA_R = crate::BitReader<bool>;
#[doc = "Field `FHOM` reader - FHOM"]
pub type FHOM_R = crate::BitReader<bool>;
#[doc = "Field `FIND` reader - FIND"]
pub type FIND_R = crate::BitReader<bool>;
#[doc = "Field `FPHB` reader - FPHB"]
pub type FPHB_R = crate::BitReader<bool>;
#[doc = "Field `FPHA` reader - FPHA"]
pub type FPHA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - HOME"]
    #[inline(always)]
    pub fn home(&self) -> HOME_R {
        HOME_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - INDEX"]
    #[inline(always)]
    pub fn index(&self) -> INDEX_R {
        INDEX_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PHB"]
    #[inline(always)]
    pub fn phb(&self) -> PHB_R {
        PHB_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PHA"]
    #[inline(always)]
    pub fn pha(&self) -> PHA_R {
        PHA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FHOM"]
    #[inline(always)]
    pub fn fhom(&self) -> FHOM_R {
        FHOM_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FIND"]
    #[inline(always)]
    pub fn find(&self) -> FIND_R {
        FIND_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FPHB"]
    #[inline(always)]
    pub fn fphb(&self) -> FPHB_R {
        FPHB_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FPHA"]
    #[inline(always)]
    pub fn fpha(&self) -> FPHA_R {
        FPHA_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Input Monitor Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [imr](index.html) module"]
pub struct IMR_SPEC;
impl crate::RegisterSpec for IMR_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [imr::R](R) reader structure"]
impl crate::Readable for IMR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for IMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
