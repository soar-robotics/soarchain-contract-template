#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Data {
    pub dataInfo: DataInfo,
    pub sign: String,
    pub pubkey: String
}

pub struct DataInfo {
    pub data: DataDetails
}

pub struct DataDetails {
    pub accelerometer: GeographicInfo, 
    pub gyroscope: GeographicInfo,
    pub magnetometer: GeographicInfo,
    pub location: LocationInfo,
    pub trip: String,
    pub contract: String, 
    pub vehicle_info: VehicleInfo
}

pub struct GeographicInfo {
    x: u64,
    y: u64,
    z: u64
}

pub struct LocationInfo {
    lat: u64,
    lng: u64
}

pub struct VehicleInfo {
    load_pct: u64,
    temp: u64,
    rpm: u64,
    vss: u64,
    iat:u64,
    maf: u64, 
    throttlepo: u64,
    runtm: u64, 
    fli: u64,
    baro: u64,
    load_abs: u64,
    fuel_rate: u64,
    odometer: u64
}


