#[doc = "Register `TX` reader"]
pub struct R(crate::R<TX_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TX_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TX_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TX_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TX` writer"]
pub struct W(crate::W<TX_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TX_SPEC>;
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
impl From<crate::W<TX_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TX_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `D_CAL` reader - Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
pub type D_CAL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `D_CAL` writer - Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
pub type D_CAL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSVD0` reader - Reserved. Note: This bit should remain clear."]
pub type RSVD0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD0` writer - Reserved. Note: This bit should remain clear."]
pub type RSVD0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXCAL45DN` reader - Decode to select a 45-Ohm resistance to the USB_DN output pin"]
pub type TXCAL45DN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCAL45DN` writer - Decode to select a 45-Ohm resistance to the USB_DN output pin"]
pub type TXCAL45DN_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSVD1` reader - Reserved. Note: This bit should remain clear."]
pub type RSVD1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RSVD1` writer - Reserved. Note: This bit should remain clear."]
pub type RSVD1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 4, O>;
#[doc = "Field `TXCAL45DP` reader - Decode to select a 45-Ohm resistance to the USB_DP output pin"]
pub type TXCAL45DP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TXCAL45DP` writer - Decode to select a 45-Ohm resistance to the USB_DP output pin"]
pub type TXCAL45DP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 4, O>;
#[doc = "Field `RSVD2` reader - Reserved."]
pub type RSVD2_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPHY_TX_EDGECTRL` reader - Controls the edge-rate of the current sensing transistors used in HS transmit"]
pub type USBPHY_TX_EDGECTRL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `USBPHY_TX_EDGECTRL` writer - Controls the edge-rate of the current sensing transistors used in HS transmit"]
pub type USBPHY_TX_EDGECTRL_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TX_SPEC, u8, u8, 3, O>;
#[doc = "Field `RSVD5` reader - Reserved."]
pub type RSVD5_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    pub fn d_cal(&self) -> D_CAL_R {
        D_CAL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub fn rsvd0(&self) -> RSVD0_R {
        RSVD0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    pub fn txcal45dn(&self) -> TXCAL45DN_R {
        TXCAL45DN_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    pub fn rsvd1(&self) -> RSVD1_R {
        RSVD1_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    pub fn txcal45dp(&self) -> TXCAL45DP_R {
        TXCAL45DP_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - Reserved."]
    #[inline(always)]
    pub fn rsvd2(&self) -> RSVD2_R {
        RSVD2_R::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    pub fn usbphy_tx_edgectrl(&self) -> USBPHY_TX_EDGECTRL_R {
        USBPHY_TX_EDGECTRL_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:31 - Reserved."]
    #[inline(always)]
    pub fn rsvd5(&self) -> RSVD5_R {
        RSVD5_R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Resistor Trimming Code: 0000 = 0.16% 0111 = Nominal 1111 = +25%"]
    #[inline(always)]
    #[must_use]
    pub fn d_cal(&mut self) -> D_CAL_W<0> {
        D_CAL_W::new(self)
    }
    #[doc = "Bits 4:7 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd0(&mut self) -> RSVD0_W<4> {
        RSVD0_W::new(self)
    }
    #[doc = "Bits 8:11 - Decode to select a 45-Ohm resistance to the USB_DN output pin"]
    #[inline(always)]
    #[must_use]
    pub fn txcal45dn(&mut self) -> TXCAL45DN_W<8> {
        TXCAL45DN_W::new(self)
    }
    #[doc = "Bits 12:15 - Reserved. Note: This bit should remain clear."]
    #[inline(always)]
    #[must_use]
    pub fn rsvd1(&mut self) -> RSVD1_W<12> {
        RSVD1_W::new(self)
    }
    #[doc = "Bits 16:19 - Decode to select a 45-Ohm resistance to the USB_DP output pin"]
    #[inline(always)]
    #[must_use]
    pub fn txcal45dp(&mut self) -> TXCAL45DP_W<16> {
        TXCAL45DP_W::new(self)
    }
    #[doc = "Bits 26:28 - Controls the edge-rate of the current sensing transistors used in HS transmit"]
    #[inline(always)]
    #[must_use]
    pub fn usbphy_tx_edgectrl(&mut self) -> USBPHY_TX_EDGECTRL_W<26> {
        USBPHY_TX_EDGECTRL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USB PHY Transmitter Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tx](index.html) module"]
pub struct TX_SPEC;
impl crate::RegisterSpec for TX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tx::R](R) reader structure"]
impl crate::Readable for TX_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tx::W](W) writer structure"]
impl crate::Writable for TX_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TX to value 0x1006_0607"]
impl crate::Resettable for TX_SPEC {
    const RESET_VALUE: Self::Ux = 0x1006_0607;
}
