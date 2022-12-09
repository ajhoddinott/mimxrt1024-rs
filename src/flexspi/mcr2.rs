#[doc = "Register `MCR2` reader"]
pub struct R(crate::R<MCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MCR2` writer"]
pub struct W(crate::W<MCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MCR2_SPEC>;
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
impl From<crate::W<MCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLRAHBBUFOPT` reader - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
pub type CLRAHBBUFOPT_R = crate::BitReader<CLRAHBBUFOPT_A>;
#[doc = "This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CLRAHBBUFOPT_A {
    #[doc = "0: AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_0 = 0,
    #[doc = "1: AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    CLRAHBBUFOPT_1 = 1,
}
impl From<CLRAHBBUFOPT_A> for bool {
    #[inline(always)]
    fn from(variant: CLRAHBBUFOPT_A) -> Self {
        variant as u8 != 0
    }
}
impl CLRAHBBUFOPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CLRAHBBUFOPT_A {
        match self.bits {
            false => CLRAHBBUFOPT_A::CLRAHBBUFOPT_0,
            true => CLRAHBBUFOPT_A::CLRAHBBUFOPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `CLRAHBBUFOPT_0`"]
    #[inline(always)]
    pub fn is_clrahbbufopt_0(&self) -> bool {
        *self == CLRAHBBUFOPT_A::CLRAHBBUFOPT_0
    }
    #[doc = "Checks if the value of the field is `CLRAHBBUFOPT_1`"]
    #[inline(always)]
    pub fn is_clrahbbufopt_1(&self) -> bool {
        *self == CLRAHBBUFOPT_A::CLRAHBBUFOPT_1
    }
}
#[doc = "Field `CLRAHBBUFOPT` writer - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
pub type CLRAHBBUFOPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR2_SPEC, CLRAHBBUFOPT_A, O>;
impl<'a, const O: u8> CLRAHBBUFOPT_W<'a, O> {
    #[doc = "AHB RX/TX Buffer will not be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline(always)]
    pub fn clrahbbufopt_0(self) -> &'a mut W {
        self.variant(CLRAHBBUFOPT_A::CLRAHBBUFOPT_0)
    }
    #[doc = "AHB RX/TX Buffer will be cleaned automatically when FlexSPI return Stop mode ACK."]
    #[inline(always)]
    pub fn clrahbbufopt_1(self) -> &'a mut W {
        self.variant(CLRAHBBUFOPT_A::CLRAHBBUFOPT_1)
    }
}
#[doc = "Field `SAMEDEVICEEN` reader - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
pub type SAMEDEVICEEN_R = crate::BitReader<SAMEDEVICEEN_A>;
#[doc = "All external devices are same devices (both in types and size) for A1/A2/B1/B2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SAMEDEVICEEN_A {
    #[doc = "0: In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 separately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    INDIVIDUAL_PARALLEL = 0,
    #[doc = "1: FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    ENABLE = 1,
}
impl From<SAMEDEVICEEN_A> for bool {
    #[inline(always)]
    fn from(variant: SAMEDEVICEEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SAMEDEVICEEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SAMEDEVICEEN_A {
        match self.bits {
            false => SAMEDEVICEEN_A::INDIVIDUAL_PARALLEL,
            true => SAMEDEVICEEN_A::ENABLE,
        }
    }
    #[doc = "Checks if the value of the field is `INDIVIDUAL_PARALLEL`"]
    #[inline(always)]
    pub fn is_individual_parallel(&self) -> bool {
        *self == SAMEDEVICEEN_A::INDIVIDUAL_PARALLEL
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == SAMEDEVICEEN_A::ENABLE
    }
}
#[doc = "Field `SAMEDEVICEEN` writer - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
pub type SAMEDEVICEEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR2_SPEC, SAMEDEVICEEN_A, O>;
impl<'a, const O: u8> SAMEDEVICEEN_W<'a, O> {
    #[doc = "In Individual mode, FLSHA1CRx/FLSHA2CRx/FLSHB1CRx/FLSHB2CRx register setting will be applied to Flash A1/A2/B1/B2 separately. In Parallel mode, FLSHA1CRx register setting will be applied to Flash A1 and B1, FLSHA2CRx register setting will be applied to Flash A2 and B2. FLSHB1CRx/FLSHB2CRx register settings will be ignored."]
    #[inline(always)]
    pub fn individual_parallel(self) -> &'a mut W {
        self.variant(SAMEDEVICEEN_A::INDIVIDUAL_PARALLEL)
    }
    #[doc = "FLSHA1CR0/FLSHA1CR1/FLSHA1CR2 register settings will be applied to Flash A1/A2/B1/B2. FLSHA2CRx/FLSHB1CRx/FLSHB2CRx will be ignored."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(SAMEDEVICEEN_A::ENABLE)
    }
}
#[doc = "Field `SCKBDIFFOPT` reader - B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set."]
pub type SCKBDIFFOPT_R = crate::BitReader<SCKBDIFFOPT_A>;
#[doc = "B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCKBDIFFOPT_A {
    #[doc = "0: B_SCLK pad is used as port B SCLK clock output. Port B flash access is available."]
    SCKBDIFFOPT_0 = 0,
    #[doc = "1: B_SCLK pad is used as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash access is not available."]
    SCKBDIFFOPT_1 = 1,
}
impl From<SCKBDIFFOPT_A> for bool {
    #[inline(always)]
    fn from(variant: SCKBDIFFOPT_A) -> Self {
        variant as u8 != 0
    }
}
impl SCKBDIFFOPT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCKBDIFFOPT_A {
        match self.bits {
            false => SCKBDIFFOPT_A::SCKBDIFFOPT_0,
            true => SCKBDIFFOPT_A::SCKBDIFFOPT_1,
        }
    }
    #[doc = "Checks if the value of the field is `SCKBDIFFOPT_0`"]
    #[inline(always)]
    pub fn is_sckbdiffopt_0(&self) -> bool {
        *self == SCKBDIFFOPT_A::SCKBDIFFOPT_0
    }
    #[doc = "Checks if the value of the field is `SCKBDIFFOPT_1`"]
    #[inline(always)]
    pub fn is_sckbdiffopt_1(&self) -> bool {
        *self == SCKBDIFFOPT_A::SCKBDIFFOPT_1
    }
}
#[doc = "Field `SCKBDIFFOPT` writer - B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set."]
pub type SCKBDIFFOPT_W<'a, const O: u8> = crate::BitWriter<'a, u32, MCR2_SPEC, SCKBDIFFOPT_A, O>;
impl<'a, const O: u8> SCKBDIFFOPT_W<'a, O> {
    #[doc = "B_SCLK pad is used as port B SCLK clock output. Port B flash access is available."]
    #[inline(always)]
    pub fn sckbdiffopt_0(self) -> &'a mut W {
        self.variant(SCKBDIFFOPT_A::SCKBDIFFOPT_0)
    }
    #[doc = "B_SCLK pad is used as port A SCLK inverted clock output (Differential clock to A_SCLK). Port B flash access is not available."]
    #[inline(always)]
    pub fn sckbdiffopt_1(self) -> &'a mut W {
        self.variant(SCKBDIFFOPT_A::SCKBDIFFOPT_1)
    }
}
#[doc = "Field `RESUMEWAIT` reader - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
pub type RESUMEWAIT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RESUMEWAIT` writer - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
pub type RESUMEWAIT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MCR2_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bit 11 - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline(always)]
    pub fn clrahbbufopt(&self) -> CLRAHBBUFOPT_R {
        CLRAHBBUFOPT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline(always)]
    pub fn samedeviceen(&self) -> SAMEDEVICEEN_R {
        SAMEDEVICEEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set."]
    #[inline(always)]
    pub fn sckbdiffopt(&self) -> SCKBDIFFOPT_R {
        SCKBDIFFOPT_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    pub fn resumewait(&self) -> RESUMEWAIT_R {
        RESUMEWAIT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 11 - This bit determines whether AHB RX Buffer and AHB TX Buffer will be cleaned automatically when FlexSPI returns STOP mode ACK. Software should set this bit if AHB RX Buffer or AHB TX Buffer will be powered off in STOP mode. Otherwise AHB read access after exiting STOP mode may hit AHB RX Buffer or AHB TX Buffer but their data entries are invalid."]
    #[inline(always)]
    #[must_use]
    pub fn clrahbbufopt(&mut self) -> CLRAHBBUFOPT_W<11> {
        CLRAHBBUFOPT_W::new(self)
    }
    #[doc = "Bit 15 - All external devices are same devices (both in types and size) for A1/A2/B1/B2."]
    #[inline(always)]
    #[must_use]
    pub fn samedeviceen(&mut self) -> SAMEDEVICEEN_W<15> {
        SAMEDEVICEEN_W::new(self)
    }
    #[doc = "Bit 19 - B_SCLK pad can be used as A_SCLK differential clock output (inverted clock to A_SCLK). In this case, port B flash access is not available. After changing the value of this field, MCR0\\[SWRESET\\]
should be set."]
    #[inline(always)]
    #[must_use]
    pub fn sckbdiffopt(&mut self) -> SCKBDIFFOPT_W<19> {
        SCKBDIFFOPT_W::new(self)
    }
    #[doc = "Bits 24:31 - Wait cycle (in AHB clock cycle) for idle state before suspended command sequence resumed."]
    #[inline(always)]
    #[must_use]
    pub fn resumewait(&mut self) -> RESUMEWAIT_W<24> {
        RESUMEWAIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Module Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mcr2](index.html) module"]
pub struct MCR2_SPEC;
impl crate::RegisterSpec for MCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mcr2::R](R) reader structure"]
impl crate::Readable for MCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mcr2::W](W) writer structure"]
impl crate::Writable for MCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MCR2 to value 0x2000_81f7"]
impl crate::Resettable for MCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_81f7;
}
