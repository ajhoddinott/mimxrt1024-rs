#[doc = "Register `VEND_SPEC2` reader"]
pub struct R(crate::R<VEND_SPEC2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<VEND_SPEC2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<VEND_SPEC2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<VEND_SPEC2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `VEND_SPEC2` writer"]
pub struct W(crate::W<VEND_SPEC2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<VEND_SPEC2_SPEC>;
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
impl From<crate::W<VEND_SPEC2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<VEND_SPEC2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CARD_INT_D3_TEST` reader - Card interrupt detection test"]
pub type CARD_INT_D3_TEST_R = crate::BitReader<CARD_INT_D3_TEST_A>;
#[doc = "Card interrupt detection test\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CARD_INT_D3_TEST_A {
    #[doc = "0: Check the card interrupt only when DATA3 is high."]
    CARD_INT_D3_TEST_0 = 0,
    #[doc = "1: Check the card interrupt by ignoring the status of DATA3."]
    CARD_INT_D3_TEST_1 = 1,
}
impl From<CARD_INT_D3_TEST_A> for bool {
    #[inline(always)]
    fn from(variant: CARD_INT_D3_TEST_A) -> Self {
        variant as u8 != 0
    }
}
impl CARD_INT_D3_TEST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CARD_INT_D3_TEST_A {
        match self.bits {
            false => CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0,
            true => CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1,
        }
    }
    #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_0`"]
    #[inline(always)]
    pub fn is_card_int_d3_test_0(&self) -> bool {
        *self == CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0
    }
    #[doc = "Checks if the value of the field is `CARD_INT_D3_TEST_1`"]
    #[inline(always)]
    pub fn is_card_int_d3_test_1(&self) -> bool {
        *self == CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1
    }
}
#[doc = "Field `CARD_INT_D3_TEST` writer - Card interrupt detection test"]
pub type CARD_INT_D3_TEST_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VEND_SPEC2_SPEC, CARD_INT_D3_TEST_A, O>;
impl<'a, const O: u8> CARD_INT_D3_TEST_W<'a, O> {
    #[doc = "Check the card interrupt only when DATA3 is high."]
    #[inline(always)]
    pub fn card_int_d3_test_0(self) -> &'a mut W {
        self.variant(CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_0)
    }
    #[doc = "Check the card interrupt by ignoring the status of DATA3."]
    #[inline(always)]
    pub fn card_int_d3_test_1(self) -> &'a mut W {
        self.variant(CARD_INT_D3_TEST_A::CARD_INT_D3_TEST_1)
    }
}
#[doc = "Field `TUNING_8bit_EN` reader - Tuning 8bit enable"]
pub type TUNING_8BIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TUNING_8bit_EN` writer - Tuning 8bit enable"]
pub type TUNING_8BIT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VEND_SPEC2_SPEC, bool, O>;
#[doc = "Field `TUNING_1bit_EN` reader - Tuning 1bit enable"]
pub type TUNING_1BIT_EN_R = crate::BitReader<bool>;
#[doc = "Field `TUNING_1bit_EN` writer - Tuning 1bit enable"]
pub type TUNING_1BIT_EN_W<'a, const O: u8> = crate::BitWriter<'a, u32, VEND_SPEC2_SPEC, bool, O>;
#[doc = "Field `TUNING_CMD_EN` reader - Tuning command enable"]
pub type TUNING_CMD_EN_R = crate::BitReader<TUNING_CMD_EN_A>;
#[doc = "Tuning command enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TUNING_CMD_EN_A {
    #[doc = "0: Auto tuning circuit does not check the CMD line."]
    TUNING_CMD_EN_0 = 0,
    #[doc = "1: Auto tuning circuit checks the CMD line."]
    TUNING_CMD_EN_1 = 1,
}
impl From<TUNING_CMD_EN_A> for bool {
    #[inline(always)]
    fn from(variant: TUNING_CMD_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl TUNING_CMD_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TUNING_CMD_EN_A {
        match self.bits {
            false => TUNING_CMD_EN_A::TUNING_CMD_EN_0,
            true => TUNING_CMD_EN_A::TUNING_CMD_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `TUNING_CMD_EN_0`"]
    #[inline(always)]
    pub fn is_tuning_cmd_en_0(&self) -> bool {
        *self == TUNING_CMD_EN_A::TUNING_CMD_EN_0
    }
    #[doc = "Checks if the value of the field is `TUNING_CMD_EN_1`"]
    #[inline(always)]
    pub fn is_tuning_cmd_en_1(&self) -> bool {
        *self == TUNING_CMD_EN_A::TUNING_CMD_EN_1
    }
}
#[doc = "Field `TUNING_CMD_EN` writer - Tuning command enable"]
pub type TUNING_CMD_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VEND_SPEC2_SPEC, TUNING_CMD_EN_A, O>;
impl<'a, const O: u8> TUNING_CMD_EN_W<'a, O> {
    #[doc = "Auto tuning circuit does not check the CMD line."]
    #[inline(always)]
    pub fn tuning_cmd_en_0(self) -> &'a mut W {
        self.variant(TUNING_CMD_EN_A::TUNING_CMD_EN_0)
    }
    #[doc = "Auto tuning circuit checks the CMD line."]
    #[inline(always)]
    pub fn tuning_cmd_en_1(self) -> &'a mut W {
        self.variant(TUNING_CMD_EN_A::TUNING_CMD_EN_1)
    }
}
#[doc = "Field `ACMD23_ARGU2_EN` reader - Argument2 register enable for ACMD23"]
pub type ACMD23_ARGU2_EN_R = crate::BitReader<ACMD23_ARGU2_EN_A>;
#[doc = "Argument2 register enable for ACMD23\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACMD23_ARGU2_EN_A {
    #[doc = "0: Disable"]
    ACMD23_ARGU2_EN_0 = 0,
    #[doc = "1: Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enabled."]
    ACMD23_ARGU2_EN_1 = 1,
}
impl From<ACMD23_ARGU2_EN_A> for bool {
    #[inline(always)]
    fn from(variant: ACMD23_ARGU2_EN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACMD23_ARGU2_EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACMD23_ARGU2_EN_A {
        match self.bits {
            false => ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0,
            true => ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACMD23_ARGU2_EN_0`"]
    #[inline(always)]
    pub fn is_acmd23_argu2_en_0(&self) -> bool {
        *self == ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0
    }
    #[doc = "Checks if the value of the field is `ACMD23_ARGU2_EN_1`"]
    #[inline(always)]
    pub fn is_acmd23_argu2_en_1(&self) -> bool {
        *self == ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1
    }
}
#[doc = "Field `ACMD23_ARGU2_EN` writer - Argument2 register enable for ACMD23"]
pub type ACMD23_ARGU2_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, VEND_SPEC2_SPEC, ACMD23_ARGU2_EN_A, O>;
impl<'a, const O: u8> ACMD23_ARGU2_EN_W<'a, O> {
    #[doc = "Disable"]
    #[inline(always)]
    pub fn acmd23_argu2_en_0(self) -> &'a mut W {
        self.variant(ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_0)
    }
    #[doc = "Argument2 register enable for ACMD23 sharing with SDMA system address register. Default is enabled."]
    #[inline(always)]
    pub fn acmd23_argu2_en_1(self) -> &'a mut W {
        self.variant(ACMD23_ARGU2_EN_A::ACMD23_ARGU2_EN_1)
    }
}
impl R {
    #[doc = "Bit 3 - Card interrupt detection test"]
    #[inline(always)]
    pub fn card_int_d3_test(&self) -> CARD_INT_D3_TEST_R {
        CARD_INT_D3_TEST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tuning 8bit enable"]
    #[inline(always)]
    pub fn tuning_8bit_en(&self) -> TUNING_8BIT_EN_R {
        TUNING_8BIT_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Tuning 1bit enable"]
    #[inline(always)]
    pub fn tuning_1bit_en(&self) -> TUNING_1BIT_EN_R {
        TUNING_1BIT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Tuning command enable"]
    #[inline(always)]
    pub fn tuning_cmd_en(&self) -> TUNING_CMD_EN_R {
        TUNING_CMD_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
    #[inline(always)]
    pub fn acmd23_argu2_en(&self) -> ACMD23_ARGU2_EN_R {
        ACMD23_ARGU2_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Card interrupt detection test"]
    #[inline(always)]
    #[must_use]
    pub fn card_int_d3_test(&mut self) -> CARD_INT_D3_TEST_W<3> {
        CARD_INT_D3_TEST_W::new(self)
    }
    #[doc = "Bit 4 - Tuning 8bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_8bit_en(&mut self) -> TUNING_8BIT_EN_W<4> {
        TUNING_8BIT_EN_W::new(self)
    }
    #[doc = "Bit 5 - Tuning 1bit enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_1bit_en(&mut self) -> TUNING_1BIT_EN_W<5> {
        TUNING_1BIT_EN_W::new(self)
    }
    #[doc = "Bit 6 - Tuning command enable"]
    #[inline(always)]
    #[must_use]
    pub fn tuning_cmd_en(&mut self) -> TUNING_CMD_EN_W<6> {
        TUNING_CMD_EN_W::new(self)
    }
    #[doc = "Bit 12 - Argument2 register enable for ACMD23"]
    #[inline(always)]
    #[must_use]
    pub fn acmd23_argu2_en(&mut self) -> ACMD23_ARGU2_EN_W<12> {
        ACMD23_ARGU2_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Vendor Specific 2 Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [vend_spec2](index.html) module"]
pub struct VEND_SPEC2_SPEC;
impl crate::RegisterSpec for VEND_SPEC2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [vend_spec2::R](R) reader structure"]
impl crate::Readable for VEND_SPEC2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [vend_spec2::W](W) writer structure"]
impl crate::Writable for VEND_SPEC2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VEND_SPEC2 to value 0x0001_9006"]
impl crate::Resettable for VEND_SPEC2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_9006;
}
