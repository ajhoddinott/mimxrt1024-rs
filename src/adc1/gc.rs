#[doc = "Register `GC` reader"]
pub struct R(crate::R<GC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GC` writer"]
pub struct W(crate::W<GC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GC_SPEC>;
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
impl From<crate::W<GC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADACKEN` reader - Asynchronous clock output enable"]
pub type ADACKEN_R = crate::BitReader<ADACKEN_A>;
#[doc = "Asynchronous clock output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADACKEN_A {
    #[doc = "0: Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    ADACKEN_0 = 0,
    #[doc = "1: Asynchronous clock and clock output enabled regardless of the state of the ADC"]
    ADACKEN_1 = 1,
}
impl From<ADACKEN_A> for bool {
    #[inline(always)]
    fn from(variant: ADACKEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ADACKEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADACKEN_A {
        match self.bits {
            false => ADACKEN_A::ADACKEN_0,
            true => ADACKEN_A::ADACKEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADACKEN_0`"]
    #[inline(always)]
    pub fn is_adacken_0(&self) -> bool {
        *self == ADACKEN_A::ADACKEN_0
    }
    #[doc = "Checks if the value of the field is `ADACKEN_1`"]
    #[inline(always)]
    pub fn is_adacken_1(&self) -> bool {
        *self == ADACKEN_A::ADACKEN_1
    }
}
#[doc = "Field `ADACKEN` writer - Asynchronous clock output enable"]
pub type ADACKEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, ADACKEN_A, O>;
impl<'a, const O: u8> ADACKEN_W<'a, O> {
    #[doc = "Asynchronous clock output disabled; Asynchronous clock only enabled if selected by ADICLK and a conversion is active."]
    #[inline(always)]
    pub fn adacken_0(self) -> &'a mut W {
        self.variant(ADACKEN_A::ADACKEN_0)
    }
    #[doc = "Asynchronous clock and clock output enabled regardless of the state of the ADC"]
    #[inline(always)]
    pub fn adacken_1(self) -> &'a mut W {
        self.variant(ADACKEN_A::ADACKEN_1)
    }
}
#[doc = "Field `DMAEN` reader - DMA Enable"]
pub type DMAEN_R = crate::BitReader<DMAEN_A>;
#[doc = "DMA Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAEN_A {
    #[doc = "0: DMA disabled (default)"]
    DMAEN_0 = 0,
    #[doc = "1: DMA enabled"]
    DMAEN_1 = 1,
}
impl From<DMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: DMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAEN_A {
        match self.bits {
            false => DMAEN_A::DMAEN_0,
            true => DMAEN_A::DMAEN_1,
        }
    }
    #[doc = "Checks if the value of the field is `DMAEN_0`"]
    #[inline(always)]
    pub fn is_dmaen_0(&self) -> bool {
        *self == DMAEN_A::DMAEN_0
    }
    #[doc = "Checks if the value of the field is `DMAEN_1`"]
    #[inline(always)]
    pub fn is_dmaen_1(&self) -> bool {
        *self == DMAEN_A::DMAEN_1
    }
}
#[doc = "Field `DMAEN` writer - DMA Enable"]
pub type DMAEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, DMAEN_A, O>;
impl<'a, const O: u8> DMAEN_W<'a, O> {
    #[doc = "DMA disabled (default)"]
    #[inline(always)]
    pub fn dmaen_0(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_0)
    }
    #[doc = "DMA enabled"]
    #[inline(always)]
    pub fn dmaen_1(self) -> &'a mut W {
        self.variant(DMAEN_A::DMAEN_1)
    }
}
#[doc = "Field `ACREN` reader - Compare Function Range Enable"]
pub type ACREN_R = crate::BitReader<ACREN_A>;
#[doc = "Compare Function Range Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACREN_A {
    #[doc = "0: Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared."]
    ACREN_0 = 0,
    #[doc = "1: Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared."]
    ACREN_1 = 1,
}
impl From<ACREN_A> for bool {
    #[inline(always)]
    fn from(variant: ACREN_A) -> Self {
        variant as u8 != 0
    }
}
impl ACREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACREN_A {
        match self.bits {
            false => ACREN_A::ACREN_0,
            true => ACREN_A::ACREN_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACREN_0`"]
    #[inline(always)]
    pub fn is_acren_0(&self) -> bool {
        *self == ACREN_A::ACREN_0
    }
    #[doc = "Checks if the value of the field is `ACREN_1`"]
    #[inline(always)]
    pub fn is_acren_1(&self) -> bool {
        *self == ACREN_A::ACREN_1
    }
}
#[doc = "Field `ACREN` writer - Compare Function Range Enable"]
pub type ACREN_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, ACREN_A, O>;
impl<'a, const O: u8> ACREN_W<'a, O> {
    #[doc = "Range function disabled. Only the compare value 1 of ADC_CV register (CV1) is compared."]
    #[inline(always)]
    pub fn acren_0(self) -> &'a mut W {
        self.variant(ACREN_A::ACREN_0)
    }
    #[doc = "Range function enabled. Both compare values of ADC_CV registers (CV1 and CV2) are compared."]
    #[inline(always)]
    pub fn acren_1(self) -> &'a mut W {
        self.variant(ACREN_A::ACREN_1)
    }
}
#[doc = "Field `ACFGT` reader - Compare Function Greater Than Enable"]
pub type ACFGT_R = crate::BitReader<ACFGT_A>;
#[doc = "Compare Function Greater Than Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACFGT_A {
    #[doc = "0: Configures \"Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive\" functionality based on the values placed in the ADC_CV register."]
    ACFGT_0 = 0,
    #[doc = "1: Configures \"Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive\" functionality based on the values placed in the ADC_CV registers."]
    ACFGT_1 = 1,
}
impl From<ACFGT_A> for bool {
    #[inline(always)]
    fn from(variant: ACFGT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACFGT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFGT_A {
        match self.bits {
            false => ACFGT_A::ACFGT_0,
            true => ACFGT_A::ACFGT_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACFGT_0`"]
    #[inline(always)]
    pub fn is_acfgt_0(&self) -> bool {
        *self == ACFGT_A::ACFGT_0
    }
    #[doc = "Checks if the value of the field is `ACFGT_1`"]
    #[inline(always)]
    pub fn is_acfgt_1(&self) -> bool {
        *self == ACFGT_A::ACFGT_1
    }
}
#[doc = "Field `ACFGT` writer - Compare Function Greater Than Enable"]
pub type ACFGT_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, ACFGT_A, O>;
impl<'a, const O: u8> ACFGT_W<'a, O> {
    #[doc = "Configures \"Less Than Threshold, Outside Range Not Inclusive and Inside Range Not Inclusive\" functionality based on the values placed in the ADC_CV register."]
    #[inline(always)]
    pub fn acfgt_0(self) -> &'a mut W {
        self.variant(ACFGT_A::ACFGT_0)
    }
    #[doc = "Configures \"Greater Than Or Equal To Threshold, Outside Range Inclusive and Inside Range Inclusive\" functionality based on the values placed in the ADC_CV registers."]
    #[inline(always)]
    pub fn acfgt_1(self) -> &'a mut W {
        self.variant(ACFGT_A::ACFGT_1)
    }
}
#[doc = "Field `ACFE` reader - Compare Function Enable"]
pub type ACFE_R = crate::BitReader<ACFE_A>;
#[doc = "Compare Function Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACFE_A {
    #[doc = "0: Compare function disabled"]
    ACFE_0 = 0,
    #[doc = "1: Compare function enabled"]
    ACFE_1 = 1,
}
impl From<ACFE_A> for bool {
    #[inline(always)]
    fn from(variant: ACFE_A) -> Self {
        variant as u8 != 0
    }
}
impl ACFE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ACFE_A {
        match self.bits {
            false => ACFE_A::ACFE_0,
            true => ACFE_A::ACFE_1,
        }
    }
    #[doc = "Checks if the value of the field is `ACFE_0`"]
    #[inline(always)]
    pub fn is_acfe_0(&self) -> bool {
        *self == ACFE_A::ACFE_0
    }
    #[doc = "Checks if the value of the field is `ACFE_1`"]
    #[inline(always)]
    pub fn is_acfe_1(&self) -> bool {
        *self == ACFE_A::ACFE_1
    }
}
#[doc = "Field `ACFE` writer - Compare Function Enable"]
pub type ACFE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, ACFE_A, O>;
impl<'a, const O: u8> ACFE_W<'a, O> {
    #[doc = "Compare function disabled"]
    #[inline(always)]
    pub fn acfe_0(self) -> &'a mut W {
        self.variant(ACFE_A::ACFE_0)
    }
    #[doc = "Compare function enabled"]
    #[inline(always)]
    pub fn acfe_1(self) -> &'a mut W {
        self.variant(ACFE_A::ACFE_1)
    }
}
#[doc = "Field `AVGE` reader - Hardware average enable"]
pub type AVGE_R = crate::BitReader<AVGE_A>;
#[doc = "Hardware average enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AVGE_A {
    #[doc = "0: Hardware average function disabled"]
    AVGE_0 = 0,
    #[doc = "1: Hardware average function enabled"]
    AVGE_1 = 1,
}
impl From<AVGE_A> for bool {
    #[inline(always)]
    fn from(variant: AVGE_A) -> Self {
        variant as u8 != 0
    }
}
impl AVGE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AVGE_A {
        match self.bits {
            false => AVGE_A::AVGE_0,
            true => AVGE_A::AVGE_1,
        }
    }
    #[doc = "Checks if the value of the field is `AVGE_0`"]
    #[inline(always)]
    pub fn is_avge_0(&self) -> bool {
        *self == AVGE_A::AVGE_0
    }
    #[doc = "Checks if the value of the field is `AVGE_1`"]
    #[inline(always)]
    pub fn is_avge_1(&self) -> bool {
        *self == AVGE_A::AVGE_1
    }
}
#[doc = "Field `AVGE` writer - Hardware average enable"]
pub type AVGE_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, AVGE_A, O>;
impl<'a, const O: u8> AVGE_W<'a, O> {
    #[doc = "Hardware average function disabled"]
    #[inline(always)]
    pub fn avge_0(self) -> &'a mut W {
        self.variant(AVGE_A::AVGE_0)
    }
    #[doc = "Hardware average function enabled"]
    #[inline(always)]
    pub fn avge_1(self) -> &'a mut W {
        self.variant(AVGE_A::AVGE_1)
    }
}
#[doc = "Field `ADCO` reader - Continuous Conversion Enable"]
pub type ADCO_R = crate::BitReader<ADCO_A>;
#[doc = "Continuous Conversion Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCO_A {
    #[doc = "0: One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_0 = 0,
    #[doc = "1: Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    ADCO_1 = 1,
}
impl From<ADCO_A> for bool {
    #[inline(always)]
    fn from(variant: ADCO_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCO_A {
        match self.bits {
            false => ADCO_A::ADCO_0,
            true => ADCO_A::ADCO_1,
        }
    }
    #[doc = "Checks if the value of the field is `ADCO_0`"]
    #[inline(always)]
    pub fn is_adco_0(&self) -> bool {
        *self == ADCO_A::ADCO_0
    }
    #[doc = "Checks if the value of the field is `ADCO_1`"]
    #[inline(always)]
    pub fn is_adco_1(&self) -> bool {
        *self == ADCO_A::ADCO_1
    }
}
#[doc = "Field `ADCO` writer - Continuous Conversion Enable"]
pub type ADCO_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, ADCO_A, O>;
impl<'a, const O: u8> ADCO_W<'a, O> {
    #[doc = "One conversion or one set of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    #[inline(always)]
    pub fn adco_0(self) -> &'a mut W {
        self.variant(ADCO_A::ADCO_0)
    }
    #[doc = "Continuous conversions or sets of conversions if the hardware average function is enabled (AVGE=1) after initiating a conversion."]
    #[inline(always)]
    pub fn adco_1(self) -> &'a mut W {
        self.variant(ADCO_A::ADCO_1)
    }
}
#[doc = "Field `CAL` reader - Calibration"]
pub type CAL_R = crate::BitReader<bool>;
#[doc = "Field `CAL` writer - Calibration"]
pub type CAL_W<'a, const O: u8> = crate::BitWriter<'a, u32, GC_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Asynchronous clock output enable"]
    #[inline(always)]
    pub fn adacken(&self) -> ADACKEN_R {
        ADACKEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    pub fn dmaen(&self) -> DMAEN_R {
        DMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Compare Function Range Enable"]
    #[inline(always)]
    pub fn acren(&self) -> ACREN_R {
        ACREN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Compare Function Greater Than Enable"]
    #[inline(always)]
    pub fn acfgt(&self) -> ACFGT_R {
        ACFGT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Compare Function Enable"]
    #[inline(always)]
    pub fn acfe(&self) -> ACFE_R {
        ACFE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Hardware average enable"]
    #[inline(always)]
    pub fn avge(&self) -> AVGE_R {
        AVGE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Continuous Conversion Enable"]
    #[inline(always)]
    pub fn adco(&self) -> ADCO_R {
        ADCO_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    pub fn cal(&self) -> CAL_R {
        CAL_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Asynchronous clock output enable"]
    #[inline(always)]
    #[must_use]
    pub fn adacken(&mut self) -> ADACKEN_W<0> {
        ADACKEN_W::new(self)
    }
    #[doc = "Bit 1 - DMA Enable"]
    #[inline(always)]
    #[must_use]
    pub fn dmaen(&mut self) -> DMAEN_W<1> {
        DMAEN_W::new(self)
    }
    #[doc = "Bit 2 - Compare Function Range Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acren(&mut self) -> ACREN_W<2> {
        ACREN_W::new(self)
    }
    #[doc = "Bit 3 - Compare Function Greater Than Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acfgt(&mut self) -> ACFGT_W<3> {
        ACFGT_W::new(self)
    }
    #[doc = "Bit 4 - Compare Function Enable"]
    #[inline(always)]
    #[must_use]
    pub fn acfe(&mut self) -> ACFE_W<4> {
        ACFE_W::new(self)
    }
    #[doc = "Bit 5 - Hardware average enable"]
    #[inline(always)]
    #[must_use]
    pub fn avge(&mut self) -> AVGE_W<5> {
        AVGE_W::new(self)
    }
    #[doc = "Bit 6 - Continuous Conversion Enable"]
    #[inline(always)]
    #[must_use]
    pub fn adco(&mut self) -> ADCO_W<6> {
        ADCO_W::new(self)
    }
    #[doc = "Bit 7 - Calibration"]
    #[inline(always)]
    #[must_use]
    pub fn cal(&mut self) -> CAL_W<7> {
        CAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "General control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gc](index.html) module"]
pub struct GC_SPEC;
impl crate::RegisterSpec for GC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gc::R](R) reader structure"]
impl crate::Readable for GC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gc::W](W) writer structure"]
impl crate::Writable for GC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets GC to value 0"]
impl crate::Resettable for GC_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
