#[doc = "Register `AUTOCMD12_ERR_STATUS` reader"]
pub struct R(crate::R<AUTOCMD12_ERR_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUTOCMD12_ERR_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUTOCMD12_ERR_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUTOCMD12_ERR_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUTOCMD12_ERR_STATUS` writer"]
pub struct W(crate::W<AUTOCMD12_ERR_STATUS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUTOCMD12_ERR_STATUS_SPEC>;
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
impl From<crate::W<AUTOCMD12_ERR_STATUS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUTOCMD12_ERR_STATUS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AC12NE` reader - Auto CMD12 not executed"]
pub type AC12NE_R = crate::BitReader<AC12NE_A>;
#[doc = "Auto CMD12 not executed\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12NE_A {
    #[doc = "0: Executed"]
    AC12NE_0 = 0,
    #[doc = "1: Not executed"]
    AC12NE_1 = 1,
}
impl From<AC12NE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12NE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12NE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12NE_A {
        match self.bits {
            false => AC12NE_A::AC12NE_0,
            true => AC12NE_A::AC12NE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12NE_0`"]
    #[inline(always)]
    pub fn is_ac12ne_0(&self) -> bool {
        *self == AC12NE_A::AC12NE_0
    }
    #[doc = "Checks if the value of the field is `AC12NE_1`"]
    #[inline(always)]
    pub fn is_ac12ne_1(&self) -> bool {
        *self == AC12NE_A::AC12NE_1
    }
}
#[doc = "Field `AC12TOE` reader - Auto CMD12 / 23 timeout error"]
pub type AC12TOE_R = crate::BitReader<AC12TOE_A>;
#[doc = "Auto CMD12 / 23 timeout error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12TOE_A {
    #[doc = "0: No error"]
    AC12TOE_0 = 0,
    #[doc = "1: Time out"]
    AC12TOE_1 = 1,
}
impl From<AC12TOE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12TOE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12TOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12TOE_A {
        match self.bits {
            false => AC12TOE_A::AC12TOE_0,
            true => AC12TOE_A::AC12TOE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12TOE_0`"]
    #[inline(always)]
    pub fn is_ac12toe_0(&self) -> bool {
        *self == AC12TOE_A::AC12TOE_0
    }
    #[doc = "Checks if the value of the field is `AC12TOE_1`"]
    #[inline(always)]
    pub fn is_ac12toe_1(&self) -> bool {
        *self == AC12TOE_A::AC12TOE_1
    }
}
#[doc = "Field `AC12EBE` reader - Auto CMD12 / 23 end bit error"]
pub type AC12EBE_R = crate::BitReader<AC12EBE_A>;
#[doc = "Auto CMD12 / 23 end bit error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12EBE_A {
    #[doc = "0: No error"]
    AC12EBE_0 = 0,
    #[doc = "1: End bit error generated"]
    AC12EBE_1 = 1,
}
impl From<AC12EBE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12EBE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12EBE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12EBE_A {
        match self.bits {
            false => AC12EBE_A::AC12EBE_0,
            true => AC12EBE_A::AC12EBE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12EBE_0`"]
    #[inline(always)]
    pub fn is_ac12ebe_0(&self) -> bool {
        *self == AC12EBE_A::AC12EBE_0
    }
    #[doc = "Checks if the value of the field is `AC12EBE_1`"]
    #[inline(always)]
    pub fn is_ac12ebe_1(&self) -> bool {
        *self == AC12EBE_A::AC12EBE_1
    }
}
#[doc = "Field `AC12CE` reader - Auto CMD12 / 23 CRC error"]
pub type AC12CE_R = crate::BitReader<AC12CE_A>;
#[doc = "Auto CMD12 / 23 CRC error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12CE_A {
    #[doc = "0: No CRC error"]
    AC12CE_0 = 0,
    #[doc = "1: CRC error met in Auto CMD12/23 response"]
    AC12CE_1 = 1,
}
impl From<AC12CE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12CE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12CE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12CE_A {
        match self.bits {
            false => AC12CE_A::AC12CE_0,
            true => AC12CE_A::AC12CE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12CE_0`"]
    #[inline(always)]
    pub fn is_ac12ce_0(&self) -> bool {
        *self == AC12CE_A::AC12CE_0
    }
    #[doc = "Checks if the value of the field is `AC12CE_1`"]
    #[inline(always)]
    pub fn is_ac12ce_1(&self) -> bool {
        *self == AC12CE_A::AC12CE_1
    }
}
#[doc = "Field `AC12IE` reader - Auto CMD12 / 23 index error"]
pub type AC12IE_R = crate::BitReader<AC12IE_A>;
#[doc = "Auto CMD12 / 23 index error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AC12IE_A {
    #[doc = "0: No error"]
    AC12IE_0 = 0,
    #[doc = "1: Error, the CMD index in response is not CMD12/23"]
    AC12IE_1 = 1,
}
impl From<AC12IE_A> for bool {
    #[inline(always)]
    fn from(variant: AC12IE_A) -> Self {
        variant as u8 != 0
    }
}
impl AC12IE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AC12IE_A {
        match self.bits {
            false => AC12IE_A::AC12IE_0,
            true => AC12IE_A::AC12IE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AC12IE_0`"]
    #[inline(always)]
    pub fn is_ac12ie_0(&self) -> bool {
        *self == AC12IE_A::AC12IE_0
    }
    #[doc = "Checks if the value of the field is `AC12IE_1`"]
    #[inline(always)]
    pub fn is_ac12ie_1(&self) -> bool {
        *self == AC12IE_A::AC12IE_1
    }
}
#[doc = "Field `CNIBAC12E` reader - Command not issued by Auto CMD12 error"]
pub type CNIBAC12E_R = crate::BitReader<CNIBAC12E_A>;
#[doc = "Command not issued by Auto CMD12 error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CNIBAC12E_A {
    #[doc = "0: No error"]
    CNIBAC12E_0 = 0,
    #[doc = "1: Not issued"]
    CNIBAC12E_1 = 1,
}
impl From<CNIBAC12E_A> for bool {
    #[inline(always)]
    fn from(variant: CNIBAC12E_A) -> Self {
        variant as u8 != 0
    }
}
impl CNIBAC12E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CNIBAC12E_A {
        match self.bits {
            false => CNIBAC12E_A::CNIBAC12E_0,
            true => CNIBAC12E_A::CNIBAC12E_1,
        }
    }
    #[doc = "Checks if the value of the field is `CNIBAC12E_0`"]
    #[inline(always)]
    pub fn is_cnibac12e_0(&self) -> bool {
        *self == CNIBAC12E_A::CNIBAC12E_0
    }
    #[doc = "Checks if the value of the field is `CNIBAC12E_1`"]
    #[inline(always)]
    pub fn is_cnibac12e_1(&self) -> bool {
        *self == CNIBAC12E_A::CNIBAC12E_1
    }
}
#[doc = "Field `EXECUTE_TUNING` reader - Execute tuning"]
pub type EXECUTE_TUNING_R = crate::BitReader<EXECUTE_TUNING_A>;
#[doc = "Execute tuning\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EXECUTE_TUNING_A {
    #[doc = "0: Tuning procedure is aborted"]
    EXECUTE_TUNING_0 = 0,
    #[doc = "1: Start tuning procedure"]
    EXECUTE_TUNING_1 = 1,
}
impl From<EXECUTE_TUNING_A> for bool {
    #[inline(always)]
    fn from(variant: EXECUTE_TUNING_A) -> Self {
        variant as u8 != 0
    }
}
impl EXECUTE_TUNING_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EXECUTE_TUNING_A {
        match self.bits {
            false => EXECUTE_TUNING_A::EXECUTE_TUNING_0,
            true => EXECUTE_TUNING_A::EXECUTE_TUNING_1,
        }
    }
    #[doc = "Checks if the value of the field is `EXECUTE_TUNING_0`"]
    #[inline(always)]
    pub fn is_execute_tuning_0(&self) -> bool {
        *self == EXECUTE_TUNING_A::EXECUTE_TUNING_0
    }
    #[doc = "Checks if the value of the field is `EXECUTE_TUNING_1`"]
    #[inline(always)]
    pub fn is_execute_tuning_1(&self) -> bool {
        *self == EXECUTE_TUNING_A::EXECUTE_TUNING_1
    }
}
#[doc = "Field `EXECUTE_TUNING` writer - Execute tuning"]
pub type EXECUTE_TUNING_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCMD12_ERR_STATUS_SPEC, EXECUTE_TUNING_A, O>;
impl<'a, const O: u8> EXECUTE_TUNING_W<'a, O> {
    #[doc = "Tuning procedure is aborted"]
    #[inline(always)]
    pub fn execute_tuning_0(self) -> &'a mut W {
        self.variant(EXECUTE_TUNING_A::EXECUTE_TUNING_0)
    }
    #[doc = "Start tuning procedure"]
    #[inline(always)]
    pub fn execute_tuning_1(self) -> &'a mut W {
        self.variant(EXECUTE_TUNING_A::EXECUTE_TUNING_1)
    }
}
#[doc = "Field `SMP_CLK_SEL` reader - Sample clock select"]
pub type SMP_CLK_SEL_R = crate::BitReader<SMP_CLK_SEL_A>;
#[doc = "Sample clock select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMP_CLK_SEL_A {
    #[doc = "0: Fixed clock is used to sample data"]
    SMP_CLK_SEL_0 = 0,
    #[doc = "1: Tuned clock is used to sample data"]
    SMP_CLK_SEL_1 = 1,
}
impl From<SMP_CLK_SEL_A> for bool {
    #[inline(always)]
    fn from(variant: SMP_CLK_SEL_A) -> Self {
        variant as u8 != 0
    }
}
impl SMP_CLK_SEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP_CLK_SEL_A {
        match self.bits {
            false => SMP_CLK_SEL_A::SMP_CLK_SEL_0,
            true => SMP_CLK_SEL_A::SMP_CLK_SEL_1,
        }
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_0`"]
    #[inline(always)]
    pub fn is_smp_clk_sel_0(&self) -> bool {
        *self == SMP_CLK_SEL_A::SMP_CLK_SEL_0
    }
    #[doc = "Checks if the value of the field is `SMP_CLK_SEL_1`"]
    #[inline(always)]
    pub fn is_smp_clk_sel_1(&self) -> bool {
        *self == SMP_CLK_SEL_A::SMP_CLK_SEL_1
    }
}
#[doc = "Field `SMP_CLK_SEL` writer - Sample clock select"]
pub type SMP_CLK_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, AUTOCMD12_ERR_STATUS_SPEC, SMP_CLK_SEL_A, O>;
impl<'a, const O: u8> SMP_CLK_SEL_W<'a, O> {
    #[doc = "Fixed clock is used to sample data"]
    #[inline(always)]
    pub fn smp_clk_sel_0(self) -> &'a mut W {
        self.variant(SMP_CLK_SEL_A::SMP_CLK_SEL_0)
    }
    #[doc = "Tuned clock is used to sample data"]
    #[inline(always)]
    pub fn smp_clk_sel_1(self) -> &'a mut W {
        self.variant(SMP_CLK_SEL_A::SMP_CLK_SEL_1)
    }
}
impl R {
    #[doc = "Bit 0 - Auto CMD12 not executed"]
    #[inline(always)]
    pub fn ac12ne(&self) -> AC12NE_R {
        AC12NE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto CMD12 / 23 timeout error"]
    #[inline(always)]
    pub fn ac12toe(&self) -> AC12TOE_R {
        AC12TOE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto CMD12 / 23 end bit error"]
    #[inline(always)]
    pub fn ac12ebe(&self) -> AC12EBE_R {
        AC12EBE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Auto CMD12 / 23 CRC error"]
    #[inline(always)]
    pub fn ac12ce(&self) -> AC12CE_R {
        AC12CE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Auto CMD12 / 23 index error"]
    #[inline(always)]
    pub fn ac12ie(&self) -> AC12IE_R {
        AC12IE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Command not issued by Auto CMD12 error"]
    #[inline(always)]
    pub fn cnibac12e(&self) -> CNIBAC12E_R {
        CNIBAC12E_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 22 - Execute tuning"]
    #[inline(always)]
    pub fn execute_tuning(&self) -> EXECUTE_TUNING_R {
        EXECUTE_TUNING_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Sample clock select"]
    #[inline(always)]
    pub fn smp_clk_sel(&self) -> SMP_CLK_SEL_R {
        SMP_CLK_SEL_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Execute tuning"]
    #[inline(always)]
    #[must_use]
    pub fn execute_tuning(&mut self) -> EXECUTE_TUNING_W<22> {
        EXECUTE_TUNING_W::new(self)
    }
    #[doc = "Bit 23 - Sample clock select"]
    #[inline(always)]
    #[must_use]
    pub fn smp_clk_sel(&mut self) -> SMP_CLK_SEL_W<23> {
        SMP_CLK_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auto CMD12 Error Status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [autocmd12_err_status](index.html) module"]
pub struct AUTOCMD12_ERR_STATUS_SPEC;
impl crate::RegisterSpec for AUTOCMD12_ERR_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [autocmd12_err_status::R](R) reader structure"]
impl crate::Readable for AUTOCMD12_ERR_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [autocmd12_err_status::W](W) writer structure"]
impl crate::Writable for AUTOCMD12_ERR_STATUS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AUTOCMD12_ERR_STATUS to value 0"]
impl crate::Resettable for AUTOCMD12_ERR_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
