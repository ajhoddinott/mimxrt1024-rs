#[doc = "Register `HCSPARAMS` reader"]
pub struct R(crate::R<HCSPARAMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HCSPARAMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HCSPARAMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HCSPARAMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `N_PORTS` reader - Number of downstream ports"]
pub type N_PORTS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PPC` reader - Port Power Control This field indicates whether the host controller implementation includes port power control"]
pub type PPC_R = crate::BitReader<bool>;
#[doc = "Field `N_PCC` reader - Number of Ports per Companion Controller This field indicates the number of ports supported per internal Companion Controller"]
pub type N_PCC_R = crate::FieldReader<u8, u8>;
#[doc = "Field `N_CC` reader - Number of Companion Controller (N_CC)"]
pub type N_CC_R = crate::FieldReader<u8, N_CC_A>;
#[doc = "Number of Companion Controller (N_CC)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum N_CC_A {
    #[doc = "0: There is no internal Companion Controller and port-ownership hand-off is not supported."]
    N_CC_0 = 0,
    #[doc = "1: There are internal companion controller(s) and port-ownership hand-offs is supported."]
    N_CC_1 = 1,
}
impl From<N_CC_A> for u8 {
    #[inline(always)]
    fn from(variant: N_CC_A) -> Self {
        variant as _
    }
}
impl N_CC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<N_CC_A> {
        match self.bits {
            0 => Some(N_CC_A::N_CC_0),
            1 => Some(N_CC_A::N_CC_1),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `N_CC_0`"]
    #[inline(always)]
    pub fn is_n_cc_0(&self) -> bool {
        *self == N_CC_A::N_CC_0
    }
    #[doc = "Checks if the value of the field is `N_CC_1`"]
    #[inline(always)]
    pub fn is_n_cc_1(&self) -> bool {
        *self == N_CC_A::N_CC_1
    }
}
#[doc = "Field `PI` reader - Port Indicators (P INDICATOR) This bit indicates whether the ports support port indicator control"]
pub type PI_R = crate::BitReader<bool>;
#[doc = "Field `N_PTT` reader - Number of Ports per Transaction Translator (N_PTT)"]
pub type N_PTT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `N_TT` reader - Number of Transaction Translators (N_TT)"]
pub type N_TT_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Number of downstream ports"]
    #[inline(always)]
    pub fn n_ports(&self) -> N_PORTS_R {
        N_PORTS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Port Power Control This field indicates whether the host controller implementation includes port power control"]
    #[inline(always)]
    pub fn ppc(&self) -> PPC_R {
        PPC_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Number of Ports per Companion Controller This field indicates the number of ports supported per internal Companion Controller"]
    #[inline(always)]
    pub fn n_pcc(&self) -> N_PCC_R {
        N_PCC_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Number of Companion Controller (N_CC)"]
    #[inline(always)]
    pub fn n_cc(&self) -> N_CC_R {
        N_CC_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Port Indicators (P INDICATOR) This bit indicates whether the ports support port indicator control"]
    #[inline(always)]
    pub fn pi(&self) -> PI_R {
        PI_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Number of Ports per Transaction Translator (N_PTT)"]
    #[inline(always)]
    pub fn n_ptt(&self) -> N_PTT_R {
        N_PTT_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Number of Transaction Translators (N_TT)"]
    #[inline(always)]
    pub fn n_tt(&self) -> N_TT_R {
        N_TT_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
#[doc = "Host Controller Structural Parameters\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hcsparams](index.html) module"]
pub struct HCSPARAMS_SPEC;
impl crate::RegisterSpec for HCSPARAMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hcsparams::R](R) reader structure"]
impl crate::Readable for HCSPARAMS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets HCSPARAMS to value 0x0001_0011"]
impl crate::Resettable for HCSPARAMS_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0011;
}
