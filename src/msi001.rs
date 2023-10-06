/*** Registro 0: Modo IC / Control de energía ***/

/* reg0: 4:8 (AM_MODE, VHF_MODE, B3_MODE, B45_MODE, BL_MODE) */
const MIRISDR_MODE_AM: u32 = 0x01;
const MIRISDR_MODE_VHF: u32 = 0x02;
const MIRISDR_MODE_B3: u32 = 0x04;
const MIRISDR_MODE_B45: u32 = 0x08;
const MIRISDR_MODE_BL: u32 = 0x10;

/* reg0: 9 (AM_MODE2) */
const MIRISDR_UPCONVERT_MIXER_OFF: u32 = 0;
const MIRISDR_UPCONVERT_MIXER_ON: u32 = 1;

/* reg0: 10 (RF_SYNTH) */
const MIRISDR_RF_SYNTHESIZER_OFF: u32 = 0;
const MIRISDR_RF_SYNTHESIZER_ON: u32 = 1;

/* reg0: 11 (AM_PORT_SEL) */
const MIRISDR_AM_PORT1: u32 = 0;
const MIRISDR_AM_PORT2: u32 = 1;

/* reg0: 12:13 (FIL_MODE_SEL0, FIL_MODE_SEL1) */
const MIRISDR_IF_MODE_2048KHZ: u32 = 0;
const MIRISDR_IF_MODE_1620KHZ: u32 = 1;
const MIRISDR_IF_MODE_450KHZ: u32 = 2;
const MIRISDR_IF_MODE_ZERO: u32 = 3;

/* reg0: 14:16 (FIL_BW_SEL0 - FIL_BW_SEL2) */

/* reg0: 17:19 (XTAL_SEL0 - XTAL_SEL2) */

/* reg0: 20:22 (IF_LPMODE0 - IF_LPMODE2) */
const MIRISDR_IF_LPMODE_NORMAL: u32 = 0;
const MIRISDR_IF_LPMODE_ONLY_Q: u32 = 1;
const MIRISDR_IF_LPMODE_ONLY_I: u32 = 2;
const MIRISDR_IF_LPMODE_LOW_POWER: u32 = 4;

/* reg0: 23 (VCO_LPMODE) */
const MIRISDR_VCO_LPMODE_NORMAL: u32 = 0;
const MIRISDR_VCO_LPMODE_LOW_POWER: u32 = 1;

/*** Registro 2: Programación del Sintetizador ***/

/* reg2: 4:15 (FRAC0 - FRAC11) */

/* reg2: 16:21 (INT0 - INT5) */

/* reg2: 22 (LNACAL_EN) */
const MIRISDR_LBAND_LNA_CALIBRATION_OFF: u32 = 0;
const MIRISDR_LBAND_LNA_CALIBRATION_ON: u8 = 1;

/*** Registro 6: Configuración del Sintetizador RF ***/

/* reg5: 4:15 (THRESH0 - THRESH11) */

/* reg5: 16:21 (reserved) */
const MIRISDR_RF_SYNTHESIZER_RESERVED_PROGRAMMING: u32 = 0x28;

/*** Registro 1: Control de Ganancia del Receptor ***/

/* reg1: 4:9 (BBGAIN0 - BBGAIN5) */
const MIRISDR_BASEBAND_GAIN_REDUCTION_MIN: u8 = 0;
const MIRISDR_BASEBAND_GAIN_REDUCTION_MAX: u8 = 0x3B;

/* reg1: 10:11 (MIXBU0, MIXBU1) - Puerto AM 1 */
const MIRISDR_AM_PORT1_BLOCKUP_CONVERT_GAIN_REDUCTION_0DB: u8 = 0;
const MIRISDR_AM_PORT1_BLOCKUP_CONVERT_GAIN_REDUCTION_6DB: u8 = 1;
const MIRISDR_AM_PORT1_BLOCKUP_CONVERT_GAIN_REDUCTION_12DB: u8 = 2;
const MIRISDR_AM_PORT1_BLOCKUP_CONVERT_GAIN_REDUCTION_18DB: u8 = 3;

/* reg1: 10:11 (MIXBU0, MIXBU1) - Puerto AM 2 */
const MIRISDR_AM_PORT2_BLOCKUP_CONVERT_GAIN_REDUCTION_0DB: u8 = 0;
const MIRISDR_AM_PORT2_BLOCKUP_CONVERT_GAIN_REDUCTION_24DB: u8 = 3;

/* reg1: 12 (MIXL) */
const MIRISDR_LNA_GAIN_REDUCTION_OFF: u8 = 0;
const MIRISDR_LNA_GAIN_REDUCTION_ON: u8 = 1;

/* reg1: 13 (LNAGR) */
const MIRISDR_MIXER_GAIN_REDUCTION_OFF: u8 = 0;
const MIRISDR_MIXER_GAIN_REDUCTION_ON: u8 = 1;

/* reg1: 14:16 (DCCAL0 - DCCAL2) */
const MIRISDR_DC_OFFSET_CALIBRATION_STATIC: u8 = 0;
const MIRISDR_DC_OFFSET_CALIBRATION_PERIODIC1: u8 = 1;
const MIRISDR_DC_OFFSET_CALIBRATION_PERIODIC2: u8 = 2;
const MIRISDR_DC_OFFSET_CALIBRATION_PERIODIC3: u8 = 3;
const MIRISDR_DC_OFFSET_CALIBRATION_ONE_SHOT: u8 = 4;
const MIRISDR_DC_OFFSET_CALIBRATION_CONTINUOUS: u8 = 5;

/* reg1: 17 (DCCAL_SPEEDUP) */
const MIRISDR_DC_OFFSET_CALIBRATION_SPEEDUP_OFF: u8 = 0;
const MIRISDR_DC_OFFSET_CALIBRATION_SPEEDUP_ON: u8 = 1;

/*** Registro 6: Configuración de Calibración de Desviación de CC ***/

/* reg6: 4:7 (DCTRK_TIM0 - DCTRK_TIM3) */

/* reg6: 8:21 (DCRATE_TIM0 - DCRATE_TIM11) */
#[derive(Clone, Copy)]
pub enum MirisdrHwFlavour {
    MirisdrHwDefault,
    MirisdrHwSdrplay,
}
#[derive(Clone, Copy, PartialEq)]
pub enum MirisdBandT {
    MirisdrBandAm1,
    MirisdrBandAm2,
    MirisdrBandVhf,
    MirisdrBand3,
    MirisdrBand45,
    MirisdrBandL,
}
#[derive(Clone, Copy)]
pub enum BandWidthEnum {
    MirisdrBw200khz = 0,
    MirisdrBw300khz,
    MirisdrBw600khz,
    MirisdrBw1536khz,
    MirisdrBw5mhz,
    MirisdrBw6mhz,
    MirisdrBw7mhz,
    MirisdrBw8mhz,
}
#[derive(Clone, Copy)]
pub enum IfFreqEnum {
    MirisdrIfZero = 0,
    MirisdrIf450khz,
    MirisdrIf1620khz,
    MirisdrIf2048khz,
}
#[derive(Clone, Copy)]
pub enum XtalEnum {
    MirisdrXtal19_2m = 0,
    MirisdrXtal22m,
    MirisdrXtal24m,
    MirisdrXtal24_576m,
    MirisdrXtal26m,
    MirisdrXtal38_4m,
}
// #[repr(C)]
#[derive(Clone, Copy)]
pub struct MirisdrDev {
    pub index: i32,
    pub freq: u32,
    pub rate: i32,
    pub gain: i16,
    pub gain_reduction_lna: i16,
    pub gain_reduction_mixbuffer: i16,
    pub gain_reduction_mixer: i16,
    pub gain_reduction_baseband: i16,
    pub hw_flavour: MirisdrHwFlavour,
    pub band: MirisdBandT,
    pub bandwidth: BandWidthEnum,
    pub if_freq: IfFreqEnum,
    pub xtal: XtalEnum,
}
#[repr(C)]
pub struct HwSwitchFreqPlan {
    pub low_cut: u32,
    pub mode: u32,
    pub upconvert_mixer_on: u32,
    pub am_port: u32,
    pub lo_div: u64,
    pub band_select_word: u32,
}

pub const HW_SWITCH_FREQ_PLAN_DEFAULT: [HwSwitchFreqPlan; 9] = [
    HwSwitchFreqPlan {
        low_cut: 0,
        mode: MIRISDR_MODE_AM,
        upconvert_mixer_on: MIRISDR_UPCONVERT_MIXER_ON,
        am_port: MIRISDR_AM_PORT2,
        lo_div: 16,
        band_select_word: 0xf780,
    },
    HwSwitchFreqPlan {
        low_cut: 12,
        mode: MIRISDR_MODE_AM,
        upconvert_mixer_on: MIRISDR_UPCONVERT_MIXER_ON,
        am_port: MIRISDR_AM_PORT2,
        lo_div: 16,
        band_select_word: 0xff80,
    },
    HwSwitchFreqPlan {
        low_cut: 30,
        mode: MIRISDR_MODE_AM,
        upconvert_mixer_on: MIRISDR_UPCONVERT_MIXER_ON,
        am_port: MIRISDR_AM_PORT2,
        lo_div: 16,
        band_select_word: 0xf280,
    },
    HwSwitchFreqPlan {
        low_cut: 50,
        mode: MIRISDR_MODE_VHF,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 32,
        band_select_word: 0xf380,
    },
    HwSwitchFreqPlan {
        low_cut: 108,
        mode: MIRISDR_MODE_B3,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 16,
        band_select_word: 0xfa80,
    },
    HwSwitchFreqPlan {
        low_cut: 250,
        mode: MIRISDR_MODE_B3,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 16,
        band_select_word: 0xf680,
    },
    HwSwitchFreqPlan {
        low_cut: 330,
        mode: MIRISDR_MODE_B45,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 4,
        band_select_word: 0xf380,
    },
    HwSwitchFreqPlan {
        low_cut: 960,
        mode: MIRISDR_MODE_BL,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 2,
        band_select_word: 0xfa80,
    },
    HwSwitchFreqPlan {
        low_cut: 2400,
        mode: u32::MAX,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 0,
        band_select_word: 0x0000,
    },
];

pub const HW_SWITCH_FREQ_PLAN_SDRPLAY: [HwSwitchFreqPlan; 9] = [
    HwSwitchFreqPlan {
        low_cut: 0,
        mode: MIRISDR_MODE_AM,
        upconvert_mixer_on: MIRISDR_UPCONVERT_MIXER_ON,
        am_port: MIRISDR_AM_PORT2,
        lo_div: 16,
        band_select_word: 0xf780,
    },
    HwSwitchFreqPlan {
        low_cut: 12,
        mode: MIRISDR_MODE_AM,
        upconvert_mixer_on: MIRISDR_UPCONVERT_MIXER_ON,
        am_port: MIRISDR_AM_PORT2,
        lo_div: 16,
        band_select_word: 0xff80,
    },
    HwSwitchFreqPlan {
        low_cut: 30,
        mode: MIRISDR_MODE_AM,
        upconvert_mixer_on: MIRISDR_UPCONVERT_MIXER_ON,
        am_port: MIRISDR_AM_PORT2,
        lo_div: 16,
        band_select_word: 0xf280,
    },
    HwSwitchFreqPlan {
        low_cut: 50,
        mode: MIRISDR_MODE_VHF,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 32,
        band_select_word: 0xf380,
    },
    HwSwitchFreqPlan {
        low_cut: 108,
        mode: MIRISDR_MODE_B3,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 16,
        band_select_word: 0xfa80,
    },
    HwSwitchFreqPlan {
        low_cut: 250,
        mode: MIRISDR_MODE_B3,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 16,
        band_select_word: 0xf680,
    },
    // HwSwitchFreqPlan {
    //     low_cut: 250,
    //     mode: MIRISDR_MODE_B3,
    //     upconvert_mixer_on:MIRISDR_UPCONVERT_MIXER_OFF,
    //     am_port: MIRISDR_AM_PORT2,
    //     lo_div: 16,
    //     band_select_word: 0xf680,
    // },
    HwSwitchFreqPlan {
        low_cut: 330,
        mode: MIRISDR_MODE_B45,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 4,
        band_select_word: 0xf380,
    },
    HwSwitchFreqPlan {
        low_cut: 960,
        mode: MIRISDR_MODE_BL,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 2,
        band_select_word: 0xfa80,
    },
    HwSwitchFreqPlan {
        low_cut: 2400,
        mode: u32::MAX,
        upconvert_mixer_on: 0,
        am_port: 0,
        lo_div: 0,
        band_select_word: 0x0000,
    },
];
pub const HW_SWITCH_FREQ_PLAN: [[HwSwitchFreqPlan; 9]; 2] =
    [HW_SWITCH_FREQ_PLAN_DEFAULT, HW_SWITCH_FREQ_PLAN_SDRPLAY];
impl MirisdrDev {
    pub fn new() -> Self {
        let default_rate = 2000000;
        let default_freq = 7150000;
        let default_gain = 43;

        MirisdrDev {
            index: 0,
            freq: default_freq,
            rate: default_rate,
            gain: default_gain,
            gain_reduction_lna: 0,
            gain_reduction_mixbuffer: 0,
            gain_reduction_mixer: 0,
            gain_reduction_baseband: 43,
            hw_flavour: MirisdrHwFlavour::MirisdrHwDefault,
            band: MirisdBandT::MirisdrBandAm2,
            bandwidth: BandWidthEnum::MirisdrBw200khz,
            if_freq: IfFreqEnum::MirisdrIfZero,
            xtal: XtalEnum::MirisdrXtal22m,
        }
    }
    pub fn mirisdr_write_reg(val: u32) {
        let a: u8 = (val & 0xFF) as u8; // Extraer el primer byte
        let b: u8 = ((val >> 8) & 0xFF) as u8; // Extraer el segundo byte
        let c: u8 = ((val >> 16) & 0xFF) as u8; // Extraer el tercer byte
                                                /*
                                                escribe la manera de esribir los datos dentro de el msi001 */
    }
    pub fn mirisdr_set_soft(mut self) {
        let mut reg0: u32 = 0;
        let mut reg2: u32 = 2;
        let mut reg5: u32 = 5;
        let mut n: u64;
        let mut thresh: u64;
        let mut frac: u64;
        let mut lo_div: u64 = 0;
        let mut fvco: u64 = 0;
        let mut offset: u64 = 0;
        let mut a: u64;
        let mut b: u64;
        let mut c: u64;
        let mut i = 0;
        loop {
            if self.freq >= 1000000 * HW_SWITCH_FREQ_PLAN[self.hw_flavour as usize][i].low_cut {
                if HW_SWITCH_FREQ_PLAN[self.hw_flavour as usize][i].mode < 0 {
                    break;
                }

                i += 1
            } else {
                break;
            }
        }
        let switch_plan = &HW_SWITCH_FREQ_PLAN[self.hw_flavour as usize][i - 1];
        if switch_plan.mode == MIRISDR_MODE_AM {
            reg0 |= MIRISDR_MODE_AM << 4;
            reg0 |= switch_plan.upconvert_mixer_on << 9;
            reg0 |= switch_plan.am_port << 11;

            if switch_plan.upconvert_mixer_on == 1 {
                offset += 110000000 as u64;
            }
            lo_div = 16;

            if switch_plan.am_port == 0 {
                self.band = MirisdBandT::MirisdrBandAm1
            } else {
                self.band = MirisdBandT::MirisdrBandAm2
            }
        } else {
            reg0 |= switch_plan.mode << 4;
            lo_div = switch_plan.lo_div;
            match switch_plan.mode {
                MIRISDR_MODE_VHF => self.band = MirisdBandT::MirisdrBandVhf,
                MIRISDR_MODE_B3 => self.band = MirisdBandT::MirisdrBand3,
                MIRISDR_MODE_B45 => self.band = MirisdBandT::MirisdrBand45,
                MIRISDR_MODE_BL => self.band = MirisdBandT::MirisdrBandL,
                _ => (),
            }
        }
        /* RF synthesizer is always active */
        reg0 |= MIRISDR_RF_SYNTHESIZER_ON << 10;
        match self.if_freq {
            IfFreqEnum::MirisdrIfZero => reg0 |= MIRISDR_IF_MODE_ZERO << 12,
            IfFreqEnum::MirisdrIf450khz => reg0 |= MIRISDR_IF_MODE_450KHZ << 12,
            IfFreqEnum::MirisdrIf1620khz => reg0 |= MIRISDR_IF_MODE_1620KHZ << 12,
            IfFreqEnum::MirisdrIf2048khz => reg0 |= MIRISDR_IF_MODE_2048KHZ << 12,
            _ => (),
        }
        match self.bandwidth {
            BandWidthEnum::MirisdrBw200khz => reg0 |= 0x00 << 14,
            BandWidthEnum::MirisdrBw300khz => reg0 |= 0x01 << 14,
            BandWidthEnum::MirisdrBw600khz => reg0 |= 0x02 << 14,
            BandWidthEnum::MirisdrBw1536khz => reg0 |= 0x03 << 14,
            BandWidthEnum::MirisdrBw5mhz => reg0 |= 0x04 << 14,
            BandWidthEnum::MirisdrBw6mhz => reg0 |= 0x05 << 14,
            BandWidthEnum::MirisdrBw7mhz => reg0 |= 0x06 << 14,
            BandWidthEnum::MirisdrBw8mhz => reg0 |= 0x07 << 14,
            _ => (),
        }

        reg0 |= 0x02 << 17; // problema con el XTAL no sé por qué

        /* 4 bits for power saving modes */
        reg0 |= MIRISDR_IF_LPMODE_NORMAL << 20;
        reg0 |= MIRISDR_VCO_LPMODE_NORMAL << 23;

        /* VCO frequency is better to use a 64-bit range */
        fvco = (self.freq as u64 + offset) * lo_div;

        /* shift the main frequency */
        n = fvco / 88000000 as u64;

        /* major registry, coarse tuning */
        thresh = 88000000 as u64 / lo_div;

        /* side register, fine tuning */
        frac = (fvco % 88000000 as u64) / lo_div;

        /* We find the greatest common divisor for thresh and frac */
        a = thresh;
        b = frac;
        loop {
            if a != 0 {
                c = a;
                a = b % a;
                b = c;
            } else {
                break;
            }
        }

        thresh /= b;
        frac /= b;

        /* divided */
        thresh /= b;
        frac /= b;

        /* In this section we reduce the resolution to the maximum extent registry */
        a = (thresh + 4094) / 4095;
        thresh = (thresh + (a / 2)) / a;
        frac = (frac + (a / 2)) / a;

        reg5 |= ((0xFFF & thresh) << 4) as u32;

        /* Reserved, must be 0x28 */
        reg5 |= MIRISDR_RF_SYNTHESIZER_RESERVED_PROGRAMMING << 16;

        reg2 |= ((0xFFF & frac) << 4) as u32;
        reg2 |= ((0x3F & n) << 16) as u32;
        reg2 |= MIRISDR_LBAND_LNA_CALIBRATION_OFF << 22;

        /* kernel driver adjusts to changing frequencies  */

        Self::mirisdr_write_reg(0xf380);
        Self::mirisdr_write_reg(0x6280);
        Self::mirisdr_write_reg(switch_plan.band_select_word);
        Self::mirisdr_write_reg(0x0e); //OK
        Self::mirisdr_write_reg(0x03); //OK
        Self::mirisdr_write_reg(reg0);
        Self::mirisdr_write_reg(reg5);
        Self::mirisdr_write_reg(reg2);
    }
    pub fn mirisdr_set_center_freq(mut self, freq: u32) {
        self.freq = freq;
        Self::mirisdr_set_soft(self);
        Self::mirisdr_set_gain(self); // restore gain
    }
    pub fn mirisdr_set_gain(mut self) {
        let mut reg1: u32 = 1;
        let mut reg6: u32 = 6;

        reg1 |= (self.gain_reduction_baseband << 4) as u32;
        if self.band == MirisdBandT::MirisdrBandAm1 {
            reg1 |= ((self.gain_reduction_mixbuffer & 0x03) << 10) as u32;
        }
        if self.band == MirisdBandT::MirisdrBandAm2 {
            reg1 |= if self.gain_reduction_mixbuffer == 0 {
                0x0
            } else {
                0x03
            } << 10;
        } else {
            reg1 |= 0x0 << 10;
        }

        reg1 |= (self.gain_reduction_mixer << 12) as u32;

        // LNA is not on AM1 nor AM2 inputs
        if self.band == MirisdBandT::MirisdrBandAm1 || self.band == MirisdBandT::MirisdrBandAm2 {
            reg1 |= 0x0 << 13;
        } else {
            reg1 |= (self.gain_reduction_lna << 13) as u32;
        }

        reg1 |= (MIRISDR_DC_OFFSET_CALIBRATION_PERIODIC2 << 14) as u32;
        reg1 |= (MIRISDR_DC_OFFSET_CALIBRATION_SPEEDUP_OFF << 17) as u32;

        Self::mirisdr_write_reg(reg1);

        /* DC Offset Calibration setup */
        reg6 |= 0x1F << 4;
        reg6 |= 0x800 << 10;
        Self::mirisdr_write_reg(reg6);
    }
    pub fn mirisdr_set_if_freq(&mut self, freq: u32) {
        match freq {
            0 => self.if_freq = IfFreqEnum::MirisdrIfZero,
            450000 => self.if_freq = IfFreqEnum::MirisdrIf450khz,
            1620000 => self.if_freq = IfFreqEnum::MirisdrIf1620khz,
            2048000 => self.if_freq = IfFreqEnum::MirisdrIf2048khz,
            _ => (),
        }

        Self::mirisdr_set_soft(*self);
        Self::mirisdr_set_gain(*self); // restore gain
    }
    pub fn mirisdr_set_bandwidth(&mut self, bw: u32) {
        match bw {
            200000 => self.bandwidth = BandWidthEnum::MirisdrBw200khz,
            300000 => self.bandwidth = BandWidthEnum::MirisdrBw300khz,
            600000 => self.bandwidth = BandWidthEnum::MirisdrBw600khz,
            1536000 => self.bandwidth = BandWidthEnum::MirisdrBw1536khz,
            5000000 => self.bandwidth = BandWidthEnum::MirisdrBw5mhz,
            6000000 => self.bandwidth = BandWidthEnum::MirisdrBw6mhz,
            7000000 => self.bandwidth = BandWidthEnum::MirisdrBw7mhz,
            8000000 => self.bandwidth = BandWidthEnum::MirisdrBw8mhz,
            _ => {} // Manejo de caso por defecto
        }

        Self::mirisdr_set_soft(*self);
        Self::mirisdr_set_gain(*self); // restore gain
    }
    pub fn mirisdr_set_offset_tuning(&mut self, on: u8) {
        if on == 1 {
            self.if_freq = IfFreqEnum::MirisdrIf450khz
        } else {
            self.if_freq = IfFreqEnum::MirisdrIfZero
        }
        Self::mirisdr_set_soft(*self)
    }
    pub fn mirisdr_set_tuner_gain(&mut self, gain: i16) {
        self.gain = gain;
        if self.gain >= 102 {
            self.gain = 102
        }if self.gain <= 0{
            self.gain=0
        }
        /* Always the highest sensitivity without reducing the mixer and LNA */
        if self.gain >= 43 {
            self.gain_reduction_lna = 0;
            self.gain_reduction_mixbuffer = 0; // LNA equivalent for AM inputs
            self.gain_reduction_mixer = 0;
            self.gain_reduction_baseband = 59 - (self.gain - 43);
        } else if self.gain >= 19 {
            self.gain_reduction_lna = 1;
            self.gain_reduction_mixbuffer = 3; // LNA equivalent for AM inputs (AM1: 18dB / AM2: 24 dB)
            self.gain_reduction_mixer = 0;
            self.gain_reduction_baseband = 59 - (self.gain - 19);
        } else {
            self.gain_reduction_lna = 1;
            self.gain_reduction_mixbuffer = 3; // LNA equivalent for AM inputs (AM1: 18dB / AM2: 24 dB)
            self.gain_reduction_mixer = 1;
            self.gain_reduction_baseband = 59 - self.gain;
        }

        Self::mirisdr_set_gain(*self);
    }
    
}
