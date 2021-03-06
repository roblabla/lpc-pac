#[doc = "Reader of register MAINCLKUEN"]
pub type R = crate::R<u32, super::MAINCLKUEN>;
#[doc = "Writer for register MAINCLKUEN"]
pub type W = crate::W<u32, super::MAINCLKUEN>;
#[doc = "Register MAINCLKUEN `reset()`'s with value 0"]
impl crate::ResetValue for super::MAINCLKUEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Enable main clock source update.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ENA_A {
    #[doc = "0: No change."]
    ENA_0 = 0,
    #[doc = "1: Update clock source."]
    ENA_1 = 1,
}
impl From<ENA_A> for bool {
    #[inline(always)]
    fn from(variant: ENA_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Reader of field `ENA`"]
pub type ENA_R = crate::R<bool, ENA_A>;
impl ENA_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ENA_A {
        match self.bits {
            false => ENA_A::ENA_0,
            true => ENA_A::ENA_1,
        }
    }
    #[doc = "Checks if the value of the field is `ENA_0`"]
    #[inline(always)]
    pub fn is_ena_0(&self) -> bool {
        *self == ENA_A::ENA_0
    }
    #[doc = "Checks if the value of the field is `ENA_1`"]
    #[inline(always)]
    pub fn is_ena_1(&self) -> bool {
        *self == ENA_A::ENA_1
    }
}
#[doc = "Write proxy for field `ENA`"]
pub struct ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> ENA_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ENA_A) -> &'a mut W {
        {
            self.bit(variant.into())
        }
    }
    #[doc = "No change."]
    #[inline(always)]
    pub fn ena_0(self) -> &'a mut W {
        self.variant(ENA_A::ENA_0)
    }
    #[doc = "Update clock source."]
    #[inline(always)]
    pub fn ena_1(self) -> &'a mut W {
        self.variant(ENA_A::ENA_1)
    }
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Enable main clock source update."]
    #[inline(always)]
    pub fn ena(&self) -> ENA_R {
        ENA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable main clock source update."]
    #[inline(always)]
    pub fn ena(&mut self) -> ENA_W {
        ENA_W { w: self }
    }
}
