    /*
    * Described in detail in section B.2.3.4.
    *  Specifically, it contains type aliases for
    * data types used for various function sets 
    * (e.g. TimeType is a 64-bit integer or "Int64")
    */
use primitives;
use std::io::{Error, ErrorKind};
use bitflags::bitflags;

// Nice to do's:
/*
 * DstRuleType p166
 * GPSLocationType.lat and GPSLocationType.lon constructer that does error checking p167
 */

type DstRuleType = HexBinary32;

type LocaleType = String42;
type mRIDType = HexBinary128;
type PENType = UInt32;
type SFDIType = UInt40;
type TimeOffsetType = Int32;
type TimeType = Int64;
type VersionType = UInt16;

struct DateTimeInterval {
    duration: UInt16,
    start: TimeType,
}

struct GPSLocationType {
    lat: String32,
    lon: String32,
}

struct RealEnergy {
    multiplier: PowerOfTenMultiplierType,
    value: UInt48,
}

struct SignedRealEnergy {
    multiplier: PowerOfTenMultiplierType,
    value: Int48,
}

// for this and all the other tuple structs, the new() method should be used
// If only there was a way to enforce it as the only form of construction
// TODO: (optional) find way to enforce new() method for construction
#[derive(Debug)]
struct OneHourRangeType(Int16);

impl OneHourRangeType {
    fn new(value: Int16)->Result<OneHourRangeType, Error>{
        if value < -3600 || value > 3600 { Err(Error::from(ErrorKind::InvalidInput)) }
        else { Ok( OneHourRangeType( value )) }
    }
}

impl fmt::Display for OneHourRangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let OneHourRangeType(a) = self;
        write!(f, "{}", a)
    }
}

// measured in 1/100ths of a percent (eg. PerCent(102) is 1.02%)
#[derive(Debug)]
struct PerCent(UInt16);

impl PerCent {
    fn new(value: UInt16)->Result<PerCent, Error>{
        if value > 10000 { Err(Error::from(ErrorKind::InvalidInput)) }
        else { Ok( PerCent( value )) }
    }
}

impl fmt::Display for PerCent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PerCent(a) = self;
        write!(f, "{:.2}%", *a as f32/100.00)
    }
}

#[derive(Debug)]
struct SignedPerCent(Int16);

impl SignedPerCent {
    fn new(value: Int16)->Result<SignedPerCent, Error>{
        if value < -10000 || value > 10000 { Err(Error::from(ErrorKind::InvalidInput)) }
        else { Ok( SignedPerCent( value )) }
    }
}

impl fmt::Display for SignedPerCent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let SignedPerCent(a) = self;
        write!(f, "{:.2}%", *a as f32/100.00)
    }
}

#[derive(Debug)]
struct PINType(UInt32);

impl PINType {
    fn new(value: UInt32)->Result<PINType, Error>{
        if value > 999999 { Err(Error::from(ErrorKind::InvalidInput)) }
        else { Ok( PINType( value )) }
    }
}

impl fmt::Display for PINType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PINType(a) = self;
        write!(f, "{}", a)
    }
}

#[derive(Debug, Default)]
struct PowerOfTenMultiplierType(Int8);

impl PowerOfTenMultiplierType {
    fn new(value: Int8)->Result<PowerOfTenMultiplierType, Error>{
        if value < -9 || value > 9 { Err(Error::from(ErrorKind::InvalidInput)) }
        else { Ok( PowerOfTenMultiplierType( value )) }
    }
}

impl fmt::Display for PowerOfTenMultiplierType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let PowerOfTenMultiplierType(a) = self;
        write!(f, "x {}", a)
    }
}


/*
 * Values possible for indication of “Primary” provider:
 * 0 = In-home energy management system
 * 1 = Contracted premises service provider
 * 2 = Non-contractual service provider
 * 3 to 64 -= Reserved
 * 65 to 191 = User-defined
 */
// this implementation isn't particularly flexible. Luckily, this is extra and not 
// particularly requried. 
// TODO: (optional) make this better if you can.
#[derive(Debug)]
#[repr(UInt8)]
enum PrimacyType {
    HEMS = 0,
    CPSP = 1,
    NCSP = 2,
    User(u8),
}
 
impl PrimacyType {
    pub fn new(value: u8)->Result<PrimacyType, Error>{
        if (value > 2 && value < 65) || value > 191 { Err(Error::from(ErrorKind::InvalidInput)) }
        else {
            match value {
                0 => Ok(PrimacyType::HEMS),
                1 => Ok(PrimacyType::CPSP),
                2 => Ok(PrimacyType::NCSP),
                _ => Ok(PrimacyType::User(value)),
            }
        }
    }
    
    pub fn get_num_value(&self) -> u8 {
        match self {
            PrimacyType::HEMS => 0,
            PrimacyType::CPSP => 1,
            PrimacyType::NCSP => 2,
            PrimacyType::User(value) => *value,
        }        
    }
    
    pub fn format_meaning(&self) -> String {
        match self {
            PrimacyType::HEMS => format!("In-home energy management system: 0"),
            PrimacyType::CPSP => format!("Contracted premises service provider: 1"),
            PrimacyType::NCSP => format!("Non-contractual service provider: 2"),
            PrimacyType::User(value) => format!("User-defined: {}", value as &u8),
        }
    }
    
    pub fn print_meaning(&self){
        match self {
            PrimacyType::HEMS => println!("In-home energy management system: 0"),
            PrimacyType::CPSP => println!("Contracted premises service provider: 1"),
            PrimacyType::NCSP => println!("Non-contractual service provider: 2"),
            PrimacyType::User(value) => println!("User-defined: {}", value as &u8),
        }
    }
}


#[repr(UInt8)]
enum DataQualifierType {
    NotApplicable = 0,
    Average = 2,
    Maximum = 8,
    Minimum = 9,
    Normal = 12,
    Standard_deviation_ofPopulation = 29,
    Standard_deviation_ofSample = 30,
}


#[repr(UInt8)]
enum CommodityType {
    NotApplicable = 0,
    ElectricitySecondaryMetered = 1,
    ElectricityPrimaryMetered = 2,
    Air = 4,
    NaturalGas = 7,
    Propane = 8,
    PotableWater = 9,
    Steam = 10,
    WasteWater = 11,
    HeatingFluid = 12,
    CoolingFluid = 13,
}


#[repr(UInt8)]
enum FlowDirectionType {
    NotApplicable = 0,
    Forward = 1,
    Reverse = 19,
}


#[repr(UInt8)]
enum TOUType {
    NotApplicable = 0,
    TOU_A = 1,
    TOU_B = 2,
    TOU_C = 3,
    TOU_D = 4,
    TOU_E = 5,
    TOU_F = 6,
    TOU_G = 7,
    TOU_H = 8,
    TOU_I = 9,
    TOU_J = 10,
    TOU_K = 11,
    TOU_L = 12,
    TOU_M = 13,
    TOU_N = 14,
    TOU_O = 15,
}


#[repr(UInt8)]
enum  UnitType {
    kWh = 0,
    kW = 1,
    Watts = 2,
    CubicMeters = 3,
    CubicFeet = 4,
    USGallons = 5,
    ImperialGallons = 6,
    BTU = 7,
    Liters = 8,
    kPAGauge = 9,
    kPAAbsolute = 10,
    Megajoule = 11,
    Unitless = 12,
}

struct UnitValueType {
    multiplier: PowerOfTenMultiplierType,
    unit: UomType,
    value: Int48,
}

#[repr(UInt8)]
enum UomType {
    NotApplicable = 0,
    Amperes = 5,
    Kelvin = 6,
    DegreesCelsius = 23,
    Voltage = 29,
    Joule = 31,
    Hz = 33,
    W = 38,
    MtrCubed = 42,
    VA = 61,
    VAr = 63,
    CosTheta = 65,
    VSquared = 67,
    ASquared = 69,
    VAh = 71,
    Wh = 72,
    VArh = 73,
    Ah = 106,
    FtCubed = 119,
    FtCubedPerHour = 122,
    MCubedPerHour = 125,
    USGallons = 128,
    UGGallonsPerHour = 129,
    ImperialGallons = 130,
    ImperialGallonsPerHour = 131,
    BTU = 132,
    BTUPerHour = 133,
    Liter = 134,
    LiterPerHour = 137,
    PAGauge = 140,
    PAAbsolute = 155,
    Therm = 169,
}


#[repr(UInt8)]
enum AccumlationBehaviourType {
    NotApplicable = 0,
    Cumulative = 3,
    DeltaData = 4,
    Indicating = 6,
    Summation = 9,
    Instantaneous = 12,
}

// p162
#[repr(UInt8)]
enum ApplianceLoadReductionType{
    DelayApplianceLoad = 0,
    TemporaryApplianceLoadReduction = 1,
}

// p164
#[repr(UInt8)]
enum CurrencyCode{
    NotApplicable = 0,
    TemporaryApplianceLoadReduction = 1,
}


#[repr(UInt8)]
enum ServiceKind {
    Electricity = 0,
    Gas = 1,
    Water = 2,
    Time = 3,
    Pressure = 4,
    Heat = 5,
    Cooling = 6,
}


#[repr(UInt8)]
enum ConsumptionBlockType {
    NotApplicable = 0,
    Block_1 = 1,
    Block_2 = 2,
    Block_3 = 3,
    Block_4 = 4,
    Block_5 = 5,
    Block_6 = 6,
    Block_7 = 7,
    Block_8 = 8,
    Block_9 = 9,
    Block_10 = 10,
    Block_11 = 11,
    Block_12 = 12,
    Block_13 = 13,
    Block_14 = 14,
    Block_15 = 15,
    Block_16 = 16,
}


// «XSDsimpleType»
#[repr(UInt8)]
enum SubscribableType {
    NoSubscriptionsSupported = 0,
    NonConditionalSubscriptions = 1,
    ConditionalSubscriptions = 2,
    AllSubscriptions = 3,
}


#[repr(UInt8)]
enum DERCurveType {
    OpModFreqWatt = 0,
    OpModHFRTMayTrip = 1,
    OpModHFRTMustTrip = 2,
    OpModHVRTMayTrip = 3,
    OpModHVRTMomentaryCessation = 4,
    OpModHVRTMustTrip = 5,
    OpModLFRTMayTrip = 6,
    OpModLFRTMustTrip = 7,
    OpModLVRTMayTrip = 8,
    OpModLVRTMomentaryCessation = 9,
    OpModLVRTMustTrip = 10,
    OpModVoltVar = 11,
    OpModVoltWatt = 12,
    OpModWattPF = 13,
    OpModWattVar = 14,
}

#[repr(UInt8)]
enum KindType {
    NotApplicable = 0,
    Currency = 3,
    Demand = 8,
    Energy = 12,
    Power = 37,
}

#[repr(UInt8)]
enum PhaseCode {
    NotApplicable = 0,
    Phase_C = 32, // and S2
    Phase_CN = 33, // and S2N
    Phase_CA = 40,
    Phase_B = 64,
    Phase_BN = 65,
    Phase_BC = 66,
    Phase_A = 128, // and S1
    Phase_AN = 129, // and S1N
    Phase_AB = 132,
    Phase_ABC = 224,
}


bitflags! {
    struct RoleFlagsType: HexBinary16 {
        IsMirror = 1;
        IsPremiseAggregationPoint = 2;
        IsPEV = 4;
        IsDER = 8;
        IsRevenueQuality = 16;
        IsDC = 32;
        IsSubmeter = 64;
    }
}

bitflags! {
    struct DeviceCategoryType{
        const ProgrammableCommunicatingThermostat = 1;
        const StripHeaters = 2;
        const BaseboardHeaters = 4;
        const WaterHeater = 8;
        const PoolPump = 16;
        const Sauna = 32;
        const HotTub = 64;
        const SmartAppliance = 128;
        const IrrigationPump = 256;
        const ManagedCommercialAndIndustrialLoads = 512;
        const SimpleMiscLoads = 1024;
        const ExteriorLighting = 2048;
        const InteriorLighting = 4096;
        const LoadControlSwitch = 8192;
        const Energy_managementSystem = 16384;
        const Smart_energy_module = 65536;
        const ElectricVehicle = 262144;
        const VirutalOrMixedDer = 524288;
        const ReciprocatingEngine = 2097152;
        const PhotovoltaicSystem = 8388608;
        const CombinedPvAndStorage = 16777216;
        const OtherGenerationSystem = 33554432;
        const OtherStorageSystem = 67108864;
    }
}

bitflags! {
    struct RoleFlagsType{
        const IsMirror = 1;
        const IsPremiseAggregationPoint = 2;
        const IsPEV = 4;
        const IsDER = 8;
        const IsRevenueQuality = 16;
        const IsDC = 32;
        const IsSubmeter = 64;
    }    
}

bitflags! {
    struct DERControlType{
        const ChargeMode = 1;
        const DischargeMode = 2;
        const OpModConnect = 4;
        const OpModEnergize = 8;
        const OpModFixedPFAbsorbW = 16;
        const OpModFixedPFInjectW = 32;
        const OpModFixedVar = 64;
        const OpModFixedW = 128;
        const OpModFreqDroop = 256;
        const OpModFreqWatt = 512;
        const OpModHFRTMayTrip = 1024;
        const OpModHFRTMustTrip = 2048;
        const OpModHVRTMayTrip = 4096;
        const OpModHVRTMomentaryCessation = 8192;
        const OpModHVRTMustTrip = 16384;
        const OpModLFRTMayTrip = 32768;
    }
}
