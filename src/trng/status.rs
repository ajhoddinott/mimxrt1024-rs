#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TF1BR0` reader - Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
pub type TF1BR0_R = crate::BitReader<bool>;
#[doc = "Field `TF1BR1` reader - Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
pub type TF1BR1_R = crate::BitReader<bool>;
#[doc = "Field `TF2BR0` reader - Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
pub type TF2BR0_R = crate::BitReader<bool>;
#[doc = "Field `TF2BR1` reader - Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
pub type TF2BR1_R = crate::BitReader<bool>;
#[doc = "Field `TF3BR0` reader - Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
pub type TF3BR0_R = crate::BitReader<bool>;
#[doc = "Field `TF3BR1` reader - Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
pub type TF3BR1_R = crate::BitReader<bool>;
#[doc = "Field `TF4BR0` reader - Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
pub type TF4BR0_R = crate::BitReader<bool>;
#[doc = "Field `TF4BR1` reader - Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
pub type TF4BR1_R = crate::BitReader<bool>;
#[doc = "Field `TF5BR0` reader - Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
pub type TF5BR0_R = crate::BitReader<bool>;
#[doc = "Field `TF5BR1` reader - Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
pub type TF5BR1_R = crate::BitReader<bool>;
#[doc = "Field `TF6PBR0` reader - Test Fail, 6 Plus Bit Run, Sampling 0s"]
pub type TF6PBR0_R = crate::BitReader<bool>;
#[doc = "Field `TF6PBR1` reader - Test Fail, 6 Plus Bit Run, Sampling 1s"]
pub type TF6PBR1_R = crate::BitReader<bool>;
#[doc = "Field `TFSB` reader - Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
pub type TFSB_R = crate::BitReader<bool>;
#[doc = "Field `TFLR` reader - Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
pub type TFLR_R = crate::BitReader<bool>;
#[doc = "Field `TFP` reader - Test Fail, Poker. If TFP=1, the Poker Test has failed."]
pub type TFP_R = crate::BitReader<bool>;
#[doc = "Field `TFMB` reader - Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
pub type TFMB_R = crate::BitReader<bool>;
#[doc = "Field `RETRY_CT` reader - RETRY COUNT"]
pub type RETRY_CT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bit 0 - Test Fail, 1-Bit Run, Sampling 0s. If TF1BR0=1, the 1-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf1br0(&self) -> TF1BR0_R {
        TF1BR0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Test Fail, 1-Bit Run, Sampling 1s. If TF1BR1=1, the 1-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf1br1(&self) -> TF1BR1_R {
        TF1BR1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Test Fail, 2-Bit Run, Sampling 0s. If TF2BR0=1, the 2-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf2br0(&self) -> TF2BR0_R {
        TF2BR0_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Test Fail, 2-Bit Run, Sampling 1s. If TF2BR1=1, the 2-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf2br1(&self) -> TF2BR1_R {
        TF2BR1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Test Fail, 3-Bit Run, Sampling 0s. If TF3BR0=1, the 3-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf3br0(&self) -> TF3BR0_R {
        TF3BR0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Test Fail, 3-Bit Run, Sampling 1s. If TF3BR1=1, the 3-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf3br1(&self) -> TF3BR1_R {
        TF3BR1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Test Fail, 4-Bit Run, Sampling 0s. If TF4BR0=1, the 4-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf4br0(&self) -> TF4BR0_R {
        TF4BR0_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Fail, 4-Bit Run, Sampling 1s. If TF4BR1=1, the 4-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf4br1(&self) -> TF4BR1_R {
        TF4BR1_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Test Fail, 5-Bit Run, Sampling 0s. If TF5BR0=1, the 5-Bit Run, Sampling 0s Test has failed."]
    #[inline(always)]
    pub fn tf5br0(&self) -> TF5BR0_R {
        TF5BR0_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Test Fail, 5-Bit Run, Sampling 1s. If TF5BR1=1, the 5-Bit Run, Sampling 1s Test has failed."]
    #[inline(always)]
    pub fn tf5br1(&self) -> TF5BR1_R {
        TF5BR1_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Test Fail, 6 Plus Bit Run, Sampling 0s"]
    #[inline(always)]
    pub fn tf6pbr0(&self) -> TF6PBR0_R {
        TF6PBR0_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Test Fail, 6 Plus Bit Run, Sampling 1s"]
    #[inline(always)]
    pub fn tf6pbr1(&self) -> TF6PBR1_R {
        TF6PBR1_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Test Fail, Sparse Bit. If TFSB=1, the Sparse Bit Test has failed."]
    #[inline(always)]
    pub fn tfsb(&self) -> TFSB_R {
        TFSB_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Test Fail, Long Run. If TFLR=1, the Long Run Test has failed."]
    #[inline(always)]
    pub fn tflr(&self) -> TFLR_R {
        TFLR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Test Fail, Poker. If TFP=1, the Poker Test has failed."]
    #[inline(always)]
    pub fn tfp(&self) -> TFP_R {
        TFP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Test Fail, Mono Bit. If TFMB=1, the Mono Bit Test has failed."]
    #[inline(always)]
    pub fn tfmb(&self) -> TFMB_R {
        TFMB_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - RETRY COUNT"]
    #[inline(always)]
    pub fn retry_ct(&self) -> RETRY_CT_R {
        RETRY_CT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
