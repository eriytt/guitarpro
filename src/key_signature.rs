use fraction::ToPrimitive;
use crate::io::*;

pub const DURATION_QUARTER_TIME: i64 = 960;
pub const DURATION_WHOLE: u8 = 1;
pub const DURATION_HALF: u8 = 2;
pub const DURATION_QUARTER: u8 = 4;
pub const DURATION_EIGHTH: u8 = 8;
pub const DURATION_SIXTEENTH: u8 = 16;
pub const DURATION_TWENTY_FOURTH: u8 = 24;
pub const DURATION_THIRTY_SECOND: u8 = 32;
pub const DURATION_SIXTY_FOURTH: u8 = 64;

/// A time signature
#[derive(Clone)]
pub struct TimeSignature {
    pub numerator: i8,
    pub denominator: Duration,
    pub beams: Vec<i32>,
}
impl Default for TimeSignature {
    fn default() -> Self { TimeSignature { numerator: 4, denominator:Duration::default(), beams: vec![2,2,2,2]}}
}

#[derive(Clone,PartialEq)]
pub struct Duration {
    pub value:u8,
    pub dotted: bool,
    pub double_dotted:bool,
    /// The time resulting with a 64th note and a 3/2 tuplet
    pub min_time: u8,
    //Tuplet division type
    pub tuplet_enters:u8, pub tuplet_times:u8
}
impl Duration {
    //fn convert_time(&self, time: u64) -> u64 { return time * self.division_times as u64 / self.division_enters as u64; }

    // /Read beat duration.
    /// Duration is composed of byte signifying duration and an integer that maps to `Tuplet`. The byte maps to following values:
    /// 
    /// * *-2*: whole note
    /// * *-1*: half note
    /// * *0*: quarter note
    /// * *1*: eighth note
    /// * *2*: sixteenth note
    /// * *3*: thirty-second note
    /// 
    /// If flag at *0x20* is true, the tuplet is read
    pub fn read(data: &Vec<u8>, seek: &mut usize, flags: u8) -> Duration {
        let mut d = Duration::default();
        d.value = 1 << (read_signed_byte(data, seek) + 2);
        d.dotted = (flags & 0x01) == 0x01;
        if (flags & 0x20) == 0x20 {
            let i_tuplet = read_int(data, seek);
            if i_tuplet == 3       {d.tuplet_enters = 3;  d.tuplet_times = 2;}
            else if i_tuplet == 5  {d.tuplet_enters = 5;  d.tuplet_times = 4;}
            else if i_tuplet == 6  {d.tuplet_enters = 6;  d.tuplet_times = 4;}
            else if i_tuplet == 7  {d.tuplet_enters = 7;  d.tuplet_times = 4;}
            else if i_tuplet == 9  {d.tuplet_enters = 9;  d.tuplet_times = 8;}
            else if i_tuplet == 10 {d.tuplet_enters = 10; d.tuplet_times = 8;}
            else if i_tuplet == 11 {d.tuplet_enters = 11; d.tuplet_times = 8;}
            else if i_tuplet == 12 {d.tuplet_enters = 12; d.tuplet_times = 8;}
            else if i_tuplet == 13 {d.tuplet_enters = 13; d.tuplet_times = 8;}
        }
        return d;
    }

    pub fn is_supported(&self) -> bool { return SUPPORTED_TUPLETS.contains(&(self.tuplet_enters, self.tuplet_times)); }

    pub fn convert_time(&self, time: u8) -> u8 {
        let result = fraction::Fraction::new(time * self.tuplet_enters, self.tuplet_times);
        if result.denom().expect("Cannot get fraction denominator") == &1 {1}
        else {result.to_u8().expect("Cannot get fraction result")}
    }

    pub fn time(&self) -> u8 {
        let mut result = (f64::from(DURATION_QUARTER_TIME as i32) * 4f64 / f64::from(self.value)).trunc();
        if self.dotted { result += (result/2f64).trunc(); }
        return self.convert_time(result as u8);
    }

    pub fn index(&self) -> u8 {
        let mut index = 0u8;
        let mut value = self.value;
        loop {
            value = value >> 1;
            if value > 0 {index += 1;}
            else {break;}
        }
        return index
    }
    //@classmethod def fromFraction(cls, frac): return cls(frac.denominator, frac.numerator)
}
impl Default for Duration {
    fn default() -> Self { Duration {
        value: DURATION_QUARTER, dotted: false, double_dotted: false,
        tuplet_enters:1, tuplet_times:1,
        min_time: 0
    }}
}

/*/// A *n:m* tuplet.
#[derive(Clone)]
struct Tuplet {
    enters: u8,
    times: u8,
}*/
const SUPPORTED_TUPLETS: [(u8, u8); 10] = [(1,1), (3,2), (5,4), (6,4), (7,4), (9,8), (10,8), (11,8), (12,8), (13,8)];
/*impl Default for Tuplet { fn default() -> Self { Tuplet { enters: 1, times: 1 }}}
impl Tuplet {
    fn is_supported(self) -> bool { return SUPPORTED_TUPLETS.contains(&(self.enters, self.times)); }
    fn convert_time(self) -> u8 {
        let result = fraction::Fraction::new(self.enters, self.times);
        if result.denom().expect("Cannot get fraction denominator") == &1 {1}
        else {result.to_u8().expect("Cannot get fraction result")}
    }
}*/

/* Enum ?: const KEY_F_MAJOR_FLAT: (i8, bool) = (-8, false);
const KEY_C_MAJOR_FLAT: (i8, bool) = (-7, false);
const KEY_G_MAJOR_FLAT: (i8, bool) = (-6, false);
const KEY_D_MAJOR_FLAT: (i8, bool) = (-5, false);
const KEY_A_MAJOR_FLAT: (i8, bool) = (-4, false);
const KEY_E_MAJOR_FLAT: (i8, bool) = (-3, false);
const KEY_B_MAJOR_FLAT: (i8, bool) = (-2, false);
const KEY_F_MAJOR: (i8, bool) = (-1, false);
const KEY_C_MAJOR: (i8, bool) = (0, false);
const KEY_G_MAJOR: (i8, bool) = (1, false);
const KEY_D_MAJOR: (i8, bool) = (2, false);
const KEY_A_MAJOR: (i8, bool) = (3, false);
const KEY_E_MAJOR: (i8, bool) = (4, false);
const KEY_B_MAJOR: (i8, bool) = (5, false);
const KEY_F_MAJOR_SHARP: (i8, bool) = (6, false);
const KEY_C_MAJOR_SHARP: (i8, bool) = (7, false);
const KEY_G_MAJOR_SHARP: (i8, bool) = (8, false);
const KEY_D_MINOR_FLAT: (i8, bool) = (-8, true);
const KEY_A_MINOR_FLAT: (i8, bool) = (-7, true);
const KEY_E_MINOR_FLAT: (i8, bool) = (-6, true);
const KEY_B_MINOR_FLAT: (i8, bool) = (-5, true);
const KEY_F_MINOR: (i8, bool) = (-4, true);
const KEY_C_MINOR: (i8, bool) = (-3, true);
const KEY_G_MINOR: (i8, bool) = (-2, true);
const KEY_D_MINOR: (i8, bool) = (-1, true);
const KEY_A_MINOR: (i8, bool) = (0, true);
const KEY_E_MINOR: (i8, bool) = (1, true);
const KEY_B_MINOR: (i8, bool) = (2, true);
const KEY_F_MINOR_SHARP: (i8, bool) = (3, true);
const KEY_C_MINOR_SHARP: (i8, bool) = (4, true);
const KEY_G_MINOR_SHARP: (i8, bool) = (5, true);
const KEY_D_MINOR_SHARP: (i8, bool) = (6, true);
const KEY_A_MINOR_SHARP: (i8, bool) = (7, true);
const KEY_E_MINOR_SHARP: (i8, bool) = (8, true);*/

pub const KEY_SIGNATURES: [&'static str; 34] = ["F♭ major", "C♭ major", "G♭ major", "D♭ major", "A♭ major", "E♭ major", "B♭ major",
            "F major", "C major", "G major", "D major", "A major", "E major", "B major",
            "F# major", "C# major", "G# major",
            "D♭ minor", "A♭ minor", "E♭ minor", "B♭ minor",
            "F minor", "C minor", "G minor", "D minor", "A minor", "E minor", "B minor",
            "F# minor", "C# minor", "G# minor", "D# minor", "A# minor", "E# minor"];

#[derive(Clone)]
pub struct KeySignature {
    pub key: i8,
    pub is_minor: bool,
}
impl Default for KeySignature { fn default() -> Self { KeySignature { key: 0, is_minor: false, }} }
impl KeySignature {
    pub fn to_string(&self) -> String {
        let index: usize = if self.is_minor {(23i8 + self.key) as usize} else {(8i8 + self.key) as usize};
        return String::from(KEY_SIGNATURES[index]);
    }
}