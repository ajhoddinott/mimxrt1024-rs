#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0800],
    #[doc = "0x800 - USB OTG1 Control Register"]
    pub usb_otg1_ctrl: USB_OTG1_CTRL,
    _reserved1: [u8; 0x14],
    #[doc = "0x818 - OTG1 UTMI PHY Control 0 Register"]
    pub usb_otg1_phy_ctrl_0: USB_OTG1_PHY_CTRL_0,
}
#[doc = "USB_OTG1_CTRL (rw) register accessor: an alias for `Reg<USB_OTG1_CTRL_SPEC>`"]
pub type USB_OTG1_CTRL = crate::Reg<usb_otg1_ctrl::USB_OTG1_CTRL_SPEC>;
#[doc = "USB OTG1 Control Register"]
pub mod usb_otg1_ctrl;
#[doc = "USB_OTG1_PHY_CTRL_0 (rw) register accessor: an alias for `Reg<USB_OTG1_PHY_CTRL_0_SPEC>`"]
pub type USB_OTG1_PHY_CTRL_0 = crate::Reg<usb_otg1_phy_ctrl_0::USB_OTG1_PHY_CTRL_0_SPEC>;
#[doc = "OTG1 UTMI PHY Control 0 Register"]
pub mod usb_otg1_phy_ctrl_0;
