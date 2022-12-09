#[doc = "Register `MTDR` writer"]
pub struct W(crate::W<MTDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MTDR_SPEC>;
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
impl From<crate::W<MTDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MTDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DATA` writer - Transmit Data"]
pub type DATA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, MTDR_SPEC, u8, u8, 8, O>;
#[doc = "Command Data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CMD_AW {
    #[doc = "0: Transmit DATA\\[7:0\\]"]
    TRANSMIT_DATA_7_THROUGH_0 = 0,
    #[doc = "1: Receive (DATA\\[7:0\\]
+ 1) bytes"]
    RECEIVE_DATA_7_THROUGH_0_PLUS_ONE = 1,
    #[doc = "2: Generate STOP condition"]
    GENERATE_STOP_CONDITION = 2,
    #[doc = "3: Receive and discard (DATA\\[7:0\\]
+ 1) bytes"]
    RECEIVE_AND_DISCARD_DATA_7_THROUGH_0_PLUS_ONE = 3,
    #[doc = "4: Generate (repeated) START and transmit address in DATA\\[7:0\\]"]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0 = 4,
    #[doc = "5: Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_EXPECT_NACK = 5,
    #[doc = "6: Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode"]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE = 6,
    #[doc = "7: Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode. This transfer expects a NACK to be returned."]
    GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE_EXPECT_NACK = 7,
}
impl From<CMD_AW> for u8 {
    #[inline(always)]
    fn from(variant: CMD_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `CMD` writer - Command Data"]
pub type CMD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u32, MTDR_SPEC, u8, CMD_AW, 3, O>;
impl<'a, const O: u8> CMD_W<'a, O> {
    #[doc = "Transmit DATA\\[7:0\\]"]
    #[inline(always)]
    pub fn transmit_data_7_through_0(self) -> &'a mut W {
        self.variant(CMD_AW::TRANSMIT_DATA_7_THROUGH_0)
    }
    #[doc = "Receive (DATA\\[7:0\\]
+ 1) bytes"]
    #[inline(always)]
    pub fn receive_data_7_through_0_plus_one(self) -> &'a mut W {
        self.variant(CMD_AW::RECEIVE_DATA_7_THROUGH_0_PLUS_ONE)
    }
    #[doc = "Generate STOP condition"]
    #[inline(always)]
    pub fn generate_stop_condition(self) -> &'a mut W {
        self.variant(CMD_AW::GENERATE_STOP_CONDITION)
    }
    #[doc = "Receive and discard (DATA\\[7:0\\]
+ 1) bytes"]
    #[inline(always)]
    pub fn receive_and_discard_data_7_through_0_plus_one(self) -> &'a mut W {
        self.variant(CMD_AW::RECEIVE_AND_DISCARD_DATA_7_THROUGH_0_PLUS_ONE)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]"]
    #[inline(always)]
    pub fn generate_start_and_transmit_address_in_data_7_through_0(self) -> &'a mut W {
        self.variant(CMD_AW::GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]. This transfer expects a NACK to be returned."]
    #[inline(always)]
    pub fn generate_start_and_transmit_address_in_data_7_through_0_expect_nack(self) -> &'a mut W {
        self.variant(CMD_AW::GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_EXPECT_NACK)
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode"]
    #[inline(always)]
    pub fn generate_start_and_transmit_address_in_data_7_through_0_using_high_speed_mode(
        self,
    ) -> &'a mut W {
        self.variant(
            CMD_AW::GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE,
        )
    }
    #[doc = "Generate (repeated) START and transmit address in DATA\\[7:0\\]
using high speed mode. This transfer expects a NACK to be returned."]
    #[inline(always)]
    pub fn generate_start_and_transmit_address_in_data_7_through_0_using_high_speed_mode_expect_nack(
        self,
    ) -> &'a mut W {
        self . variant (CMD_AW :: GENERATE_START_AND_TRANSMIT_ADDRESS_IN_DATA_7_THROUGH_0_USING_HIGH_SPEED_MODE_EXPECT_NACK)
    }
}
impl W {
    #[doc = "Bits 0:7 - Transmit Data"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DATA_W<0> {
        DATA_W::new(self)
    }
    #[doc = "Bits 8:10 - Command Data"]
    #[inline(always)]
    #[must_use]
    pub fn cmd(&mut self) -> CMD_W<8> {
        CMD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Master Transmit Data\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mtdr](index.html) module"]
pub struct MTDR_SPEC;
impl crate::RegisterSpec for MTDR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [mtdr::W](W) writer structure"]
impl crate::Writable for MTDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MTDR to value 0"]
impl crate::Resettable for MTDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
